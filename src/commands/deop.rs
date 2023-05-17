use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};

pub fn run(command: &ApplicationCommandInteraction) -> String {
    let to_be_deoped = command
        .data
        .resolved
        .users
        .keys()
        .next()
        .expect("Could not get required data from command.");
    let calling_member = command
        .member
        .clone()
        .expect("failed to get command member.")
        .user
        .id;
    println!("to be oped: {:?}", to_be_deoped);
    println!("calling member: {:?}", calling_member);
    let option = command
        .data
        .options
        .get(0)
        .expect("Failed to get option data.")
        .resolved
        .as_ref()
        .expect("Expected user object");

    let mut config = crate::Config::read_from_file("config.toml");
    if config
        .interaction
        .operators
        .contains(&i64::try_from(calling_member.as_u64().clone()).unwrap())
    {
        if let CommandDataOptionValue::User(_user, _member) = option {
            for i in 0..config.interaction.operators.len() {
                if config.interaction.operators.get(i).unwrap().clone()
                    == to_be_deoped.as_u64().clone() as i64
                {
                    config.interaction.operators.remove(i);
                    config.save();
                    return format!(
                        "<@{}> has been removed from the bot operators list by <@{}>.",
                        to_be_deoped, calling_member
                    );
                }
            }
            return format!("User <@{}> not found in operators list.", to_be_deoped);
        } else {
            return "Please provide a valid user.".into();
        }
    } else {
        return "You do not have permission to access that command.".into();
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("deop")
        .description("Disallows a privileged user from accessing privileged bot features.")
        .create_option(|option| {
            option
                .name("user")
                .description("The user to remove")
                .kind(CommandOptionType::User)
                .required(true)
        })
}
