use crate::types::{APIResponse, Method, User};
use serde_json::Value;

#[derive(Debug)]
pub enum APIError {
    RequestFailed(String),
    JsonParsingFailed(String),
}

pub struct API {
    token: String,
    client: reqwest::Client,
}

impl API {
    const BASE_URL: &'static str = "https://api.telegram.org/bot";

    pub fn new(token: String) -> Self {
        Self {
            token,
            client: reqwest::Client::new(),
        }
    }

    fn make_url(&self, method: Method) -> String {
        match method {
            Method::GetMe => format!("{}{}/getMe", Self::BASE_URL, self.token),
        }
    }

    async fn make_request(&self, method: Method) -> Result<Value, APIError> {
        let url = self.make_url(method);
        let response = match self.client.post(&url).send().await {
            Ok(response) => response,
            Err(err) => return Err(APIError::RequestFailed(err.to_string())),
        };

        match response.json::<APIResponse>().await {
            Ok(api_res) => Ok(api_res.result),
            Err(err) => Err(APIError::JsonParsingFailed(err.to_string())),
        }
    }

    pub async fn get_me(&self) -> Result<User, APIError> {
        match self.make_request(Method::GetMe).await {
            Ok(data) => serde_json::from_value::<User>(data)
                .map_err(|e| APIError::JsonParsingFailed(e.to_string())),
            Err(e) => Err(e),
        }
    }
}
