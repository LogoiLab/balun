use std::ops::Deref;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::config::{Config, ConfigData};

pub async fn run(
    ctx: &mut Context,
    command: &ApplicationCommandInteraction,
    dbcon: &sqlx::SqlitePool,
) -> String {
    let calling_guild = command.guild_id.expect("failed to get guild_id");
    let to_be_oped = command
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
        if to_be_oped.as_u64() == calling_member.as_u64() {
            return "You are already a bot operator.".into();
        } else {
            println!(
                "to be oped: {:?}, by: {:?}, in: {:?}",
                to_be_oped, calling_member, calling_guild
            );
        }

        if let CommandDataOptionValue::User(_user, _member) = option {
            crate::permissions::add_operator(to_be_oped.as_u64(), calling_guild.as_u64(), dbcon)
                .await;
            config.save();
            return format!(
                "<@{}> has been added to the bot operators list by <@{}>.",
                to_be_oped, calling_member
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
        .name("op")
        .description("Allow a user to use privileged bot features")
        .create_option(|option| {
            option
                .name("user")
                .description("The user to add")
                .kind(CommandOptionType::User)
                .required(true)
        })
}
