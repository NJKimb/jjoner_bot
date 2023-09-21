use poise::serenity_prelude as serenity;
use crate::{Context, Error};

///Ping a user
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>, #[description = "Who do you want to ping?"] user: Option<serenity::User>) -> Result<(), Error> {
    let response = format!("<@{}>", user.as_ref().unwrap().id.0);
    ctx.say(response).await?;
    Ok(())
}