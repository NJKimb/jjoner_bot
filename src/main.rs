mod commands;
mod json;

use std::sync::Mutex;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::UserId;
use serde::{Deserialize, Serialize};
use crate::json::deserialize_users;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Clone, Serialize, Deserialize)]
pub struct UserInformation {
    points: u32,
    user_id: UserId
}

pub struct Data {
    users: Mutex<Vec<UserInformation>>,
}

#[tokio::main]
async fn main() {

    let users = deserialize_users();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions{
            commands: vec![
                commands::ping::ping(),
                commands::coin_flip::flip(),
                commands::points::dig(),
                commands::points::points(),
                commands::points::get_user_count(),
                commands::points::create(),
                commands::dice::roll(),
            ],

            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("Missing token"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework|{
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data{
                    users: Mutex::new(users)
                })
        })
    });

    framework.run().await.unwrap();
}
