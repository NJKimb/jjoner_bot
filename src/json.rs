use std::fs;
use std::fs::File;
use std::io::{BufReader};
use std::ops::DerefMut;
use crate::{Context, Error, UserInformation};

pub async fn serialize_users(ctx: Context<'_>) -> Result<(), Error> {
    let mut users_vec_mutex = ctx.data().users.lock().unwrap();
    let users_vec = users_vec_mutex.deref_mut();

    for info in users_vec{
        let json = serde_json::to_string_pretty(&info)?;
        fs::write("user_information.txt", &json).expect("TODO: panic message");
    }

    Ok(())
}

pub async fn deserialize_users(ctx: Context<'_>) -> Result<(), Error> {
    let mut users_vec_mutex = ctx.data().users.lock().unwrap();
    let users_vec = users_vec_mutex.deref_mut();

    let file = File::options().read(true).open("user_information.txt")?;
    let reader = BufReader::new(file);
    let user: UserInformation = serde_json::from_reader(reader)?;

    users_vec.push(user);

    Ok(())
}