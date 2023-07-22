use std::sync::Arc;

use twilight_model::gateway::payload::incoming::MemberRemove;

use crate::types::{context::Context, Result};

pub async fn handle_member_remove(
    context: Arc<Context>,
    payload: MemberRemove,
) -> Result<()> {
    let guild_id = payload.guild_id;
    let user_id = payload.user.id;

    context.cache.remove_member(guild_id, user_id);

    Ok(())
}
