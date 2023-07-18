use std::{collections::HashSet, sync::Arc};

use twilight_model::gateway::payload::incoming::GuildDelete;

use crate::types::{context::Context, Result};

pub async fn handle_guild_delete(
    context: Arc<Context>,
    payload: GuildDelete,
) -> Result<()> {
    let guild_id = payload.id;

    context.database.remove_guild(guild_id).await?;
    context
        .database
        .update_member_owned_role_ids(guild_id, None, HashSet::new())
        .await?;
    context.cache.remove_guild(guild_id, payload.unavailable);

    Ok(())
}
