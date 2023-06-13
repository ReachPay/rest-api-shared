use my_http_server::HttpFailResult;
use my_http_server_swagger::{MyHttpIntegerEnum, MyHttpObjectStructure};
use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct UnathorizedRequestResponse {
    pub reason: UnauthorizedReasonCode,
    pub message: String,
    #[serde(rename = "claim")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim: Option<String>,
}

impl UnathorizedRequestResponse {
    pub fn new(
        reason: UnauthorizedReasonCode,
        message: String,
        claim: Option<String>,
    ) -> HttpFailResult {
        let result = UnathorizedRequestResponse {
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
