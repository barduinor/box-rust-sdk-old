#![allow(dead_code)]
use reqwest::{Client, Response};

use super::api_config::ApiConfig;

#[derive(Debug)]
struct ApiClient {
    api_config: ApiConfig,
    client: Client,
}

impl Default for ApiClient {
    fn default() -> ApiClient {
        ApiClient {
            api_config: ApiConfig::default(),
            client: Client::new(),
        }
    }
}

impl ApiClient {
    pub fn new() -> ApiClient {
        ApiClient::default()
    }

    pub async fn get(&self, path: &str) -> Result<Response, reqwest::Error> {
        let url = format!("{}{}", self.api_config.base_api_url(), path);
        let response = self.client.get(&url).send().await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use crate::api::api_config::ApiConfig;

    use super::ApiClient;

    #[tokio::test]
    async fn test_generic_get() {
        // Create a new instance of ApiClient
        let client = ApiClient::new();

        // Perform a GET request
        let result = client.get("/example").await;

        // Check if the GET request was successful
        assert!(result.is_ok());

        // Optionally, you can further validate the response content or other aspects
        // For example, you can assert specific response content using regex or exact matching.
        // You may need to mock the network request or use a test server to provide a response.
    }

    #[tokio::test]
    async fn test_generic_get_invalid_url() {
        let mut config = ApiConfig::new();
        config.set_base_api_url("http://localhost:3000".to_string());

        let client = ApiClient {
            api_config: config,
            ..Default::default()
        };
        let result = client.get("/example").await;
        match result {
            Ok(_) => panic!("Expected an error"),
            Err(err) => {
                assert!(true, "Got error:{}", err.to_string())
            }
        }
    }
}
