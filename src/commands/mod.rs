use teloxide::{
    prelude::*,
    utils::command::BotCommands,
    dispatching::{
        dialogue::{InMemStorage, Dialogue},
        UpdateHandler,
        HandlerExt,
    },
};
use std::sync::Arc;
use crate::bot::ServerFatherBot;

type MyDialogue = Dialogue<State, InMemStorage<State>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    AwaitingServerName {
        host: String,
        port: i32,
    },
    AwaitingServerHost,
    AwaitingServerPort {
        name: String,
    },
    AwaitingServerId,
}

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
    #[command(description = "Start monitoring servers")]
    Monitor,
}

pub fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(
            case![State::Start]
                .branch(case![Command::Start].endpoint(start))
                .branch(case![Command::AddServer].endpoint(add_server))
                .branch(case![Command::Status].endpoint(status))
                .branch(case![Command::Monitor].endpoint(start_monitoring))
                .branch(case![Command::RemoveServer].endpoint(remove_server))
                .branch(case![Command::Check].endpoint(check_server)),
        );

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::AwaitingServerHost].endpoint(receive_host))
        .branch(case![State::AwaitingServerPort { name }].endpoint(receive_port))
        .branch(case![State::AwaitingServerName { host, port }].endpoint(receive_name));

    message_handler.endpoint(invalid_state)
}

async fn start(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(
        msg.chat.id,
        "👋 Welcome to Server Father!\nI'll help you monitor your servers.",
    )
    .await?;
    Ok(())
}

async fn add_server(bot: Bot, dialogue: MyDialogue, msg: Message) -> ResponseResult<()> {
    dialogue.update(State::AwaitingServerHost).await?;
    
    bot.send_message(
        msg.chat.id,
        "Please enter the server host (IP or domain):",
    )
    .await?;
    
    Ok(())
}

async fn receive_host(bot: Bot, dialogue: MyDialogue, msg: Message) -> ResponseResult<()> {
    let host = msg.text().unwrap_or_default().to_string();
    
    dialogue
        .update(State::AwaitingServerPort { name: host })
        .await?;
    
    bot.send_message(msg.chat.id, "Please enter the port number:")
        .await?;
    
    Ok(())
}

async fn receive_port(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    name: String,
) -> ResponseResult<()> {
    let port = msg.text()
        .and_then(|text| text.parse::<i32>().ok())
        .unwrap_or(0);

    if port <= 0 || port > 65535 {
        bot.send_message(
            msg.chat.id,
            "Invalid port number. Please enter a number between 1 and 65535:",
        )
        .await?;
        return Ok(());
    }

    dialogue
        .update(State::AwaitingServerName { host: name, port })
        .await?;

    bot.send_message(msg.chat.id, "Please enter a name for this server:")
        .await?;

    Ok(())
}

async fn receive_name(
    bot: Bot,
    dialogue: MyDialogue,
    server_father: Arc<ServerFatherBot>,
    msg: Message,
    host: String,
    port: i32,
) -> ResponseResult<()> {
    let name = msg.text().unwrap_or_default().to_string();

    match server_father.server_service().add_server(name.clone(), host.clone(), port, None).await {
        Ok(_) => {
            bot.send_message(
                msg.chat.id,
                format!("✅ Server '{}' ({}) added successfully!", name, host),
            )
            .await?;
        }
        Err(e) => {
            bot.send_message(
                msg.chat.id,
                format!("❌ Failed to add server: {}", e),
            )
            .await?;
        }
    }

    dialogue.update(State::Start).await?;
    Ok(())
}

async fn status(
    bot: Bot,
    server_father: Arc<ServerFatherBot>,
    msg: Message,
) -> ResponseResult<()> {
    let servers = match server_father.server_service().list_servers().await {
        Ok(servers) => servers,
        Err(e) => {
            bot.send_message(
                msg.chat.id,
                format!("❌ Failed to fetch servers: {}", e),
            )
            .await?;
            return Ok(());
        }
    };

    if servers.is_empty() {
        bot.send_message(
            msg.chat.id,
            "No servers added yet. Use /addserver to add one.",
        )
        .await?;
        return Ok(());
    }

    let mut status_message = String::from("📊 *Server Status*\n\n");
    
    for server in servers {
        let is_up = server_father
            .check_server_status(&server)
            .await
            .unwrap_or(false);

        let status_emoji = if is_up { "🟢" } else { "🔴" };
        
        status_message.push_str(&format!(
            "{} *{}* (ID: {})\n`{}:{}`\n\n",
            status_emoji,
            server.name,
            server.id,
            server.host,
            server.port
        ));
    }

    bot.send_message(msg.chat.id, status_message)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .await?;

    Ok(())
}

