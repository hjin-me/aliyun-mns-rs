//! Consumer
//! 参考 Lapin 的 Consumer 实现，使用 tokio 进行异步处理
//!
//! # Example
//! ```rust
//! use mns::Client;
//! use mns::Queue;
//! use mns::queue::MessageSendRequest;
//! use mns::consumer::{Consumer, DeliveryResult, ConsumeOptions};
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("https://xxx.mns.cn-hangzhou.aliyuncs.com", "your id", "your key");
//!     let queue = Queue::new("your queue name", &client);
//!     let consumer = Consumer::new(queue, ConsumeOptions::default());
//!     consumer
//!         .set_delegate(|msg: DeliveryResult| async move {
//!             let m = msg.unwrap();
//!             dbg!(m);
//!         })
//!         .await;
//!     consumer.run();
//! }
//! ```
pub use crate::options::ConsumeOptions;
use crate::queue::QueueOperation;
use crate::Queue;
use anyhow::Result;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
#[cfg(feature = "tokio")]
use tokio::sync::{Mutex, Semaphore};
use tracing::warn;

pub type DeliveryResult = Result<Option<Delivery>>;

#[derive(Debug, Clone)]
pub struct Delivery {
    pub data: Vec<u8>,
    receipt_handle: String,
    next_visible_time: i64,
    queue: Queue,
    // WIP
    // auto_ack: bool,
}

impl Delivery {
    pub async fn ack(&self) -> Result<()> {
        // delete
        Ok(self
            .queue
            .delete_message(self.receipt_handle.as_str())
            .await?)
    }
    pub async fn reject(&self) -> Result<()> {
        // change visibility
        Ok(self
            .queue
            .change_message_visibility(self.receipt_handle.as_str(), 1)
            .await
            .map(|_| ())?)
    }
}

impl Drop for Delivery {
    fn drop(&mut self) {}
}

pub trait ConsumerDelegate: Send + Sync {
    fn on_new_delivery(&self, delivery: DeliveryResult)
        -> Pin<Box<dyn Future<Output = ()> + Send>>;
    fn drop_prefetched_messages(&self) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(async move {})
    }
}

impl<
        F: Future<Output = ()> + Send + 'static,
        DeliveryHandler: Fn(DeliveryResult) -> F + Send + Sync + 'static,
    > ConsumerDelegate for DeliveryHandler
{
    fn on_new_delivery(
        &self,
        delivery: DeliveryResult,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        Box::pin(self(delivery))
    }
}

struct ConsumerInner {
    pub delegate: Option<Arc<Box<dyn ConsumerDelegate>>>,
}

#[derive(Clone)]
pub struct Consumer {
    queue: Queue,
    options: ConsumeOptions,
    inner: Arc<Mutex<ConsumerInner>>,
}

impl Consumer {
    pub fn new(queue: Queue, options: ConsumeOptions) -> Consumer {
        Consumer {
            queue,
            options,
            inner: Arc::new(Mutex::new(ConsumerInner { delegate: None })),
        }
    }

    #[cfg(feature = "tokio")]
    pub async fn set_delegate<D: ConsumerDelegate + 'static>(&self, delegate: D) {
        let mut inner = self.inner.lock().await;
        inner.delegate = Some(Arc::new(Box::new(delegate)));
    }

    #[cfg(feature = "tokio")]
    pub fn run(&self) {
        let c = self.clone();
        tokio::spawn(async move {
            // let semaphore = Semaphore::new(3);
            let semaphore = Arc::new(Semaphore::new(c.options.prefetch_count as usize));
            loop {
                let c = c.clone();
                let permit = semaphore.clone().acquire_owned().await.unwrap();
                let m = match c.queue.receive_message(Some(30)).await {
                    Ok(m) => m,
                    Err(e) => {
                        warn!("receive message error, {:?}", e);
                        return;
                    }
                };

                let d = Delivery {
                    data: m.message_body.into_bytes(),
                    receipt_handle: m.receipt_handle,
                    next_visible_time: m.next_visible_time,
                    queue: c.queue.clone(),
                    // auto_ack: c.options.auto_ack,
                };
                let inner = c.inner.lock().await;
                if let Some(delegate) = inner.delegate.as_ref() {
                    let delegate = delegate.clone();
                    tokio::spawn(async move {
                        let _permit = permit;
                        delegate.on_new_delivery(Ok(Some(d))).await;
                    });
                }
            }
        });
    }
}

impl Queue {
    fn consumer(&self, opt: ConsumeOptions) -> Consumer {
        Consumer::new(self.clone(), opt)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConsumerState {
    Active,
    ActiveWithDelegate,
    Canceling,
    Canceled,
}
