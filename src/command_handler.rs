use teloxide::{prelude::*};
use std::error::Error;
use crate::{Command};
use crate::commands::CommandHandler;
use crate::commands::{start::StartCommand, add_wallet::AddWalletCommand};

pub async fn handler(bot: AutoSend<Bot>, message: Message, command: Command)
                     -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    match command {
        Command::Start => {
            let mut start_command = StartCommand::new();

            bot.send_message(message.chat.id, start_command.handle(message)).await?;
        }
        Command::Add { address } => {
            let mut wallet_command = AddWalletCommand {
                address,
                bot: &bot,
            };

            bot.send_message(message.chat.id, wallet_command.handle(message)).await?;
        }
    };

    Ok(())
}
