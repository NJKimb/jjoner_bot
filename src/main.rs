mod commands;

use std::sync::Mutex;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::UserId;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Clone)]
struct UserInformation {
    points: u32,
    user_id: UserId
}

pub struct Data {
    users: Mutex<Vec<UserInformation>>,
}

#[tokio::main]
async fn main() {

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions{
            commands: vec![
                commands::ping::ping(),
                commands::coin_flip::flip(),
                commands::points::dig(),
                commands::points::points(),
                commands::points::get_user_count(),
                commands::dice::roll()
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
                Ok(Data{
                    users: Mutex::new(Vec::new())
                })
        })
    });
    framework.run().await.unwrap();
}
