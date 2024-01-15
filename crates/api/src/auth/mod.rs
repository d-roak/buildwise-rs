
pub struct APIKey {
    key: String,
    scope: Vec<String>,
}

pub fn get_api_key() -> APIKey {
    APIKey {
        key: "test".to_string(),
        scope: vec![],
    }
}
