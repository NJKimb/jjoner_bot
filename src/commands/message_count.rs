use poise::futures_util::StreamExt;
use poise::serenity_prelude as serenity;
use crate::{Context, Error};
use crate::serenity::CommandType::Message;

///Count the amount of messages sent in the channel for a given user
#[poise::command(slash_command)]
pub async fn message_count(ctx: Context<'_>, #[description = "User to get message count of"] user: serenity::User
    ,#[description = "Channel to get messages from"] channel: serenity::model::channel::Channel) -> Result<(), Error> {
    let mut message_count = 0;
    let mut channel_iter = channel.id().messages_iter(&ctx).boxed();

    ctx.say("Counting messages...").await?;

    while let Some(message_result) = channel_iter.next().await {
        match message_result {
            Ok(message) => {
                if message.author == user {
                    message_count += 1;
                }
            }
            _ => {}
        }
    }
    let response = format!("<@{}> has sent {} messages in {}", user.id.0, message_count, channel);
    ctx.say(response).await?;
    Ok(())
}