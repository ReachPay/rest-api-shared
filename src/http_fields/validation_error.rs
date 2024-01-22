use serde::*;
use service_sdk::my_telemetry::TelemetryEventTagsBuilder;

use crate::shared_contracts::ResultStatus;

service_sdk::macros::use_my_http_server!();

#[derive(Serialize, Deserialize, Debug, MyHttpObjectStructure)]
pub struct ValidationError {
    pub result: ResultStatus,
    pub message: String,
}

impl ValidationError {
    pub fn new(message: String) -> Self {
        Self {
            result: ResultStatus::ValidationError,
            message,
        }
    }

    pub fn as_fail_result(&self) -> HttpFailResult {
        HttpFailResult {
            content_type: my_http_server::WebContentType::Json,
            status_code: 200,
            content: serde_json::to_vec(self).unwrap(),
            write_telemetry: true,
            write_to_log: false,
            add_telemetry_tags: TelemetryEventTagsBuilder::new(),
        }
    }
}
