use poise::futures_util::{Stream, StreamExt};
use poise::serenity_prelude::futures;
use crate::{Context, Error};

/// Roll a dice
#[poise::command(slash_command)]
pub async fn roll(ctx: Context<'_>, #[description = "Choose a number of sides (d4, d6, d8, d24 supported)"]
    #[autocomplete = "autocomplete_die"] sides: String) -> Result<(), Error> {

    let num = rand::random::<u32>();
    let mut rolled_num= 0;
    let mut valid = true;

    match sides.as_str() {
        "d4"=> rolled_num = (num % 3) + 1,
        "d6"=> rolled_num = (num % 5) + 1,
        "d8"=> rolled_num = (num % 7) + 1,
        "d24"=> rolled_num = (num % 23) + 1,
        _ => { ctx.say(format!("{sides} is not a valid die size!")).await?; valid = false;}
    }

    if valid == true {
        let response = format!("You rolled a {rolled_num} using die size {sides}");
        ctx.say(response).await?;
    }

    Ok(())
}

async fn autocomplete_die<'a>(_ctx: Context<'_>, partial: &'a str,) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(&["d4", "d6", "d8", "d24"])
        .filter(move |name| futures::future::ready(name.starts_with(partial)))
        .map(|name| name.to_string())
}