#[derive(Debug, Clone)]
pub struct Config {
    pub endpoint: String,
    pub id: String,
    pub sec: String,
    pub queue: String,
}

pub fn get_conf() -> Config {
    Config {
        endpoint: env!("MNS_ENDPOINT").to_string(),
        id: env!("MNS_ID").to_string(),
        sec: env!("MNS_SEC").to_string(),
        queue: env!("MNS_QUEUE").to_string(),
    }
}
