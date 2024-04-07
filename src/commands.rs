use chrono::{Duration, Utc};
use std::default;
use poise::{Command, CreateReply};
use poise::serenity_prelude as serenity;
use serenity::all::CommandInteraction;
use crate::{Data, Error};
use crate::create_event::{create_event_components, create_event_modal};
use crate::event_management::events;

type ContextLT<'a> = poise::Context<'a, Data, Error>;

#[poise::command(prefix_command)]
async fn register(ctx: ContextLT<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}


#[derive(Debug, poise::Modal)]
#[allow(dead_code)] // fields only used for Debug print
struct MyModal {
    first_input: String,
    second_input: Option<String>,
}

#[poise::command(slash_command)]
pub async fn inline_choice_multi(
    ctx: ContextLT<'_>,
    #[description = "Event name"] event_name: String,
    #[description = "Which division?"]
    #[choices("Loose Mixed", "Mixed", "Open", "Women's")]
    division: &'static str,
    #[description = "What event type?"]
    #[choices("Beach", "Indoors", "Outdoors")]
    surface: &'static str,
) -> Result<(), Error> {
    let reply = CreateReply {
        content: Some("Event Created".to_string()),
        ephemeral: Some(true),
        ..Default::default()
    };
    ctx.send(reply).await?;
    ctx.channel_id().say(&ctx.http(), format!("Creating event {event_name} which is a {division} {surface} tournament ", event_name=event_name, division=division, surface=surface))
        .await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn create_event(
    ctx: ContextLT<'_>,
    #[description = "Event name"] event_name: String,
    #[description = "Which division?"]
    #[choices("Loose Mixed", "Mixed", "Open", "Women's")]
    division: &'static str,
    #[description = "What event type?"]
    #[choices("Beach", "Indoors", "Outdoors")]
    surface: &'static str,
) -> Result<(), Error> {
    let event = events::FlyghtEventStrings::new();
    let reply = CreateReply {
        content: Some("Event Created".to_string()),
        ephemeral: Some(true),
        components: Some(create_event_components()),
        ..Default::default()
    };
    ctx.send(reply).await?;
    ctx.channel_id().say(&ctx.http(), format!("Creating event {event_name} which is a {division} {surface} tournament ", event_name=event_name, division=division, surface=surface))
        .await?;
    Ok(())
}


// #[poise::command(prefix_command, slash_command)]
// async fn sss_create_event(
//     ctx: ContextLT<'_>
// ) -> Result<(), Error> {
//     let event = events::FlyghtEventStrings::new();
//     // Without timeout, loop could run forever if user never clicks confirm button
//     let timeout = Utc::now() + Duration::minutes(10);
//     'main_loop: while Utc::now() < timeout {
//         // Create main user message with interaction buttons and current state of event creation
//         let reply = {
//             let components = vec![
//                 serenity::CreateActionRow::Buttons(vec![
//                     serenity::CreateButton::new("open_modal_1")
//                         .label("Open modal 1")
//                         .style(poise::serenity_prelude::ButtonStyle::Primary),
//                     serenity::CreateButton::new("open_modal_2")
//                         .label("Open modal 2")
//                         .style(poise::serenity_prelude::ButtonStyle::Primary),
//                     serenity::CreateButton::new("finish")
//                         .label("Finish")
//                         .style(poise::serenity_prelude::ButtonStyle::Success),
//                     serenity::CreateButton::new("cancel")
//                         .label("Finish")
//                         .style(poise::serenity_prelude::ButtonStyle::Success),
//                 ]),
//             ];
//             CreateReply::default()
//                 .content(event.format_discord())
//                 .components(components)
//                 .ephemeral(true)
//         };
//
//         ctx.send(reply).await?;
//         let modal = create_event_modal("My_event");
//
//         // There is no way to handle closing of the modal so just have to hope users don't :(
//         while let Some(mci) = serenity::ComponentInteractionCollector::new(ctx.serenity_context())
//             .timeout(std::time::Duration::from_secs(600)).await {
//             if mci.data.custom_id == "finish" {
//                 let reply = CreateReply::default()
//                     .content("Event Created")
//                     .ephemeral(true);
//                 ctx.send(reply).await?;
//                 break 'main_loop
//             } else if mci.data.custom_id == "cancel" {
//                 let reply = CreateReply::default()
//                     .content("Event creation cancelled")
//                     .ephemeral(true);
//                 ctx.send(reply).await?;
//                 break 'main_loop
//             } else if (mci.data.custom_id == "open_modal_1") | (mci.data.custom_id == "open_modal_2") {
//                 // todo - create actual modals and feed data back
//                 let data =
//                     poise::execute_modal_on_component_interaction::<modal>(ctx, mci, None, None).await?;
//                 println!("Got data: {:?}", data);
//             }
//         }
//     };
//
//     Ok(())
// }

#[poise::command(prefix_command, slash_command)]
async fn ss_create_event(
    ctx: ContextLT<'_>
) -> Result<(), Error> {
    let reply = {
        let components = vec![serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new("open_modal")
                .label("Open modal")
                .style(poise::serenity_prelude::ButtonStyle::Success),
        ])];

        CreateReply::default()
            .content("Click the button below to open the modal")
            .components(components)
    };

    ctx.send(reply).await?;

    while let Some(mci) = serenity::ComponentInteractionCollector::new(ctx.serenity_context())
        .timeout(std::time::Duration::from_secs(120))
        .filter(move |mci| mci.data.custom_id == "open_modal")
        .await
    {
        let data =
            poise::execute_modal_on_component_interaction::<MyModal>(ctx, mci, None, None).await?;
        println!("Got data: {:?}", data);
    }


    // let modal = create_event_modal(title);
    // let com_interaction = CommandInteraction{};
    // com_interaction.create_response(
    //     ctx,
    //     modal
    // ).await?;
    Ok(())
}

/// Responds with "world!"
#[poise::command(slash_command)]
async fn hello(ctx: ContextLT<'_>) -> Result<(), Error> {
    ctx.say("world!").await?;
    Ok(())
}

pub fn generate_command_list() -> Vec<Command<Data, Error>> {
    vec![register(), hello(), create_event(), inline_choice_multi()]
}