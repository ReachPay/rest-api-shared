use serde::{Deserialize, Serialize};
use service_sdk::my_no_sql_sdk;

pub const SESSION_PARTITION_KEY_VALUE: &str = "t";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionClaim {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Expires")]
    pub expires: i64,
}

#[service_sdk::my_no_sql_sdk::macros::my_no_sql_entity("sessionsentites")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SessionEntity {
    #[serde(rename = "TraderId")]
    pub trader_id: String,
    #[serde(rename = "Expires")]
    pub expires: String,
    #[serde(rename = "Claims")]
    pub claims: Option<Vec<SessionClaim>>,
}

impl SessionEntity {
    pub fn get_pk() -> String {
        SESSION_PARTITION_KEY_VALUE.to_string()
    }

    pub fn get_session_token(&self) -> &str {
        &self.row_key
    }
}
