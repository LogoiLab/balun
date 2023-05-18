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
    let to_be_banned = command
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
        if to_be_banned.as_u64() == calling_member.as_u64() {
            return "Operators cannot be banned. Try deoping them first.".into();
        } else {
            println!(
                "to be banned: {:?}, by: {:?}, in: {:?}",
                to_be_banned, calling_member, calling_guild
            );
        }

        if let CommandDataOptionValue::User(_user, _member) = option {
            crate::permissions::ban(to_be_banned.as_u64(), calling_guild.as_u64(), dbcon).await;
            config.save();
            return format!(
                "<@{}> has been banned from using the bot by <@{}>.",
                to_be_banned, calling_member
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
        .name("balun_ban")
        .description("Bans a user from interacting with the bot")
        .create_option(|option| {
            option
                .name("user")
                .description("The user to ban")
                .kind(CommandOptionType::User)
                .required(true)
        })
}
