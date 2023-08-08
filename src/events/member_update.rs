use std::{sync::Arc, collections::HashSet};

use twilight_model::gateway::payload::incoming::MemberUpdate;

use crate::types::{cache, context::Context, Result};

pub async fn handle_member_update(
    context: Arc<Context>,
    payload: MemberUpdate,
) -> Result<()> {
    let guild_id = payload.guild_id;
    let user_id = payload.user.id;

    context.cache.update_member(
        guild_id,
        user_id,
        cache::MemberUpdate {
            role_ids: Some(HashSet::from_iter(payload.roles)),
            ..Default::default()
        },
    );

    Ok(())
}
