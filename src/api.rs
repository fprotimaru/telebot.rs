use std::sync::mpsc;
use crate::types::{APIResponse, Method, User, Update};
use serde_json::Value;

#[derive(Debug)]
pub enum APIError {
    RequestFailed(String),
    JsonParsingFailed(String),
}

#[derive(Clone)]
pub struct API {
    // offset: i32,
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
            Method::GetUpdates => format!("{}{}/getUpdates", Self::BASE_URL, self.token),
        }
    }

    async fn make_request(&self, method: Method) -> Result<Value, APIError> {
        let url = self.make_url(method);

        let response = self
            .client
            .post(&url)
            .send()
            .await
            .map_err(|e| APIError::RequestFailed(e.to_string()))?;

        let result = response
            .json::<APIResponse>()
            .await
            .map_err(|e| APIError::JsonParsingFailed(e.to_string()))?;

        Ok(result.result)
    }

    pub async fn get_updates(&self) -> Result<Vec<Update>, APIError> {
        let response = self.make_request(Method::GetUpdates).await?;
        let updates = serde_json::from_value::<Vec<Update>>(response)
            .map_err(|e| APIError::RequestFailed(e.to_string()))?;
        Ok(updates)
    }

    pub async fn get_updates_chan(&self) -> mpsc::Receiver<Vec<Update>> {
        let (sender, recv) = mpsc::channel();

        let api_arc = std::sync::Arc::new(self.clone());

        tokio::spawn(async move {
            loop {
                match api_arc.make_request(Method::GetUpdates).await {
                    Ok(result) => {
                        match serde_json::from_value::<Vec<Update>>(result) {
                            Ok(updates) => {
                                sender.send(updates).unwrap();
                            }
                            Err(e) => {
                                println!("got error: {:?}, trying again in 4 seconds...", e);
                                tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;
                            }
                        }
                    }
                    Err(e) => {
                        println!("got error: {:?}, trying again in 3 seconds...", e);
                        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                    }
                };
            }
        });

        recv
    }

    pub async fn get_me(&self) -> Result<User, APIError> {
        match self.make_request(Method::GetMe).await {
            Ok(data) => serde_json::from_value::<User>(data)
                .map_err(|e| APIError::JsonParsingFailed(e.to_string())),
            Err(e) => Err(e),
        }
    }
}
