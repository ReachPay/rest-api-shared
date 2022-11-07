use my_http_server::{HttpContext, RequestCredentials};

const ENGLISH: &str = "en";

pub enum LanguageId {
    English,
}

impl LanguageId {
    pub fn as_str(&self) -> &str {
        match self {
            LanguageId::English => ENGLISH,
        }
    }
}

impl Default for LanguageId {
    fn default() -> Self {
        Self::English
    }
}

pub trait GetLanguageId {
    fn get_language_id(&self) -> LanguageId;
}

impl<TRequestCredentials: RequestCredentials + Send + Sync + 'static> GetLanguageId
    for HttpContext<TRequestCredentials>
{
    fn get_language_id(&self) -> LanguageId {
        LanguageId::default()
    }
}
