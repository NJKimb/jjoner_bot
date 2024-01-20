use crate::{Context, Error, UserInformation};
use std::clone::Clone;
use std::ops::{DerefMut};
use crate::json::serialize_users;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{Colour};

///Dig in the ground for treasure
#[poise::command(slash_command, prefix_command)]
pub async fn dig(ctx: Context<'_>) -> Result<(), Error> {

    let points_added: u32 = rand::random::<u32>() % 250;
    let mut response = format!("Successfully added {points_added} points!");

    add_points(ctx, points_added)
        .unwrap_or_else(|_e| {
            response = "User does not exist! Create a new user with /create!".to_string()
        });

    ctx.say(response).await?;
    serialize_users(ctx).await?; // TODO: Change this to serialize at another point

    Ok(())
}

///Play a game of High or Low!
#[poise::command(slash_command)]
pub async fn high_low(ctx: Context<'_>, #[description = "Enter an amount to wager"] wager: u32) -> Result<(), Error> {
    let hint_number: u32 = (rand::random::<u32>() % 99) + 1;
    let hidden_number: u32 = (rand::random::<u32>() % 99) + 1;
    let jackpot_amount = wager * 100;

    let reply = ctx.send(|m|{
        m.embed(|e| {
            e.description("Select a choice!")
                .title("Higher or Lower!").color(Colour::BLITZ_BLUE)
                .description(format!("A number was randomly chosen between 1 and 100, \
                is it higher, lower, or the same as the number {hint_number}?"))
        })
            .components(|c| {
            c.create_action_row(|r| {
                r.create_button(|b|{
                    b.custom_id("higher")
                    .label("Higher")
                });
                r.create_button(|b|{
                    b.custom_id("jackpot")
                        .label("Jackpot")
                        .style(serenity::ButtonStyle::Danger)
                });
                r.create_button(|b|{
                    b.custom_id("lower")
                    .label("Lower")
                })
            })
        })
    }).await?;

    let interaction = reply
        .message().await?
        .await_component_interaction(ctx)
        .author_id(ctx.author().id).await;

    reply.edit(ctx, |b| {
            b.components(|b| b).content("")
        }).await?;

    let mut response = format!("You guessed incorrectly! The hint was {hint_number} \
    and the hidden number was {hidden_number}! You lost {wager} points!");
    let mut continue_game = true;
    let mut color = Colour::RED;

    let pressed_button = match &interaction {
        Some(m) => &m.data.custom_id,
        None => {ctx.say("You did not interact in time!").await?;
            return Ok(());
        }
    };

    remove_points(ctx, wager).unwrap_or_else(|_e| {
        response = format!("You do not have enough points! You need at least {wager} points for this game!");
        continue_game = false;
    });

    if continue_game {
        match &**pressed_button {
            "higher" => {
                if hint_number < hidden_number {
                    add_points(ctx, wager * 2).unwrap_or_else(|_e| {
                        response = "User does not exist! Create a new user with /create!".to_string();
                        return;
                    });
                    response = format!("You guessed correctly! The hint was {hint_number} \
                    and the hidden number was {hidden_number}! You won {wager} points!");
                    color = Colour::KERBAL;
                }
            },
            "lower" => {
                if hint_number > hidden_number {
                    add_points(ctx, wager * 2).unwrap_or_else(|_e| {
                        response = "User does not exist! Create a new user with /create!".to_string();
                        return;
                    });
                    response = format!("You guessed correctly! The hint was {hint_number} \
                    and the hidden number was {hidden_number}! You won {wager} points!");
                    color = Colour::KERBAL
                }
            },
            "jackpot" => {
                if hint_number == hidden_number {
                    add_points(ctx, jackpot_amount).unwrap_or_else(|_e| {
                        response = "User does not exist! Create a new user with /create!".to_string();
                        return;
                    });
                    response = format!("You guessed correctly! The hint was {hint_number} \
                    and the hidden number was {hidden_number}! You won {jackpot_amount} points!")
                }
            },
            _ => {}
        };
    }

    reply.edit(ctx, |b| {
        b.embed(|e|{
            e.title("Higher or Lower!").color(color)
            .description(response)
        })
    }).await?;

    serialize_users(ctx).await?;

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

pub fn add_points(ctx: Context, points_added: u32) -> Result<(), ()> {
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

    match exists{
        true => Ok(()),
        false => Err(())
    }
}

pub fn remove_points(ctx: Context, points_removed: u32) -> Result<(), ()> {

    let mut exists = false;
    {
        let mut users_vec_mutex = ctx.data().users.lock().unwrap();
        let users_vec = users_vec_mutex.deref_mut();
        let sender_uid = ctx.author().id;

        for info in &mut *users_vec {
            if info.user_id == sender_uid {
                if info.points >= points_removed {
                    info.points -= points_removed;
                    exists = true;
                }
            }
        }
    }

    match exists{
        true => Ok(()),
        false => Err(())
    }
}