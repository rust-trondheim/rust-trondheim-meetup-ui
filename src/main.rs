use slint::SharedString;
use octocrab::*;
use tokio::runtime::Runtime;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let tokio_runtime = Runtime::new().unwrap();

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
            let ui = ui_handle.unwrap();
            tokio_runtime.block_on(async move {
                let title = get_first_issue_title().await;
                ui.set_first_issue_title(SharedString::from(title));
            });
        }
    });

    ui.run()
}

async fn get_first_issue_title() -> String {
    let client = octocrab::instance();
    let issue = client.issues("Rust-Trondheim", ".github").get(1).await.unwrap();
    issue.title
}