use crate::commands::CommandHandler;
use crate::{DataRepository, logger, Message, SqliteDb};

// todo I think it could be implement a little better with macros, structs and other stuff
pub struct StartCommand {}

impl StartCommand {
    pub fn new() -> Self {
        StartCommand {}
    }
}

impl CommandHandler for StartCommand {
    fn handle<'a>(&mut self, message: Message) -> &'a str {
        let mut db = SqliteDb::get_connection();

        if db.get_user(message.chat.id.0).is_none() {
            db.add_user(message.chat.id.0);
            logger!("New user has been added.");
        } else {
            logger!("User exists.");
        }

        db.drop();
        "Send wallet address: /add <wallet_address>"
    }
}
