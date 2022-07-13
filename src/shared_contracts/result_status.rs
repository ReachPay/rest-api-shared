use my_http_server_swagger::MyHttpStringEnum;
use serde_repr::*;

#[derive(Serialize_repr, Deserialize_repr, MyHttpStringEnum, Debug)]
#[repr(i16)]
pub enum ResultStatus {
    #[http_enum_case(id="0"; description="Operations was successful")]
    Ok,

    #[http_enum_case(id="-1"; description="User already Exists")]
    UserAlreadyExists = -1,

    #[http_enum_case(id="-2"; description="Invalid Username or Password")]
    InvalidUserNameOrPassword = -2,

    #[http_enum_case(id="-3"; description="Invalid token")]
    InvalidToken = -3,

    #[http_enum_case(id="-4"; description="Token is expired")]
    TokenIsExpired = -4,

    #[http_enum_case(id="-5"; description="Validation Error")]
    ValidationError = -5,

    #[http_enum_case(id="-6"; description="Not enough funds")]
    NotEnoughFunds = -6,

    #[http_enum_case(id="-7"; description="Asset not found")]
    AssetNoFound = -7,

    #[http_enum_case(id="-8"; description="No liquidity")]
    NoLiquidity = -8,

    #[http_enum_case(id="-9"; description="Exchange by this pair is disabled")]
    ExchangeByThisPairIsDisabled = -9,
}

#[cfg(test)]
mod test {
    use super::ResultStatus;
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Debug)]
    pub struct TestStruct {
        result: ResultStatus,
    }

    #[test]
    pub fn test_reult_deserialization() {
        let test_struct = TestStruct {
            result: ResultStatus::TokenIsExpired,
        };

        let result = serde_json::to_string(&test_struct).unwrap();

        println!("{}", result);
    }
}
