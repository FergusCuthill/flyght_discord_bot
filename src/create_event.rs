use serenity::all::{CreateSelectMenu, InputTextStyle, CreateInputText, CreateSelectMenuKind, CreateSelectMenuOption, CreateModal, CreateActionRow, CreateInteractionResponse};

fn create_event_type_row() -> CreateSelectMenu {
    let outdoors = CreateSelectMenuOption::new("Outdoors", "outdoors");
    let indoors = CreateSelectMenuOption::new("Indoors", "indoors");
    let beach = CreateSelectMenuOption::new("Beach", "beach");
    let select_menu = CreateSelectMenuKind::String{options: vec![outdoors, indoors, beach]};
    CreateSelectMenu::new("event_type", select_menu)
}

fn create_division_row() -> CreateSelectMenu {
    let open = CreateSelectMenuOption::new("Open", "open");
    let mixed = CreateSelectMenuOption::new("Mixed", "mixed");
    let loose_mixed = CreateSelectMenuOption::new("Loose Mixed", "loose_mixed");
    let women = CreateSelectMenuOption::new("Women's", "women");
    let select_menu = CreateSelectMenuKind::String{options: vec![open, mixed, loose_mixed, women]};
    CreateSelectMenu::new("event_division", select_menu)
}

fn create_datetime_row() -> CreateInputText {
    CreateInputText::new(InputTextStyle::Short, "Period", "event_period")
}

fn create_start_time_row() -> CreateInputText {
    CreateInputText::new(InputTextStyle::Short, "Start", "event_start")
}

fn create_end_time_row() -> CreateInputText {
    CreateInputText::new(InputTextStyle::Short, "End", "event_end")
}

fn create_location_row() -> CreateInputText {
    CreateInputText::new(InputTextStyle::Short, "Location", "event_location")
}

fn create_description_row() -> CreateInputText {
    CreateInputText::new(InputTextStyle::Paragraph, "Event Description", "event_description")
}

pub fn create_event_modal(event_title: &str) -> CreateInteractionResponse {
    let mut modal = CreateModal::new(event_title, event_title);
    let components = vec![
        CreateActionRow::SelectMenu(create_event_type_row()),
        CreateActionRow::SelectMenu(create_division_row()),
        CreateActionRow::InputText(create_datetime_row()),
        CreateActionRow::InputText(create_location_row()),
        CreateActionRow::InputText(create_description_row())
    ];
    CreateInteractionResponse::Modal(modal.components(components))
}

pub fn create_event_components() -> Vec<CreateActionRow> {
    vec![
        CreateActionRow::InputText(create_start_time_row()),
        CreateActionRow::InputText(create_end_time_row()),
        CreateActionRow::InputText(create_location_row()),
        CreateActionRow::InputText(create_description_row())
    ]
}