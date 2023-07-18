use std::sync::Arc;

use twilight_model::gateway::payload::incoming::MemberChunk;

use crate::types::{context::Context, Result};

pub async fn handle_member_chunk(
    context: Arc<Context>,
    payload: MemberChunk,
) -> Result<()> {
    let guild_id = payload.guild_id;

    for member in payload.members {
        let user_id = member.user.id;
        let (last_message_timestamp, owned_role_ids) =
            context.database.insert_member(guild_id, user_id).await?;

        context.cache.insert_member(
            guild_id,
            user_id,
            None,
            last_message_timestamp,
            owned_role_ids,
            None,
        );
    }

    Ok(())
}
