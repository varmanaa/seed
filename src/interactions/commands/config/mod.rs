mod add_level_role;
mod remove_level_role;
mod set_xp_multiplier;
mod view_level_roles;

use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::guild::Permissions;
use twilight_util::builder::embed::EmbedBuilder;

use self::{
    add_level_role::ConfigAddLevelRoleCommand,
    remove_level_role::ConfigRemoveLevelRoleCommand,
    set_xp_multiplier::ConfigSetXpMultiplierCommand,
    view_level_roles::ConfigViewLevelRolesCommand,
};
use crate::types::{
    context::Context,
    interaction::{ApplicationCommandInteraction, ResponsePayload},
    Result,
};

#[derive(CommandModel, CreateCommand)]
#[command(desc = "Manage configuration", name = "config")]
pub enum ConfigCommand {
    #[command(name = "add-level-role")]
    AddLevelRole(ConfigAddLevelRoleCommand),
    #[command(name = "remove-level-role")]
    RemoveLevelRole(ConfigRemoveLevelRoleCommand),
    #[command(name = "set-xp-multiplier")]
    SetXpMultiplier(ConfigSetXpMultiplierCommand),
    #[command(name = "view-level-roles")]
    ViewLevelRoles(ConfigViewLevelRolesCommand),
}

impl ConfigCommand {
    pub async fn run(
        context: &Context,
        interaction: &mut ApplicationCommandInteraction<'_>,
    ) -> Result<()> {
        if !interaction.user_permissions.map_or(false, |permissions| {
            permissions.contains(Permissions::ADMINISTRATOR)
        }) {
            let embed = EmbedBuilder::new()
                .color(0xF8F8FF)
                .description(
                    "You must be have administrator permissions in order to use this command."
                        .to_owned(),
                )
                .build();

            interaction
                .context
                .respond(ResponsePayload {
                    embeds: vec![embed],
                    ephemeral: true,
                    ..Default::default()
                })
                .await?;
        } else {
            match ConfigCommand::from_interaction(interaction.input_data())? {
                ConfigCommand::AddLevelRole(options) => {
                    ConfigAddLevelRoleCommand::run(context, interaction, options).await?
                }
                ConfigCommand::RemoveLevelRole(options) => {
                    ConfigRemoveLevelRoleCommand::run(context, interaction, options).await?
                }
                ConfigCommand::SetXpMultiplier(options) => {
                    ConfigSetXpMultiplierCommand::run(context, interaction, options).await?
                }
                ConfigCommand::ViewLevelRoles(_) => {
                    ConfigViewLevelRolesCommand::run(context, interaction).await?
                }
            }
        }

        Ok(())
    }
}
