use crate::conf::get_conf;
use mns::client::Client;
use mns::queue::{MessageSendRequest, Queue};

pub mod conf;

#[tokio::test]
async fn test_send_msg() {
    let conf = dbg!(get_conf());

    let c = Client::new(conf.endpoint.as_str(), conf.id.as_str(), conf.sec.as_str());
    let q = Queue::new(conf.queue.as_str(), &c);
    let r = dbg!(q
        .send_message(&MessageSendRequest {
            message_body: "aa".to_string(),
            delay_seconds: 1,
            priority: 9,
        })
        .await
        .unwrap());
    dbg!(q
        .delete_message(r.receipt_handle.unwrap().as_str())
        .await
        .unwrap());

    let r = q
        .send_message(&MessageSendRequest {
            message_body: "aa".to_string(),
            delay_seconds: 0,
            priority: 8,
        })
        .await
        .unwrap();
    dbg!(r);
}
