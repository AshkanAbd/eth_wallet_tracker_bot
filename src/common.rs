use std::time::{Duration};
use crypto::{sha3::Sha3, digest::Digest};
use structopt::StructOpt;
use teloxide::{prelude::*, types::ChatId};
use tokio::{time, task};
use crate::{AppConfig, DataRepository, SqliteDb};
use crate::models::{etherscan::*, transaction::Transaction};

#[macro_export]
macro_rules! logger {
    () => ({
        print!("\n")
    });
    ($($arg:tt)*) => ({
        let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        println!("{}: {}",timestamp, format!($($arg)*));
    })
}

#[macro_export]
macro_rules! logger_l {
    () => ({
        print!("")
    });
    ($($arg:tt)*) => ({
        let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        print!("{}: {}",timestamp, format!($($arg)*));
    })
}


pub fn valid_eth_address(address: &str) -> bool {
    if address.len() != 42 {
        return false;
    }

    let normalized_address = normalize_address(address);

    if normalized_address.len() != 40 {
        return false;
    }

    let mut hasher = Sha3::keccak256();
    hasher.input_str(&normalized_address);
    let address_hash = hasher.result_str();

    for i in 0..40 {
        if u32::from_str_radix(&address_hash[i..i + 1], 16).unwrap() > 7 {
            if address[i + 2..i + 3].to_ascii_uppercase() != address[i + 2..i + 3] {
                return false;
            }
        } else {
            if address[i + 2..i + 3].to_ascii_lowercase() != address[i + 2..i + 3] {
                return false;
            }
        }
    }

    true
}

fn normalize_address(address: &str) -> String {
    address.to_ascii_lowercase().trim_start_matches("0x").to_string()
}

pub async fn background_wallet_worker<R>(bot: &AutoSend<Bot>, chat_id: ChatId, wallet: String, user_id: i64, repo: &mut R)
    where R: DataRepository {
    let config = AppConfig::from_args();
    let mut interval = time::interval(Duration::from_secs(60));

    let address = wallet.clone();
    let wallet_address = repo.get_wallet(Some(user_id), wallet).unwrap();

    interval.tick().await;

    loop {
        if repo.get_wallet(Some(user_id), wallet_address.address.clone()).is_none() {
            break;
        }

        let trx_res = check_trx(config.ether_api.as_str(), address.as_str()).await;

        if let Ok(v) = trx_res {
            if let Some(d) = v.result.get(0) {
                let trx = Transaction::new(
                    d.from.to_owned(),
                    d.to.to_owned(),
                    d.value.to_owned(),
                    d.hash.to_owned(),
                    "ETH".to_string(),
                    wallet_address.id.unwrap(),
                    None,
                    Some(d.isError != "0"),
                );

                if repo.get_transaction(d.hash.to_owned(), wallet_address.id, Some("ETH".to_string())).is_none() {
                    repo.add_transaction(trx);
                    bot.send_message(chat_id, d.format_as_str().as_str()).await.unwrap();
                }
            }
        }

        let erc_res = check_erc(config.ether_api.as_str(), address.as_str()).await;

        if let Ok(v) = erc_res {
            if let Some(d) = v.result.get(0) {
                let trx = Transaction::new(
                    d.from.to_owned(),
                    d.to.to_owned(),
                    d.value.to_owned(),
                    d.hash.to_owned(),
                    d.tokenName.to_owned(),
                    wallet_address.id.unwrap(),
                    None,
                    Some(true),
                );

                if repo.get_transaction(d.hash.to_owned(), wallet_address.id, Some(trx.token.clone())).is_none() {
                    repo.add_transaction(trx);
                    bot.send_message(chat_id, d.format_as_str().as_str()).await.unwrap();
                }
            }
        }

        interval.tick().await;
    }
}

async fn check_trx(api_token: &str, wallet: &str) -> Result<EtherScanTrx, reqwest::Error> {
    let url = format!("https://api.etherscan.io/api?module=account&action=txlist&address={wallet}&startblock=0&endblock=99999999&page=1&offset=1&sort=desc&apikey={api_token}",
                      wallet = wallet, api_token = api_token);

    let resp = reqwest::get(url).await?
        .json::<EtherScanTrx>().await?;
    Ok(resp)
}

async fn check_erc(api_token: &str, wallet: &str) -> Result<EtherScanErc, reqwest::Error> {
    let url = format!("https://api.etherscan.io/api?module=account&action=tokentx&address={wallet}&page=1&offset=1&startblock=0&endblock=9999999999&sort=desc&apikey={api_token}",
                      wallet = wallet, api_token = api_token);

    let resp = reqwest::get(url).await?
        .json::<EtherScanErc>().await?;
    Ok(resp)
}

pub async fn start_previous_workers<R>(bot: AutoSend<Bot>, repo: R) where R: DataRepository {
    let wallets = repo.get_all_wallets_with_user();

    logger!("Starting previous workers...");
    for wallet in wallets {
        let user = match wallet.user {
            Some(u) => {
                u
            }
            None => {
                continue;
            }
        };

        let clone_bot = bot.clone();
        let user_id = user.chat_id.parse::<i64>().unwrap();
        let wallet_address = wallet.address.clone();

        task::spawn(async move {
            let mut repo = SqliteDb::get_connection();
            background_wallet_worker::<SqliteDb>(&clone_bot, ChatId(user_id),
                                                 wallet_address, user.id.unwrap(), &mut repo).await;
        });

        bot.send_message(ChatId(user_id), format!("Worker for {} wallet started.", wallet.address).as_str()).await.unwrap();
    }
}
