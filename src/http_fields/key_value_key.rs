use service_sdk::rust_extensions::StrOrString;

use super::ValidationError;
use service_sdk::rust_extensions;
service_sdk::macros::use_my_http_server!();

#[http_input_field]
pub struct KeyValueKey(String);

fn process_value(src: &str) -> Result<StrOrString, HttpFailResult> {
    if src.len() > 24 {
        return Err(ValidationError::new(
            "Key must be more than 3 and less than 24 symbols".to_string(),
        )
        .as_fail_result());
    }

    for c in src.as_bytes() {
        if *c >= '0' as u8 && *c <= '9' as u8 {
            continue;
        }

        if *c >= 'a' as u8 && *c <= 'z' as u8 {
            continue;
        }

        if *c == '-' as u8 {
            continue;
        }

        return Err(ValidationError::new(format!(
            "Invalid character {} of key {}",
            *c as char, src
        ))
        .as_fail_result());
    }

    Ok(src.into())
}
