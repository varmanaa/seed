use std::sync::Arc;

use rand::{rngs::StdRng, Rng, SeedableRng};
use time::{ext::NumericalDuration, OffsetDateTime};
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::types::{cache::MemberUpdate, context::Context, Result};

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
    let user_id = payload.0.author.id;
    let message_epoch = (payload.0.id.get() >> 22) + 1_420_070_400_000;
    let message_timestamp = OffsetDateTime::from_unix_timestamp(message_epoch as i64).unwrap();

    if let Some(member) = context.cache.get_member(guild_id, user_id) {
        if let Some(last_message_timestamp) = member.last_message_timestamp.read().clone() {
            let new_message_timestamp_threshold =
                last_message_timestamp.saturating_add(1.minutes());

            if new_message_timestamp_threshold.gt(&message_timestamp) {
                return Ok(());
            }
        }
    };

    let mut rng: StdRng = SeedableRng::from_entropy();
    let base_xp = rng.gen_range(35 ..= 45);
    let xp_multiplier = *guild.xp_multiplier.read();
    let xp = ((base_xp as f64) * xp_multiplier).floor() as i64;

    context
        .database
        .update_member_xp(guild_id, user_id, xp, Some(message_timestamp))
        .await?;
    context.cache.update_member(
        guild_id,
        user_id,
        MemberUpdate {
            last_message_timestamp: Some(Some(message_timestamp)),
            ..Default::default()
        },
    );

    Ok(())
}
