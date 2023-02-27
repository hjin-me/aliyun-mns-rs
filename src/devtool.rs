// 本地开发测试使用
use crate::Client;

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
pub fn get_client() -> Client {
    let conf = get_conf();
    Client::new(&conf.endpoint, &conf.id, &conf.sec)
}
pub fn get_client_with_wrong_key() -> Client {
    let conf = get_conf();
    Client::new(&conf.endpoint, &conf.id, "wrong_key")
}
pub fn get_queue_name() -> String {
    let conf = get_conf();
    conf.queue
}
