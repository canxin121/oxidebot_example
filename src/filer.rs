use oxidebot::filter::{FilterObject, FilterTrait};

// A Filter that filters out all events except MessageEvent
pub struct MessageEventFilter;

impl Into<FilterObject> for MessageEventFilter {
    fn into(self) -> FilterObject {
        Box::new(self)
    }
}
impl FilterTrait for MessageEventFilter {
    fn filter<'life0, 'async_trait>(
        &'life0 self,
        matcher: oxidebot::matcher::Matcher,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = bool> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            match matcher.event.as_ref() {
                oxidebot::event::Event::MessageEvent(_) => true,
                _ => false,
            }
        })
    }

    fn get_priority(&self) -> u8 {
        0
    }
}
