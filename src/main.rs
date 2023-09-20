mod commands;

use std::env;
use poise::serenity_prelude as serenity;

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[poise::command(slash_command, prefix_command)]
async fn ping(ctx: Context<'_>, #[description = "Who do you want to ping?"] user: Option<serenity::User>) -> Result<(), Error> {
    let response = format!("<@{}>", user.as_ref().unwrap().id.0);
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions{
            commands: vec![ping()],
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
