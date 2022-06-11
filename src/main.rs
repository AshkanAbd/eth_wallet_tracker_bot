extern crate core;

mod app_config;
mod models;
mod repositories;
mod commands;
mod command_handler;
mod common;

use crate::app_config::AppConfig;
use crate::repositories::{DataRepository};
use crate::commands::{Command};
use structopt::StructOpt;
use teloxide::{prelude::*, utils::command::BotCommands};
use crate::command_handler::{handler};
use crate::common::{notice_changelog, start_previous_workers};
use crate::repositories::sqlite_db::SqliteDb;

#[tokio::main]
async fn main() {
    let app_config = AppConfig::from_args();

    logger!("Starting bot...");
    let bot = Bot::new(app_config.bot_token).auto_send();

    let worker_db = SqliteDb::get_connection();
    let notice_db = SqliteDb::get_connection();
    let bot_clone = bot.clone();

    notice_changelog::<SqliteDb>(bot_clone.clone(), notice_db).await;
    start_previous_workers::<SqliteDb>(bot_clone, worker_db).await;

    teloxide::commands_repl(bot, handler, Command::ty()).await;
}
