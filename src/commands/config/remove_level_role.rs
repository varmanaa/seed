use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::id::{marker::RoleMarker, Id};
use twilight_util::builder::embed::EmbedBuilder;

use crate::types::{
    cache::GuildUpdate,
    context::Context,
    interaction::{ApplicationCommandInteraction, DeferInteractionPayload, UpdateResponsePayload},
    Result,
};

#[derive(CommandModel, CreateCommand)]
#[command(desc = "Remove a level role", name = "remove-level-role")]
pub struct ConfigRemoveLevelRoleCommand {
    #[command(desc = "The level to remove the role from")]
    level: i64,
    #[command(desc = "The role to remove", rename = "role")]
    role_id: Id<RoleMarker>,
}

impl ConfigRemoveLevelRoleCommand {
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
            None => format!("<@&{role_id}> is not an added role at level {level}."),
            Some(guild_level) => {
                if !guild_level.1.contains(&role_id) {
                    format!("<@&{role_id}> is already not an added role at level {level}.")
                } else if guild_level.1.len() == 1 {
                    context
                        .database
                        .remove_level(interaction.cached_guild.guild_id, guild_level.0)
                        .await?;

                    guild_levels.retain(|guild_level| !level.eq(&(guild_level.0 as i64)));

                    format!("All roles at level {level} have been removed.")
                } else {
                    guild_level.1.remove(&role_id);

                    context
                        .database
                        .insert_level(
                            interaction.cached_guild.guild_id,
                            guild_level.0,
                            guild_level.1.clone(),
                        )
                        .await?;

                    format!("Members will no longer receive <@&{role_id}> at level {level}.")
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
            .update_response(UpdateResponsePayload {
                embeds: vec![embed],
                ..Default::default()
            })
            .await?;

        Ok(())
    }
}
