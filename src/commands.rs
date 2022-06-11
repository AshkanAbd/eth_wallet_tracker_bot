pub mod start;
pub mod add_wallet;
pub mod remove_wallet;
pub mod get_transaction;
pub mod get_wallets;

use teloxide::{prelude::*, utils::command::BotCommands};
use strum_macros::AsRefStr;

#[derive(Clone, BotCommands, AsRefStr)]
#[command(rename = "lowercase")]
pub enum Command {
    #[command()]
    Start,
    #[command()]
    Add { address: String },
    #[command()]
    Remove { address: String },
    #[command()]
    TxList { address: String },
    #[command()]
    List,
}

pub trait CommandHandler {
    fn handle<'a>(&mut self, message: Message) -> &'a str;
}
