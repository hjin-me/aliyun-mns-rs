use mns::client::Client;
use mns::conf::get_conf;
use mns::queue::Queue;


#[tokio::test]
async fn test_send_msg() {
    let conf = dbg!(get_conf());

    let c = Client::new(conf.endpoint.as_str(), conf.id.as_str(), conf.sec.as_str());
    let q = Queue::new(conf.queue.as_str(), &c);
    let r = dbg!(q.send_msg("aa", Some(1), Some(8)).await.unwrap());
    dbg!(q.delete_msg(r.receipt_handle.unwrap().as_str()).await.unwrap());

    let r = q.send_msg("aa", None, None).await.unwrap();
    dbg!(r);
}