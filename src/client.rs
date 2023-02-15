use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use hmac::{Hmac, Mac};
use md5::{Digest, Md5};
use reqwest::{Method, StatusCode};
use sha1::Sha1;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Client {
    endpoint: String,
    id: String,
    sec: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new(endpoint: &str, id: &str, sec: &str) -> Self {
        Self {
            endpoint: endpoint.to_string(),
            id: id.to_string(),
            sec: sec.to_string(),
            client: reqwest::Client::new(),
        }
    }
    pub async fn request(
        &self,
        resource: &str,
        method: &str,
        content_type: &str,
        body: &str,
    ) -> Result<(StatusCode, Vec<u8>)> {
        let date = gmt_now()?;
        let m = {
            let mut hasher = Md5::new();
            hasher.update(body.as_bytes());
            let r = hasher.finalize();
            let mut buf = [0u8; 32];
            let m = base16ct::lower::encode_str(r.as_slice(), &mut buf).unwrap();
            STANDARD.encode(m)
        };

        let s = req_sign(
            &self.sec,
            method.to_string(),
            m.to_string(),
            date.clone(),
            resource.to_string(),
        )?;

        let res = self
            .client
            .request(
                Method::from_str(method)?,
                format!("{}{}", self.endpoint, resource).as_str(),
            )
            .header("Date", date)
            .header("Authorization", format!("MNS {}:{}", self.id, s))
            .header("Content-Type", content_type)
            .header("Content-Md5", m)
            .header("x-mns-version", "2015-06-06")
            .timeout(std::time::Duration::from_secs(5))
            .body(body.to_string())
            .send()
            .await?;

        Ok((res.status(), res.bytes().await?.as_ref().to_vec()))
    }
}

fn req_sign(
    sk: &str,
    method: String,
    lower_md5_base64: String,
    date: String,
    resource: String,
) -> Result<String> {
    let s = format!(
        "{method}\n{lower_md5_base64}\napplication/xml\n{date}\nx-mns-version:2015-06-06\n{resource}"
    );
    sign(sk, s.as_str())
}

fn sign<S: Into<String>>(key: S, body: &str) -> Result<String> {
    let mut mac = Hmac::<Sha1>::new_from_slice(key.into().as_bytes())?;
    mac.update(body.as_bytes());
    let result = mac.finalize();
    let s = STANDARD.encode(result.into_bytes());
    Ok(s)
}

fn gmt_now() -> Result<String> {
    Ok(time::OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc2822)?
        .split("+0")
        .next()
        .unwrap()
        .to_string()
        + "GMT")
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::error::Error;
    use crate::error::Error::MNSSignatureDoesNotMatch;
    use crate::queue::ErrorResponse;

    #[test]
    fn test_sign() {
        let r = sign(
            "bb",
            "POST
666
777
Thu, 02 Feb 2023 02:09:48 GMT
/queues/$queueName/messages",
        )
        .unwrap();
        assert_eq!("pSxntRmmzwO95loQNbiaKzs0fsE=", r);

        let r = req_sign(
            "t5I8e",
            "POST".to_string(),
            "YTM5OGY1YmYxODRkY2M0YmM1NjU5OGYzYTJkMDMyZGQ=".to_string(),
            "Thu, 02 Feb 2023 12:27:22 GMT".to_string(),
            "/queues/market-process-log/messages".to_string(),
        )
        .unwrap();
        assert_eq!("6nhdhorU7xdV6x+P1Tmzyi6A6KY=", r);

        let r = req_sign(
            "t5I8e",
            "GET".to_string(),
            "ZDQxZDhjZDk4ZjAwYjIwNGU5ODAwOTk4ZWNmODQyN2U=".to_string(),
            "Wed, 08 Feb 2023 09:36:03 GMT".to_string(),
            "/queues/market-process-log/messages?waitseconds=30".to_string(),
        )
        .unwrap();
        assert_eq!("zVO3Buq0YfEW1yLI0SXOaO6guq8=", r);
    }

    #[tokio::test]
    async fn test_sign_req() {
        let c = Client::new(env!("MNS_ENDPOINT"), env!("MNS_ID"), env!("MNS_SEC"));
        let (status_code, r) = c.request(
            &format!("/queues/{}/messages", env!("MNS_QUEUE")),
            "POST",
            "application/xml",
            &"<Message><MessageBody>hello &lt;&#34;aliyun-mns-go-sdk&#34;&gt;</MessageBody><DelaySeconds>0</DelaySeconds><Priority>8</Priority></Message>",
        )
            .await
            .unwrap();
        assert_eq!(403, status_code);
        let s: ErrorResponse = serde_xml_rs::from_reader(r.as_slice()).unwrap();
        let e = Error::from(s);
        match e {
            MNSSignatureDoesNotMatch(_) => (),
            _ => panic!("wrong error"),
        }
    }

    #[test]
    fn test_md5() {
        // 得先 hex，再 base64
        // let mut buf = [0u8; 32];
        // let b = base16ct::mixed::decode("a398f5bf184dcc4bc56598f3a2d032dd".as_bytes(), &mut buf)
        //     .unwrap();
        // dbg!(b);
        // let raw = b"a398f5bf184dcc4bc56598f3a2d032dd";
        // assert_eq!(raw, b);
        // let a = STANDARD.encode(raw.as_ref());
        // assert_eq!(a, "YTM5OGY1YmYxODRkY2M0YmM1NjU5OGYzYTJkMDMyZGQ=")

        let mut hasher = Md5::new();
        hasher.update("".as_bytes());
        let r = hasher.finalize();
        let mut buf = [0u8; 32];
        let m = dbg!(base16ct::lower::encode_str(r.as_slice(), &mut buf).unwrap());
        assert_eq!(
            "ZDQxZDhjZDk4ZjAwYjIwNGU5ODAwOTk4ZWNmODQyN2U=",
            dbg!(STANDARD.encode(m))
        );
    }

    #[test]
    fn test_gmt() {
        dbg!(gmt_now().unwrap());
    }
}
