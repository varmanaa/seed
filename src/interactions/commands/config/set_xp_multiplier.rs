use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_util::builder::embed::EmbedBuilder;

use crate::types::{
    cache::GuildUpdate,
    context::Context,
    interaction::{ApplicationCommandInteraction, DeferInteractionPayload, UpdatePayload},
    Result,
};

#[derive(CommandModel, CreateCommand)]
#[command(
    desc = "Set the experience multiplier for the guild",
    name = "set-xp-multiplier"
)]
pub struct ConfigSetXpMultiplierCommand {
    #[command(desc = "The multiplier", max_value = 5f64, min_value = 1f64)]
    multiplier: f64,
}

impl ConfigSetXpMultiplierCommand {
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
            multiplier,
        } = options;
        let xp_multiplier = f64::trunc(multiplier * 10.0) / 10.0;

        context
            .database
            .update_xp_multiplier(interaction.cached_guild.guild_id, xp_multiplier)
            .await?;
        context.cache.update_guild(
            interaction.cached_guild.guild_id,
            GuildUpdate {
                xp_multiplier: Some(xp_multiplier),
                ..Default::default()
            },
        );

        let embed = EmbedBuilder::new()
            .color(0xF8F8FF)
            .description(format!("The XP multipler is now {xp_multiplier}x."))
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
