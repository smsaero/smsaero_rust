use chrono::{DateTime, Utc};
use reqwest::blocking::Client;
use reqwest::header::{CONTENT_TYPE, USER_AGENT};
use serde_json::json;
use std::error::Error as StdError;
use std::fmt;
use std::time::Duration;

const DEFAULT_TIMEOUT: u64 = 30;
const DEFAULT_CONNECT_TIMEOUT: u64 = 10;
const MAX_RESPONSE_SIZE: usize = 10 * 1024 * 1024;

#[derive(Debug)]
pub struct SmsAeroError {
    message: String,
}

impl SmsAeroError {
    pub fn new(message: impl Into<String>) -> Self {
        SmsAeroError {
            message: message.into(),
        }
    }
}

impl fmt::Display for SmsAeroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl StdError for SmsAeroError {}

pub struct SmsAero {
    pub email: String,
    pub api_key: String,
    pub url_gate: Option<String>,
    pub signature: String,
    pub client: Client,
}

impl SmsAero {
    const SIGNATURE: &'static str = "Sms Aero";
    const GATE_URL: &'static str = "https://gate.smsaero.ru/v2/";

    pub fn new(
        email: String,
        api_key: String,
        url_gate: Option<String>,
        signature: Option<String>,
    ) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT))
            .connect_timeout(Duration::from_secs(DEFAULT_CONNECT_TIMEOUT))
            .build()
            .unwrap_or_else(|_| Client::new());

        SmsAero {
            email,
            api_key,
            url_gate,
            signature: signature.unwrap_or_else(|| Self::SIGNATURE.to_string()),
            client,
        }
    }

    fn request(
        &self,
        selector: &str,
        data: Option<serde_json::Value>,
        page: Option<i32>,
    ) -> Result<serde_json::Value, Box<dyn StdError>> {
        let base = self.url_gate.as_deref().unwrap_or(Self::GATE_URL);
        let mut url = format!("{}{}", base, selector);
        if let Some(page) = page {
            url = format!("{}?page={}", url, page);
        }

        let response = self
            .client
            .post(&url)
            .basic_auth(&self.email, Some(&self.api_key))
            .header(CONTENT_TYPE, "application/json")
            .header(USER_AGENT, "SARustClient/1.0.0")
            .json(&data.unwrap_or_else(|| json!({})))
            .send()?;

        let content = response.text()?;
        if content.len() > MAX_RESPONSE_SIZE {
            return Err(Box::new(SmsAeroError::new("Response size exceeds limit")));
        }

        let result: serde_json::Value = serde_json::from_str(&content)?;

        if result
            .get("success")
            .and_then(|s| s.as_bool())
            .unwrap_or(false)
        {
            Ok(result)
        } else {
            let msg = result
                .get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("Unknown error");
            Err(Box::new(SmsAeroError::new(msg)))
        }
    }

    pub fn send_sms(
        &self,
        number: &str,
        text: &str,
        date_send: Option<DateTime<Utc>>,
        callback_url: Option<&str>,
    ) -> Result<serde_json::Value, Box<dyn StdError>> {
        let mut data = json!({
            "number": number,
            "sign": self.signature,
            "text": text,
            "callbackUrl": callback_url.unwrap_or(""),
        });

        if let Some(date_send) = date_send {
            data["dateSend"] = json!(date_send.timestamp());
        }

        self.request("sms/send", Some(data), None)
    }

    pub fn sms_status(&self, sms_id: i32) -> Result<serde_json::Value, Box<dyn StdError>> {
        self.request("sms/status", Some(json!({"id": sms_id})), None)
    }

    pub fn sms_list(
        &self,
        number: Option<&str>,
        text: Option<&str>,
        page: Option<i32>,
    ) -> Result<serde_json::Value, Box<dyn StdError>> {
        let mut data = serde_json::Map::new();
        if let Some(number) = number {
            data.insert("number".to_string(), json!(number));
        }
        if let Some(text) = text {
            data.insert("text".to_string(), json!(text));
        }

        self.request("sms/list", Some(serde_json::Value::Object(data)), page)
    }

    pub fn balance(&self) -> Result<serde_json::Value, Box<dyn StdError>> {
        self.request("balance", None, None)
    }

    pub fn auth(&self) -> Result<serde_json::Value, Box<dyn StdError>> {
        self.request("auth", None, None)
    }

    pub fn cards(&self) -> Result<serde_json::Value, Box<dyn StdError>> {
        self.request("cards", None, None)
    }

    pub fn add_balance(
        &self,
        sum: f64,
        card_id: i32,
    ) -> Result<serde_json::Value, Box<dyn StdError>> {
        self.request(
            "balance/add",
            Some(json!({ "sum": sum, "cardId": card_id })),
            None,
        )
    }

    pub fn tariffs(&self) -> Result<serde_json::Value, Box<dyn StdError>> {
        self.request("tariffs", None, None)
    }

    pub fn sign_add(&self, name: &str) -> Result<serde_json::Value, Box<dyn StdError>> {
        self.request("sign/add", Some(json!({ "name": name })), None)
    }

    pub fn sign_list(&self, page: Option<i32>) -> Result<serde_json::Value, Box<dyn StdError>> {
        self.request("sign/list", None, page)
    }

    pub fn group_add(&self, name: &str) -> Result<serde_json::Value, Box<dyn StdError>> {
        self.request("group/add", Some(json!({ "name": name })), None)
    }

    pub fn group_delete(&self, group_id: i32) -> Result<serde_json::Value, Box<dyn StdError>> {
        self.request("group/delete", Some(json!({ "id": group_id })), None)
    }

    pub fn group_list(&self, page: Option<i32>) -> Result<serde_json::Value, Box<dyn StdError>> {
        self.request("group/list", None, page)
    }

    pub fn contact_add(
        &self,
        number: &str,
        group_id: Option<i32>,
        birthday: Option<&str>,
        sex: Option<&str>,
        lname: Option<&str>,
        fname: Option<&str>,
        sname: Option<&str>,
        param1: Option<&str>,
        param2: Option<&str>,
        param3: Option<&str>,
    ) -> Result<serde_json::Value, Box<dyn StdError>> {
        self.request(
            "contact/add",
            Some(json!({
                "number": number,
                "groupId": group_id,
                "birthday": birthday,
                "sex": sex,
                "lname": lname,
                "fname": fname,
                "sname": sname,
                "param1": param1,
                "param2": param2,
                "param3": param3
            })),
            None,
        )
    }

    pub fn contact_delete(&self, contact_id: i32) -> Result<serde_json::Value, Box<dyn StdError>> {
        self.request("contact/delete", Some(json!({ "id": contact_id })), None)
    }

    pub fn contact_list(
        &self,
        number: Option<&str>,
        group_id: Option<i32>,
        birthday: Option<&str>,
        sex: Option<&str>,
        operator: Option<&str>,
        lname: Option<&str>,
        fname: Option<&str>,
        sname: Option<&str>,
        page: Option<i32>,
    ) -> Result<serde_json::Value, Box<dyn StdError>> {
        self.request(
            "contact/list",
            Some(json!({
                "number": number,
                "groupId": group_id,
                "birthday": birthday,
                "sex": sex,
                "operator": operator,
                "lname": lname,
                "fname": fname,
                "sname": sname
            })),
            page,
        )
    }

    pub fn blacklist_add(&self, number: &str) -> Result<serde_json::Value, Box<dyn StdError>> {
        self.request("blacklist/add", Some(json!({ "number": number })), None)
    }

    pub fn blacklist_list(
        &self,
        number: Option<&str>,
        page: Option<i32>,
    ) -> Result<serde_json::Value, Box<dyn StdError>> {
        let data = number.map(|number| json!({ "number": number }));
        self.request("blacklist/list", data, page)
    }

    pub fn blacklist_delete(
        &self,
        blacklist_id: i32,
    ) -> Result<serde_json::Value, Box<dyn StdError>> {
        self.request(
            "blacklist/delete",
            Some(json!({ "id": blacklist_id })),
            None,
        )
    }

    pub fn hlr_check(&self, number: &str) -> Result<serde_json::Value, Box<dyn StdError>> {
        self.request("hlr/check", Some(json!({ "number": number })), None)
    }

    pub fn hlr_status(&self, hlr_id: i32) -> Result<serde_json::Value, Box<dyn StdError>> {
        self.request("hlr/status", Some(json!({ "id": hlr_id })), None)
    }

    pub fn number_operator(&self, number: &str) -> Result<serde_json::Value, Box<dyn StdError>> {
        self.request("number/operator", Some(json!({ "number": number })), None)
    }

    pub fn viber_send(
        &self,
        sign: &str,
        channel: &str,
        text: &str,
        number: Option<&str>,
        group_id: Option<i32>,
        image_source: Option<&str>,
        text_button: Option<&str>,
        link_button: Option<&str>,
        date_send: Option<DateTime<Utc>>,
        sign_sms: Option<&str>,
        channel_sms: Option<&str>,
        text_sms: Option<&str>,
        price_sms: Option<f64>,
    ) -> Result<serde_json::Value, Box<dyn StdError>> {
        let data = json!({
            "sign": sign,
            "channel": channel,
            "text": text,
            "number": number,
            "groupId": group_id,
            "imageSource": image_source,
            "textButton": text_button,
            "linkButton": link_button,
            "dateSend": date_send.map(|d| d.timestamp()),
            "signSms": sign_sms,
            "channelSms": channel_sms,
            "textSms": text_sms,
            "priceSms": price_sms
        });

        self.request("viber/send", Some(data), None)
    }

    pub fn viber_sign_list(&self) -> Result<serde_json::Value, Box<dyn StdError>> {
        self.request("viber/sign/list", None, None)
    }

    pub fn viber_list(&self, page: Option<i32>) -> Result<serde_json::Value, Box<dyn StdError>> {
        self.request("viber/list", None, page)
    }

    pub fn send_telegram(
        &self,
        number: &str,
        code: i32,
        sign: Option<&str>,
        text: Option<&str>,
    ) -> Result<serde_json::Value, Box<dyn StdError>> {
        let mut data = json!({
            "number": number,
            "code": code,
        });

        if let Some(sign) = sign {
            data["sign"] = json!(sign);
        }
        if let Some(text) = text {
            data["text"] = json!(text);
        }

        self.request("telegram/send", Some(data), None)
    }

    pub fn telegram_status(
        &self,
        telegram_id: i32,
    ) -> Result<serde_json::Value, Box<dyn StdError>> {
        self.request("telegram/status", Some(json!({"id": telegram_id})), None)
    }

    pub fn send_mobile_id(
        &self,
        number: &str,
        sign: &str,
        callback_url: &str,
    ) -> Result<serde_json::Value, Box<dyn StdError>> {
        self.request(
            "mobile-id/send",
            Some(json!({
                "number": number,
                "sign": sign,
                "callbackUrl": callback_url,
            })),
            None,
        )
    }

    pub fn mobile_id_status(&self, req_id: i32) -> Result<serde_json::Value, Box<dyn StdError>> {
        self.request("mobile-id/status", Some(json!({"id": req_id})), None)
    }

    pub fn verify_mobile_id(
        &self,
        req_id: i32,
        code: &str,
        sign: &str,
    ) -> Result<serde_json::Value, Box<dyn StdError>> {
        self.request(
            "mobile-id/verify",
            Some(json!({
                "id": req_id,
                "code": code,
                "sign": sign,
            })),
            None,
        )
    }
}
