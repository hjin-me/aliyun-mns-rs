use crate::queue::ErrorResponse;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("sign message failed")]
    SignMessageFailed,
    #[error("serialize message failed: {0}")]
    SerializeMessageFailed(serde_xml_rs::Error),
    #[error("create new request failed: {0}")]
    GeneralAuthHeaderFailed(#[from] anyhow::Error),
    #[error("create new request failed")]
    CreateNewRequestFailed,
    #[error("send request failed")]
    SendReQuestFailed,
    #[error("read response body failed")]
    ReadResponseBodyFailed,
    #[error("deserialized error response failed: {0}")]
    DeserializeErrorResponseFailed(serde_xml_rs::Error),
    #[error("deserialized response failed: {0}")]
    DeserializeResponseFailed(serde_xml_rs::Error),
    #[error("decode body failed")]
    DecodeBodyFailed,
    #[error("get body decode element error")]
    GetBodyDecodeElementError,

    #[error("unknown error: {0}")]
    MNSUnknown(ErrorResponse),
    #[error("{0}")]
    MNSAccessDenied(ErrorResponse),
    #[error("{0}")]
    MNSInvalidAccessKeyId(ErrorResponse),
    #[error("{0}")]
    MNSInternalError(ErrorResponse),
    #[error("{0}")]
    MNSInvalidAuthorizationHeader(ErrorResponse),
    #[error("{0}")]
    MNSInvalidDateHeader(ErrorResponse),
    #[error("{0}")]
    MNSInvalidArgument(ErrorResponse),
    #[error("{0}")]
    MNSInvalidDigest(ErrorResponse),
    #[error("{0}")]
    MNSInvalidReQuestUrl(ErrorResponse),
    #[error("{0}")]
    MNSInvalidQueryString(ErrorResponse),
    #[error("{0}")]
    MNSMalformedXml(ErrorResponse),
    #[error("{0}")]
    MNSMissingAuthorizationHeader(ErrorResponse),
    #[error("{0}")]
    MNSMissingDateHeader(ErrorResponse),
    #[error("{0}")]
    MNSMissingVersionHeader(ErrorResponse),
    #[error("{0}")]
    MNSMissingReceiptHandle(ErrorResponse),
    #[error("{0}")]
    MNSMissingVisibilityTimeout(ErrorResponse),
    #[error("{0}")]
    MNSMessageNotExist(ErrorResponse),
    #[error("{0}")]
    MNSQueueAlreadyExist(ErrorResponse),
    #[error("{0}")]
    MNSQueueDeletedRecently(ErrorResponse),
    #[error("{0}")]
    MNSInvalidQueueName(ErrorResponse),
    #[error("{0}")]
    MNSInvalidVersionHeader(ErrorResponse),
    #[error("{0}")]
    MNSInvalidContentType(ErrorResponse),
    #[error("{0}")]
    MNSQueueNameLengthError(ErrorResponse),
    #[error("{0}")]
    MNSQueueNotExist(ErrorResponse),
    #[error("{0}")]
    MNSReceiptHandleError(ErrorResponse),
    #[error("{0}")]
    MNSSignatureDoesNotMatch(ErrorResponse),
    #[error("{0}")]
    MNSTimeExpired(ErrorResponse),
    #[error("{0}")]
    MNSQpsLimitExceeded(ErrorResponse),
    #[error("{0}")]
    MNSUnKnoWnCode(ErrorResponse),
    #[error("{0}")]
    MNSTopicNameLengthError(ErrorResponse),
    #[error("{0}")]
    MNSSubscriptionNameLengthError(ErrorResponse),
    #[error("{0}")]
    MNSTopicNotExist(ErrorResponse),
    #[error("{0}")]
    MNSTopicAlreadyExist(ErrorResponse),
    #[error("{0}")]
    MNSInvalidTopicName(ErrorResponse),
    #[error("{0}")]
    MNSInvalidSubscriptionName(ErrorResponse),
    #[error("{0}")]
    MNSSubscriptionAlreadyExist(ErrorResponse),
    #[error("{0}")]
    MNSInvalidEndpoint(ErrorResponse),
    #[error("{0}")]
    MNSSubscriberNotExist(ErrorResponse),
    #[error("{0}")]
    MNSTopicNameIsTooLong(ErrorResponse),
    #[error("{0}")]
    MNSTopicAlreadyExistAndHaveSameAttr(ErrorResponse),
    #[error("{0}")]
    MNSSubscriptionAlreadyExistAndHaveSameAttr(ErrorResponse),
    #[error("{0}")]
    MNSQueueNameIsTooLong(ErrorResponse),
    #[error("{0}")]
    MNSDelaySecondsRangeError(ErrorResponse),
    #[error("{0}")]
    MNSMaXMessageSiZeRangeError(ErrorResponse),
    #[error("{0}")]
    MNSMsgRetentionPeriodRangeError(ErrorResponse),
    #[error("{0}")]
    MNSMsgVisibilityTimeoutRangeError(ErrorResponse),
    #[error("{0}")]
    MNSMsgPoolingWaitSecondsRangeError(ErrorResponse),
    #[error("{0}")]
    MNSRetNumberRangeError(ErrorResponse),
    #[error("{0}")]
    MNSQueueAlreadyExistAndHaveSameAttr(ErrorResponse),
    #[error("{0}")]
    MNSBatchOpFail(ErrorResponse),
}

