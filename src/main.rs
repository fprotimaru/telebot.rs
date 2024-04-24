use telebot::api::{APIError, API};
use tokio::{
    time::{
        Duration,
        sleep,
    },
};

#[tokio::main]
async fn main() -> Result<(), APIError> {
    let bot = API::new(std::env::var("TOKEN").unwrap());
    // let user = bot.get_me().await?;
    // println!("{:?}", user);

    // let updates = bot.get_updates().await?;
    // println!("{updates:?}");
    let rv = bot.get_updates_chan().await;
    for message in rv {
        sleep(Duration::from_secs(1)).await;
        println!("{:?}", message);
    }

    Ok(())
}