async fn invalid_state(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(
        msg.chat.id,
        "⚠️ Invalid command for current state. Try /start",
    )
    .await?;
    Ok(())
}

async fn start_monitoring(
    bot: Bot,
    server_father: Arc<ServerFatherBot>,
    msg: Message,
) -> ResponseResult<()> {
    match server_father.start_monitoring(msg.chat.id).await {
        Ok(_) => {
            bot.send_message(
                msg.chat.id,
                "✅ Monitoring started! You'll receive notifications when server status changes.",
            )
            .await?;
        }
        Err(e) => {
            bot.send_message(
                msg.chat.id,
                format!("❌ Failed to start monitoring: {}", e),
            )
            .await?;
        }
    }
    Ok(())
}

async fn remove_server(bot: Bot, dialogue: MyDialogue, msg: Message) -> ResponseResult<()> {
    dialogue.update(State::AwaitingServerId).await?;
    
    bot.send_message(
        msg.chat.id,
        "Please enter the server ID to remove (use /status to see server IDs):",
    )
    .await?;
    
    Ok(())
}

async fn receive_server_id(
    bot: Bot,
    dialogue: MyDialogue,
    server_father: Arc<ServerFatherBot>,
    msg: Message,
) -> ResponseResult<()> {
    let server_id = match msg.text().and_then(|text| text.parse::<i32>().ok()) {
        Some(id) => id,
        None => {
            bot.send_message(msg.chat.id, "Invalid server ID. Please enter a number.")
                .await?;
            return Ok(());
        }
    };

    match server_father.server_service().get_server(server_id).await {
        Ok(Some(server)) => {
            match server_father.server_service().remove_server(server_id).await {
                Ok(true) => {
                    bot.send_message(
                        msg.chat.id,
                        format!("✅ Server '{}' removed successfully!", server.name),
                    )
                    .await?;
                }
                Ok(false) => {
                    bot.send_message(msg.chat.id, "❌ Server not found.")
                        .await?;
                }
                Err(e) => {
                    bot.send_message(
                        msg.chat.id,
                        format!("❌ Failed to remove server: {}", e),
                    )
                    .await?;
                }
            }
        }
        Ok(None) => {
            bot.send_message(msg.chat.id, "❌ Server not found.")
                .await?;
        }
        Err(e) => {
            bot.send_message(
                msg.chat.id,
                format!("❌ Failed to fetch server: {}", e),
            )
            .await?;
        }
    }

    dialogue.update(State::Start).await?;
    Ok(())
}

async fn check_server(bot: Bot, server_father: Arc<ServerFatherBot>, msg: Message) -> ResponseResult<()> {
    let args = msg.text().unwrap_or_default().split_whitespace().collect::<Vec<_>>();
    
    if args.len() != 2 {
        bot.send_message(
            msg.chat.id,
            "Please provide a server ID (use /check <server_id>)",
        )
        .await?;
        return Ok(());
    }

    let server_id = match args[1].parse::<i32>() {
        Ok(id) => id,
        Err(_) => {
            bot.send_message(msg.chat.id, "Invalid server ID. Please enter a number.")
                .await?;
            return Ok(());
        }
    };

    match server_father.server_service().get_server(server_id).await {
        Ok(Some(server)) => {
            let is_up = server_father.check_server_status(&server).await?;
            let status_emoji = if is_up { "🟢" } else { "🔴" };
            
            bot.send_message(
                msg.chat.id,
                format!(
                    "Server Status:\n{} *{}*\n`{}:{}`\nStatus: {}",
                    status_emoji,
                    server.name,
                    server.host,
                    server.port,
                    if is_up { "Online" } else { "Offline" }
                ),
            )
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;
        }
        Ok(None) => {
            bot.send_message(msg.chat.id, "❌ Server not found.")
                .await?;
        }
        Err(e) => {
            bot.send_message(
                msg.chat.id,
                format!("❌ Failed to fetch server: {}", e),
            )
            .await?;
        }
    }

    Ok(())
}

// Continue with other command handlers... 