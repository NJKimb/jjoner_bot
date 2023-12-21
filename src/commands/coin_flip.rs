use poise::serenity_prelude as serenity;
use crate::{Context, Error};
use rand::{random, Rng};

///Ping a user
#[poise::command(slash_command)]
pub async fn flip(ctx: Context<'_>) -> Result<(), Error> {
    let mut response = "";
    let random_number: bool = random::<bool>();
    if (random_number == true){
        response = "Heads";
    }
    else if (random_number == false){
        response = "Tails";
    }
    else {
       response = "Error";
    }
    ctx.say(response).await?;
    Ok(())
}