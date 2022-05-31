use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[structopt(short = "bt", long = "bot-token", env = "BOT_TOKEN")]
    pub bot_token: String,

    #[structopt(short = "ea", long = "ether-api", env = "ETHER_API")]
    pub ether_api: String,

    #[structopt(short = "db", long = "db", env = "DB_PATH")]
    pub db_path: String,
}