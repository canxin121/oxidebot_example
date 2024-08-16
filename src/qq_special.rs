use anyhow::Result;
use onebot_v11::api::payload::SendLike;
use onebot_v11_oxidebot::OnebotV11ReverseWsBot;
use oxidebot::{handler::Handler, source::message::MessageSegment, EventHandlerTrait};
pub struct QQSpecialHandler;
impl Into<Handler> for QQSpecialHandler {
    fn into(self) -> Handler {
        Handler {
            event_handler: Some(Box::new(self)),
            active_handler: None,
        }
    }
}

impl EventHandlerTrait for QQSpecialHandler {
    fn handle<'life0, 'async_trait>(
        &'life0 self,
        matcher: oxidebot::matcher::Matcher,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let Some(message) = matcher.try_get_message() {
                if message.starts_with_text("/like") {
                    let user_id = &matcher.try_get_user().ok_or(anyhow::anyhow!("no user"))?.id;

                    let bot = matcher
                        .bot
                        .as_any()
                        .downcast_ref::<OnebotV11ReverseWsBot>()
                        .ok_or(anyhow::anyhow!("downcast failed"))?;
                    bot.call_api(onebot_v11::api::payload::ApiPayload::SendLike(SendLike {
                        user_id: user_id.parse::<i64>()?,
                        times: 1,
                    }))
                    .await?;

                    matcher
                        .try_send_message(vec![
                            MessageSegment::text("Sent 1 like to you."),
                            MessageSegment::reply(message.id.clone()),
                        ])
                        .await?;
                }
            }
            Ok(())
        })
    }
}
