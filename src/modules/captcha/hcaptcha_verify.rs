use actix_web::web;
use crate::AppConfig;
use serde::{Serialize,Deserialize};

/**
{
"success": true|false,     // is the passcode valid, and does it meet security criteria you specified, e.g. sitekey?
"challenge_ts": timestamp, // timestamp of the challenge (ISO format yyyy-MM-dd'T'HH:mm:ssZZ)
"hostname": string,        // the hostname of the site where the challenge was solved
"credit": true|false,      // optional: whether the response will be credited
"error-codes": [...]       // optional: any error codes
"score": float,            // ENTERPRISE feature: a score denoting malicious activity.
"score_reason": [...]      // ENTERPRISE feature: reason(s) for score.
}
*/

#[derive(Deserialize, Serialize)]
pub struct VerifyResult {
    pub success: bool,
    pub challenge_ts: String,
    pub hostname: String,
    pub credit: bool,
}


pub async fn hcaptch_verify(input_str: String, conf: web::Data<AppConfig> ) -> Option<bool> {

    //请求hcaptcha的接口，
    let secret_key = &conf.h_captcha_secret_key;
    // This will POST a body of `foo=bar&baz=quux`
    let params = [("response", input_str), ("secret", secret_key.to_owned())];
    let client = reqwest::Client::new();
    let res  = client.post("https://hcaptcha.com/siteverify")
        .form(&params)
        .send().await.unwrap();

    let t = res
        .text()
        .await.unwrap();
       let r: VerifyResult = serde_json::from_str(&t).unwrap();
        if r.success == true {
            log::info!("校验通过: {}", true);
            return Some(true);
        }

    //默认返回不成功
    return Some(false)
}