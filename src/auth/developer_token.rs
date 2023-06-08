#![allow(dead_code)]

struct DeveloperToken {
    access_token: String,
}

impl DeveloperToken {
    pub fn new(access_token: String) -> Self {
        Self { access_token }
    }
}

impl DeveloperToken {
    pub fn is_valid(&self) -> bool {
        todo!("implement a check to verify if the developer token is valid");
    }
}

#[cfg(test)]
mod tests {
    use super::DeveloperToken;

    #[test]
    fn test_new_developer_token() {
        let developer_token = DeveloperToken::new("test_access_token".to_string());
        assert_eq!(developer_token.access_token, "test_access_token");
    }
}
