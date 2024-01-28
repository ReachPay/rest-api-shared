use serde::{Deserialize, Serialize};
use service_sdk::{my_no_sql_sdk, rust_extensions::date_time::DateTimeAsMicroseconds};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionClaim {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Expires")]
    pub expires: i64,
}

#[service_sdk::my_no_sql_sdk::macros::my_no_sql_entity("sessions")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SessionEntity {
    #[serde(rename = "TraderId")]
    pub client_id: String,
    #[serde(rename = "MerchantId")]
    pub merchant_id: String,
    #[serde(rename = "Ip")]
    pub ip: String,
    #[serde(rename = "Expires")]
    pub expires: String,
    #[serde(rename = "Claims")]
    pub claims: Vec<SessionClaim>,
}

impl SessionEntity {
    pub fn get_pk() -> &'static str {
        "s"
    }

    pub fn get_session_token(&self) -> &str {
        &self.row_key
    }

    pub fn new(
        session_id: String,
        client_id: String,
        merchant_id: String,
        ip: String,
        expires: DateTimeAsMicroseconds,
        claims: Vec<SessionClaim>,
    ) -> Self {
        Self {
            partition_key: Self::get_pk().to_string(),
            row_key: session_id,
            client_id,
            merchant_id,
            expires: expires.to_rfc3339(),
            claims,
            ip,
            time_stamp: "".to_string(),
        }
    }
}
