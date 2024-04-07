use chrono;
use regex::Regex;

#[derive(Debug)]
enum Divisions {
    LooseMixed,
    Mixed,
    Open,
    Womens
}

#[derive(Debug)]
enum Surface {
    Outdoors,
    Indoors,
    Beach
}

#[derive(Default, Debug)]
pub struct FlyghtEvent {
    title: Option<String>,
    start: Option<chrono::NaiveDateTime>,
    end: Option<chrono::NaiveDateTime>,
    division: Option<Divisions>,
    surface: Option<Surface>,
    location: Option<String>,
    description: Option<String>
}

#[derive(Default, Debug)]
pub struct FlyghtEventStrings {
    title: String,
    start: String,
    end: String,
    division: String,
    surface: String,
    location: String,
    description: String
}

impl FlyghtEventStrings {
    pub fn new () -> Self {
        FlyghtEventStrings {..Default::default()}
    }

    /// Validates the input strings to ensure that they can be converted to a datetime format
    pub fn validate_inputs (&self) -> Option<String> {
        let dt_err: &str = " time parameter is incorrect - format should be either 'yyyy/mm/dd' \
            or 'yyyy/mm/dd HH:MM'";

        let date_regex = Regex::new(r"^\d{4}/\d{2}/\d{2}$").unwrap();
        let datetime_regex = Regex::new(r"^\d{4}/\d{2}/\d{2}( \d{2}:\d{2})?$").unwrap();

        if !datetime_regex.is_match(&self.start) {
            return Some(format!("{} {}", "Start", dt_err))
        };
        if self.end.is_empty() {
            return None
        }
        if !datetime_regex.is_match(&self.end) {
            return Some(format!("{} {} {}", "End", dt_err, " or empty"))
        }
        if date_regex.is_match(&self.start) ^ date_regex.is_match(&self.end) {
            return Some("Format mismatch between start and end times".to_string())
        }
        None
    }

    /// Creates a string that can be shown in the event builder in discord
    pub fn format_discord (&self) -> String {
        let validation = match self.validate_inputs() {
            Some(val_message) => {format!("\n\nError: \n{}\n", val_message)},
            None => {"".to_string()}
        };

        format!(
            "Welcome to the event builder. Please use the buttons below to open the configuration options. \
            {validation}\n\
            \n\
            Event preview:\n\
            \n\
            {title}\n\
            Division: {division} - {surface}\n\
            When: {start}{end}\n\
            Where: {location}\n\
            \n\
            Details:\n\
            {description}",
            validation=validation,
            title=&self.title,
            division=&self.division,
            surface=&self.surface,
            start=&self.start,
            end=format!(" - {}", &self.end),
            location=self.location,
            description=self.description
        )
    }
}