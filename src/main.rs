use std::env;
use std::fs::File;
use std::io::Read;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, CommandResult};
use serenity::utils::Content;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler{}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);

    let token = get_token();
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents).event_handler(Handler)
        .framework(framework)
        .await.expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occoured while running client: {:?}", why);
    }
}

fn get_token() -> String{
    let mut file = File::open("./token.txt").unwrap();
    let mut token = String::new();
    file.read_to_string(&mut token).expect("Failed to read from file");
    return token;
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult{
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}
