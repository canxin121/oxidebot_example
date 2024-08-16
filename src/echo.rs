use anyhow::Result;
use oxidebot::{handler::Handler, matcher::Matcher, EventHandlerTrait};

// Echo the message that starts with "/echo"
pub struct EchoHandler;

impl Into<Handler> for EchoHandler {
    fn into(self) -> Handler {
        Handler {
            event_handler: Some(Box::new(self)),
            active_handler: None,
        }
    }
}

impl EventHandlerTrait for EchoHandler {
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
                if message.starts_with_text("/echo") {
                    matcher
                        .try_send_message(message.trim_head_text("/echo"))
                        .await?;
                }
            }
            Ok(())
        })
    }
}
