#![allow(dead_code)]
#[derive(Debug)]
pub struct ApiConfig {
    base_api_url: String,
    upload_url: String,
    oauth2_api_url: String,
    oauth2_authorize_url: String,
    // api_version: String,
    // TODO: implement api_versioning
    max_retry_attempts: u8,
    chunk_upload_threads: u8,
}

//     BASE_API_URL = 'https://api.box.com/2.0'
//     UPLOAD_URL = 'https://upload.box.com/api/2.0'
//     OAUTH2_API_URL = 'https://api.box.com/oauth2'  # <https://developer.box.com/reference/post-oauth2-token>
//     OAUTH2_AUTHORIZE_URL = 'https://account.box.com/api/oauth2/authorize'  # <https://developer.box.com/reference/get-authorize/>
//     MAX_RETRY_ATTEMPTS = 5
//     CHUNK_UPLOAD_THREADS = 5

impl Default for ApiConfig {
    fn default() -> Self {
        ApiConfig {
            base_api_url: String::from("https://api.box.com/2.0"),
            upload_url: String::from("https://upload.box.com/api/2.0"),
            oauth2_api_url: String::from("https://api.box.com/oauth2"),
            oauth2_authorize_url: String::from("https://account.box.com/api/oauth2/authorize"),
            max_retry_attempts: 5,
            chunk_upload_threads: 5,
        }
    }
}

impl ApiConfig {
    fn new() -> Self {
        ApiConfig::default()
    }
    pub fn base_api_url(&self) -> &str {
        &self.base_api_url
    }
}

#[cfg(test)]
mod tests {
    use super::ApiConfig;

    #[test]
    fn test_default_config_values() {
        let config = ApiConfig::default();
        // println!("{:#?}", config);

        assert_eq!(config.base_api_url, "https://api.box.com/2.0");
        assert_eq!(config.upload_url, "https://upload.box.com/api/2.0");
        assert_eq!(config.oauth2_api_url, "https://api.box.com/oauth2");
        assert_eq!(
            config.oauth2_authorize_url,
            "https://account.box.com/api/oauth2/authorize"
        );
        assert_eq!(config.max_retry_attempts, 5);
        assert_eq!(config.chunk_upload_threads, 5);
    }

    #[test]
    fn test_new_config_values() {
        let config = ApiConfig {
            base_api_url: ("https://api.box.com/3.0").to_string(),
            upload_url: ("https://upload.box.com/api/3.0").to_string(),
            ..Default::default()
        };
        println!("{:#?}", config);

        assert_eq!(config.base_api_url, "https://api.box.com/3.0");
        assert_eq!(config.upload_url, "https://upload.box.com/api/3.0");
        assert_eq!(config.oauth2_api_url, "https://api.box.com/oauth2");
        assert_eq!(
            config.oauth2_authorize_url,
            "https://account.box.com/api/oauth2/authorize"
        );
        assert_eq!(config.max_retry_attempts, 5);
        assert_eq!(config.chunk_upload_threads, 5);
    }
}
