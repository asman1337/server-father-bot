use crate::bot::ServerFatherBot;
use crate::error::{BotError, Result};
use std::sync::Arc;
use teloxide::{
    dispatching::{
        dialogue::{Dialogue, InMemStorage},
        UpdateHandler,
    },
    prelude::*,
    utils::command::BotCommands,
};

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
    AwaitingGroupName,
    AwaitingGroupId,
    AwaitingServerForGroup {
        group_id: i32,
    },
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
    #[command(description = "List all groups")]
    Groups,
    #[command(description = "Add server to group")]
    AddToGroup,
    #[command(description = "Remove a group")]
    RemoveGroup,
    #[command(description = "Check group status")]
    CheckGroup,
}

pub fn schema() -> UpdateHandler<BotError> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>().branch(
        case![State::Start]
            .branch(case![Command::Start].endpoint(start))
            .branch(case![Command::AddServer].endpoint(add_server))
            .branch(case![Command::Status].endpoint(status))
            .branch(case![Command::Monitor].endpoint(start_monitoring))
            .branch(case![Command::RemoveServer].endpoint(remove_server))
            .branch(case![Command::Check].endpoint(check_server))
            .branch(case![Command::CreateGroup].endpoint(create_group))
            .branch(case![Command::Groups].endpoint(list_groups))
            .branch(case![Command::AddToGroup].endpoint(add_to_group))
            .branch(case![Command::RemoveGroup].endpoint(remove_group))
            .branch(case![Command::CheckGroup].endpoint(check_group)),
    );

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::AwaitingServerHost].endpoint(receive_host))
        .branch(case![State::AwaitingServerPort { name }].endpoint(receive_port))
        .branch(case![State::AwaitingServerName { host, port }].endpoint(receive_name))
        .branch(case![State::AwaitingServerId].endpoint(receive_server_id))
        .branch(case![State::AwaitingGroupName].endpoint(receive_group_name))
        .branch(case![State::AwaitingGroupId].endpoint(receive_group_id_for_server))
        .branch(
            case![State::AwaitingServerForGroup { group_id }].endpoint(receive_server_for_group),
        )
        .branch(case![State::AwaitingGroupId].endpoint(receive_group_id_for_removal));

    message_handler.endpoint(invalid_state)
}

async fn start(bot: Bot, msg: Message) -> Result<()> {
    bot.send_message(
        msg.chat.id,
        "👋 Welcome to Server Father!\nI'll help you monitor your servers.",
    )
    .await?;
    Ok(())
}

async fn add_server(bot: Bot, dialogue: MyDialogue, msg: Message) -> Result<()> {
    dialogue.update(State::AwaitingServerHost).await?;
    bot.send_message(msg.chat.id, "Please enter the server host (IP or domain):")
        .await?;
    Ok(())
}

async fn receive_host(bot: Bot, dialogue: MyDialogue, msg: Message) -> Result<()> {
    let host = msg.text().unwrap_or_default().to_string();

    dialogue
        .update(State::AwaitingServerPort { name: host })
        .await?;

    bot.send_message(msg.chat.id, "Please enter the port number:")
        .await?;

    Ok(())
}

async fn receive_port(bot: Bot, dialogue: MyDialogue, msg: Message, state: State) -> Result<()> {
    let port = msg
        .text()
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

    if let State::AwaitingServerPort { name } = state {
        dialogue
            .update(State::AwaitingServerName { host: name, port })
            .await?;

        bot.send_message(msg.chat.id, "Please enter a name for this server:")
            .await?;
    }

    Ok(())
}

async fn receive_name(
    bot: Bot,
    dialogue: MyDialogue,
    server_father: Arc<ServerFatherBot>,
    msg: Message,
    state: State,
) -> Result<()> {
    let name = msg.text().unwrap_or_default().to_string();

    if let State::AwaitingServerName { host, port } = state {
        match server_father
            .server_service()
            .add_server(name.clone(), host.clone(), port, None)
            .await
        {
            Ok(_) => {
                bot.send_message(
                    msg.chat.id,
                    format!("✅ Server '{}' ({}) added successfully!", name, host),
                )
                .await?;
            }
            Err(e) => {
                bot.send_message(msg.chat.id, format!("❌ Failed to add server: {}", e))
                    .await?;
            }
        }

        dialogue.update(State::Start).await?;
    }

    Ok(())
}

