use sqlite::Statement;

pub struct Transaction {
    pub id: Option<i64>,
    pub from: String,
    pub to: String,
    pub amount: String,
    pub tx_hash: String,
    pub token: String,
    pub status: bool,
    pub wallet_id: i64,
}

impl Transaction {
    pub fn new(from: String, to: String, amount: String, tx_hash: String, token: String, wallet_id: i64, id: Option<i64>, status: Option<bool>) -> Self {
        let mut trx = Transaction {
            from,
            to,
            amount,
            tx_hash,
            wallet_id,
            token,
            id,
            status: true,
        };

        if let Some(s) = status {
            trx.status = s
        }

        trx
    }

    pub fn read_from_statement(statement: &Statement) -> Transaction {
        Transaction::new(
            statement.read::<String>(1).unwrap(),
            statement.read::<String>(3).unwrap(),
            statement.read::<String>(4).unwrap(),
            statement.read::<String>(5).unwrap(),
            statement.read::<String>(7).unwrap(),
            statement.read::<i64>(2).unwrap(),
            Some(statement.read::<i64>(0).unwrap()),
            Some(statement.read::<i64>(6).unwrap() != 0),
        )
    }
}
