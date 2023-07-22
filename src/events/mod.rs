mod channel_create;
mod channel_delete;
mod guild_create;
mod guild_delete;
mod guild_update;
mod interaction_create;
mod member_add;
mod member_chunk;
mod member_remove;
mod message_create;
mod ready;
mod role_delete;
mod unavailable_guild;
mod voice_state_update;

use std::sync::Arc;

use twilight_gateway::{Event, MessageSender};
use twilight_model::gateway::payload::outgoing::RequestGuildMembers;

use self::{
    channel_create::handle_channel_create,
    channel_delete::handle_channel_delete,
    guild_create::handle_guild_create,
    guild_delete::handle_guild_delete,
    guild_update::handle_guild_update,
    interaction_create::handle_interaction_create,
    member_add::handle_member_add,
    member_chunk::handle_member_chunk,
    member_remove::handle_member_remove,
    message_create::handle_message_create,
    ready::handle_ready,
    role_delete::handle_role_delete,
    unavailable_guild::handle_unavailable_guild,
    voice_state_update::handle_voice_state_update,
};
use crate::types::{context::Context, Result};

pub async fn handle_event(
    context: Arc<Context>,
    shard_id: u64,
    shard_sender: MessageSender,
    event: Event,
) -> Result<()> {
    match event {
        Event::ChannelCreate(payload) => handle_channel_create(context, *payload),
        Event::ChannelDelete(payload) => handle_channel_delete(context, *payload),
        Event::GuildCreate(payload) => {
            shard_sender.command(&RequestGuildMembers::builder(payload.0.id).query("", None))?;

            handle_guild_create(context, *payload).await
        }
        Event::GuildDelete(payload) => handle_guild_delete(context, payload).await,
        Event::GuildUpdate(payload) => handle_guild_update(context, *payload),
        Event::InteractionCreate(payload) => {
            handle_interaction_create(context, shard_id, *payload).await
        }
        Event::MemberAdd(payload) => handle_member_add(context, *payload).await,
        Event::MemberChunk(payload) => handle_member_chunk(context, payload).await,
        Event::MemberRemove(payload) => handle_member_remove(context, payload).await,
        Event::MessageCreate(payload) => handle_message_create(context, *payload).await,
        Event::Ready(payload) => handle_ready(context, *payload),
        Event::RoleDelete(payload) => handle_role_delete(context, payload).await,
        Event::UnavailableGuild(payload) => handle_unavailable_guild(context, payload),
        Event::VoiceStateUpdate(payload) => handle_voice_state_update(context, *payload).await,
        _ => Ok(()),
    }
}
