use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_options: &[CommandDataOption]) -> String {
    "```\nBalun © 2023 DosLab Electronics, LLC\nA simple utility discord bot.\n/op       <username>\tAllows a user to run privileged commands.\n/deop     <username>\tDisallows privileged access for a privileged user.\n\n/truncate <num> <m/h/d>       Sets disappering message window for the channel it is run in.\n\n/help               \tShows this info.\n```".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("help").description("How to use the bot.")
}
