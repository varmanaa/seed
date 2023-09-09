use std::{cmp, collections::HashSet, sync::Arc};

use twilight_model::{
    gateway::payload::incoming::MemberAdd,
    id::{marker::RoleMarker, Id},
};

use crate::{
    types::{context::Context, Result},
    utility::constants::FLUCTUATING_XP,
};

pub async fn handle_member_add(
    context: Arc<Context>,
    payload: MemberAdd,
) -> Result<()> {
    let guild_id = payload.guild_id;
    let user_id = payload.user.id;
    let Some(guild) = context.cache.get_guild(guild_id) else {
        return Ok(());
    };
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
    let (current_xp, last_message_timestamp) = context
        .database
        .get_member(guild_id, user_id)
        .await?
        .unwrap_or_default();
    let current_level = FLUCTUATING_XP
        .iter()
        .position(|&level| current_xp.lt(&level.1))
        .map_or(100, |position| FLUCTUATING_XP[cmp::max(position - 1, 0)].0);
    let mut role_ids = HashSet::from_iter(payload.roles.clone());
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
        payload.user.bot,
        payload.user.discriminator,
        guild_id,
        None,
        last_message_timestamp,
        role_ids,
        user_id,
        payload.user.name.clone(),
        None,
        current_xp,
    );

    Ok(())
}
