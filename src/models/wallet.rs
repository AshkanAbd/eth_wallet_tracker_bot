use sqlite::Statement;
use crate::models::transaction::Transaction;
use crate::models::user::User;

pub struct Wallet {
    pub id: Option<i64>,
    pub address: String,
    pub user_id: i64,
    pub transactions: Vec<Transaction>,
    pub user: Option<User>,
}

impl Wallet {
    pub fn new(address: String, user_id: i64, id: Option<i64>) -> Self {
        Wallet {
            address,
            user_id,
            id,
            transactions: vec![],
            user: None,
        }
    }

    pub fn read_from_statement(statement: &Statement) -> Self {
        Wallet::new(
            statement.read::<String>(2).unwrap(),
            statement.read::<i64>(1).unwrap(),
            Some(statement.read::<i64>(0).unwrap()),
        )
    }
}
