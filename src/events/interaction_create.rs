use std::{mem::take, sync::Arc};

use twilight_model::{
    application::interaction::{Interaction, InteractionData},
    gateway::payload::incoming::InteractionCreate,
    guild::Permissions,
};
use twilight_util::builder::embed::EmbedBuilder;

use crate::{
    interactions::{
        commands::{
            config::ConfigCommand,
            latency::LatencyCommand,
            leaderboard::LeaderboardCommand,
            rank::RankCommand,
        },
        components::{next::NextComponent, previous::PreviousComponent},
    },
    types::{
        context::Context,
        interaction::{
            ApplicationCommandInteraction,
            InteractionContext,
            MessageComponentInteraction,
            ResponsePayload,
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
        message,
        token,
        ..
    } = payload.0;
    let interaction_context = InteractionContext {
        id,
        interaction_client: context.http.interaction(context.application_id),
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
    let Some(cached_guild) = context.cache.get_guild(guild_id) else {
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
    };

    if !app_permissions.contains(
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

    match data {
        Some(InteractionData::ApplicationCommand(data)) => {
            let mut interaction = ApplicationCommandInteraction {
                cached_guild,
                context: interaction_context,
                data,
                shard_id,
            };
            let command_name = take(&mut interaction.data.name);

            match command_name.as_str() {
                "config" => ConfigCommand::run(&context, &mut interaction).await?,
                "latency" => LatencyCommand::run(&context, &interaction).await?,
                "leaderboard" => LeaderboardCommand::run(&context, &interaction).await?,
                "rank" => RankCommand::run(&context, &mut interaction).await?,
                _ => {
                    interaction
                        .context
                        .respond(ResponsePayload {
                            embeds: vec![embed_builder
                                .description(
                                    "I have received an unknown command with the name \"{}\".",
                                )
                                .build()],
                            ..Default::default()
                        })
                        .await?;
                }
            };
        }
        Some(InteractionData::MessageComponent(data)) => {
            let mut interaction = MessageComponentInteraction {
                cached_guild,
                context: interaction_context,
                data,
                message: message.unwrap(),
                shard_id,
            };
            let component_name = take(&mut interaction.data.custom_id);

            match component_name.as_str() {
                "next" => NextComponent::run(&context, &interaction).await?,
                "previous" => PreviousComponent::run(&context, &interaction).await?,
                _ => {
                    interaction
                        .context
                        .respond(ResponsePayload {
                            embeds: vec![embed_builder
                                .description(
                                    "I have received an unknown message component interaction with the name \"{}\".",
                                )
                                .build()],
                            ..Default::default()
                        })
                        .await?;
                }
            }
        }
        _ => {
            return interaction_context
                .respond(ResponsePayload {
                    embeds: vec![embed_builder
                        .description("I have received an unknown interaction.".to_owned())
                        .build()],
                    ephemeral: true,
                    ..Default::default()
                })
                .await
        }
    }

    Ok(())
}
