use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Ping {
    pub status: String,
    #[serde(rename = "yourIp")]
    pub ipaddress: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct DomainRecord {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub record_type: String,
    pub content: Option<String>,
    pub ttl: Option<String>,
    pub prio: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct GetRecordsResponse {
    pub status: String,
    pub records: Vec<DomainRecord>,
}
