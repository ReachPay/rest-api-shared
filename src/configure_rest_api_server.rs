use std::sync::Arc;

use service_sdk::{
    my_http_server::controllers::{ControllersAuthorization, RequiredClaims},
    my_no_sql_sdk::reader::MyNoSqlDataReaderTcp,
    HttpServerBuilder,
};

use crate::middlewares::{AuthFailResponseFactory, AuthMiddleware, SessionEntity};

pub fn configure_rest_api_server(
    http_server_builder: &mut HttpServerBuilder,
    sessions_reader: Arc<MyNoSqlDataReaderTcp<SessionEntity>>,
) {
    http_server_builder.set_authorization(ControllersAuthorization::BearerAuthentication {
        global: true,
        global_claims: RequiredClaims::no_claims(),
    });

    http_server_builder.set_auth_error_factory(AuthFailResponseFactory);

    http_server_builder.add_auth_middleware(Arc::new(AuthMiddleware::new(sessions_reader)));
}
