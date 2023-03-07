//! Aliyun Message Service (MNS) SDK for Rust
//!
//! # Example
//! ```rust
//! async fn main() {
//!     let client = aliyun_mns::Client::new("https://xxx.mns.cn-hangzhou.aliyuncs.com", "your id", "your key");
//!     let queue = aliyun_mns::Queue::new("your queue name", &client);
//!     queue.send_message(&aliyun_mns::queue::MessageSendRequest {
//!         message_body: "aa".to_string(),
//!         delay_seconds: Some(1),
//!         priority: Some(9),
//!     }).await;
//! }
//! ```
pub mod client;
pub mod consumer;
#[cfg(test)]
pub mod devtool;
pub mod error;
pub mod options;
pub mod queue;
pub mod queue_manager;

pub type Queue = queue::Queue;
pub type Client = client::Client;
pub type QueueManager = queue_manager::QueueManager;
