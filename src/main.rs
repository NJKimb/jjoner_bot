mod commands;

use std::env;
use poise::serenity_prelude as serenity;

pub struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;



#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions{
            commands: vec![
                commands::ping::ping(),
                commands::message_count::message_count()],
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("Missing token"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework|{
        Box::pin(async move {
            poise::builtins::register_globally(ctx, &framework.options().commands).await?;
            Ok(Data{})
        })
    });
    framework.run().await.unwrap();
}
