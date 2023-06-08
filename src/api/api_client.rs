#![allow(dead_code)]
use reqwest::Client;

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

    pub async fn get(&self, path: &str) -> Result<String, reqwest::Error> {
        let url = format!("{}{}", self.api_config.base_api_url(), path);
        let response = self.client.get(&url).send().await?;

        match response.error_for_status() {
            Ok(response) => response.text().await,
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ApiClient;

    #[tokio::test]
    async fn test_generic_get() {
        // Create a new instance of ApiClient
        let client = ApiClient::new();

        // Perform a GET request
        let result = client.get("/example").await;
        println!("\nresult {:#?}\n", result);
        // print!("result {:#?}\n", result);

        // Check if the GET request was successful
        assert!(result.is_ok());

        // Optionally, you can further validate the response content or other aspects
        // For example, you can assert specific response content using regex or exact matching.
        // You may need to mock the network request or use a test server to provide a response.
    }
}
