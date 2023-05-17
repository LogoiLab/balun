use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

pub fn run(command: &ApplicationCommandInteraction) -> String {
    let calling_member = command
        .member
        .clone()
        .expect("failed to get command member.")
        .user
        .id;
    return format!("<@{}> PONG!", calling_member);
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("Are. you. still. there?")
}
