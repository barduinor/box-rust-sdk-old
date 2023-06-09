use crate::api::api_client::ApiClient;
use crate::api::api_error::ApiError;
use crate::users::models::UserFull;

// pub async fn me(client: &ApiClient) -> Result<UserFull, ApiError> {
//     let req_resp = client.get("/users/me").await?;

//     let api_resp = req_resp.error_for_status()?;
//     let api_result = serde_json::from_str(&api_resp.text().await?).map_err(|e| {
//         serde_json::from_str(&e.to_string()).unwrap_or_else(|_| ApiError {
//             message: Some(e.to_string()),
//             ..Default::default()
//         })
//     })?;

//     Ok(api_result)
// }

pub async fn me(client: &ApiClient) -> Result<UserFull, ApiError> {
    let req_resp = client.get("/users/me").await;
    match req_resp {
        Ok(req_resp) => match req_resp.error_for_status() {
            Ok(api_resp) => {
                let api_result = serde_json::from_str(&api_resp.text().await.unwrap());
                match api_result {
                    Ok(result) => return Ok(result),
                    Err(e) => {
                        let error = serde_json::from_str(&e.to_string());
                        match error {
                            Ok(error) => return Err(error),
                            Err(e) => Err(ApiError {
                                message: Some(e.to_string()),
                                ..Default::default()
                            }),
                        }
                    }
                }
            }
            Err(e) => {
                let error = serde_json::from_str(&e.to_string());
                match error {
                    Ok(error) => return Err(error),
                    Err(e) => Err(ApiError {
                        message: Some(e.to_string()),
                        ..Default::default()
                    }),
                }
            }
        },
        Err(e) => {
            // Error sending request, i.g. network error
            let error = serde_json::from_str(&e.to_string());
            match error {
                Ok(error) => return Err(error),
                Err(e) => Err(ApiError {
                    message: Some(e.to_string()),
                    ..Default::default()
                }),
            }
        }
    }
}
