use mns::queue::{MessageSendRequest, Queue};
use mns::Client;
use mns::QueueManager;
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
#[tokio::test]
async fn test_send_msg() {
    let conf = dbg!(get_conf());

    let c = Client::new(conf.endpoint.as_str(), conf.id.as_str(), conf.sec.as_str());
    let qm = QueueManager::new(&c);
    let q = Queue::new(conf.queue.as_str(), &c);

    let r = qm.get_queue_attributes(conf.queue.as_str()).await.unwrap();
    for _ in 0..(r.active_messages - 1) {
        let m = q.receive_message(None).await.unwrap();
        dbg!(q.delete_message(m.receipt_handle.as_str()).await.unwrap());
    }
    let r = dbg!(q
        .send_message(&MessageSendRequest {
            message_body: "aa".to_string(),
            delay_seconds: Some(1),
            priority: Some(9),
        })
        .await
        .unwrap());
    dbg!(q
        .delete_message(r.receipt_handle.unwrap().as_str())
        .await
        .unwrap());

    dbg!(q.receive_message(None).await.unwrap());
    let r = q
        .send_message(&MessageSendRequest {
            message_body: "aa".to_string(),
            delay_seconds: None,
            priority: None,
        })
        .await
        .unwrap();
    dbg!(r);
    dbg!(q.batch_receive_message(10, None).await.unwrap());
}
