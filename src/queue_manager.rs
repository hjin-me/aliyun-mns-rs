use crate::client::Client;
use crate::error::Error::{DeserializeErrorResponseFailed, DeserializeResponseFailed};
use crate::error::Result;
use crate::queue::ErrorResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename = "Queue")]
pub struct QueueAttribute {
    #[serde(rename = "QueueName")]
    pub queue_name: String, //   `xml:"QueueName,omitempty" json:"queue_name,omitempty"`
    #[serde(rename = "DelaySeconds")]
    pub delay_seconds: i32, //    `xml:"DelaySeconds,omitempty" json:"delay_seconds,omitempty"`
    #[serde(rename = "MaximumMessageSize")]
    pub maximum_message_size: i32, //    `xml:"MaximumMessageSize,omitempty" json:"maximum_message_size,omitempty"`
    #[serde(rename = "MessageRetentionPeriod")]
    pub message_retention_period: i32, //    `xml:"MessageRetentionPeriod,omitempty" json:"message_retention_period,omitempty"`
    #[serde(rename = "VisibilityTimeout")]
    pub visibility_timeout: i32, //    `xml:"VisibilityTimeout,omitempty" json:"visibility_timeout,omitempty"`
    #[serde(rename = "PollingWaitSeconds")]
    pub polling_wait_seconds: i32, //    `xml:"PollingWaitSeconds,omitempty" json:"polling_wait_secods,omitempty"`
    #[serde(rename = "ActiveMessages")]
    pub active_messages: i64, //    `xml:"ActiveMessages,omitempty" json:"active_messages,omitempty"`
    #[serde(rename = "InactiveMessages")]
    pub inactive_messages: i64, //    `xml:"InactiveMessages,omitempty" json:"inactive_messages,omitempty"`
    #[serde(rename = "DelayMessages")]
    pub delay_messages: i64, //    `xml:"DelayMessages,omitempty" json:"delay_messages,omitempty"`
    #[serde(rename = "CreateTime")]
    pub create_time: i64, //    `xml:"CreateTime,omitempty" json:"create_time,omitempty"`
    #[serde(rename = "LastModifyTime")]
    pub last_modify_time: i64, //    `xml:"LastModifyTime,omitempty" json:"last_modify_time,omitempty"`
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateQueueRequest {
    pub queue_name: String,
    pub delay_seconds: Option<i32>,
    pub maximum_message_size: Option<i32>,
    pub message_retention_period: Option<i32>,
    pub visibility_timeout: Option<i32>,
    pub polling_wait_seconds: Option<i32>,
    pub logging_enabled: Option<bool>,
}
impl CreateQueueRequest {
    fn to_xml(&self) -> String {
        let mut s = String::new();
        s.push_str("<Queue>");
        s.push_str(&format!("<QueueName>{}</QueueName>", self.queue_name));
        if let Some(v) = self.delay_seconds {
            s.push_str(&format!("<DelaySeconds>{v}</DelaySeconds>"));
        }
        if let Some(v) = self.maximum_message_size {
            s.push_str(&format!("<MaximumMessageSize>{v}</MaximumMessageSize>"));
        }
        if let Some(v) = self.message_retention_period {
            s.push_str(&format!(
                "<MessageRetentionPeriod>{v}</MessageRetentionPeriod>"
            ));
        }
        if let Some(v) = self.visibility_timeout {
            s.push_str(&format!("<VisibilityTimeout>{v}</VisibilityTimeout>"));
        }
        if let Some(v) = self.polling_wait_seconds {
            s.push_str(&format!("<PollingWaitSeconds>{v}</PollingWaitSeconds>"));
        }
        if let Some(v) = self.logging_enabled {
            let v = match v {
                true => "True",
                false => "False",
            };
            s.push_str(&format!("<LoggingEnabled>{v}</LoggingEnabled>"));
        }
        s.push_str("</Queue>");
        s
    }
}

pub struct QueueManager {
    client: Client,
}

impl QueueManager {
    pub fn new(c: &Client) -> Self {
        Self { client: c.clone() }
    }

    // pub async fn list_queues(&self) -> Result<Vec<Queue>> {
    //     let (status_code, v) = self
    //         .client
    //         .request("/queues", "GET", "application/xml", "")
    //         .await?;
    //     if status_code.is_success() {
    //         let res: QueueListResponse =
    //             serde_xml_rs::from_reader(v.as_slice()).map_err(DeserializeResponseFailed)?;
    //         Ok(res.queues)
    //     } else {
    //         let res: ErrorResponse =
    //             serde_xml_rs::from_reader(v.as_slice()).map_err(DeserializeErrorResponseFailed)?;
    //         Err(res.into())
    //     }
    // }
    //
    pub async fn create_queue(&self, q: &CreateQueueRequest) -> Result<()> {
        let (status_code, v) = self
            .client
            .request(
                &format!("/queues/{}", q.queue_name),
                "PUT",
                "application/xml",
                &q.to_xml(),
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
    pub async fn delete_queue(&self, name: &str) -> Result<()> {
        let (status_code, v) = self
            .client
            .request(&format!("/queues/{name}"), "DELETE", "application/xml", "")
            .await?;
        if status_code.is_success() {
            Ok(())
        } else {
            let res: ErrorResponse =
                serde_xml_rs::from_reader(v.as_slice()).map_err(DeserializeErrorResponseFailed)?;
            Err(res.into())
        }
    }

    pub async fn get_queue_attributes(&self, queue: &str) -> Result<QueueAttribute> {
        let (status_code, v) = self
            .client
            .request(&format!("/queues/{queue}"), "GET", "application/xml", "")
            .await?;

        if status_code.is_success() {
            let res: QueueAttribute =
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

    #[tokio::test]
    async fn test_queue_manager() {
        let c = Client::new(
            &std::env::var("MNS_ENDPOINT").unwrap(),
            &std::env::var("MNS_ID").unwrap(),
            &std::env::var("MNS_SEC").unwrap(),
        );
        let qm = QueueManager::new(&c);
        qm.create_queue(&CreateQueueRequest {
            queue_name: "sstest".to_string(),
            ..CreateQueueRequest::default()
        })
        .await
        .unwrap();
        dbg!(qm.get_queue_attributes("sstest").await.unwrap());
    }
}
