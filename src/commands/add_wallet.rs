use tokio::{task};
use crate::commands::CommandHandler;
use crate::{DataRepository, Message, SqliteDb};
use crate::common::{background_wallet_worker, valid_eth_address};
use teloxide::{prelude::*};

pub struct AddWalletCommand<'a> {
    pub address: String,
    pub bot: &'a AutoSend<Bot>,
}

impl CommandHandler for AddWalletCommand<'_> {
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

        if wallet.is_some() {
            return "This wallet address is currently being tracked.";
        }

        db.add_wallet(user_id, self.address.to_string());
        db.drop();

        let address = self.address.clone();
        let bot = self.bot.clone();
        let chat_id = message.chat.id;

        task::spawn(async move {
            let mut repo = SqliteDb::get_connection();
            background_wallet_worker::<SqliteDb>(&bot.clone(), chat_id, address, user_id, &mut repo).await;
        });

        "The wallet added to tracker."
    }
}