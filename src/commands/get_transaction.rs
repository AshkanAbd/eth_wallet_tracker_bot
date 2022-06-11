use crate::commands::CommandHandler;
use crate::{DataRepository, Message, SqliteDb};
use crate::common::{valid_eth_address};
use teloxide::{prelude::*};
use tokio::{task};

pub struct GetTransactionCommand<'a> {
    pub address: String,
    pub bot: &'a AutoSend<Bot>,
}

impl CommandHandler for GetTransactionCommand<'_> {
    fn handle<'a>(&mut self, message: Message) -> &'a str {
        if !valid_eth_address(self.address.as_str()) {
            return "Invalid eth address";
        }

        let mut db = SqliteDb::get_connection();

        let user = db.get_user(message.chat.id.0);

        if user.is_none() {
            return "Please send /start command.";
        }

        let user_id = user.unwrap().id.unwrap();

        let wallet = db.get_wallet(Some(user_id), self.address.to_string());

        if wallet.is_none() {
            return "This wallet address is not tracked by you.";
        }

        let txs = db.get_all_transactions(wallet.unwrap().id.unwrap());
        db.drop();

        let bot = self.bot.clone();
        task::spawn(async move {
            for tx in txs {
                bot.send_message(message.chat.id, tx.to_string().as_str()).await.unwrap();
            }
        });

        "Here is transactions for your address:"
    }
}