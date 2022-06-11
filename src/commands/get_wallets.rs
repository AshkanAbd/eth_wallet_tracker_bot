use tokio::task;
use crate::commands::CommandHandler;
use crate::{DataRepository, Message, SqliteDb};
use teloxide::{prelude::*};

pub struct GetWalletsCommand<'a> {
    pub bot: &'a AutoSend<Bot>,
}

impl CommandHandler for GetWalletsCommand<'_> {
    fn handle<'a>(&mut self, message: Message) -> &'a str {
        let mut db = SqliteDb::get_connection();

        let user = db.get_user(message.chat.id.0);

        if user.is_none() {
            return "Please send /start command.";
        }

        let user_id = user.unwrap().id.unwrap();

        let wallets = db.get_user_wallets(user_id);
        db.drop();

        let bot = self.bot.clone();

        task::spawn(async move {
            for wallet in wallets {
                bot.send_message(message.chat.id, format!("https://etherscan.io/address/{wallet}", wallet = wallet.address)).await.unwrap();
            }
        });

        "Here is your wallets that are tracking by me:"
    }
}