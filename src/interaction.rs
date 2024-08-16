use std::sync::Arc;

use dashmap::DashMap;
use oxidebot::{
    handler::Handler,
    matcher::Matcher,
    source::message::MessageSegment,
    utils::interaction::{Interaction, InteractionTrait},
    wait_for_input_generic, EventHandlerTrait,
};

pub struct HelloInteraction {
    pub times: DashMap<String, u64>,
}

impl HelloInteraction {
    pub fn new() -> Self {
        Self {
            times: DashMap::new(),
        }
    }
}

impl InteractionTrait for HelloInteraction {
    fn handle_interaction<'a, 'async_trait>(
        &'a self,
        matcher: Matcher,
        mut receiver: tokio::sync::mpsc::Receiver<oxidebot::matcher::Matcher>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = anyhow::Result<()>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'a: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            matcher
                .try_send_message(vec![MessageSegment::text("What's your name?")])
                .await?;
            let (name, matcher) = wait_for_input_generic!(&mut receiver)?;
            matcher
                .try_send_message(vec![MessageSegment::text("How old are you?")])
                .await?;
            let (age, matcher) = wait_for_input_generic!(
                &mut receiver,
                u8,
                "Input error, please enter a number (0-255)"
            )?;
            let time = {
                let mut time = self.times.entry(name.clone()).or_insert(0);
                *time += 1;
                time.clone()
            };

            matcher
                .try_send_message(vec![MessageSegment::text(format!(
                    "({time})Hello, {name}, you are {age} years old",
                ))])
                .await?;
            Ok(())
        })
    }

    fn should_start<'life0, 'async_trait>(
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
            matcher
                .try_get_message()
                .and_then(|m| Some(m.get_raw_text().starts_with("/hello")))
                .unwrap_or(false)
        })
    }
}

pub struct InteractionHandler {
    pub hello_interaction: Arc<Interaction<HelloInteraction>>,
}

impl Into<Handler> for InteractionHandler {
    fn into(self) -> Handler {
        Handler {
            event_handler: Some(Box::new(self)),
            active_handler: None,
        }
    }
}

impl InteractionHandler {
    pub fn new() -> Self {
        Self {
            hello_interaction: Interaction::<HelloInteraction>::new(HelloInteraction::new()),
        }
    }
}

impl EventHandlerTrait for InteractionHandler {
    fn handle<'life0, 'async_trait>(
        &'life0 self,
        matcher: Matcher,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = anyhow::Result<()>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let Err(e) = self.hello_interaction.clone().interact(&matcher) {
                tracing::error!("HelloInteraction error: {:?}", e);
            }
            Ok(())
        })
    }
}
