use std::{collections::HashSet, iter};

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::RoleMarker, Id};
use twilight_util::builder::embed::EmbedBuilder;

use crate::types::{
    cache::GuildUpdate,
    context::Context,
    interaction::{ApplicationCommandInteraction, DeferInteractionPayload, UpdatePayload},
    Result,
};

#[derive(CommandModel, CreateCommand)]
#[command(desc = "Add a level role", name = "add-level-role")]
pub struct ConfigAddLevelRoleCommand {
    #[command(desc = "The level to apply the role to")]
    level: i64,
    #[command(desc = "The role to add", rename = "role")]
    role_id: Id<RoleMarker>,
}

impl ConfigAddLevelRoleCommand {
    pub async fn run(
        context: &Context,
        interaction: &ApplicationCommandInteraction<'_>,
        options: Self,
    ) -> Result<()> {
        interaction
            .context
            .defer(DeferInteractionPayload {
                ephemeral: false,
            })
            .await?;

        let Self {
            level,
            role_id,
        } = options;
        let mut guild_levels = interaction.cached_guild.levels.read().clone();
        let description = match guild_levels
            .iter_mut()
            .find(|guild_level| level.eq(&(guild_level.0 as i64)))
        {
            None => {
                let l = level as u64;
                let r_ids = HashSet::from_iter(iter::once(role_id));

                context
                    .database
                    .insert_level(interaction.cached_guild.guild_id, l, r_ids.clone())
                    .await?;

                guild_levels.push((l, r_ids));

                format!("Members will now receive <@&{role_id}> at level {level}.")
            }
            Some(guild_level) => {
                if guild_level.1.contains(&role_id) {
                    format!("<@&{role_id}> is already an added role at level {level}.")
                } else {
                    guild_level.1.insert(role_id);

                    context
                        .database
                        .insert_level(
                            interaction.cached_guild.guild_id,
                            guild_level.0,
                            guild_level.1.clone(),
                        )
                        .await?;

                    format!("Members will now receive <@&{role_id}> at level {level}.")
                }
            }
        };

        context.cache.update_guild(
            interaction.cached_guild.guild_id,
            GuildUpdate {
                levels: Some(guild_levels),
                ..Default::default()
            },
        );

        let embed = EmbedBuilder::new()
            .color(0xF8F8FF)
            .description(description)
            .build();

        interaction
            .context
            .update_response(UpdatePayload {
                embeds: vec![embed],
                ..Default::default()
            })
            .await?;

        Ok(())
    }
}
