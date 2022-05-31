use crate::models::wallet::Wallet;

pub struct User {
    pub id: Option<i64>,
    pub chat_id: String,
    pub wallets: Vec<Wallet>,
}

impl User {
    pub fn new(chat_id: String, id: Option<i64>) -> Self {
        User {
            id,
            chat_id,
            wallets: vec![],
        }
    }
}
