use reqwest::Client;
use serde_json::json;

use crate::porkbun::{
    API_KEY, Result, SECRET_KEY,
    models::{DomainRecord, GetRecordsResponse, Ping},
};

// #[derive(Debug)]
// struct Error {
//     message: String,
// }
// impl Error {
//     pub fn new(message: &str) -> Self {
//         Self {
//             message: message.to_string(),
//         }
//     }
// }
// impl std::fmt::Display for Error {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str(&self.message)
//     }
// }
// impl std::error::Error for Error {}

pub async fn ping(client: &Client) -> Result<Ping> {
    let auth_json = json!({
        "secretapikey": *SECRET_KEY,
        "apikey": *API_KEY
    })
    .to_string();
    let ping_uri = "https://api.porkbun.com/api/json/v3/ping";
    let response = client.post(ping_uri).body(auth_json.clone()).send().await?;
    let ping = response
        .json::<Ping>()
        .await
        .expect("failed to deserialize body to expected format");
    Ok(ping)
}

pub async fn get_records(domain: &str, client: &Client) -> Result<Vec<DomainRecord>> {
    let auth_json = json!({
        "secretapikey": *SECRET_KEY,
        "apikey": *API_KEY
    })
    .to_string();
    let get_records_uri = "https://api.porkbun.com/api/json/v3/dns/retrieve/";
    let response = client
        .post(get_records_uri.to_owned() + domain)
        .body(auth_json.clone())
        .send()
        .await?;
    let records = response
        .json::<GetRecordsResponse>()
        .await
        .expect("failed to deserialize body to expected format")
        .records;
    Ok(records)
}

pub async fn update_record(domain: &str, record: DomainRecord, client: &Client) -> Result<()> {
    let payload_json = json!({
        "secretapikey": *SECRET_KEY,
        "apikey": *API_KEY,
        "name": record.name,
        "type": record.record_type,
        "content": record.content.unwrap(),
        "ttl": 600,
    })
    .to_string();

    let update_record_uri = "https://api.porkbun.com/api/json/v3/dns/edit/";
    let _ = client
        .post(update_record_uri.to_owned() + &domain + "/" + &record.id)
        .body(payload_json)
        .send()
        .await?;
    Ok(())
}

// fn parse_domain(text: &str) -> Result<String> {
//     let parts: Vec<&str> = text.split_inclusive(".").collect();
//     match parts.len() {
//         0..2 => {
//             println!("DEBUG: {:?}", parts);
//             Err(Box::new(Error::new("not enough parts")))
//         }
//         _ => {
//             println!("DEBUG: {:?}", parts.len());
//             Ok(parts[parts.len() - 2..parts.len()].concat())
//         }
//     }
// }
