use clap::Parser;
use teloxide::prelude2::*;
use xmu_health_report_rust_sdk::{
    create_client, get_system_date, is_today_reported, login, get_continuous_report_day_count,
  };


/// send xmu health checkin status to telegram user 
#[derive(Parser, Debug)]
#[clap(author = "wsyxbcl <wsyxbcl@gmail.com>", version = "1.0")]
struct Args {
    /// chat_id of tg user
    #[clap(short, long)]
    chat_id: i64,

    /// username of xmu
    #[clap(short, long)]
    username: String,

    /// password of xmu
    #[clap(short, long)]
    password: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    // bot initiate
    teloxide::enable_logging!();
    log::info!("Starting simple_commands_bot...");
    let bot = Bot::from_env().auto_send();

    let username = args.username;
    let password = args.password;
    let client = create_client().await?;
    login(&client, username.as_str(), password.as_str()).await?;
    let (today, form_date) = is_today_reported(&client).await?;
    let days_count = get_continuous_report_day_count(&client).await?;
    bot.send_message(args.chat_id, 
                format!("Local date: {}\n Server date: {}\n Status: {}, checked for {} days", 
                        get_system_date(), form_date, if today { "check" } else { "uncheck" }, days_count)).await?;
    Ok(())
}