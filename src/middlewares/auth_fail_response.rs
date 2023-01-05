use my_http_server::HttpFailResult;
use my_http_server_swagger::MyHttpIntegerEnum;
use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct UnathorizedRequestResponse {
    pub reason: UnauthorizedReasonCode,
    pub message: String,
    #[serde(rename = "claim")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim: Option<String>,
}

impl UnathorizedRequestResponse {
    pub fn get_http_data_structure(
    ) -> my_http_server_controllers::controllers::documentation::data_types::HttpObjectStructure
    {
        use my_http_server_controllers::controllers::documentation::*;
        data_types::HttpObjectStructure {
            struct_id: "UnathorizedRequestResponse",
            fields: vec![
                data_types::HttpField::new(
                    "reason",
                    UnauthorizedReasonCode::get_data_type(),
                    true,
                    None,
                ),
                data_types::HttpField::new("message", String::get_data_type(), true, None),
                data_types::HttpField::new("claim", String::get_data_type(), true, None),
            ],
        }
    }
}
impl<'s> TryFrom<my_http_server::InputParamValue<'s>> for UnathorizedRequestResponse {
    type Error = my_http_server::HttpFailResult;
    fn try_from(value: my_http_server::InputParamValue) -> Result<Self, Self::Error> {
        value.from_json()
    }
}
impl my_http_server_controllers::controllers::documentation::DataTypeProvider
    for UnathorizedRequestResponse
{
    fn get_data_type(
    ) -> my_http_server_controllers::controllers::documentation::data_types::HttpDataType {
        use my_http_server_controllers::controllers::documentation::*;
        let mut __hos = data_types::HttpObjectStructure::new("UnathorizedRequestResponse");
        __hos.fields.push(data_types::HttpField::new(
            "reason",
            UnauthorizedReasonCode::get_data_type(),
            true,
            None,
        ));
        __hos.fields.push(data_types::HttpField::new(
            "message",
            String::get_data_type(),
            true,
            None,
        ));
        __hos.fields.push(data_types::HttpField::new(
            "claim",
            String::get_data_type(),
            true,
            None,
        ));
        __hos.into_http_data_type_object()
    }
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
