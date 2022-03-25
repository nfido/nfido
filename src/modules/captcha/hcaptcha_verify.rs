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
    pub challenge_ts: string,
    pub hostname: string,
    pub credit: boo,
}


pub fn hcaptch_verify(input_str: Strin, conf: web::Data<AppConfig> ) -> (bool) {

    //请求hcaptcha的接口，
    let secret_key = &conf.h_captcha_secret_key;
    // This will POST a body of `foo=bar&baz=quux`
    let params = [("response", input_str), ("secret", secret_key)];
    let client = reqwest::Client::new();
    let res = client.post("https://hcaptcha.com/siteverify")
        .form(&params)
        .send()
        .await?;
    if res.status().is_success() {
        let r: VerifyResult = res.json()?;
        if r.success == true {
            log::info!("校验通过: {}", true);
            return true;
        }
    } else {

        log::info!("resp status code {}", res.status()?);
    }

    //默认返回不成功
    return false
}