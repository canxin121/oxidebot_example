use std::time::Duration;

use anyhow::Result;
use oxidebot::{
    handler::Handler,
    manager::BroadcastSender,
    matcher::Matcher,
    source::message::MessageSegment,
    utils::wait::{wait_user_text_generic, EasyBool},
    EventHandlerTrait,
};

pub struct WaitHandler {
    pub broadcast_sender: BroadcastSender,
}

impl WaitHandler {
    pub fn new(broadcast_sender: BroadcastSender) -> Self {
        Self { broadcast_sender }
    }
}

impl Into<Handler> for WaitHandler {
    fn into(self) -> Handler {
        Handler {
            event_handler: Some(Box::new(self)),
            active_handler: None,
        }
    }
}

impl EventHandlerTrait for WaitHandler {
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
            if let Some(message) = matcher.try_get_message() {
                if message.starts_with_text("/wait") {
                    matcher
                        .try_send_message(vec![MessageSegment::text("Please send a unsigned int8")])
                        .await?;
                    let (number, matcher) = wait_user_text_generic::<u8>(
                        &matcher,
                        &self.broadcast_sender,
                        Duration::from_secs(30),
                        3,
                        Some("Please send a unsigned int8".to_string()),
                    )
                    .await?;

                    matcher
                        .try_reply_message(vec![MessageSegment::text(format!(
                            "You sent a number: {}",
                            number
                        ))])
                        .await?;

                    matcher
                        .try_send_message(vec![MessageSegment::text("Please send a bool")])
                        .await?;

                    let (easy_bool, matcher) = wait_user_text_generic::<EasyBool>(
                        &matcher,
                        &self.broadcast_sender,
                        Duration::from_secs(30),
                        1,
                        Some("Please send a bool".to_string()),
                    )
                    .await?;

                    if easy_bool.into() {
                        matcher
                            .try_reply_message(vec![MessageSegment::text("You confirmed")])
                            .await?;
                    } else {
                        matcher
                            .try_reply_message(vec![MessageSegment::text("You rejected")])
                            .await?;
                    }
                }
            }
            Ok(())
        })
    }
}
