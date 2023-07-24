use std::{collections::HashSet, sync::Arc};

use time::OffsetDateTime;
use twilight_model::{
    gateway::payload::incoming::GuildCreate,
    guild::Guild as TwilightGuild,
    id::{
        marker::{ChannelMarker, RoleMarker, UserMarker},
        Id,
    },
};

use crate::types::{context::Context, Result};

pub async fn handle_guild_create(
    context: Arc<Context>,
    payload: GuildCreate,
) -> Result<()> {
    let TwilightGuild {
        channels,
        id: guild_id,
        members: guild_members,
        name,
        roles,
        voice_states,
        ..
    } = payload.0;
    let guild_role_ids = roles
        .into_iter()
        .map(|role| role.id)
        .collect::<HashSet<Id<RoleMarker>>>();

    context
        .database
        .update_guild_levels(guild_id, guild_role_ids)
        .await?;

    let levels = context.database.get_levels(guild_id).await?;
    let database_members = context.database.get_members(guild_id).await?;
    let members = guild_members
        .into_iter()
        .map(|member| {
            let user_id = member.user.id;
            let last_message_timestamp = database_members
                .iter()
                .find(|(database_member_user_id, _)| user_id.eq(database_member_user_id))
                .map(|(_, last_message_timestamp)| last_message_timestamp.clone())
                .unwrap_or_default();
            let voice_channel_id = voice_states
                .iter()
                .find(|voice_state| voice_state.user_id.eq(&user_id))
                .map_or(None, |voice_state| voice_state.channel_id);

            (user_id, last_message_timestamp, voice_channel_id)
        })
        .collect::<Vec<(
            Id<UserMarker>,
            Option<OffsetDateTime>,
            Option<Id<ChannelMarker>>,
        )>>();
    let xp_multiplier = context.database.insert_guild(guild_id).await?;

    context
        .cache
        .insert_guild(channels, guild_id, levels, members, name, xp_multiplier);

    Ok(())
}
