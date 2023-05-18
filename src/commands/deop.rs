use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::config::ConfigData;

pub async fn run(
    ctx: &mut Context,
    command: &ApplicationCommandInteraction,
    dbcon: &sqlx::SqlitePool,
) -> String {
    let calling_guild = command.guild_id.expect("failed to get guild_id");
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
    let option = command
        .data
        .options
        .get(0)
        .expect("Failed to get option data.")
        .resolved
        .as_ref()
        .expect("Expected user object");

    let mut data = ctx.data.write().await;
    let config = data.get_mut::<ConfigData>().unwrap();

    if crate::permissions::is_operator(
        calling_member.as_u64(),
        calling_guild.as_u64(),
        dbcon,
        config,
    )
    .await
    {
        if to_be_deoped.as_u64().clone() == config.interaction.owner as u64 {
            return "You cannot deop the bot owner.".into();
        } else {
            println!(
                "to be deoped: {:?}, by: {:?}, in: {:?}",
                to_be_deoped, calling_member, calling_guild
            );
        }

        if let CommandDataOptionValue::User(_user, _member) = option {
            crate::permissions::remove_operator(
                to_be_deoped.as_u64(),
                calling_guild.as_u64(),
                dbcon,
            )
            .await;
            config.save();
            return format!(
                "<@{}> has been removed from the bot operators list by <@{}>.",
                to_be_deoped, calling_member
            );
        } else {
            return "Please provide a valid user".into();
        }
    } else {
        return "You do not have permission to access that command.".into();
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("deop")
        .description("Disallows a privileged user from accessing privileged bot features")
        .create_option(|option| {
            option
                .name("user")
                .description("The user to remove")
                .kind(CommandOptionType::User)
                .required(true)
        })
}
