use std::sync::Arc;

use twilight_model::gateway::payload::incoming::RoleDelete;

use crate::types::{cache::MemberUpdate, context::Context, Result};

pub async fn handle_role_delete(
    context: Arc<Context>,
    payload: RoleDelete,
) -> Result<()> {
    let guild_id = payload.guild_id;
    let role_id = payload.role_id;

    let updated_members = context
        .database
        .remove_owned_role(guild_id, role_id)
        .await?;

    for (user_id, owned_role_ids) in updated_members {
        context.cache.update_member(
            guild_id,
            user_id,
            MemberUpdate {
                owned_role_ids: Some(owned_role_ids),
                ..Default::default()
            },
        )
    }

    Ok(())
}
