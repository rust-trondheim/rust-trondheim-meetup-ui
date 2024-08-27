use octocrab::*;
use slint::{ModelRc, SharedString, VecModel};
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
                let issues = tokio_runtime.spawn(get_first_issue_title()).await.unwrap();
                ui.set_issues(ModelRc::new(VecModel::from(issues)));
            })
            .unwrap();
        }
    });

    ui.run()
}

async fn get_first_issue_title() -> Vec<Issue> {
    let client = octocrab::instance();
    let issues = client
        .issues("Rust-Trondheim", ".github")
        .list()
        .per_page(10)
        .send()
        .await
        .unwrap();
    issues.into_iter().map(std::convert::Into::into).collect()
}

impl Into<Issue> for octocrab::models::issues::Issue {
    fn into(self) -> Issue {
        let octocrab::models::issues::Issue { title, .. } = self;
        Issue {
            title: SharedString::from(title),
        }
    }
}
