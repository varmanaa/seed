use std::{cmp, collections::HashSet, sync::Arc};

use twilight_model::{
    gateway::payload::incoming::MemberChunk,
    id::{marker::RoleMarker, Id},
};

use crate::{
    types::{context::Context, Result},
    utility::constants::FLUCTUATING_XP,
};

pub async fn handle_member_chunk(
    context: Arc<Context>,
    payload: MemberChunk,
) -> Result<()> {
    let guild_id = payload.guild_id;
    let Some(guild) = context.cache.get_guild(guild_id) else {
        return Ok(());
    };

    for member in payload.members {
        let user_id = member.user.id;

        let avatar_url = if let Some(member_avatar) = member.avatar {
            format!("https://cdn.discordapp.com/guilds/{guild_id}/users/{user_id}/avatars/{member_avatar}.png")
        } else if let Some(user_avatar) = member.user.avatar {
            format!("https://cdn.discordapp.com/avatars/{user_id}/{user_avatar}.png")
        } else {
            let index = if member.user.discriminator == 0 {
                (user_id.get() >> 22) % 6
            } else {
                (member.user.discriminator % 5) as u64
            };

            format!("https://cdn.discordapp.com/embed/avatars/{index}.png")
        };

        let (current_xp, last_message_timestamp) = context
            .database
            .get_member(guild_id, user_id)
            .await?
            .unwrap_or_default();
        let current_level = FLUCTUATING_XP
            .iter()
            .position(|&level| current_xp.lt(&level.1))
            .map_or(100, |position| FLUCTUATING_XP[cmp::max(position - 1, 0)].0);
        let mut role_ids = HashSet::from_iter(member.roles.clone());
        let level_role_ids = guild
            .levels
            .read()
            .to_owned()
            .into_iter()
            .filter_map(|(level, role_ids)| level.le(&current_level).then(|| role_ids))
            .flatten()
            .collect::<HashSet<Id<RoleMarker>>>();

        role_ids.extend(level_role_ids);

        context
            .http
            .update_guild_member(guild_id, user_id)
            .roles(
                &role_ids
                    .clone()
                    .into_iter()
                    .collect::<Vec<Id<RoleMarker>>>(),
            )
            .await?;

        context.cache.insert_member(
            avatar_url,
            member.user.bot,
            member.user.discriminator,
            guild_id,
            None,
            last_message_timestamp,
            role_ids,
            user_id,
            member.user.name.clone(),
            None,
            current_xp,
        );
    }

    Ok(())
}