async fn status(bot: Bot, server_father: Arc<ServerFatherBot>, msg: Message) -> Result<()> {
    let servers = match server_father.server_service().list_servers().await {
        Ok(servers) => servers,
        Err(e) => {
            bot.send_message(msg.chat.id, format!("❌ Failed to fetch servers: {}", e))
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
    let mut status_checks = Vec::with_capacity(servers.len());

    // First collect all servers and their status
    for server in servers.iter() {
        let is_up = server_father
            .check_server_status(server)
            .await
            .unwrap_or(false);
        status_checks.push((server, is_up));
    }

    // Then format the message
    for (server, is_up) in status_checks {
        let status_emoji = if is_up { "🟢" } else { "🔴" };
        let escaped_name = server
            .name
            .replace(|c: char| "[]()~`>#+-=|{}.!".contains(c), r"\$0");
        let escaped_host = server
            .host
            .replace(|c: char| "[]()~`>#+-=|{}.!".contains(c), r"\$0");
        status_message.push_str(&format!(
            "{} *{}* \\(ID: {}\\)\n`{}:{}`\n\n",
            status_emoji, escaped_name, server.id, escaped_host, server.port
        ));
    }

    bot.send_message(msg.chat.id, status_message)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .await?;

    Ok(())
}

async fn invalid_state(bot: Bot, msg: Message) -> Result<()> {
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
) -> Result<()> {
    match server_father.start_monitoring(msg.chat.id).await {
        Ok(_) => {
            bot.send_message(
                msg.chat.id,
                "✅ Monitoring started! You'll receive notifications when server status changes.",
            )
            .await?;
        }
        Err(e) => {
            bot.send_message(msg.chat.id, format!("❌ Failed to start monitoring: {}", e))
                .await?;
        }
    }
    Ok(())
}

async fn remove_server(bot: Bot, dialogue: MyDialogue, msg: Message) -> Result<()> {
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
) -> Result<()> {
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
            match server_father
                .server_service()
                .remove_server(server_id)
                .await
            {
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
                    bot.send_message(msg.chat.id, format!("❌ Failed to remove server: {}", e))
                        .await?;
                }
            }
        }
        Ok(None) => {
            bot.send_message(msg.chat.id, "❌ Server not found.")
                .await?;
        }
        Err(e) => {
            bot.send_message(msg.chat.id, format!("❌ Failed to fetch server: {}", e))
                .await?;
        }
    }

    dialogue.update(State::Start).await?;
    Ok(())
}

async fn check_server(bot: Bot, server_father: Arc<ServerFatherBot>, msg: Message) -> Result<()> {
    let args = msg
        .text()
        .unwrap_or_default()
        .split_whitespace()
        .collect::<Vec<_>>();

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
            bot.send_message(msg.chat.id, format!("❌ Failed to fetch server: {}", e))
                .await?;
        }
    }

    Ok(())
}

async fn create_group(bot: Bot, dialogue: MyDialogue, msg: Message) -> Result<()> {
    dialogue.update(State::AwaitingGroupName).await?;

    bot.send_message(msg.chat.id, "Please enter the name for the new group:")
        .await?;

    Ok(())
}

async fn receive_group_name(
    bot: Bot,
    dialogue: MyDialogue,
    server_father: Arc<ServerFatherBot>,
    msg: Message,
) -> Result<()> {
    let name = msg.text().unwrap_or_default().to_string();

    match server_father
        .group_service()
        .create_group(name.clone())
        .await
    {
        Ok(group) => {
            bot.send_message(
                msg.chat.id,
                format!(
                    "✅ Group '{}' created successfully! (ID: {})",
                    name, group.id
                ),
            )
            .await?;
        }
        Err(e) => {
            bot.send_message(msg.chat.id, format!("❌ Failed to create group: {}", e))
                .await?;
        }
    }

    dialogue.update(State::Start).await?;
    Ok(())
}

async fn list_groups(bot: Bot, server_father: Arc<ServerFatherBot>, msg: Message) -> Result<()> {
    match server_father.group_service().list_groups().await {
        Ok(groups) => {
            if groups.is_empty() {
                bot.send_message(
                    msg.chat.id,
                    "No groups created yet. Use /creategroup to create one.",
                )
                .await?;
                return Ok(());
            }

            let mut message = String::from("📁 *Server Groups*\n\n");
            for group in groups {
                let servers = server_father
                    .server_service()
                    .list_servers_by_group(group.id)
                    .await?;

                message.push_str(&format!(
                    "👥 *{}* (ID: {})\nServers: {}\n\n",
                    group.name,
                    group.id,
                    servers.len()
                ));
            }

            bot.send_message(msg.chat.id, message)
                .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                .await?;
        }
        Err(e) => {
            bot.send_message(msg.chat.id, format!("❌ Failed to fetch groups: {}", e))
                .await?;
        }
    }
    Ok(())
}

async fn add_to_group(bot: Bot, dialogue: MyDialogue, msg: Message) -> Result<()> {
    dialogue.update(State::AwaitingGroupId).await?;

    bot.send_message(
        msg.chat.id,
        "Please enter the group ID (use /groups to see group IDs):",
    )
    .await?;

    Ok(())
}

async fn receive_group_id_for_server(bot: Bot, dialogue: MyDialogue, msg: Message) -> Result<()> {
    let group_id = match msg.text().and_then(|text| text.parse::<i32>().ok()) {
        Some(id) => id,
        None => {
            bot.send_message(msg.chat.id, "Invalid group ID. Please enter a number.")
                .await?;
            return Ok(());
        }
    };

    dialogue
        .update(State::AwaitingServerForGroup { group_id })
        .await?;

    bot.send_message(
        msg.chat.id,
        "Please enter the server ID to add to this group (use /status to see server IDs):",
    )
    .await?;

    Ok(())
}

