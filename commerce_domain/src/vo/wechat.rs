use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone, Getters, Setters, Default)]
#[getset(get = "pub", set = "pub")]
pub struct WechatSignInVo {
    code: Option<String>,
    login_type: Option<String>,
    iv: Option<String>,
    encryptedData: Option<String>,
    signature: Option<String>,
    rawData: Option<String>,
}