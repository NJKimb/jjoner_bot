use crate::{Context, Error, UserInformation};
use std::clone::Clone;
use std::ops::{DerefMut};

///Dig in the ground for treasure
#[poise::command(slash_command, prefix_command)]
pub async fn dig(ctx: Context<'_>) -> Result<(), Error> {
    let points_added: u32 = rand::random::<u32>() % 250;
    let mut exists = false;
    {
        let mut users_vec_mutex = ctx.data().users.lock().unwrap();
        let users_vec = users_vec_mutex.deref_mut();
        let sender_uid = ctx.author().id;


        for info in &mut *users_vec {
            if info.user_id == sender_uid {
                info.points += points_added;
                exists = true;
            }
        }
    }
    let response;

    match exists{
        true => response = format!("Added {points_added} points!"),
        false => response = "User does not exist! Create a new user with /create!".to_string()
    }

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

///Creates a new user
#[poise::command(slash_command, prefix_command)]
pub async fn create(ctx: Context<'_>) -> Result<(), Error> {
    let sender_uid = ctx.author().id;
    let sender_name = ctx.author().name.clone();
    let mut response = "Error creating user".to_string();
    let mut exists = false;
    {
        let mut users_vec_mutex = ctx.data().users.lock().unwrap();
        let users_vec = users_vec_mutex.deref_mut();

        for info in &mut *users_vec {
            if info.user_id == sender_uid {
                response = format!("User {sender_name} already exists!");
                exists = true;
            }
        }

        if exists == false {
            let user = UserInformation {
                points: 0,
                user_id: sender_uid
            };
            users_vec.push(user);
            response = format!("User {sender_name} successfully created!");
        }
    }

    ctx.say(response).await?;
    Ok(())
}

