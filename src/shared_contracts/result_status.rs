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

    #[http_enum_case(id="-8"; description="Asset pair not found")]
    AssetPairNotFound = -8,

    #[http_enum_case(id="-9"; description="No liquidity")]
    NoLiquidity = -9,

    #[http_enum_case(id="-10"; description="Exchange by this pair is disabled")]
    ExchangeByThisPairIsDisabled = -10,

    #[http_enum_case(id="-11"; description="Order Not Found")]
    OrderIsNotFound = -11,

    #[http_enum_case(id="-12"; description="Order is already expired")]
    OrderIsAlreadyExpired = -12,

    #[http_enum_case(id="-13"; description="Order is paid")]
    OrderIsPaid = -13,

    #[http_enum_case(id="-14"; description="Issuer and payer can not be the same")]
    IssuerAndPayerCanNotBeTheSame = -14,

    #[http_enum_case(id="-15"; description="Order payer is not set")]
    OrderPayerIsNotSet = -15,

    #[http_enum_case(id="-16"; description="You are not the payer of the order")]
    YouAreNotThePayerOfTheOrder = -16,

    #[http_enum_case(id="-17"; description="Crypto deposit operation is not supported")]
    CryptoDepositIsNotSupported = -17,

    #[http_enum_case(id="-18"; description="Order is canceled")]
    OrderIsCanceled = -18,

    #[http_enum_case(id="-19"; description="Not merchant")]
    NotMerchant = -19,
    
    #[http_enum_case(id="-20"; description="Wallet not whitelisted")]
    WalletNotWhitelisted = -20,
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
