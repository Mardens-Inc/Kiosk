use std::thread;
use tauri::Manager;
use tauri_plugin_updater::UpdaterExt;

mod windows_api;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            use tauri::Manager;
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }))
        .setup(|app| {
            #[cfg(desktop)]
            {
                use tauri_plugin_autostart::MacosLauncher;
                use tauri_plugin_autostart::ManagerExt;

                let handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    update(&handle).await.unwrap();
                    enable_autocomplete(&handle).await.unwrap();
                });

                thread::spawn(|| {
                    println!("Starting Windows key hook...");
                    windows_api::block_windows_key();
                });

                app.handle().plugin(tauri_plugin_autostart::init(
                    MacosLauncher::LaunchAgent,
                    Some(vec!["--flag1", "--flag2"]),
                ))?;

                // Get the autostart manager
                let autostart_manager = app.autolaunch();
                // Enable autostart
                let _ = autostart_manager.enable();
                // Check enable state
                println!(
                    "registered for autostart? {}",
                    autostart_manager.is_enabled().unwrap()
                );
            }
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn update(app: &tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
        let mut downloaded = 0;

        // alternatively we could also call update.download() and update.install() separately
        update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                    println!("downloaded {downloaded} from {content_length:?}");
                },
                || {
                    println!("download finished");
                },
            )
            .await?;

        println!("update installed");
        app.restart();
    }

    Ok(())
}

async fn enable_autocomplete(app: &tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    if let Some(window) = app.get_webview_window("main") {
        let mut last_url = window.url()?;
        loop {
            let current_url = window.url()?;
            if current_url != last_url {
                last_url = current_url;
                println!("Url: {}", window.url()?);
                window.eval(AUTOCOMPLETE_SCRIPT)?;
            }
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    }

    Ok(())
}

const AUTOCOMPLETE_SCRIPT: &str = r#"
console.log('ticking page')
const update = setInterval(()=>{
    document.querySelectorAll("\#fmiwebd-742712558-overlays \#login_dialog_body input:not([autocomplete='custom'])")
    .forEach(input=>
            {
                console.log(`Adding autocomplete listener to`, input)
                const result = getState(input)
                if(result) input.value = result;
                input.setAttribute("autocomplete", "custom")

                input.addEventListener("blur", ()=>saveState(input))
                clearInterval(update)
            }
        )
}, 100)

function saveState(input)
{
    const selector = `${input.getAttribute("type")}-${input.getAttribute("placeholder")}`
    localStorage.setItem(selector, input.value);
    console.log("Saving State", selector, input.value)
}

function getState(input)
{
    const selector = `${input.getAttribute("type")}-${input.getAttribute("placeholder")}`
    console.log("Loading State", selector)
    return localStorage.getItem(selector);
}
"#;
