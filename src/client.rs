use std::str::FromStr;
use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use hmac::{Hmac, Mac};
use md5::{Digest, Md5};
use reqwest::Method;
use sha1::Sha1;

#[derive(Debug, Clone)]
pub struct Client {
    endpoint: String,
    id: String,
    sec: String,
}

impl Client {
    pub fn new(endpoint: &str, id: &str, sec: &str) -> Self {
        Self {
            endpoint: endpoint.to_string(),
            id: id.to_string(),
            sec: sec.to_string(),
        }
    }
    pub async fn request(&self,
                         resource: &str,
                         method: &str,
                         content_type: &str,
                         body: &str) -> Result<()> {
        let body = body.clone();
        let client = reqwest::Client::new();
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
            "POST".to_string(),
            m.to_string(),
            date.clone(),
            resource.to_string(),
        )?;

        let res = client
            .request(Method::from_str(method)?, format!("{}{}", self.endpoint, resource).as_str())
            .header("Date", date)
            .header("Authorization", format!("MNS {}:{}", self.id, s))
            .header("Content-Type", content_type)
            .header("Content-md5", m)
            .header("x-mns-version", "2015-06-06")
            .timeout(std::time::Duration::from_secs(5))
            .body(body.to_string()).send().await?;

        if res.status().is_success() {
            dbg!(String::from_utf8_lossy(res.bytes().await?.as_ref()));
            Ok(())
        } else {
            let s = String::from_utf8_lossy(res.bytes().await?.as_ref()).to_string();
            Err(anyhow::anyhow!("send msg failed: {}", s))
        }
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
        "{}\n{}\napplication/xml\n{}\nx-mns-version:2015-06-06\n{}",
        method, lower_md5_base64, date, resource
    );
    sign(sk, s.as_str())
}

fn sign<S: Into<String>>(key: S, body: &str) -> Result<String> {
    dbg!(body);
    let mut mac = Hmac::<Sha1>::new_from_slice(key.into().as_bytes())?;
    mac.update(body.as_bytes());
    let result = mac.finalize();
    let s = STANDARD.encode(result.into_bytes());
    dbg!(s.clone());
    Ok(s)
}

fn gmt_now() -> Result<String> {
    Ok(time::OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc2822)?
        .splitn(2, "+0")
        .next().unwrap()
        .to_string()
        + "GMT")
}

#[cfg(test)]
mod test {
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
        assert_eq!("6nhdhorU7xdV6x+P1Tmzyi6A6KY=", r)
    }

    #[tokio::test]
    async fn test_send_msg() {
        let conf = dbg!(get_conf());

        let c = Client::new(conf.endpoint.as_str(), conf.id.as_str(), conf.sec.as_str());
        c.request(
            &format!("/queues/{}/messages", conf.queue),
            "POST",
            "application/xml",
            &"<Message><MessageBody>hello &lt;&#34;aliyun-mns-go-sdk&#34;&gt;</MessageBody><DelaySeconds>0</DelaySeconds><Priority>8</Priority></Message>",
        )
            .await
            .unwrap();
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
    }

    #[test]
    fn test_gmt() {
        gmt_now().unwrap();
    }
}