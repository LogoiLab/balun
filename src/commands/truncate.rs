use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::{
    ApplicationCommandInteraction, CommandDataOptionValue,
};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::config::ConfigData;

#[derive(Debug)]
enum Timescale {
    MINUTES,
    HOURS,
    DAYS,
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
