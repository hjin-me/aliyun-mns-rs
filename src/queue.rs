use std::fmt::Display;
use crate::client::Client;
use anyhow::Result;
use thiserror::Error;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct Queue {
    pub name: String,
    client: Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "Message")]
pub struct MessageSendRequest {
    #[serde(rename = "MessageBody")]
    pub message_body: String,
    #[serde(rename = "DelaySeconds")]
    pub delay_seconds: u32,
    #[serde(rename = "Priority")]
    pub priority: u8,
    // XMLName      xml.Name `xml:"Message" json:"-"`
    // MessageBody  string   `xml:"MessageBody" json:"message_body"`
    // DelaySeconds int64    `xml:"DelaySeconds" json:"delay_seconds"`
    // Priority     int64    `xml:"Priority" json:"priority"`
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "Message")]
pub struct MessageReceiveResponse {
    #[serde(rename = "MessageId")]
    pub message_id: String,
    #[serde(rename = "ReceiptHandle")]
    pub receipt_handle: String,
    #[serde(rename = "MessageBodyMD5")]
    pub message_body_md5: String,
    #[serde(rename = "MessageBody")]
    pub message_body: String,
    #[serde(rename = "EnqueueTime")]
    pub enqueue_time: i64,
    #[serde(rename = "NextVisibleTime")]
    pub next_visible_time: i64,
    #[serde(rename = "FirstDequeueTime")]
    pub first_dequeue_time: i64,
    #[serde(rename = "DequeueCount")]
    pub dequeue_count: i64,
    #[serde(rename = "Priority")]
    pub priority: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "ChangeVisibility")]
pub struct MessageVisibilityChangeResponse {
    #[serde(rename = "ReceiptHandle")]
    receipt_handle: String,
    #[serde(rename = "NextVisibleTime")]
    next_visible_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "Error")]
pub struct ErrorResponse {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
    #[serde(rename = "HostId")]
    pub host_id: String,
    #[serde(rename = "Message")]
    pub message: String,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, code: {}, request_id: {}, host_id: {}", self.message, self.code, self.request_id, self.host_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "Message")]
pub struct MessageSendResponse {
    #[serde(rename = "MessageId")]
    pub message_id: String,
    #[serde(rename = "MessageBodyMD5")]
    pub message_body_md5: String,
    // ReceiptHandle is assigned when any DelayMessage is sent
    #[serde(rename = "ReceiptHandle")]
    pub receipt_handle: Option<String>,
}

#[derive(Error, Debug)]
pub enum QueueError {
    #[error("Unknown error: {0}")]
    Unknown(ErrorResponse),
}

impl Queue {
    pub fn new(name: &str, c: &Client) -> Self {
        Self {
            name: name.to_string(),
            client: c.clone(),
        }
    }

    pub async fn send_msg(&self, msg_body: &str, delay_secs: Option<u32>, priority: Option<u8>) -> Result<MessageSendResponse> {
        let m = MessageSendRequest {
            message_body: msg_body.to_string(),
            delay_seconds: delay_secs.unwrap_or(0),
            priority: priority.unwrap_or(7) + 1,
        };
        let (status_code, v) = self.client.request(
            &format!("/queues/{}/messages", self.name),
            "POST",
            "application/xml",
            &serde_xml_rs::to_string(&m).unwrap(),
        )
            .await?;
        if status_code.is_success() {
            let res: MessageSendResponse = serde_xml_rs::from_reader(v.as_slice())?;
            Ok(res)
        } else {
            let res: ErrorResponse = serde_xml_rs::from_reader(v.as_slice())?;
            Err(QueueError::Unknown(res).into())
            // Err(anyhow::anyhow!("[{}]{}[{}]", res.code, res.message, res.request_id))
        }
    }
    pub async fn receive_msg(&self, wait_seconds: Option<i32>) -> Result<MessageReceiveResponse> {
        let resource = wait_seconds.map_or_else(|| format!("/queues/{}/messages", self.name), |w| format!("/queues/{}/messages?waitseconds={}", self.name, w));
        let (status_code, v) = self.client.request(
            &resource,
            "GET",
            "application/xml",
            "",
        )
            .await?;
        if status_code.is_success() {
            let res: MessageReceiveResponse = serde_xml_rs::from_reader(v.as_slice())?;
            Ok(res)
        } else {
            let res: ErrorResponse = serde_xml_rs::from_reader(v.as_slice())?;
            Err(QueueError::Unknown(res).into())
            // Err(anyhow::anyhow!("[{}]{}[{}]", res.code, res.message, res.request_id))
        }
    }
    pub async fn delete_msg(&self, receipt_handle: &str) -> Result<()> {
        let (status_code, v) = self.client.request(
            &format!("/queues/{}/messages?ReceiptHandle={}", self.name, receipt_handle),
            "DELETE",
            "application/xml",
            "",
        )
            .await?;
        if status_code.is_success() {
            Ok(())
        } else {
            let res: ErrorResponse = serde_xml_rs::from_reader(v.as_slice())?;
            Err(QueueError::Unknown(res).into())
            // Err(anyhow::anyhow!("[{}]{}[{}]", res.code, res.message, res.request_id))
        }
    }

    pub async fn change_msg_visibility(&self, receipt_handle: &str, visibility_timeout: i32) -> Result<MessageVisibilityChangeResponse> {
        let (status_code, v) = self.client.request(
            &format!("/queues/{}/messages?ReceiptHandle={}&VisibilityTimeout={}", self.name, receipt_handle, visibility_timeout),
            "PUT",
            "application/xml",
            "",
        )
            .await?;
        if status_code.is_success() {
            let res: MessageVisibilityChangeResponse = serde_xml_rs::from_reader(v.as_slice())?;
            Ok(res)
        } else {
            let res: ErrorResponse = serde_xml_rs::from_reader(v.as_slice())?;
            Err(QueueError::Unknown(res).into())
        }
    }
}

#[cfg(test)]
mod test {
    use serde_xml_rs::{to_string};
    use crate::conf::get_conf;
    use super::*;


    #[test]
    fn test_serde() {
        let src = r#"<?xml version="1.0" encoding="UTF-8"?><Message><MessageBody>aa</MessageBody><DelaySeconds>1</DelaySeconds><Priority>9</Priority></Message>"#;

        let m = MessageSendRequest {
            message_body: "aa".to_string(),
            delay_seconds: 1,
            priority: 9,
        };
        let reserialized_item = to_string(&m).unwrap();
        assert_eq!(src, reserialized_item);
    }

    #[tokio::test]
    async fn test_send_msg() {
        let conf = dbg!(get_conf());

        let c = Client::new(conf.endpoint.as_str(), conf.id.as_str(), conf.sec.as_str());
        let q = Queue::new(conf.queue.as_str(), &c);
        let r = q.send_msg("aa", Some(1), Some(8)).await.unwrap();
        dbg!(r);
    }

    #[tokio::test]
    async fn test_recv_msg() {
        let conf = dbg!(get_conf());

        let c = Client::new(conf.endpoint.as_str(), conf.id.as_str(), conf.sec.as_str());
        let q = Queue::new(conf.queue.as_str(), &c);
        let r = q.receive_msg(Some(10)).await.unwrap();
        dbg!(r);
    }
}