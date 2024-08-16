use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use oxidebot::{
    event::MessageEvent, handler::Handler, matcher::Matcher, source::message::MessageSegment,
    BotObject, EventHandlerTrait,
};
use sqlx::sqlite::SqlitePool;
use sqlx::{query, query_as};
use tokio::task::JoinHandle;

#[derive(Clone)]
pub struct SetMessageEventReactionHandler {
    pool: Arc<SqlitePool>,
}

impl Into<Handler> for SetMessageEventReactionHandler {
    fn into(self) -> Handler {
        Handler {
            event_handler: Some(Box::new(self)),
            active_handler: None,
        }
    }
}

impl SetMessageEventReactionHandler {
    pub async fn new(file_path: &str) -> SetMessageEventReactionHandler {
        let pool = SqlitePool::connect(file_path)
            .await
            .expect("Failed to create pool");

        if !Path::new(file_path).exists() {
            query(
                "CREATE TABLE reactions (
                    user_id TEXT PRIMARY KEY,
                    reaction_type TEXT NOT NULL
                )",
            )
            .execute(&pool)
            .await
            .expect("Failed to create table");
        }
        SetMessageEventReactionHandler {
            pool: Arc::new(pool),
        }
    }

    pub fn handle_matcher(&self, matcher: Matcher) {
        let self_clone = self.clone();
        tokio::spawn(async move {
            match matcher.event.as_ref() {
                oxidebot::event::Event::MessageEvent(event) => {
                    let text = Arc::new(event.message.get_raw_text());
                    let user_id = Arc::new(event.sender.id.clone());
                    self_clone.handle_add_target(text.clone(), user_id.clone(), matcher.clone());
                    self_clone.handle_delete_target(text, user_id, matcher.clone());
                    self_clone.handle_reaction(matcher.bot.clone(), event).await;
                }
                _ => {}
            }
        });
    }

    async fn add_target(&self, user_id: &str) -> Result<()> {
        query("INSERT INTO reactions (user_id, reaction_type) VALUES (?, ?)")
            .bind(user_id)
            .bind("like")
            .execute(self.pool.as_ref())
            .await?;
        Ok(())
    }

    async fn remove_target(&self, user_id: &str) -> Result<()> {
        query("DELETE FROM reactions WHERE user_id = ? AND reaction_type = 'like'")
            .bind(user_id)
            .execute(self.pool.as_ref())
            .await
            .expect("Failed to remove target");
        Ok(())
    }

    async fn check_target(&self, user_id: &str) -> bool {
        let count: (i64,) =
            query_as("SELECT COUNT(*) FROM reactions WHERE user_id = ? AND reaction_type = 'like'")
                .bind(user_id)
                .fetch_one(self.pool.as_ref())
                .await
                .expect("Failed to check target");
        count.0 > 0
    }

    pub fn handle_add_target(&self, text: Arc<String>, user_id: Arc<String>, matcher: Matcher) {
        let self_clone = self.clone();
        let _: JoinHandle<Result<()>> = tokio::spawn(async move {
            if text.starts_with("/set action") {
                self_clone.add_target(&user_id).await?;
                if let Err(e) = matcher
                    .try_reply_message(vec![MessageSegment::text("Set reaction to your message")])
                    .await
                {
                    tracing::error!("send message failed: {:?}", e);
                }
            }
            Ok(())
        });
    }

    pub fn handle_delete_target(&self, text: Arc<String>, user_id: Arc<String>, matcher: Matcher) {
        let self_clone = self.clone();
        let _: JoinHandle<Result<()>> = tokio::spawn(async move {
            if text.starts_with("/cancel action") {
                self_clone.remove_target(user_id.as_str()).await?;
                if let Err(e) = matcher
                    .try_reply_message(vec![MessageSegment::text("Cancel reaction to your message")])
                    .await
                {
                    tracing::error!("send message failed: {:?}", e);
                }
            }
            Ok(())
        });
    }

    pub async fn handle_reaction(&self, bot: BotObject, event: &MessageEvent) {
        if self.check_target(&event.sender.id).await {
            match event.set_reactions(bot, vec!["201".to_string()]).await {
                Ok(_) => {}
                Err(e) => {
                    tracing::error!("set reaction failed: {:?}", e);
                }
            }
        }
    }
}

impl EventHandlerTrait for SetMessageEventReactionHandler {
    #[must_use]
    #[allow(clippy::type_complexity, clippy::type_repetition_in_bounds)]
    fn handle<'life0, 'async_trait>(
        &'life0 self,
        matcher: Matcher,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            self.handle_matcher(matcher);
            Ok(())
        })
    }
}
