use crate::client::Client;
use crate::error::Error::{DeserializeErrorResponseFailed, DeserializeResponseFailed};
use crate::error::Result;
use crate::queue::ErrorResponse;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename = "Queue")]
pub struct QueueAttribute {
    #[serde(rename = "QueueName")]
    pub queue_name: String,
    #[serde(rename = "DelaySeconds")]
    pub delay_seconds: i32,
    #[serde(rename = "MaximumMessageSize")]
    pub maximum_message_size: i32,
    #[serde(rename = "MessageRetentionPeriod")]
    pub message_retention_period: i32,
    #[serde(rename = "VisibilityTimeout")]
    pub visibility_timeout: i32,
    #[serde(rename = "PollingWaitSeconds")]
    pub polling_wait_seconds: i32,
    #[serde(rename = "ActiveMessages")]
    pub active_messages: i64,
    #[serde(rename = "InactiveMessages")]
    pub inactive_messages: i64,
    #[serde(rename = "DelayMessages")]
    pub delay_messages: i64,
    #[serde(rename = "CreateTime")]
    pub create_time: i64,
    #[serde(rename = "LastModifyTime")]
    pub last_modify_time: i64,
}
#[derive(Debug, Clone, Default, Deserialize)]
pub struct CreateQueueRequest {
    pub queue_name: String,
    pub delay_seconds: Option<i32>,
    pub maximum_message_size: Option<i32>,
    pub message_retention_period: Option<i32>,
    pub visibility_timeout: Option<i32>,
    pub polling_wait_seconds: Option<i32>,
    pub logging_enabled: Option<bool>,
}
impl Serialize for CreateQueueRequest {
    fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut msg = serializer.serialize_struct("Queue", 3)?;
        msg.serialize_field("QueueName", &self.queue_name)?;
        if let Some(d) = self.delay_seconds {
            msg.serialize_field("DelaySeconds", &d)?;
        }
        if let Some(v) = self.maximum_message_size {
            msg.serialize_field("MaximumMessageSize", &v)?;
        }
        if let Some(v) = self.message_retention_period {
            msg.serialize_field("MessageRetentionPeriod", &v)?;
        }
        if let Some(v) = self.visibility_timeout {
            msg.serialize_field("VisibilityTimeout", &v)?;
        }
        if let Some(v) = self.polling_wait_seconds {
            msg.serialize_field("PollingWaitSeconds", &v)?;
        }
        if let Some(v) = self.logging_enabled {
            let v = match v {
                true => "True",
                false => "False",
            };
            msg.serialize_field("LoggingEnabled", &v)?;
        }
        msg.end()
    }
}

/// 队列管理实例
/// https://help.aliyun.com/document_detail/140734.html
#[derive(Debug, Clone)]
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
                &serde_xml_rs::to_string(q).unwrap(),
                Some(5),
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
            .request(
                &format!("/queues/{name}"),
                "DELETE",
                "application/xml",
                "",
                Some(5),
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

    pub async fn get_queue_attributes(&self, queue: &str) -> Result<QueueAttribute> {
        let (status_code, v) = self
            .client
            .request(
                &format!("/queues/{queue}"),
                "GET",
                "application/xml",
                "",
                Some(5),
            )
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
    use crate::devtool::{get_client, get_conf};

    #[tokio::test]
    async fn test_queue_manager() {
        let conf = get_conf();
        let c = get_client();

        let qm = QueueManager::new(&c);
        qm.create_queue(&CreateQueueRequest {
            queue_name: conf.queue,
            ..CreateQueueRequest::default()
        })
        .await
        .unwrap();
        dbg!(qm.get_queue_attributes("sstest").await.unwrap());
    }
}
