use poise::serenity_prelude as serenity;
use chrono::{DateTime, Local};
use crate::{Context, Error};
use poise::futures_util::StreamExt;

pub struct Quote {
    author: String,
    date: String,
    quote_message: String,
}

#[poise::command(slash_command)]
pub async fn add_quote(ctx: Context<'_>, #[description = "ID of message to quote"] message_id: String,
   #[description = "Channel to get message from"] channel: serenity::model::channel::Channel,
   #[description = "Name for the quote"] quote_name: String) -> Result<(), Error> {

    let mut channel_iter = channel.id().messages_iter(&ctx).boxed();

    while let Some(message) = channel_iter.next().await {
        match message {
            Ok(message) => {
                if message.id == message_id.parse::<u64>().unwrap() {
                    let new_quote = Quote {
                        author: message.as_ref().to_string(),
                        date: Local::now().format("%M-%D-%Y").to_string(),
                        quote_message: message.content
                    };
                    ctx.data().quotes.lock().unwrap().insert(quote_name.clone(), new_quote);
                    break
                }
            }
            _ => {}
        }
    }

    ctx.say("Added quote!").await.expect("TODO: panic message");

    Ok(())
}

