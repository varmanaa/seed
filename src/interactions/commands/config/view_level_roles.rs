use std::time::Duration;

use tokio::time::sleep;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::channel::message::{
    component::{ActionRow, Button, ButtonStyle},
    Component,
    ReactionType,
};
use twilight_util::builder::embed::{EmbedBuilder, EmbedFieldBuilder, EmbedFooterBuilder};

use crate::types::{
    context::Context,
    interaction::{ApplicationCommandInteraction, DeferInteractionPayload, UpdatePayload},
    Result,
};

#[derive(CommandModel, CreateCommand)]
#[command(desc = "View level roles", name = "view-level-roles")]
pub struct ConfigViewLevelRolesCommand {}

impl ConfigViewLevelRolesCommand {
    pub async fn run(
        _context: &Context,
        interaction: &ApplicationCommandInteraction<'_>,
    ) -> Result<()> {
        interaction
            .context
            .defer(DeferInteractionPayload {
                ephemeral: false,
            })
            .await?;

        let mut guild_levels = interaction.cached_guild.levels.read().clone();
        let mut embed_builder = EmbedBuilder::new()
            .color(0xF8F8FF)
            .title(format!("{} level role(s)", interaction.cached_guild.name));

        if guild_levels.is_empty() {
            embed_builder = embed_builder.description("There are no level roles in this guild");

            interaction
                .context
                .update_response(UpdatePayload {
                    embeds: vec![embed_builder.build()],
                    ..Default::default()
                })
                .await?;

            return Ok(());
        }

        guild_levels.sort_unstable_by_key(|guild_level| guild_level.0);

        for (level, role_ids) in guild_levels.iter().take(5) {
            embed_builder = embed_builder.field(
                EmbedFieldBuilder::new(
                    format!("Level {level}"),
                    role_ids
                        .into_iter()
                        .map(|role_id| format!("- <@&{role_id}>"))
                        .collect::<Vec<String>>()
                        .join("\n"),
                )
                .build(),
            )
        }

        if guild_levels.len() > 5 {
            embed_builder = embed_builder.footer(EmbedFooterBuilder::new(format!(
                "Page 1 of {}",
                (guild_levels.len() as f32 / 5.0).ceil()
            )))
        }

        let components = if guild_levels.len() > 5 {
            vec![Component::ActionRow(ActionRow {
                components: vec![
                    Component::Button(Button {
                        custom_id: Some("level-roles-previous".to_owned()),
                        disabled: false,
                        emoji: Some(ReactionType::Unicode {
                            name: "⬅️".to_owned(),
                        }),
                        label: None,
                        style: ButtonStyle::Primary,
                        url: None,
                    }),
                    Component::Button(Button {
                        custom_id: Some("level-roles-next".to_owned()),
                        disabled: false,
                        emoji: Some(ReactionType::Unicode {
                            name: "➡️".to_owned(),
                        }),
                        label: None,
                        style: ButtonStyle::Primary,
                        url: None,
                    }),
                ],
            })]
        } else {
            Vec::new()
        };
        let embed = embed_builder.build();

        interaction
            .context
            .update_response(UpdatePayload {
                components: components.clone(),
                embeds: vec![embed.clone()],
                ..Default::default()
            })
            .await?;

        sleep(Duration::from_secs(15)).await;

        interaction
            .context
            .update_response(UpdatePayload {
                embeds: interaction.context.response().await?.embeds,
                ..Default::default()
            })
            .await?;

        Ok(())
    }
}
