use sqlite::Statement;

pub struct Transaction {
    pub id: Option<i64>,
    pub from: String,
    pub to: String,
    pub amount: String,
    pub tx_hash: String,
    pub token: String,
    pub decimal: i64,
    pub status: bool,
    pub wallet_id: i64,
}

impl Transaction {
    pub fn new(from: String, to: String, amount: String, tx_hash: String, token: String, wallet_id: i64,
               decimal: i64, id: Option<i64>, status: Option<bool>) -> Self {
        let mut trx = Transaction {
            from,
            to,
            amount,
            tx_hash,
            wallet_id,
            token,
            decimal,
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
            statement.read::<i64>(8).unwrap(),
            Some(statement.read::<i64>(0).unwrap()),
            Some(statement.read::<i64>(6).unwrap() != 0),
        )
    }

    pub fn to_string(&self) -> String {
        let mut decimal = match self.token.as_str() {
            "ETH" => 18,
            "Tether USD" => 6,
            _ => 0,
        };

        decimal = if self.decimal != 0i64 {
            self.decimal as i32
        } else {
            decimal
        };

        format!("Transfer {a} {tn}, From {f} To {t}.\nLink: https://etherscan.io/tx/{tx}",
                tn = self.token, f = self.from, t = self.to, tx = self.tx_hash,
                a = (self.amount.parse::<f64>().unwrap() / (10f64.powi(decimal)))
        )
    }
}
