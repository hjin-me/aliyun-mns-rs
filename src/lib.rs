//! Aliyun Message Service (MNS) SDK for Rust
//! ![docs.rs](https://img.shields.io/docsrs/aliyun-mns?style=for-the-badge)
//!
//! # Example
//! ```rust
//! use mns::Client;
//! use mns::Queue;
//! use mns::queue::MessageSendRequest;
//! #[tokio::main]
//! async fn main() {
//!     let client = Client::new("https://xxx.mns.cn-hangzhou.aliyuncs.com", "your id", "your key");
//!     let queue = Queue::new("your queue name", &client);
//!     queue.send_message(&MessageSendRequest {
//!         message_body: "aa".to_string(),
//!         delay_seconds: Some(1),
//!         priority: Some(9),
//!     }).await.unwrap();
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
