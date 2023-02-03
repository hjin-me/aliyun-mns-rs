use mns::client::Client;
use mns::queue::Queue;
use crate::conf::get_conf;

mod conf;

#[tokio::test]
async fn test_send_msg() {
    let conf = dbg!(get_conf());

    let c = Client::new(conf.endpoint.as_str(), conf.id.as_str(), conf.sec.as_str());
    let q = Queue::new(conf.queue.as_str(), &c);
    let r = q.send_msg("aa", Some(1), Some(8)).await.unwrap();
    dbg!(r);
    let r = q.send_msg("aa", None, None).await.unwrap();
    dbg!(r);
}