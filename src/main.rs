use std::{net::{Ipv4Addr, Ipv6Addr}, str::FromStr};

use clap::Parser;

use crate::{
    cli::config::Config,
    porkbun::{
        actions::{get_records, ping, update_record},
        models::DomainRecord,
    },
};

mod cli;
mod porkbun;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let args = cli::Args::parse();

    let client = reqwest::Client::builder()
        .https_only(true)
        .local_address("[::]:0".parse().ok())
        .build()?;

    let ping = ping(&client).await?;
    println!("INFO: Ping! {}", ping.ipaddress);

    match args.command {
        cli::Command::LIST { domain } => {
            let records = get_records(&domain, &client).await?;
            for record in records {
                println!(
                    "ID: {} Name: {} Type: {} Content: {}",
                    record.id,
                    record.name,
                    record.record_type,
                    record.content.unwrap()
                );
            }
        }
        cli::Command::UPDATE => {
            let config = Config::read(&args.config_path)?;
            let record_type;
            let ipv4_result = Ipv4Addr::from_str(&ping.ipaddress);
            let ipv6_result = Ipv6Addr::from_str(&ping.ipaddress);
            if ipv6_result.is_ok() {
                record_type = "AAAA"
            } else if ipv4_result.is_ok() {
                record_type = "A"
            } else {
                panic!("not a valid ip address");
            }

            let records = config.domains;
            for record in records {
                let update = DomainRecord {
                    id: record.id,
                    name: record.name,
                    record_type: record_type.to_string(),
                    content: Some(ping.ipaddress.to_string()),
                    ttl: None,
                    prio: None,
                    notes: None,
                };
                update_record(&record.domain, update, &client).await?;
            }
        }
        cli::Command::PING => {}
    };

    Ok(())
}
