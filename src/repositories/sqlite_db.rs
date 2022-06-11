use sqlite::{Connection, State};
use crate::{AppConfig, DataRepository, logger, logger_l};
use crate::models::user::User;
use crate::models::wallet::Wallet;
use structopt::StructOpt;
use crate::models::transaction::Transaction;

pub struct SqliteDb {
    connection: Option<Connection>,
}

impl SqliteDb {
    pub fn new() -> Self {
        SqliteDb {
            connection: None,
        }
    }

    pub fn get_connection() -> Self {
        let config = AppConfig::from_args();

        let mut db = SqliteDb::new();

        db.init(vec![config.db_path.as_str()]);
        db.load();
        db
    }
}

impl DataRepository for SqliteDb {
    fn init(&mut self, params: Vec<&str>) {
        if params.len() != 1 {
            panic!("Invalid sqlite db");
        }

        let connection = sqlite::open(params.get(0).unwrap());

        match connection {
            Ok(v) => {
                self.connection = Some(v);
            }
            Err(e) => {
                panic!("Error on sqlite: {}", e);
            }
        }
    }

    fn load(&self) {
        if !self.connected() {
            panic!("Connection error.");
        }

        let connection = self.connection.as_ref().unwrap();

        let result = connection.execute(r#"create table if not exists users("id" integer not null constraint users_pk primary key autoincrement, "chat_id" varchar not null);"#);
        if let Err(e) = result {
            panic!("Error configuring users table: {}", e);
        }

        let result = connection.execute(r#"create table if not exists wallets ("id" integer not null constraint wallets_pk primary key autoincrement, "user_id" integer not null constraint wallets_users_id_fk references users (id) on update cascade on delete cascade, "address" varchar not null);"#);
        if let Err(e) = result {
            panic!("Error configuring wallets table: {}", e);
        }

        let result = connection.execute(r#"create table if not exists transactions( "id" integer constraint transactions_pk primary key autoincrement, "from" varchar not null, "wallet_id" integer not null constraint transactions_wallets_id_fk references transactions(id) on update cascade on delete cascade, "to" varchar not null, "amount" varchar not null, "tx_hash" varchar not null, "status" bool default TRUE, "token" varchar not null);"#);
        if let Err(e) = result {
            panic!("Error on configuring transactions table: {}", e);
        }

        connection.execute(r#"alter table transactions add decimal integer default 0;"#).unwrap_or_default();
    }

    fn connected(&self) -> bool {
        !self.connection.is_none()
    }

    fn add_user(&self, chat_id: i64) -> bool {
        if !self.connected() {
            panic!("Connection error.");
        }

        let connection = self.connection.as_ref().unwrap();

        logger!("-> adding user to db...");

        let mut statement = connection.prepare(r#"insert into users (chat_id) values (:chat_id);"#).unwrap();

        statement.bind_by_name(":chat_id", chat_id.to_string().as_str()).unwrap();

        statement.next().unwrap();

        logger!("-> user added successfully");

        true
    }

    fn get_user(&self, chat_id: i64) -> Option<User> {
        if !self.connected() {
            panic!("Connection error.");
        }

        let connection = self.connection.as_ref().unwrap();

        logger!("-> retrieving user from db...");

        let mut statement = connection.prepare(r#"select * from users where chat_id = :chat_id;"#).unwrap();

        statement.bind_by_name(":chat_id", chat_id.to_string().as_str()).unwrap();

        let state = statement.next().unwrap();
        return if state == State::Done {
            logger!("-> user with chat id {} notfound", chat_id);
            None
        } else {
            logger!("-> user with chat id {} found", chat_id);
            Some(User::new(
                statement.read::<String>(1).unwrap(),
                Some(statement.read::<i64>(0).unwrap()),
            ))
        };
    }

    fn get_all_user(&self) -> Vec<User> {
        if !self.connected() {
            panic!("Connection error.");
        }

        let connection = self.connection.as_ref().unwrap();

        logger!("-> retrieving all users from db...");

        let mut statement = connection.prepare(r#"select * from users;"#).unwrap();

        let mut users = vec![];

        while let State::Row = statement.next().unwrap() {
            users.push(User::new(
                statement.read::<String>(1).unwrap(),
                Some(statement.read::<i64>(0).unwrap()),
            ))
        }

        logger!("{} user retrieved", users.len());

        users
    }

    fn add_wallet(&self, user_id: i64, wallet_address: String) -> bool {
        if !self.connected() {
            panic!("Connection error.");
        }

        let connection = self.connection.as_ref().unwrap();

        logger!("-> adding wallet for user {}...", user_id);

        let mut statement = connection.prepare(r#"insert into wallets (user_id, address) values (:user_id,:address);"#).unwrap();

        statement.bind_by_name(":user_id", user_id).unwrap();
        statement.bind_by_name(":address", wallet_address.as_str()).unwrap();

        statement.next().unwrap();

        logger!("-> wallet added successfully");

        true
    }

    fn remove_wallet(&self, user_id: i64, wallet_address: String) -> bool {
        if !self.connected() {
            panic!("Connection error.");
        }

        let connection = self.connection.as_ref().unwrap();

        logger!("-> remove wallet {} for user {}...", wallet_address, user_id);

        let mut statement = connection.prepare(r#"delete from wallets where address = :wallet_address;"#).unwrap();

        statement.bind_by_name(":wallet_address", wallet_address.as_str()).unwrap();

        statement.next().unwrap();

        logger!("-> wallet removed successfully.");

        true
    }

    fn get_wallet(&self, user_id: Option<i64>, wallet_address: String) -> Option<Wallet> {
        if !self.connected() {
            panic!("Connection error.");
        }

        let connection = self.connection.as_ref().unwrap();

        let mut statement = if let Some(u) = user_id {
            logger!("-> retrieving wallet with user {} and wallet {}...", u, wallet_address);

            let mut statement = connection
                .prepare(r#"select * from wallets where address = :address and user_id = :user_id;"#).unwrap();

            statement.bind_by_name(":user_id", u).unwrap();

            statement
        } else {
            logger!("-> retrieving wallet with wallet {}...", wallet_address);

            connection.prepare(r#"select * from wallets where address = :address;"#).unwrap()
        };

        statement.bind_by_name(":address", wallet_address.as_str()).unwrap();

        let state = statement.next().unwrap();
        return if state == State::Done {
            logger_l!("-> wallet {} notfound", wallet_address);

            if user_id.is_some() {
                println!(" for user {}", user_id.unwrap());
            }

            None
        } else {
            logger_l!("-> wallet {} found", wallet_address);

            if user_id.is_some() {
                println!(" for user {}", user_id.unwrap());
            }

            Some(Wallet::read_from_statement(&statement))
        };
    }

    fn add_transaction(&self, transaction: Transaction) -> bool {
        if !self.connected() {
            panic!("Connection error.");
        }

        let connection = self.connection.as_ref().unwrap();

        logger!("-> adding transaction for wallet {}...", transaction.wallet_id);

        let mut statement = connection.prepare(r#"insert into transactions ("from", wallet_id, "to", amount, tx_hash, status, token) values (:from, :wallet_id, :to, :amount, :tx_hash, :status, :token);"#).unwrap();

        statement.bind_by_name(":from", transaction.from.as_str()).unwrap();
        statement.bind_by_name(":wallet_id", transaction.wallet_id).unwrap();
        statement.bind_by_name(":to", transaction.to.as_str()).unwrap();
        statement.bind_by_name(":amount", transaction.amount.as_str()).unwrap();
        statement.bind_by_name(":tx_hash", transaction.tx_hash.as_str()).unwrap();
        statement.bind_by_name(":status", transaction.status.to_string().as_str()).unwrap();
        statement.bind_by_name(":token", transaction.token.as_str()).unwrap();

        statement.next().unwrap();

        logger!("-> transaction added successfully");

        true
    }

    fn get_transaction(&self, tx_hash: String, wallet_id: Option<i64>, token_name: Option<String>) -> Option<Transaction> {
        if !self.connected() {
            panic!("Connection error.");
        }

        let connection = self.connection.as_ref().unwrap();

        let mut statement = if let Some(u) = wallet_id {
            logger!("-> retrieving transaction with tx_hash {} and wallet {}...", tx_hash, u);

            let mut statement = if let Some(token) = token_name {
                let mut statement = connection
                    .prepare(r#"select * from transactions where tx_hash = :tx_hash and wallet_id = :wallet_id and token = :token;"#).unwrap();

                statement.bind_by_name(":token", token.as_str()).unwrap();

                statement
            } else {
                connection.prepare(r#"select * from transactions where tx_hash = :tx_hash and wallet_id = :wallet_id;"#).unwrap()
            };

            statement.bind_by_name(":wallet_id", u).unwrap();

            statement
        } else {
            logger!("-> retrieving transaction with tx_hash {}...", tx_hash);

            if let Some(token) = token_name {
                let mut statement = connection
                    .prepare(r#"select * from transactions where tx_hash = :tx_hash and token = :token;"#).unwrap();

                statement.bind_by_name(":token", token.as_str()).unwrap();

                statement
            } else {
                connection.prepare(r#"select * from transactions where tx_hash = :tx_hash;"#).unwrap()
            }
        };

        statement.bind_by_name(":tx_hash", tx_hash.as_str()).unwrap();

        let state = statement.next().unwrap();
        return if state == State::Done {
            logger_l!("-> transaction {} notfound", tx_hash);

            if wallet_id.is_some() {
                println!(" for wallet {}", wallet_id.unwrap());
            }

            None
        } else {
            logger_l!("-> transaction {} found", tx_hash);

            if wallet_id.is_some() {
                println!(" for wallet {}", wallet_id.unwrap());
            }

            Some(Transaction::read_from_statement(&statement))
        };
    }

    fn get_all_transactions(&self, wallet_id: i64) -> Vec<Transaction> {
        let mut res = vec![];

        if !self.connected() {
            panic!("Connection error.");
        }

        let connection = self.connection.as_ref().unwrap();

        logger!("-> retrieving all transactions from database...");

        let mut statement = connection.prepare(r#"select * from transactions where wallet_id = :wallet_id;"#).unwrap();

        statement.bind_by_name(":wallet_id", wallet_id).unwrap();

        while let State::Row = statement.next().unwrap() {
            res.push(Transaction::read_from_statement(&statement));
        }

        logger!("-> {} transactions retrieved.", res.len());

        res
    }

    fn get_all_wallets_with_user(&self) -> Vec<Wallet> {
        let mut res = vec![];

        if !self.connected() {
            panic!("Connection error.");
        }

        let connection = self.connection.as_ref().unwrap();

        logger!("-> retrieving all wallets from database...");

        let mut statement = connection.prepare(r#"select * from wallets inner join users on users.id = wallets.user_id;"#).unwrap();

        while let State::Row = statement.next().unwrap() {
            let mut wallet = Wallet::read_from_statement(&statement);

            wallet.user = Some(User::new(
                statement.read::<String>(4).unwrap(),
                Some(statement.read::<i64>(3).unwrap()),
            ));

            res.push(wallet);
        }

        logger!("-> {} wallets retrieved.", res.len());

        res
    }

    fn get_user_wallets(&self, user_id: i64) -> Vec<Wallet> {
        let mut res = vec![];

        if !self.connected() {
            panic!("Connection error.");
        }

        let connection = self.connection.as_ref().unwrap();

        logger!("-> retrieving wallets for user {} from database...", user_id);

        let mut statement = connection.prepare(r#"select * from wallets where user_id = :user_id;"#).unwrap();

        statement.bind_by_name(":user_id", user_id).unwrap();

        while let State::Row = statement.next().unwrap() {
            let wallet = Wallet::read_from_statement(&statement);

            res.push(wallet);
        }

        logger!("-> {} wallets retrieved.", res.len());

        res
    }

    fn drop(&mut self) {
        if self.connection.is_none() {
            return;
        }

        self.connection = None;
    }
}
