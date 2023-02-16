use crate::client::Client;
use crate::error::Error::{
    DeserializeErrorResponseFailed, DeserializeResponseFailed, SerializeMessageFailed,
};
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Queue {
    pub name: String,
    client: Client,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename = "Message")]
pub struct MessageSendRequest {
    #[serde(rename = "MessageBody")]
    pub message_body: String,
    #[serde(rename = "DelaySeconds")]
    pub delay_seconds: Option<u32>,
    #[serde(rename = "Priority")]
    pub priority: Option<u8>,
}
impl MessageSendRequest {
    pub fn to_xml(&self) -> String {
        let delay_seconds = self.delay_seconds.map_or_else(
            || "".to_string(),
            |v| format!("<DelaySeconds>{v}</DelaySeconds>"),
        );
        let priority = self
            .priority
            .map_or_else(|| "".to_string(), |v| format!("<Priority>{v}</Priority>"));
        format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Message><MessageBody><![CDATA[{}]]></MessageBody>{}{}</Message>",
            self.message_body,
            delay_seconds,
            priority
        )
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename = "Messages")]
struct MessageBatchSendRequest {
    #[serde(rename = "Message")]
    pub messages: Vec<MessageSendRequest>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename = "Messages")]
struct MessageBatchSendResponse {
    #[serde(rename = "Message")]
    pub messages: Vec<MessageSendResponse>,
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
        write!(
            f,
            "ali mns err, code: {}, message: {}, request_id: {}, host_id: {}",
            self.code, self.message, self.request_id, self.host_id
        )
    }
}

impl Queue {
    pub fn new(name: &str, c: &Client) -> Self {
        Self {
            name: name.to_string(),
            client: c.clone(),
        }
    }

    pub async fn send_message(&self, m: &MessageSendRequest) -> Result<MessageSendResponse> {
        let (status_code, v) = self
            .client
            .request(
                &format!("/queues/{}/messages", self.name),
                "POST",
                "application/xml",
                &m.to_xml(),
            )
            .await?;
        if status_code.is_success() {
            let res: MessageSendResponse =
                serde_xml_rs::from_reader(v.as_slice()).map_err(DeserializeResponseFailed)?;
            Ok(res)
        } else {
            let res: ErrorResponse =
                serde_xml_rs::from_reader(v.as_slice()).map_err(DeserializeErrorResponseFailed)?;
            Err(res.into())
        }
    }

    pub async fn batch_send_messages(
        &self,
        ms: &Vec<MessageSendRequest>,
    ) -> Result<Vec<MessageSendResponse>> {
        let (status_code, v) = self
            .client
            .request(
                &format!("/queues/{}/messages", self.name),
                "POST",
                "application/xml",
                &serde_xml_rs::to_string(&ms).map_err(SerializeMessageFailed)?,
            )
            .await?;
        if status_code.is_success() {
            let res: MessageBatchSendResponse =
                serde_xml_rs::from_reader(v.as_slice()).map_err(DeserializeResponseFailed)?;
            Ok(res.messages)
        } else {
            let res: ErrorResponse =
                serde_xml_rs::from_reader(v.as_slice()).map_err(DeserializeErrorResponseFailed)?;
            Err(res.into())
        }
    }

    pub async fn receive_message(
        &self,
        wait_seconds: Option<i32>,
    ) -> Result<MessageReceiveResponse> {
        let resource = wait_seconds.map_or_else(
            || format!("/queues/{}/messages", self.name),
            |w| format!("/queues/{}/messages?waitseconds={}", self.name, w),
        );
        let (status_code, v) = self
            .client
            .request(&resource, "GET", "application/xml", "")
            .await?;
        if status_code.is_success() {
            let res: MessageReceiveResponse =
                serde_xml_rs::from_reader(v.as_slice()).map_err(DeserializeResponseFailed)?;
            Ok(res)
        } else {
            let res: ErrorResponse =
                serde_xml_rs::from_reader(v.as_slice()).map_err(DeserializeErrorResponseFailed)?;
            Err(res.into())
        }
    }
    pub async fn delete_message(&self, receipt_handle: &str) -> Result<()> {
        let (status_code, v) = self
            .client
            .request(
                &format!(
                    "/queues/{}/messages?ReceiptHandle={}",
                    self.name, receipt_handle
                ),
                "DELETE",
                "application/xml",
                "",
            )
            .await?;
        if status_code.is_success() {
            Ok(())
        } else {
            let res: ErrorResponse =
                serde_xml_rs::from_reader(v.as_slice()).map_err(DeserializeErrorResponseFailed)?;
            Err(res.into())
        }
    }

