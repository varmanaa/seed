use std::{cmp, collections::HashSet, sync::Arc};

use rand::{rngs::StdRng, Rng, SeedableRng};
use time::{ext::NumericalDuration, OffsetDateTime};
use twilight_model::{
    gateway::payload::incoming::MessageCreate,
    id::{marker::RoleMarker, Id},
};

use crate::{
    types::{cache::MemberUpdate, context::Context, Result},
    utility::constants::FLUCTUATING_XP,
};

pub async fn handle_message_create(
    context: Arc<Context>,
    payload: MessageCreate,
) -> Result<()> {
    let Some(guild_id) = payload.0.guild_id else {
        return Ok(());
    };
    let Some(guild) = context.cache.get_guild(guild_id) else {
        return Ok(());
    };

    if payload.0.author.bot {
        return Ok(());
    }

    let user_id = payload.0.author.id;
    let message_epoch = ((payload.0.id.get() >> 22) + 1_420_070_400_000) / 1000;
    let message_timestamp = OffsetDateTime::from_unix_timestamp(message_epoch as i64).unwrap();
    let Some(member) = context.cache.get_member(guild_id, user_id) else {
        return Ok(())
    };

    if let Some(last_message_timestamp) = member.last_message_timestamp.read().clone() {
        let new_message_timestamp_threshold = last_message_timestamp.saturating_add(1.minutes());

        if new_message_timestamp_threshold.gt(&message_timestamp) {
            return Ok(());
        }
    }

    let mut rng: StdRng = SeedableRng::from_entropy();
    let base_xp = rng.gen_range(35 ..= 45);
    let xp_multiplier = *guild.xp_multiplier.read();
    let xp = ((base_xp as f64) * xp_multiplier).floor() as i64;

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
        .update_member_xp(guild_id, user_id, updated_xp, Some(message_timestamp))
        .await?;
    context.cache.update_member(
        guild_id,
        user_id,
        MemberUpdate {
            last_message_timestamp: Some(Some(message_timestamp)),
            xp: Some(updated_xp),
            ..Default::default()
        },
    );

    Ok(())
}