impl From<ErrorResponse> for Error {
    fn from(value: ErrorResponse) -> Self {
        match value.code.as_str() {
            "AccessDenied" => Error::MNSAccessDenied(value),
            "InvalidAccessKeyId" => Error::MNSInvalidAccessKeyId(value),
            "InternalError" => Error::MNSInternalError(value),
            "InvalidAuthorizationHeader" => Error::MNSInvalidAuthorizationHeader(value),
            "InvalidDateHeader" => Error::MNSInvalidDateHeader(value),
            "InvalidArgument" => Error::MNSInvalidArgument(value),
            "InvalidDegist" => Error::MNSInvalidDigest(value),
            "InvalidRequestURL" => Error::MNSInvalidReQuestUrl(value),
            "InvalidQueryString" => Error::MNSInvalidQueryString(value),
            "MalformedXML" => Error::MNSMalformedXml(value),
            "MissingAuthorizationHeader" => Error::MNSMissingAuthorizationHeader(value),
            "MissingDateHeader" => Error::MNSMissingDateHeader(value),
            "MissingVersionHeader" => Error::MNSMissingVersionHeader(value),
            "MissingReceiptHandle" => Error::MNSMissingReceiptHandle(value),
            "MissingVisibilityTimeout" => Error::MNSMissingVisibilityTimeout(value),
            "MessageNotExist" => Error::MNSMessageNotExist(value),
            "QueueAlreadyExist" => Error::MNSQueueAlreadyExist(value),
            "QueueDeletedRecently" => Error::MNSQueueDeletedRecently(value),
            "InvalidQueueName" => Error::MNSInvalidQueueName(value),
            "InvalidVersionHeader" => Error::MNSInvalidVersionHeader(value),
            "InvalidContentType" => Error::MNSInvalidContentType(value),
            "QueueNameLengthError" => Error::MNSQueueNameLengthError(value),
            "QueueNotExist" => Error::MNSQueueNotExist(value),
            "ReceiptHandleError" => Error::MNSReceiptHandleError(value),
            "SignatureDoesNotMatch" => Error::MNSSignatureDoesNotMatch(value),
            "TimeExpired" => Error::MNSTimeExpired(value),
            "QpsLimitExceeded" => Error::MNSQpsLimitExceeded(value),
            "TopicNameLengthError" => Error::MNSTopicNameLengthError(value),
            "SubscriptionNameLengthError" => Error::MNSSubscriptionNameLengthError(value),
            "TopicNotExist" => Error::MNSTopicNotExist(value),
            "TopicAlreadyExist" => Error::MNSTopicAlreadyExist(value),
            "InvalidTopicName" => Error::MNSInvalidTopicName(value),
            "InvalidSubscriptionName" => Error::MNSInvalidSubscriptionName(value),
            "SubscriptionAlreadyExist" => Error::MNSSubscriptionAlreadyExist(value),
            "EndpointInvalid" => Error::MNSInvalidEndpoint(value),
            "SubscriberNotExist" => Error::MNSSubscriberNotExist(value),
            "TopicNameIsTooLong" => Error::MNSTopicNameIsTooLong(value),
            "TopicAlreadyExistAndHaveSameAttr" => Error::MNSTopicAlreadyExistAndHaveSameAttr(value),
            "SubscriptionAlreadyExistAndHaveSameAttr" => {
                Error::MNSSubscriptionAlreadyExistAndHaveSameAttr(value)
            }
            "QueueNameIsTooLong" => Error::MNSQueueNameIsTooLong(value),
            "DelaySecondsRangeError" => Error::MNSDelaySecondsRangeError(value),
            "MaxMessageSiZeRangeError" => Error::MNSMaXMessageSiZeRangeError(value),
            "MsgRetentionPeriodRangeError" => Error::MNSMsgRetentionPeriodRangeError(value),
            "PollingWaitSecondsRangeError" => Error::MNSMsgPoolingWaitSecondsRangeError(value),
            "SubsriptionNameInvalid" => Error::MNSInvalidSubscriptionName(value),
            _ => Error::MNSUnknown(value),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
