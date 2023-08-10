use std::sync::Arc;

use twilight_http::client::InteractionClient;
use twilight_model::{
    application::interaction::{
        application_command::CommandData,
        message_component::MessageComponentInteractionData,
    },
    channel::message::{Component, Embed, Message},
    http::attachment::Attachment,
    id::{
        marker::{InteractionMarker, UserMarker},
        Id,
    },
};

use crate::types::cache::Guild;

pub struct ApplicationCommandInteraction<'a> {
    pub cached_guild: Arc<Guild>,
    pub context: InteractionContext<'a>,
    pub data: Box<CommandData>,
    pub shard_id: u64,
    pub user_id: Id<UserMarker>,
}

pub struct InteractionContext<'a> {
    pub id: Id<InteractionMarker>,
    pub interaction_client: InteractionClient<'a>,
    pub token: String,
}

#[derive(Default)]
pub struct DeferInteractionPayload {
    pub ephemeral: bool,
}

pub struct MessageComponentInteraction<'a> {
    pub cached_guild: Arc<Guild>,
    pub context: InteractionContext<'a>,
    pub data: MessageComponentInteractionData,
    pub message: Message,
    pub shard_id: u64,
}

#[derive(Default)]
pub struct ResponsePayload {
    pub components: Vec<Component>,
    pub embeds: Vec<Embed>,
    pub ephemeral: bool,
}

#[derive(Default)]
pub struct UpdatePayload {
    pub attachments: Vec<Attachment>,
    pub components: Vec<Component>,
    pub embeds: Vec<Embed>,
}