    pub async fn change_message_visibility(
        &self,
        receipt_handle: &str,
        visibility_timeout: i32,
    ) -> Result<MessageVisibilityChangeResponse> {
        let (status_code, v) = self
            .client
            .request(
                &format!(
                    "/queues/{}/messages?ReceiptHandle={}&VisibilityTimeout={}",
                    self.name, receipt_handle, visibility_timeout
                ),
                "PUT",
                "application/xml",
                "",
            )
            .await?;
        if status_code.is_success() {
            let res: MessageVisibilityChangeResponse =
                serde_xml_rs::from_reader(v.as_slice()).map_err(DeserializeResponseFailed)?;
            Ok(res)
        } else {
            let res: ErrorResponse =
                serde_xml_rs::from_reader(v.as_slice()).map_err(DeserializeErrorResponseFailed)?;
            Err(res.into())
        }
    }

    pub async fn peek_message(&self) -> Result<MessageReceiveResponse> {
        let (status_code, v) = self
            .client
            .request(
                &format!("/queues/{}/messages?peekonly=true", self.name),
                "GET",
                "application/xml",
                "",
            )
            .await?;
        if status_code.is_success() {
            let res: MessageReceiveResponse =
                serde_xml_rs::from_reader(v.as_slice()).map_err(DeserializeResponseFailed)?;
            Ok(res)
        } else {
            let res: ErrorResponse =
                serde_xml_rs::from_reader(v.as_slice()).map_err(DeserializeErrorResponseFailed)?;
            Err(res.into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_xml_rs::to_string;

    #[test]
    fn test_serde() {
        let src = r#"<?xml version="1.0" encoding="UTF-8"?><Message><MessageBody>aa</MessageBody><DelaySeconds>1</DelaySeconds><Priority>9</Priority></Message>"#;

        let m = MessageSendRequest {
            message_body: "aa".to_string(),
            delay_seconds: Some(1),
            priority: Some(9),
        };
        let reserialized_item = to_string(&m).unwrap();
        assert_eq!(src, reserialized_item);

        let src = r#"<?xml version="1.0" encoding="UTF-8"?><Message><MessageBody><![CDATA[aa]]></MessageBody></Message>"#;

        let m = MessageSendRequest {
            message_body: "aa".to_string(),
            delay_seconds: None,
            priority: None,
        };
        assert_eq!(src, m.to_xml());

        let src = r#"<?xml version="1.0" encoding="UTF-8"?><Message><MessageBody><![CDATA[]]></MessageBody></Message>"#;

        let m = MessageSendRequest::default();
        assert_eq!(src, m.to_xml());
    }

    #[tokio::test]
    async fn test_send_message() {
        let c = Client::new(
            &std::env::var("MNS_ENDPOINT").unwrap(),
            &std::env::var("MNS_ID").unwrap(),
            &std::env::var("MNS_SEC").unwrap(),
        );
        let q = Queue::new(&std::env::var("MNS_QUEUE").unwrap(), &c);
        let r = q
            .send_message(&MessageSendRequest {
                message_body: "<aa href='abc'>".to_string(),
                delay_seconds: None,
                priority: Some(1),
            })
            .await
            .unwrap();
        dbg!(r);
    }

    #[tokio::test]
    async fn test_recv_message() {
        let c = Client::new(
            &std::env::var("MNS_ENDPOINT").unwrap(),
            &std::env::var("MNS_ID").unwrap(),
            &std::env::var("MNS_SEC").unwrap(),
        );
        let q = Queue::new(&std::env::var("MNS_QUEUE").unwrap(), &c);
        let r = q.receive_message(Some(10)).await.unwrap();
        dbg!(r);
    }
}
