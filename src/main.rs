mod commands;

use std::collections::HashMap;
use std::sync::Mutex;
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
                commands::coin_flip::flip(),
            ],

            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                additional_prefixes: vec![
                    poise::Prefix::Literal("hey bot"),
                    poise::Prefix::Literal("hey bot,"),
                ],
                ..Default::default()
            },

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
