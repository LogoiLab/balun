use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_options: &[CommandDataOption]) -> String {
    //update to your pingsock id by typing `\:pingsock:` in your guild and sending the message.
    "<:pingsock:1019706326751907870>".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("Are. you. still. there?")
}
