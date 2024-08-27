use slint::{ModelRc, SharedString, VecModel};
use octocrab::*;
use tokio::runtime::Runtime;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let tokio_runtime: &'static Runtime = Box::leak(Box::new(Runtime::new().unwrap()));

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
                slint::spawn_local(async move {
                        let titles = tokio_runtime.spawn(get_first_issue_title()).await.unwrap();
                        ui.set_issue_titles(ModelRc::new(VecModel::from(titles)));
                    }).unwrap();
            }
        });

    ui.run()
}

async fn get_first_issue_title() -> Vec<SharedString> {
    let client = octocrab::instance();
    let issues = client.issues("Rust-Trondheim", ".github").list().per_page(10).send().await.unwrap();
    issues.into_iter().map(|i| i.title.into()).collect()
}