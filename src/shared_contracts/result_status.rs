use my_http_server_swagger::MyHttpStringEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, MyHttpStringEnum, Debug)]
pub enum ResultStatus {
    #[http_enum_case(id="0"; description="Operations was successful")]
    Ok,

    #[http_enum_case(id="-1"; description="User already Exists")]
    UserAlreadyExists = -1,

    #[http_enum_case(id="-2"; description="Invalid Username or Password")]
    InvalidUserNameOrPassword = -2,

    #[http_enum_case(id="-3"; description="Invalid token")]
    InvalidToken = -3,
}
