pub mod start;
pub mod add_wallet;

use teloxide::{prelude::*, utils::command::BotCommands};
use strum_macros::AsRefStr;

#[derive(Clone, BotCommands, AsRefStr)]
#[command(rename = "lowercase")]
pub enum Command {
    #[command()]
    Start,
    #[command()]
    Add { address: String },
    // todo add rm command to remove wallet from tracker.
}

pub trait CommandHandler {
    fn handle<'a>(&mut self, message: Message) -> &'a str;
}