async fn receive_server_for_group(
    bot: Bot,
    dialogue: MyDialogue,
    server_father: Arc<ServerFatherBot>,
    msg: Message,
    group_id: i32,
) -> Result<()> {
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
            match server_father
                .server_service()
                .assign_to_group(server_id, group_id)
                .await
            {
                Ok(true) => {
                    bot.send_message(
                        msg.chat.id,
                        format!("✅ Server '{}' added to group successfully!", server.name),
                    )
                    .await?;
                }
                Ok(false) => {
                    bot.send_message(msg.chat.id, "❌ Failed to add server to group.")
                        .await?;
                }
                Err(e) => {
                    bot.send_message(
                        msg.chat.id,
                        format!("❌ Error adding server to group: {}", e),
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
            bot.send_message(msg.chat.id, format!("❌ Error fetching server: {}", e))
                .await?;
        }
    }

    dialogue.update(State::Start).await?;
    Ok(())
}

async fn remove_group(bot: Bot, dialogue: MyDialogue, msg: Message) -> Result<()> {
    dialogue.update(State::AwaitingGroupId).await?;

    bot.send_message(
        msg.chat.id,
        "Please enter the group ID to remove (use /groups to see group IDs):",
    )
    .await?;

    Ok(())
}

async fn receive_group_id_for_removal(
    bot: Bot,
    dialogue: MyDialogue,
    server_father: Arc<ServerFatherBot>,
    msg: Message,
) -> Result<()> {
    let group_id = match msg.text().and_then(|text| text.parse::<i32>().ok()) {
        Some(id) => id,
        None => {
            bot.send_message(msg.chat.id, "Invalid group ID. Please enter a number.")
                .await?;
            return Ok(());
        }
    };

    // First check if group exists and get its name
    let groups = server_father.group_service().list_groups().await?;
    let group = groups.iter().find(|g| g.id == group_id);

    match group {
        Some(group) => match server_father.group_service().delete_group(group_id).await {
            Ok(true) => {
                bot.send_message(
                    msg.chat.id,
                    format!("✅ Group '{}' removed successfully!", group.name),
                )
                .await?;
            }
            Ok(false) => {
                bot.send_message(msg.chat.id, "❌ Group not found.").await?;
            }
            Err(e) => {
                bot.send_message(msg.chat.id, format!("❌ Failed to remove group: {}", e))
                    .await?;
            }
        },
        None => {
            bot.send_message(msg.chat.id, "❌ Group not found.").await?;
        }
    }

    dialogue.update(State::Start).await?;
    Ok(())
}

async fn check_group(bot: Bot, server_father: Arc<ServerFatherBot>, msg: Message) -> Result<()> {
    let args = msg
        .text()
        .unwrap_or_default()
        .split_whitespace()
        .collect::<Vec<_>>();

    if args.len() != 2 {
        bot.send_message(
            msg.chat.id,
            "Please provide a group ID (use /checkgroup <group_id>)",
        )
        .await?;
        return Ok(());
    }

    let group_id = match args[1].parse::<i32>() {
        Ok(id) => id,
        Err(_) => {
            bot.send_message(msg.chat.id, "Invalid group ID. Please enter a number.")
                .await?;
            return Ok(());
        }
    };

    match server_father.group_service().list_groups().await {
        Ok(groups) => {
            let group = groups.iter().find(|g| g.id == group_id);

            match group {
                Some(group) => {
                    let servers = server_father
                        .server_service()
                        .list_servers_by_group(group_id)
                        .await?;

                    if servers.is_empty() {
                        bot.send_message(
                            msg.chat.id,
                            format!("Group '{}' has no servers.", group.name),
                        )
                        .await?;
                        return Ok(());
                    }

                    let mut status_message = format!("📊 *Group: {}*\n\n", group.name);
                    let mut total_up = 0;
                    let total_servers = servers.len();

                    for server in servers {
                        let is_up = server_father
                            .check_server_status(&server)
                            .await
                            .unwrap_or(false);

                        if is_up {
                            total_up += 1;
                        }

                        let status_emoji = if is_up { "🟢" } else { "🔴" };

                        status_message.push_str(&format!(
                            "{} *{}*\n`{}:{}`\n\n",
                            status_emoji, server.name, server.host, server.port
                        ));
                    }

                    // Add summary
                    status_message.push_str(&format!(
                        "Summary: {} of {} servers online",
                        total_up, total_servers
                    ));

                    bot.send_message(msg.chat.id, status_message)
                        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
                        .await?;
                }
                None => {
                    bot.send_message(msg.chat.id, "❌ Group not found.").await?;
                }
            }
        }
        Err(e) => {
            bot.send_message(msg.chat.id, format!("❌ Failed to fetch groups: {}", e))
                .await?;
        }
    }

    Ok(())
}

// Continue with other command handlers...
