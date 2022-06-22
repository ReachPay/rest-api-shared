use my_http_server::HttpContext;

const DEFAULT_LANG: &str = "en";

pub trait GetLangId {
    fn get_lang_id(&self) -> &str;
}

impl GetLangId for HttpContext {
    fn get_lang_id(&self) -> &str {
        DEFAULT_LANG
    }
}
