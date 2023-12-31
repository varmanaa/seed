use std::{cmp, collections::HashSet, sync::Arc};

use time::OffsetDateTime;
use twilight_model::{
    gateway::payload::incoming::VoiceStateUpdate,
    id::{marker::RoleMarker, Id},
};

use crate::{
    types::{
        cache::{ChannelUpdate, MemberUpdate},
        context::Context,
        Result,
    },
    utility::constants::FLUCTUATING_XP,
};

pub async fn handle_voice_state_update(
    context: Arc<Context>,
    payload: VoiceStateUpdate,
) -> Result<()> {
    let voice_state = payload.0;
    let Some(guild_id) = voice_state.guild_id else {
        return Ok(())
    };
    let Some(guild) = context.cache.get_guild(guild_id) else {
        return Ok(())
    };
    let user_id = voice_state.user_id;
    let Some(member) = context.cache.get_member(guild_id, user_id) else {
        return Ok(())
    };

    if member.bot {
        return Ok(());
    }

    if let Some(channel_id) = voice_state.channel_id {
        let joined_voice_timestamp = if voice_state.deaf
            | voice_state.mute
            | voice_state.self_deaf
            | voice_state.self_mute
            | voice_state.suppress
        {
            Some(OffsetDateTime::now_utc())
        } else {
            member.joined_voice_timestamp.read().clone()
        };

        context.cache.update_member(
            guild_id,
            user_id,
            MemberUpdate {
                joined_voice_timestamp: Some(joined_voice_timestamp),
                voice_channel_id: Some(Some(channel_id)),
                ..Default::default()
            },
        );

        let Some(channel) = context.cache.get_channel(channel_id) else {
            return Ok(())
        };
        let mut channel_user_ids = channel.user_ids.read().clone();

        channel_user_ids.insert(user_id);

        context.cache.update_channel(
            channel_id,
            ChannelUpdate {
                user_ids: Some(channel_user_ids),
            },
        );
    } else {
        let Some(channel_id) = *member.voice_channel_id.read() else {
            return Ok(())
        };
        let Some(channel) = context.cache.get_channel(channel_id) else {
            return Ok(())
        };
        let mut channel_user_ids = channel.user_ids.read().clone();

        channel_user_ids.remove(&user_id);

        context.cache.update_channel(
            channel_id,
            ChannelUpdate {
                user_ids: Some(channel_user_ids.clone()),
            },
        );

        let Some(joined_voice_timestamp) = *member.joined_voice_timestamp.read() else {
            return Ok(())
        };
        let now = OffsetDateTime::now_utc();
        let elapsed_seconds = now.unix_timestamp() - joined_voice_timestamp.unix_timestamp();
        let xp_multiplier = *guild.xp_multiplier.read();
        let xp = ((elapsed_seconds as f64) * xp_multiplier / 4.0).floor() as i64;

        let current_xp = member.xp.read().to_owned();
        let current_level = FLUCTUATING_XP
            .iter()
            .position(|&level| current_xp.lt(&level.1))
            .map_or(100, |position| FLUCTUATING_XP[cmp::max(position - 1, 0)].0);
        let updated_xp = current_xp + xp;
        let updated_level = FLUCTUATING_XP
            .iter()
            .position(|&level| updated_xp.lt(&level.1))
            .map_or(100, |position| FLUCTUATING_XP[cmp::max(position - 1, 0)].0);

        if updated_level.ne(&current_level) {
            let mut member_role_ids = member.role_ids.read().to_owned();
            let level_role_ids = guild
                .levels
                .read()
                .to_owned()
                .into_iter()
                .filter_map(|(level, role_ids)| level.le(&updated_level).then(|| role_ids))
                .flatten()
                .collect::<HashSet<Id<RoleMarker>>>();

            member_role_ids.extend(level_role_ids);

            context
                .http
                .update_guild_member(guild_id, user_id)
                .roles(&member_role_ids.into_iter().collect::<Vec<Id<RoleMarker>>>())
                .await?;
        }

        context
            .database
            .update_member_xp(guild_id, user_id, updated_xp, None)
            .await?;
        context.cache.update_member(
            guild_id,
            user_id,
            MemberUpdate {
                joined_voice_timestamp: Some(None),
                voice_channel_id: Some(None),
                xp: Some(updated_xp),
                ..Default::default()
            },
        );

        if channel_user_ids.len() == 1 {
            let Some(only_user_id) = channel_user_ids.iter().next().cloned() else {
                return Ok(())
            };
            let Some(only_member) = context.cache.get_member(guild_id, only_user_id) else {
                return Ok(())
            };

            let Some(only_user_joined_voice_timestamp) = *only_member.joined_voice_timestamp.read() else {
                return Ok(())
            };
            let only_user_elapsed_seconds =
                now.unix_timestamp() - only_user_joined_voice_timestamp.unix_timestamp();
            let only_user_xp =
                ((only_user_elapsed_seconds as f64) * xp_multiplier / 4.0).floor() as i64;

            let only_user_current_xp = only_member.xp.read().to_owned();
            let only_user_current_level = FLUCTUATING_XP
                .iter()
                .position(|&level| only_user_current_xp.lt(&level.1))
                .map_or(100, |position| FLUCTUATING_XP[cmp::max(position - 1, 0)].0);
            let only_user_updated_xp = only_user_current_xp + only_user_xp;
            let only_user_updated_level = FLUCTUATING_XP
                .iter()
                .position(|&level| only_user_updated_xp.lt(&level.1))
                .map_or(100, |position| FLUCTUATING_XP[cmp::max(position - 1, 0)].0);

            if only_user_updated_level.ne(&only_user_current_level) {
                let mut only_user_role_ids = only_member.role_ids.read().to_owned();
                let level_role_ids = guild
                    .levels
                    .read()
                    .to_owned()
                    .into_iter()
                    .filter_map(|(level, role_ids)| level.le(&updated_level).then(|| role_ids))
                    .flatten()
                    .collect::<HashSet<Id<RoleMarker>>>();

                only_user_role_ids.extend(level_role_ids);

                context
                    .http
                    .update_guild_member(guild_id, only_user_id)
                    .roles(
                        &only_user_role_ids
                            .into_iter()
                            .collect::<Vec<Id<RoleMarker>>>(),
                    )
                    .await?;
            }

            context
                .database
                .update_member_xp(guild_id, only_user_id, only_user_updated_xp, None)
                .await?;
            context.cache.update_member(
                guild_id,
                only_user_id,
                MemberUpdate {
                    joined_voice_timestamp: Some(None),
                    voice_channel_id: Some(None),
                    xp: Some(only_user_current_xp),
                    ..Default::default()
                },
            );
        }
    }

    Ok(())
}
