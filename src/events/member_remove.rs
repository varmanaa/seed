use std::{collections::HashSet, sync::Arc};

use twilight_model::gateway::payload::incoming::MemberRemove;

use crate::types::{context::Context, Result};

pub async fn handle_member_remove(
    context: Arc<Context>,
    payload: MemberRemove,
) -> Result<()> {
    let guild_id = payload.guild_id;
    let user_id = payload.user.id;

    context
        .database
        .update_member_owned_role_ids(guild_id, Some(user_id), HashSet::new())
        .await?;
    context.cache.remove_member(guild_id, user_id);

    Ok(())
}
