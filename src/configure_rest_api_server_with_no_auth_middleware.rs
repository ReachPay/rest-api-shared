use service_sdk::HttpServerBuilder;

use crate::middlewares::AuthFailResponseFactory;

pub fn configure_rest_api_server(http_server_builder: &mut HttpServerBuilder) {
    http_server_builder.set_auth_error_factory(AuthFailResponseFactory);
}
