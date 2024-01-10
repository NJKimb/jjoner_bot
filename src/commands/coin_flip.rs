use crate::{Context, Error};
use rand::{random};

///Flip a coin!
#[poise::command(slash_command, prefix_command)]
pub async fn flip(ctx: Context<'_>) -> Result<(), Error> {
    let response;
    let random_number: bool = random::<bool>();

    match random_number {
        true => response = "Heads",
        false => response = "Tails",
    }
    ctx.say(response).await?;
    Ok(())
}