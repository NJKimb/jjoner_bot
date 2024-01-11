use crate::{Context, Error, UserInformation};
use std::clone::Clone;
use std::ops::{Deref, DerefMut};

///Dig in the ground for treasure
#[poise::command(slash_command, prefix_command)]
pub async fn dig(ctx: Context<'_>) -> Result<(), Error> {
    let points_added: u32 = rand::random::<u32>() % 250;
    let mut user: Vec<UserInformation> = get_or_create_user(ctx).to_owned();

     user[0].points += points_added;

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

///Get the amount of users
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

fn get_or_create_user(ctx: Context<'_>) -> &[UserInformation] {
    let mut users_vec_mutex = ctx.data().users.lock().unwrap();
    let users_vec = users_vec_mutex.deref_mut();
    let sender_uid = ctx.author().id;

    for info in &mut *users_vec {
        if info.user_id == sender_uid {
            return std::slice::from_ref(info);
        }
    }

    let user = UserInformation {
        points: 0,
        user_id: sender_uid
    };
    users_vec.push(user);
    return std::slice::from_ref(&user);
}