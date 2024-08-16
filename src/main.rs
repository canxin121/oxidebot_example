use onebot_v11_oxidebot::OnebotV11ReverseWsBot;

use oxidebot_example::{
    echo::EchoHandler, filer::MessageEventFilter, interaction::InteractionHandler,
    qq_special::QQSpecialHandler, schedule::ScheduleHandler,
    set_message_reaction::SetMessageEventReactionHandler,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let manager = oxidebot::OxideBotManager::new()
        .bot(OnebotV11ReverseWsBot::new(Default::default()).await)
        .await
        // A pre-event filter for globally controlling event handling
        .filter(MessageEventFilter)
        // /echo repeat message
        .handler(EchoHandler)
        // A simple plugin to add responses to your messages
        .handler(SetMessageEventReactionHandler::new("./set_msg_reaction.db").await)
        // Plugin for sending scheduled messages
        .handler(ScheduleHandler)
        // QQ special function, friend praise
        .handler(QQSpecialHandler)
        // An example interactive plugin
        .handler(InteractionHandler::new());

    manager.run_block().await;
}
