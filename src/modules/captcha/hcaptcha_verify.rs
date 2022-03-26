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
    pub challenge_ts: Option<String>,
    pub hostname: Option<String>,
    pub credit: Option<bool>,

}


pub async fn hcaptch_verify(input_str: String, conf: web::Data<AppConfig> ) -> Result<bool, &'static str> {

    //请求hcaptcha的接口，
    let secret_key = &conf.h_captcha_secret_key;
    // This will POST a body of `foo=bar&baz=quux`
    let params = [("response", input_str), ("secret", secret_key.to_owned())];
    let client = reqwest::Client::new();
    let response = client.post("https://hcaptcha.com/siteverify")
        .form(&params)
        .send()
        .await;
    match response {
        Err(e) => println!("{}", e),
        Ok(res) => {
            //TODO 加强校验
            let r  = res
                .json::<VerifyResult>()
                .await;


            match r {
                Err(e) => println!("{}", e),
                Ok(rr) => {

                    log::info!(" verifyResult: {}", &serde_json::to_string(&rr).unwrap());
                    if rr.success == true {
                        log::info!("校验通过: {}", true);
                        return Ok(true);
                    }
                }
            }


        }
    }


    //默认返回不成功
    return    Err("不成功");
}