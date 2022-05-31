use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct EtherScanTrx {
    pub status: String,
    pub message: String,
    pub result: Vec<EtherScanTrxDetail>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct EtherScanTrxDetail {
    pub blockNumber: String,
    pub timeStamp: String,
    pub hash: String,
    pub nonce: String,
    pub blockHash: String,
    pub transactionIndex: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub gas: String,
    pub gasPrice: String,
    pub isError: String,
    pub txreceipt_status: String,
    pub input: String,
    pub contractAddress: String,
    pub cumulativeGasUsed: String,
    pub gasUsed: String,
    pub confirmations: String,
}

impl EtherScanTrxDetail {
    pub fn format_as_str(&self) -> String {
        format!("Transfer {a} ETH, From {f} To {t}.\nLink: https://etherscan.io/tx/{tx}",
                f = self.from, t = self.to, tx = self.hash,
                a = (self.value.parse::<f64>().unwrap() / 10f64.powi(18i32))
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EtherScanErc {
    pub status: String,
    pub message: String,
    pub result: Vec<EtherScanErcDetails>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct EtherScanErcDetails {
    pub blockNumber: String,
    pub timeStamp: String,
    pub hash: String,
    pub nonce: String,
    pub blockHash: String,
    pub from: String,
    pub contractAddress: String,
    pub to: String,
    pub value: String,
    pub tokenName: String,
    pub tokenSymbol: String,
    pub tokenDecimal: String,
    pub transactionIndex: String,
    pub gas: String,
    pub gasPrice: String,
    pub gasUsed: String,
    pub cumulativeGasUsed: String,
    pub input: String,
    pub confirmations: String,
}

impl EtherScanErcDetails {
    pub fn format_as_str(&self) -> String {
        format!("Transfer {a} {tn}, From {f} To {t}.\nLink: https://etherscan.io/tx/{tx}",
                tn = self.tokenName, f = self.from, t = self.to, tx = self.hash,
                a = (self.value.parse::<f64>().unwrap() / (10f64.powi(self.tokenDecimal.parse::<i32>().unwrap())))
        )
    }
}
