use serenity::all::{CreateCommand, ResolvedOption};

use crate::state::StateHandle;

pub trait SlashCommand {
    fn new(state_ref: StateHandle) -> Self;

    fn run(&self, _options: &[ResolvedOption]) -> String;

    fn register() -> CreateCommand;
}
