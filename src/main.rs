mod commands;

use std::collections::HashMap;
use std::sync::Mutex;
use poise::serenity_prelude as serenity;
use crate::commands::quotes::Quote;

pub struct Data {
    quotes: Mutex<HashMap<String, Quote>>
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions{
            commands: vec![
                commands::ping::ping(),
                commands::message_count::message_count(),
                commands::quotes::add_quote()],
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("Missing token"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework|{
        Box::pin(async move {
            poise::builtins::register_globally(ctx, &framework.options().commands).await?;
            Ok(Data{
                quotes: Mutex::new(HashMap::new())
            })
        })
    });
    framework.run().await.unwrap();
}
