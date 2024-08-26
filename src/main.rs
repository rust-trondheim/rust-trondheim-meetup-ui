use slint::SharedString;
use serde::{Serialize, Deserialize};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.on_request_first_issue_title({
        let ui_handle = ui.as_weak();
        move || {
            let title = get_first_issue_title();
            let ui = ui_handle.unwrap();
            ui.set_first_issue_title(SharedString::from(title));
        }
    });

    ui.run()
}

fn get_first_issue_title() -> String {
    let client = reqwest::blocking::Client::new();
    
    let body = client
    .get("https://api.github.com/repos/Rust-Trondheim/.github/issues")
    .header("User-Agent", "Rust-Trondheim-Meetup-UI")
    .send()
    .unwrap()
    .text()
    .unwrap();

    let issue: Vec<Issue> = serde_json::from_str(&body).unwrap();

    issue.first().unwrap().title.clone()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    pub id: u64,
    pub node_id: String,
    pub url: String,
    pub number: u64,
    pub title: String,
}