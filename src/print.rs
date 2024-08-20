use anyhow::Result;
use oxidebot::{handler::Handler, matcher::Matcher, EventHandlerTrait};

pub struct PrintHandler;

impl Into<Handler> for PrintHandler {
    fn into(self) -> Handler {
        Handler {
            event_handler: Some(Box::new(self)),
            active_handler: None,
        }
    }
}

impl EventHandlerTrait for PrintHandler {
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
            tracing::info!("{:#?}", matcher.event);
            Ok(())
        })
    }
}
