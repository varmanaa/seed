use std::{mem::take, sync::Arc};

use twilight_model::{
    application::interaction::{Interaction, InteractionData},
    gateway::payload::incoming::InteractionCreate,
    guild::Permissions,
};
use twilight_util::builder::embed::EmbedBuilder;

use crate::{
    commands::latency::LatencyCommand,
    types::{
        context::Context,
        interaction::{
            ApplicationCommandInteraction,
            ApplicationCommandInteractionContext,
            ResponsePayload,
            UpdateResponsePayload,
        },
        Result,
    },
};

pub async fn handle_interaction_create(
    context: Arc<Context>,
    shard_id: u64,
    payload: InteractionCreate,
) -> Result<()> {
    let Interaction {
        app_permissions,
        data,
        guild_id,
        id,
        token,
        ..
    } = payload.0;

    let interaction_context = ApplicationCommandInteractionContext {
        id,
        interaction_client: context.interaction_client(),
        token,
    };
    let embed_builder = EmbedBuilder::new().color(0xF8F8FF);

    let (Some(app_permissions), Some(guild_id)) = (app_permissions, guild_id) else {
        return interaction_context.respond(ResponsePayload {
            embeds: vec![embed_builder.description(format!("{} only works in guilds.", context.application_name)).build()],
            ephemeral: true,
            ..Default::default()
        })
        .await
    };

    if app_permissions.contains(
        Permissions::EMBED_LINKS
            | Permissions::MANAGE_ROLES
            | Permissions::SEND_MESSAGES
            | Permissions::VIEW_CHANNEL,
    ) {
        return interaction_context.respond(ResponsePayload {
            embeds: vec![embed_builder
                .description(format!(
                    "{} requires the **Embed Links**, **Manage Roles**, **Send Messages**, and **View Channels** permissions in this channel.",
                    context.application_name
                ))
                .build()
            ],
            ephemeral: true,
            ..Default::default()
        })
        .await;
    }

    if context.cache.get_guild(guild_id).is_none() {
        return interaction_context
            .respond(ResponsePayload {
                embeds: vec![embed_builder
                    .description(format!(
                        "Please kick and re-invite {}",
                        context.application_name
                    ))
                    .build()],
                ephemeral: true,
                ..Default::default()
            })
            .await;
    }

    let Some(InteractionData::ApplicationCommand(data)) = data else {
        return interaction_context.respond(ResponsePayload {
            embeds: vec![embed_builder.description("I have received an unknown interaction.".to_owned()).build()],
            ephemeral: true,
            ..Default::default()
        })
        .await
    };
    let mut interaction = ApplicationCommandInteraction {
        context: interaction_context,
        data,
        guild_id,
        shard_id,
    };
    let command_name = take(&mut interaction.data.name);
    let command_result = match command_name.as_str() {
        "latency" => LatencyCommand::run(&context, &interaction).await,
        _ => {
            return interaction
                .context
                .update_response(UpdateResponsePayload {
                    embeds: vec![embed_builder
                        .description("I have received an unknown command with the name \"{}\".")
                        .build()],
                    ..Default::default()
                })
                .await
        }
    };

    if let Err(error) = command_result {
        return interaction
            .context
            .update_response(UpdateResponsePayload {
                embeds: vec![embed_builder.description(error.to_string()).build()],
                ..Default::default()
            })
            .await;
    }

    Ok(())
}
