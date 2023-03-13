#[derive(Debug, Clone)]
pub struct Config {
    pub endpoint: String,
    pub id: String,
    pub sec: String,
    pub queue: String,
}
pub fn get_conf() -> Config {
    Config {
        endpoint: std::env::var("MNS_ENDPOINT").unwrap(),
        id: std::env::var("MNS_ID").unwrap(),
        sec: std::env::var("MNS_SEC").unwrap(),
        queue: std::env::var("MNS_QUEUE").unwrap(),
    }
}
