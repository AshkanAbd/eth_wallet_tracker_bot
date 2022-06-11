use teloxide::{prelude::*};
use std::error::Error;
use crate::{Command};
use crate::commands::CommandHandler;
use crate::commands::{start::StartCommand, add_wallet::AddWalletCommand, remove_wallet::RemoveWalletCommand};
use crate::commands::get_transaction::GetTransactionCommand;
use crate::commands::get_wallets::GetWalletsCommand;

pub async fn handler(bot: AutoSend<Bot>, message: Message, command: Command)
                     -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    match command {
        Command::Start => {
            let mut start_command = StartCommand::new();

            bot.send_message(message.chat.id, start_command.handle(message)).await?;
        }
        Command::Add { address } => {
            let mut wallet_command = AddWalletCommand {
                address: address.trim().to_string(),
                bot: &bot,
            };

            bot.send_message(message.chat.id, wallet_command.handle(message)).await?;
        }
        Command::Remove { address } => {
            let mut remove_wallet_command = RemoveWalletCommand {
                address: address.trim().to_string(),
                bot: &bot,
            };

            bot.send_message(message.chat.id, remove_wallet_command.handle(message)).await?;
        }
        Command::TxList { address } => {
            let mut get_transaction = GetTransactionCommand {
                address: address.trim().to_string(),
                bot: &bot,
            };

            bot.send_message(message.chat.id, get_transaction.handle(message)).await?;
        }
        Command::List => {
            let mut get_wallets = GetWalletsCommand {
                bot: &bot,
            };

            bot.send_message(message.chat.id, get_wallets.handle(message)).await?;
        }
    };

    Ok(())
}
