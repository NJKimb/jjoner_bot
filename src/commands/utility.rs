use crate::{Context, Error};
use crate::json::{deserialize_users, serialize_users};

///Deserialize users from JSON (Utility only)
#[poise::command(slash_command, prefix_command)]
pub async fn deserialize(ctx: Context<'_>) -> Result<(), Error> {
    deserialize_users(ctx).await?;
    Ok(())
}

///Serialize users to JSON (Utility only)
#[poise::command(slash_command, prefix_command)]
pub async fn serialize(ctx: Context<'_>) -> Result<(), Error> {
    serialize_users(ctx).await?;
    Ok(())
}