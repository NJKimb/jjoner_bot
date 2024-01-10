use crate::{Context, Error, UserInformation};
use std::clone::Clone;

///Dig in the ground for treasure
#[poise::command(slash_command, prefix_command)]
pub async fn dig(ctx: Context<'_>) -> Result<(), Error> {
    let points_added: u32 = rand::random::<u32>() % 250;
    {
        let mut users_vec = ctx.data().users.lock().unwrap();
        let sender_uid = ctx.author().id;
        let mut sender = UserInformation { points: 0, user_id: sender_uid };
        let mut index: usize = 0;

        for info in users_vec.clone().into_iter() {
            if info.user_id == sender_uid {
                sender = info;
                users_vec.remove(index); // I'd like a better way to do this but this works for now
                index += 1;
            }
        }

        sender.points += points_added;
        users_vec.push(sender);
    }

    let response = format!("Added {points_added} points!");
    ctx.say(response).await?;

    Ok(())
}

///Get your balance of points
#[poise::command(slash_command, prefix_command)]
pub async fn points(ctx: Context<'_>) -> Result<(), Error> {
    let mut response;
    response = format!("No points");
    {
        for user in ctx.data().users.lock().unwrap().clone().into_iter() {
            if user.user_id == ctx.author().id {
                let points = user.points;
                response = format!("Balance is {points}!");
            }
        }
    }
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn get_user_count(ctx: Context<'_>) -> Result<(), Error> {
    let response;
    let mut user_count = 0;
    {
        for _user in ctx.data().users.lock().unwrap().clone().into_iter() {
            user_count = user_count + 1;
        }
    }

    response = format!("There are {user_count} users!");
    ctx.say(response).await?;
    Ok(())
}