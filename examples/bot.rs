// This bot throws a dice on each incoming message.

use teloxide::prelude::*;
use teloxide::RequestError;
use teloxide_core::requests::JsonRequest;
use teloxide::payloads;
use teloxide_core::payloads::GetUpdates;

type UpdatesManager = JsonRequest<payloads::GetUpdates>;


#[tokio::main]
async fn main() {
    use teloxide_core::{
        prelude::*,
        types::{DiceEmoji, ParseMode},
    };

    let bot = Bot::new("");
    let mut get_updates = GetUpdates::new();
    loop {
        println!("{:?}", get_updates);
        let updates = UpdatesManager::new(bot.clone(), get_updates.clone()).await.unwrap();
        if updates.len() > 0 {
            let last_update = updates.last().unwrap();
            get_updates = GetUpdates {
                offset: Some(last_update.id),
                limit: None,
                timeout: None,
                allowed_updates: None,
            };
        }
        println!("{:?} {:?}", updates.len(), updates);
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    }

    // let handle = tokio::spawn(run_bot_until(bot));
    // handle.await.unwrap();
    // println!("wow");
}