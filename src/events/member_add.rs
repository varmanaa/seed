use std::sync::Arc;

use twilight_model::gateway::payload::incoming::MemberAdd;

use crate::types::{context::Context, Result};

pub async fn handle_member_add(
    context: Arc<Context>,
    payload: MemberAdd,
) -> Result<()> {
    let guild_id = payload.guild_id;
    let user_id = payload.user.id;
    let last_message_timestamp = context.database.insert_member(guild_id, user_id).await?;

    context.cache.insert_member(
        payload.user.discriminator,
        guild_id,
        None,
        last_message_timestamp,
        user_id,
        payload.user.name.clone(),
        None,
    );

    Ok(())
}
