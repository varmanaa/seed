use std::sync::Arc;

use twilight_model::gateway::payload::incoming::GuildCreate;

use crate::types::{context::Context, Result};

pub async fn handle_guild_create(
    context: Arc<Context>,
    payload: GuildCreate,
) -> Result<()> {
    let guild_id = payload.0.id;
    let database_members = context.database.get_members(guild_id).await?;
    let xp_multiplier = context.database.insert_guild(guild_id).await?;

    context
        .cache
        .insert_guild(payload.0, database_members, xp_multiplier);

    Ok(())
}
