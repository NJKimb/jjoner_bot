use std::fs::File;
use std::io::{BufReader, Write};
use std::ops::DerefMut;
use crate::{Context, Error, UserInformation};

pub async fn serialize_users(ctx: Context<'_>) -> Result<(), Error> {
    let mut users_vec_mutex = ctx.data().users.lock().unwrap();
    let users_vec = users_vec_mutex.deref_mut();

    let json = serde_json::to_vec_pretty(users_vec).expect("Could not parse JSON!");
    let mut file = File::options().write(true).open("user_information.txt").expect("Could not open file!");
    file.write(&*json).expect("TODO: panic message");

    Ok(())
}

pub fn deserialize_users() -> Vec<UserInformation> {
    let mut users_vec = Vec::new();

    let file = File::options().read(true).open("user_information.txt");
    let reader = BufReader::new(file.expect("Could not read file!"));
    let users: Vec<UserInformation> = serde_json::from_reader(reader).expect("Could not read from file to JSON!");

    for user in users {
        users_vec.push(user);
    }

    return users_vec;
}