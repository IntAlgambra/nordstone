use std::path::PathBuf;
use std::fs;

use serde::{Serialize, Deserialize};
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

use crate::models::Folder;
use crate::sync::SyncManager;


#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "setting up bot")]
    Start,
}

async fn answer(bot: Bot, msg: Message, command: Command) -> ResponseResult<()> {
    match command {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?
        }
        Command::Start => {
            bot.send_message(msg.chat.id, "Bot has been set up").await?
        }
    };
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct BotSettings {
    token: String,
    chat_id: String,
}

struct TelegramSyncManager {
    bot_settings: BotSettings,
}

impl TelegramSyncManager {
    fn new(bot_settings: BotSettings) -> Self
    {
        Self { bot_settings }
    }

    async fn configure_bot(&self) {
        let bot = Bot::new(&self.bot_settings.token);
        Command::repl(bot, answer).await;
    }
}

impl SyncManager for TelegramSyncManager {
    fn upload(&self, folder: Folder) {
        todo!()
    }

    fn download(&self) -> Folder {
        todo!()
    }

    fn merge(&self, remote_data: Folder, local_folder: Folder) -> Folder {
        todo!()
    }
}