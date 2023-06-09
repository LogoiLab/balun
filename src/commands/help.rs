use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_options: &[CommandDataOption]) -> String {
    return "Please contact <@247463430666059776> for help".into();
}
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("help").description("How to use the bot")
}
