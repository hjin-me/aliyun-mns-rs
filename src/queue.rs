use crate::client::Client;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct Queue {
    pub name: String,
    client: Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "Message")]
struct MessageSendRequest {
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

impl Queue {
    pub fn new(name: &str, c: &Client) -> Self {
        Self {
            name: name.to_string(),
            client: c.clone(),
        }
    }

    pub async fn send_msg(&self, msg_body: &str, delay_secs: Option<u32>, priority: Option<u8>) {
        let m = MessageSendRequest {
            message_body: msg_body.to_string(),
            delay_seconds: delay_secs.unwrap_or(0),
            priority: priority.unwrap_or(7) + 1,
        };
        self.client.request(
            &format!("/queues/{}/messages", self.name),
            "POST",
            "application/xml",
            &serde_xml_rs::to_string(&m).unwrap(),
        )
            .await
            .unwrap();
    }
}

#[cfg(test)]
mod test {
    use serde_xml_rs::{from_str, to_string};
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
}