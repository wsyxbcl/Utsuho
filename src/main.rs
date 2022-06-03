use clap::Parser;
use teloxide::prelude2::*;
use xmu_health_report_rust_sdk::{
    create_client, is_today_reported, login, get_continuous_report_day_count, report, ReportStage,
  };

use std::collections::HashMap;

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
    let bot = Bot::from_env().auto_send();

    let username = args.username;
    let password = args.password;
    let client = create_client().await?;
    login(&client, username.as_str(), password.as_str()).await?;

    let report_result = report(&client, &HashMap::new()).await?;

    bot.send_message(args.chat_id, 
        format!(
                "Checking in... : {}",
                if report_result.status_code == ReportStage::ReportSuccess {
                "CHECKED"
                } else {
                "FAILED"
                }
            )
        ).await?;

    let (today, _) = is_today_reported(&client).await?;
    let days_count = get_continuous_report_day_count(&client).await?;
    bot.send_message(args.chat_id, 
                format!("Status: {}, checked for {} days", 
                        if today { "checked" } else { "unchecked" }, days_count)).await?;
    Ok(())
}