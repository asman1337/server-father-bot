use teloxide::{
    prelude::*,
    utils::command::BotCommands,
};
use tracing::info;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Server Father commands:")]
pub enum Command {
    #[command(description = "Start the bot")]
    Start,
    #[command(description = "Add a new server")]
    AddServer,
    #[command(description = "Remove a server")]
    RemoveServer,
    #[command(description = "Create a new server group")]
    CreateGroup,
    #[command(description = "Check server status")]
    Check,
    #[command(description = "Set check interval")]
    SetInterval,
    #[command(description = "View all servers status")]
    Status,
}

pub async fn command_handler(bot: Bot) -> anyhow::Result<()> {
    let handler = Update::filter_message()
        .filter_command::<Command>()
        .branch(
            dptree::entry()
                .filter_command::<Command>()
                .endpoint(answer),
        );

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => {
            bot.send_message(
                msg.chat.id,
                "👋 Welcome to Server Father!\nI'll help you monitor your servers.",
            )
            .await?;
        }
        Command::AddServer => {
            bot.send_message(
                msg.chat.id,
                "Adding server functionality coming soon...",
            )
            .await?;
        }
        // Add other command handlers...
        _ => {
            bot.send_message(
                msg.chat.id,
                "This command is under development.",
            )
            .await?;
        }
    }
    Ok(())
} 