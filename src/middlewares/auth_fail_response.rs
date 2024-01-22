use serde::{Deserialize, Serialize};
use serde_repr::*;
use service_sdk::my_http_server;
use service_sdk::my_http_server::macros::*;
use service_sdk::my_http_server::HttpFailResult;
use service_sdk::my_telemetry::TelemetryEventTagsBuilder;
#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct UnauthorizedRequestResponse {
    pub reason: UnauthorizedReasonCode,
    pub message: String,
    #[serde(rename = "claim")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim: Option<String>,
}

impl UnauthorizedRequestResponse {
    pub fn new(
        reason: UnauthorizedReasonCode,
        message: String,
        claim: Option<String>,
    ) -> HttpFailResult {
        let result = UnauthorizedRequestResponse {
            reason,
            message,
            claim,
        };

        let content = serde_json::to_vec(&result).unwrap();
        HttpFailResult {
            content_type: my_http_server::WebContentType::Json,
            status_code: 401,
            content,
            write_telemetry: false,
            write_to_log: false,
            add_telemetry_tags: TelemetryEventTagsBuilder::new(),
        }
    }
}

#[derive(Serialize_repr, Deserialize_repr, MyHttpIntegerEnum, Debug)]
#[repr(i16)]
pub enum UnauthorizedReasonCode {
    #[http_enum_case(id=1; description="Invalid session token")]
    InvalidSessionToken = 1,
    #[http_enum_case(id=2; description="Session token is expired")]
    SessionTokenIsExpired = 2,
    #[http_enum_case(id=3; description="Refresh token is not valid")]
    RefreshTokenIsNotValid = 3,
    #[http_enum_case(id=4; description="Claim authorization is required")]
    Unauthorzed = 4,
}
