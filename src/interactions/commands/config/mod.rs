mod add_level_role;
mod remove_level_role;
mod set_xp_multiplier;

use twilight_interactions::command::{CommandModel, CreateCommand};

use self::{
    add_level_role::ConfigAddLevelRoleCommand,
    remove_level_role::ConfigRemoveLevelRoleCommand,
    set_xp_multiplier::ConfigSetXpMultiplierCommand,
};
use crate::types::{context::Context, interaction::ApplicationCommandInteraction, Result};

#[derive(CommandModel, CreateCommand)]
#[command(desc = "Manage configuration", name = "config")]
pub enum ConfigCommand {
    #[command(name = "add-level-role")]
    AddLevelRole(ConfigAddLevelRoleCommand),
    #[command(name = "remove-level-role")]
    RemoveLevelRole(ConfigRemoveLevelRoleCommand),
    #[command(name = "set-xp-multiplier")]
    SetXpMultiplier(ConfigSetXpMultiplierCommand),
}

impl ConfigCommand {
    pub async fn run(
        context: &Context,
        interaction: &mut ApplicationCommandInteraction<'_>,
    ) -> Result<()> {
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
        }

        Ok(())
    }
}
