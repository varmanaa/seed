use std::sync::Arc;

use twilight_model::gateway::payload::incoming::RoleDelete;

use crate::types::{context::Context, Result};

pub async fn handle_role_delete(
    context: Arc<Context>,
    payload: RoleDelete,
) -> Result<()> {
    let guild_id = payload.guild_id;
    let role_id = payload.role_id;

    context
        .database
        .update_guild_levels(guild_id, vec![role_id])
        .await?;

    Ok(())
}
