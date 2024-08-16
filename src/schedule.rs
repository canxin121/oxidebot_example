use anyhow::Result;
use oxidebot::{
    api::payload::SendMessageTarget,
    bot::get_bot,
    handler::{ActiveHandlerTrait, Handler},
    source::message::MessageSegment,
};
use tokio_schedule::{every, Job};

pub struct ScheduleHandler;

impl Into<Handler> for ScheduleHandler {
    fn into(self) -> Handler {
        Handler {
            active_handler: Some(Box::new(self)),
            ..Default::default()
        }
    }
}

impl ActiveHandlerTrait for ScheduleHandler {
    fn run_forever<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<dyn ::core::future::Future<Output = Result<()>> + ::core::marker::Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async {
            async fn send_message() -> Result<()> {
                let bot = get_bot("123456789")
                    .await
                    .ok_or(anyhow::anyhow!("get bot failed"))?;
                bot.send_message(
                    vec![MessageSegment::text("Tik")],
                    SendMessageTarget::Private("123456789".into()),
                )
                .await
                .map_err(|e| anyhow::anyhow!("send message failed: {:?}", e))?;
                Ok(())
            }
            every(10)
                .minute()
                .at(30)
                .perform(|| async {
                    if let Err(e) = send_message().await {
                        tracing::error!("schedule send message failed: {:?}", e);
                    }
                })
                .await;
            Ok(())
        })
    }
}
