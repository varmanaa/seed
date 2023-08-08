use std::sync::Arc;

use time::OffsetDateTime;
use twilight_model::gateway::payload::incoming::VoiceStateUpdate;

use crate::types::{
    cache::{ChannelUpdate, MemberUpdate},
    context::Context,
    Result,
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

        context
            .database
            .update_member_xp(guild_id, user_id, xp, None)
            .await?;
        context.cache.update_member(
            guild_id,
            user_id,
            MemberUpdate {
                joined_voice_timestamp: Some(None),
                voice_channel_id: Some(None),
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

            context
                .database
                .update_member_xp(guild_id, only_user_id, only_user_xp, None)
                .await?;
            context.cache.update_member(
                guild_id,
                user_id,
                MemberUpdate {
                    joined_voice_timestamp: Some(None),
                    voice_channel_id: Some(None),
                    ..Default::default()
                },
            );
        }
    }

    Ok(())
}
