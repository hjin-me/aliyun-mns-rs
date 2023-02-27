use crate::options::ConsumeOptions;
use crate::Queue;
use anyhow::Result;
use std::future::Future;
use std::pin::Pin;

// TODO
pub type DeliveryResult = Result<Option<Delivery>>;

// TODO
#[derive(Debug, PartialEq)]
pub struct Delivery {
    pub data: Vec<u8>,
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

#[derive(Clone)]
pub struct Consumer {
    queue: Queue,
    options: ConsumeOptions,
}

impl Consumer {
    pub fn new(queue: Queue, options: ConsumeOptions) -> Consumer {
        Consumer { queue, options }
    }

    pub fn set_delegate<D: ConsumerDelegate + 'static>(&self, _delegate: D) {}
}
