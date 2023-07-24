pub mod config;
pub mod latency;

use twilight_interactions::command::CreateCommand;
use twilight_model::application::command::Command;

pub fn get_commands() -> Vec<Command> {
    vec![
        config::ConfigCommand::create_command().into(),
        latency::LatencyCommand::create_command().into(),
    ]
}
