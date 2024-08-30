use chrono::{DateTime, Utc};
use reqwest::blocking::Client;
use reqwest::header::{CONTENT_TYPE, USER_AGENT};
use serde_json::json;
use std::error::Error;
use std::fmt;
use std::error::Error as StdError;

#[derive(Debug)]
pub struct SmsAeroError {
    message: String,
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

    pub fn new(email: String, api_key: String, url_gate: Option<String>, signature: Option<String>) -> Self {
        SmsAero {
            email,
            api_key,
            url_gate,
            signature: signature.unwrap_or_else(|| Self::SIGNATURE.to_string()),
            client: Client::new(),
        }
    }

    fn request(&self, selector: &str, data: Option<serde_json::Value>, page: Option<i32>) -> Result<serde_json::Value, Box<dyn Error>> {
        let url_base = format!(
            "https://{}:{}@gate.smsaero.ru/v2/",
            url::form_urlencoded::byte_serialize(self.email.as_bytes()).collect::<String>(),
            self.api_key
        );
        let mut url = format!("{}{}", url_base, selector);
        if let Some(page) = page {
            url = format!("{}?page={}", url, page);
        }

        let response = self.client.post(&url)
            .header(CONTENT_TYPE, "application/json")
            .header(USER_AGENT, "SARustClient/1.0.0")
            .json(&data.unwrap_or_else(|| json!({})))
            .send()?;

        let content = response.text()?;
        self.check_response(&content)?;

        Ok(serde_json::from_str(&content)?)
    }

    fn check_response(&self, content: &str) -> Result<(), Box<dyn Error>> {
        let response: serde_json::Value = serde_json::from_str(content)?;
        if response.get("success").and_then(|s| s.as_bool()).unwrap_or(false) {
            Ok(())
        } else {
            Err(Box::new(SmsAeroError {
                message: response.get("message").and_then(|m| m.as_str()).unwrap_or("Unknown error").to_string(),
            }))
        }
    }

    pub fn send_sms(&self, number: &str, text: &str, date_send: Option<DateTime<Utc>>, callback_url: Option<&str>) -> Result<serde_json::Value, Box<dyn Error>> {
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

    pub fn sms_status(&self, sms_id: i32) -> Result<serde_json::Value, Box<dyn Error>> {
        self.request("sms/status", Some(json!({"id": sms_id})), None)
    }

    pub fn sms_list(&self, number: Option<&str>, text: Option<&str>, page: Option<i32>) -> Result<serde_json::Value, Box<dyn Error>> {
        let mut data = serde_json::Map::new();
        if let Some(number) = number {
            data.insert("number".to_string(), json!(number));
        }
        if let Some(text) = text {
            data.insert("text".to_string(), json!(text));
        }

        self.request("sms/list", Some(serde_json::Value::Object(data)), page)
    }

    pub fn balance(&self) -> Result<serde_json::Value, Box<dyn Error>> {
        self.request("balance", None, None)
    }

    pub fn auth(&self) -> Result<serde_json::Value, Box<dyn Error>> {
        self.request("auth", None, None)
    }

    pub fn cards(&self) -> Result<serde_json::Value, Box<dyn Error>> {
        self.request("cards", None, None)
    }

    pub fn add_balance(&self, _sum: f64, card_id: i32) -> Result<serde_json::Value, Box<dyn Error>> {
        self.request("balance/add", Some(json!({ "sum": _sum, "cardId": card_id })), None)
    }

    pub fn tariffs(&self) -> Result<serde_json::Value, Box<dyn Error>> {
        self.request("tariffs", None, None)
    }

    pub fn sign_add(&self, name: &str) -> Result<serde_json::Value, Box<dyn Error>> {
        self.request("sign/add", Some(json!({ "name": name })), None)
    }

    pub fn sign_list(&self, page: Option<i32>) -> Result<serde_json::Value, Box<dyn Error>> {
        self.request("sign/list", None, page)
    }

    pub fn group_add(&self, name: &str) -> Result<serde_json::Value, Box<dyn Error>> {
        self.request("group/add", Some(json!({ "name": name })), None)
    }

    pub fn group_delete(&self, group_id: i32) -> Result<serde_json::Value, Box<dyn Error>> {
        self.request("group/delete", Some(json!({ "id": group_id })), None)
    }

    pub fn group_list(&self, page: Option<i32>) -> Result<serde_json::Value, Box<dyn Error>> {
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
        param3: Option<&str>
    ) -> Result<serde_json::Value, Box<dyn Error>> {
        self.request("contact/add", Some(json!({
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
        })), None)
    }

    pub fn contact_delete(&self, contact_id: i32) -> Result<serde_json::Value, Box<dyn Error>> {
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
        page: Option<i32>
    ) -> Result<serde_json::Value, Box<dyn Error>> {
        self.request("contact/list", Some(json!({
            "number": number,
            "groupId": group_id,
            "birthday": birthday,
            "sex": sex,
            "operator": operator,
            "lname": lname,
            "fname": fname,
            "sname": sname
        })), page)
    }

    pub fn blacklist_add(&self, number: &str) -> Result<serde_json::Value, Box<dyn Error>> {
        self.request("blacklist/add", Some(json!({ "number": number })), None)
    }

    pub fn blacklist_list(&self, number: Option<&str>, page: Option<i32>) -> Result<serde_json::Value, Box<dyn Error>> {
        let data = if let Some(number) = number {
            Some(json!({ "number": number }))
        } else {
            None
        };
        self.request("blacklist/list", data, page)
    }

    pub fn blacklist_delete(&self, blacklist_id: i32) -> Result<serde_json::Value, Box<dyn Error>> {
        self.request("blacklist/delete", Some(json!({ "id": blacklist_id })), None)
    }

    pub fn hlr_check(&self, number: &str) -> Result<serde_json::Value, Box<dyn Error>> {
        self.request("hlr/check", Some(json!({ "number": number })), None)
    }

    pub fn hlr_status(&self, hlr_id: i32) -> Result<serde_json::Value, Box<dyn Error>> {
        self.request("hlr/status", Some(json!({ "id": hlr_id })), None)
    }

    pub fn number_operator(&self, number: &str) -> Result<serde_json::Value, Box<dyn Error>> {
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
        price_sms: Option<f64>
    ) -> Result<serde_json::Value, Box<dyn Error>> {
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

    pub fn viber_sign_list(&self) -> Result<serde_json::Value, Box<dyn Error>> {
        self.request("viber/sign/list", None, None)
    }

    pub fn viber_list(&self, page: Option<i32>) -> Result<serde_json::Value, Box<dyn Error>> {
        self.request("viber/list", None, page)
    }
}
