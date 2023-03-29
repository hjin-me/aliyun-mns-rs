mod common;
use crate::common::get_conf;
use mns::consumer::{Consumer, DeliveryResult};
use mns::options::ConsumeOptions;
use mns::queue::{MessageSendRequest, QueueOperation};
use mns::{Client, Queue};

#[tokio::test]
async fn test_consumer() {
    let conf = dbg!(get_conf());

    let c = Client::new(conf.endpoint.as_str(), conf.id.as_str(), conf.sec.as_str());
    let q = Queue::new(conf.queue.as_str(), &c);
    for i in 0..4 {
        q.send_message(&MessageSendRequest {
            message_body: format!("aa{}", i),
            delay_seconds: None,
            priority: None,
        })
        .await
        .expect("send message failed");
    }
    let consumer = Consumer::new(q, ConsumeOptions::default());

    consumer
        .set_delegate(|msg: DeliveryResult| async move {
            let m = msg.unwrap().unwrap();
            dbg!(String::from_utf8(m.data.clone()).unwrap());
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        })
        .await;
    consumer.run();

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}
