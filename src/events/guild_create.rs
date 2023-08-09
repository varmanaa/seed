use std::{collections::HashSet, sync::Arc};

use time::OffsetDateTime;
use twilight_model::{
    gateway::payload::incoming::GuildCreate,
    guild::Guild as TwilightGuild,
    id::{
        marker::{ChannelMarker, RoleMarker, UserMarker},
        Id,
    }
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
    let formatted_members = guild_members
        .into_iter()
        .map(|member| {
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
            let (xp, last_message_timestamp) = database_members
                .iter()
                .find(|(database_member_user_id, ..)| user_id.eq(database_member_user_id))
                .map(|(.., xp, last_message_timestamp)| (xp.to_owned(), last_message_timestamp.to_owned()))
                .unwrap_or_default();
            let voice_channel_id = voice_states
                .iter()
                .find(|voice_state| voice_state.user_id.eq(&user_id))
                .map_or(None, |voice_state| voice_state.channel_id);

            (
                avatar_url,
                member.user.bot,
                member.user.discriminator,
                last_message_timestamp,
                member.roles,
                user_id,
                member.user.name,
                voice_channel_id,
                xp
            )
        })
        .collect::<Vec<(
            String,
            bool,
            u16,
            Option<OffsetDateTime>,
            Vec<Id<RoleMarker>>,
            Id<UserMarker>,
            String,
            Option<Id<ChannelMarker>>,
            i64
        )>>();
    let xp_multiplier = context.database.insert_guild(guild_id).await?;

    context
        .cache
        .insert_guild(channels, guild_id, levels, formatted_members, name, xp_multiplier);

    Ok(())
}
