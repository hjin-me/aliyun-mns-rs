use crate::client::Client;
use anyhow::Result;
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
    message_body: String,
    #[serde(rename = "DelaySeconds")]
    delay_seconds: u32,
    #[serde(rename = "Priority")]
    priority: u8,
    // XMLName      xml.Name `xml:"Message" json:"-"`
    // MessageBody  string   `xml:"MessageBody" json:"message_body"`
    // DelaySeconds int64    `xml:"DelaySeconds" json:"delay_seconds"`
    // Priority     int64    `xml:"Priority" json:"priority"`
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "Error")]
pub struct ErrorResponse {
    #[serde(rename = "Code")]
    code: String,
    #[serde(rename = "RequestId")]
    request_id: String,
    #[serde(rename = "HostId")]
    host_id: String,
    #[serde(rename = "Message")]
    message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "Message")]
pub struct MessageSendResponse {
    #[serde(rename = "MessageId")]
    message_id: String,
    #[serde(rename = "MessageBodyMD5")]
    message_body_md5: String,
    // ReceiptHandle is assigned when any DelayMessage is sent
    #[serde(rename = "ReceiptHandle")]
    receipt_handle: String,
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
            Err(anyhow::anyhow!("[{}]{}[{}]", res.code, res.message, res.request_id))
        }
    }
}

#[cfg(test)]
mod test {
    use serde_xml_rs::{to_string};
    use super::*;

    #[derive(Debug, Clone)]
    struct Config {
        endpoint: String,
        id: String,
        sec: String,
        queue: String,
    }

    fn get_conf() -> Config {
        Config {
            endpoint: env!("MNS_ENDPOINT").to_string(),
            id: env!("MNS_ID").to_string(),
            sec: env!("MNS_SEC").to_string(),
            queue: env!("MNS_QUEUE").to_string(),
        }
    }

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
}