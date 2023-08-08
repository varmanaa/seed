use std::{collections::HashSet, sync::Arc};

use twilight_model::gateway::payload::incoming::MemberAdd;

use crate::types::{context::Context, Result};

pub async fn handle_member_add(
    context: Arc<Context>,
    payload: MemberAdd,
) -> Result<()> {
    let guild_id = payload.guild_id;
    let user_id = payload.user.id;
    let last_message_timestamp = context.database.insert_member(guild_id, user_id).await?;
    let avatar_url = if let Some(member_avatar) = payload.avatar {
        format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{member_avatar}.png")
    } else if let Some(user_avatar) = payload.user.avatar {
        format!("https://cdn.discordapp.com/avatars/{user_id}/{user_avatar}.png")
    } else {
        let index = if payload.user.discriminator == 0 {
            (user_id.get() >> 22) % 6
        } else {
            (payload.user.discriminator % 5) as u64
        };

        format!("https://cdn.discordapp.com/embed/avatars/{index}.png")
    };
    context.cache.insert_member(
        avatar_url,
        payload.user.bot,
        payload.user.discriminator,
        guild_id,
        None,
        last_message_timestamp,
        HashSet::from_iter(payload.roles.clone()),
        user_id,
        payload.user.name.clone(),
        None,
    );

    Ok(())
}
