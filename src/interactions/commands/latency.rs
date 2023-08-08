use thousands::Separable;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_util::builder::embed::EmbedBuilder;

use crate::types::{
    context::Context,
    interaction::{ApplicationCommandInteraction, DeferInteractionPayload, UpdatePayload},
    Result,
};

#[derive(CommandModel, CreateCommand)]
#[command(desc = "Check Discord API latency", name = "latency")]
pub struct LatencyCommand {}

impl LatencyCommand {
    pub async fn run(
        context: &Context,
        interaction: &ApplicationCommandInteraction<'_>,
    ) -> Result<()> {
        interaction
            .context
            .defer(DeferInteractionPayload {
                ephemeral: false,
            })
            .await?;

        let response = interaction.context.response().await?;
        let rtt_ms = ((response.id.get() >> 22) + 1_420_070_400_000)
            - ((interaction.context.id.get() >> 22) + 1_420_070_400_000);
        let rtt_description = format!("🚀 **RTT**: {} ms", rtt_ms.separate_with_commas());
        let shard_ping_description = if let Some(latency) = context.latency(interaction.shard_id) {
            latency.average().map_or("".to_owned(), |duration| {
                let duration_ms = duration.as_millis();

                format!("🏓 **Shard:** {} ms", duration_ms.separate_with_commas())
            })
        } else {
            "".to_owned()
        };
        let description = vec![shard_ping_description, rtt_description]
            .join("\n")
            .trim()
            .to_owned();
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
