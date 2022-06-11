pub mod sqlite_db;

use crate::models::{user::User, wallet::Wallet};
use crate::models::transaction::Transaction;

pub trait DataRepository {
    fn init(&mut self, params: Vec<&str>);
    fn load(&self);
    fn connected(&self) -> bool;
    fn add_user(&self, chat_id: i64) -> bool;
    fn get_user(&self, chat_id: i64) -> Option<User>;
    fn get_all_user(&self) -> Vec<User>;
    fn add_wallet(&self, user_id: i64, wallet_address: String) -> bool;
    fn remove_wallet(&self, user_id: i64, wallet_address: String) -> bool;
    fn get_wallet(&self, user_id: Option<i64>, wallet_address: String) -> Option<Wallet>;
    fn add_transaction(&self, transaction: Transaction) -> bool;
    fn get_transaction(&self, tx_hash: String, wallet_id: Option<i64>, token_name: Option<String>) -> Option<Transaction>;
    fn get_all_transactions(&self, wallet_id: i64) -> Vec<Transaction>;
    fn get_all_wallets_with_user(&self) -> Vec<Wallet>;
    fn get_user_wallets(&self, user_id: i64) -> Vec<Wallet>;
    fn drop(&mut self);
}
