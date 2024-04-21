use telebot::api::{APIError, API};

#[tokio::main]
async fn main() -> Result<(), APIError> {
    let bot = API::new("6006223443:AAGhX_9Z01CiybMWs--GvAw50FQPWkyZ2pQ".to_string());
    let user = bot.get_me().await?;

    println!("{:?}", user);
    Ok(())
}
