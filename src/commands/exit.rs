use bevy::app::AppExit;
use bevy::prelude::*;

use crate as leafwing_terminal;
use crate::ConsoleCommand;

/// Exits the app
#[derive(ConsoleCommand)]
#[console_command(name = "exit")]
pub(crate) struct ExitCommand;

pub(crate) fn exit_command(
    mut exit: ConsoleCommand<ExitCommand>,
    mut exit_writer: EventWriter<AppExit>,
) {
    if exit.take().is_some() {
        exit_writer.send(AppExit);
        exit.ok();
    }
}
