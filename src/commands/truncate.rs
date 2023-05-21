use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::config::ConfigData;

#[derive(Debug)]
enum Timescale {
    MINUTES,
    HOURS,
    DAYS,
}

pub struct Manage {
    thread: tokio::task::JoinHandle<()>,
    seen_messages: HashMap<u64, Vec<(u64, u64)>>,
}

impl Manage {
    pub fn register(dbcon: &sqlx::SqlitePool) -> Self {
        let thread = Manage {
            thread: tokio::spawn(async move {
                // https://serenity-rs.github.io/serenity/current/serenity/builder/struct.GetMessages.html
                // https://serenity-rs.github.io/serenity/current/serenity/model/prelude/struct.ChannelId.html#method.delete_message
            }),
            seen_messages: HashMap::new(),
        };
        return thread;
    }
}

pub struct ManageData;

impl serenity::prelude::TypeMapKey for ManageData {
    type Value = Manage;
}

pub async fn run(
    ctx: &mut Context,
    options: &ApplicationCommandInteraction,
    dbcon: &sqlx::SqlitePool,
) -> String {
    let calling_guild = options.guild_id.expect("Could not get guild id.");
    let CommandDataOptionValue::Integer(amount) = options
        .data
        .options
        .get(0)
        .expect("Expected amount option")
        .resolved
        .as_ref()
        .expect("Expected integer") else {
            return "Expected integer for amount.".into();
        };
    let CommandDataOptionValue::String(timescale) = options
        .data
        .options
        .get(1)
        .expect("Expected timescale option")
        .resolved
        .as_ref()
        .expect("Expected string") else {
            return "Expected string for timescale.".into();
        };
    let calling_member = options
        .member
        .clone()
        .expect("failed to get command member.")
        .user
        .id;

    let mut data = ctx.data.write().await;
    let config = data.get_mut::<ConfigData>().unwrap();

    if !crate::permissions::is_operator(
        calling_member.as_u64(),
        calling_guild.as_u64(),
        dbcon,
        config,
    )
    .await
    {
        return "You must be a bot operator to use this command.".into();
    }
    let parsed_timescale: Timescale = match timescale.to_lowercase().as_str() {
        "m" | "min" | "mins" | "minute" | "minutes" => Timescale::MINUTES,
        "h" | "hour" | "hours" => Timescale::HOURS,
        "d" | "day" | "days" => Timescale::DAYS,
        _ => return "The specified timescale is not a valid timescale".into(),
    };

    return format!(
        "Nothing here yet. {} {}, {:?}",
        amount, timescale, parsed_timescale
    )
    .into();
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("truncate")
        .description("Set a dissapearing message time limit for the current channel")
        .create_option(|option| {
            option
                .name("amount")
                .description("An integer number amount for <timescale>")
                .kind(serenity::model::prelude::command::CommandOptionType::Integer)
                .min_int_value(1)
                .max_int_value(999)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("timescale")
                .description("Time scale: minutes/hours/days")
                .kind(serenity::model::prelude::command::CommandOptionType::String)
                .required(true)
        })
}
