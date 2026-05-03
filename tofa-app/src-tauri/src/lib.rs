mod commands;
mod state;

use state::AppState;
use std::sync::Mutex;
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Listener, Manager, WebviewUrl, WebviewWindowBuilder,
};
use tauri_plugin_positioner::WindowExt;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_positioner::init())
        .manage(Mutex::new(AppState::new()))
        .invoke_handler(tauri::generate_handler![
            commands::vault_exists,
            commands::create_vault,
            commands::unlock,
            commands::get_entries,
            commands::copy_code,
            commands::get_settings,
            commands::save_settings,
            commands::get_secret,
            commands::lock,
            commands::scan_screen,
            commands::scan_camera,
            commands::scan_image_bytes,
            commands::add_from_uri,
            commands::delete_entry,
            commands::pick_vault_folder,
            commands::import_file,
        ])
        .setup(|app| {
            #[cfg(target_os = "macos")]
            let _ = app
                .handle()
                .set_activation_policy(tauri::ActivationPolicy::Accessory);

            let _window =
                WebviewWindowBuilder::new(app, "popover", WebviewUrl::App("index.html".into()))
                    .decorations(false)
                    .always_on_top(true)
                    .resizable(false)
                    .visible(false)
                    .inner_size(320.0, 480.0)
                    .build()?;

            let item_scan_screen =
                MenuItem::with_id(app, "scan-screen", "Scan Screen", false, None::<&str>)?;
            let item_scan_camera =
                MenuItem::with_id(app, "scan-camera", "Scan Camera", false, None::<&str>)?;
            let item_lock = MenuItem::with_id(app, "lock", "Lock", false, None::<&str>)?;

            let menu = Menu::with_items(
                app,
                &[
                    &item_scan_screen,
                    &item_scan_camera,
                    &MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?,
                    &PredefinedMenuItem::separator(app)?,
                    &item_lock,
                    &PredefinedMenuItem::separator(app)?,
                    &PredefinedMenuItem::quit(app, None)?,
                ],
            )?;

            // Enable scan/lock items when unlocked, disable when locked
            let ss = item_scan_screen.clone();
            let sc = item_scan_camera.clone();
            let lk = item_lock.clone();
            app.listen("session-unlocked", move |_| {
                let _ = ss.set_enabled(true);
                let _ = sc.set_enabled(true);
                let _ = lk.set_enabled(true);
            });

            let ss2 = item_scan_screen.clone();
            let sc2 = item_scan_camera.clone();
            let lk2 = item_lock.clone();
            app.listen("session-locked", move |_| {
                let _ = ss2.set_enabled(false);
                let _ = sc2.set_enabled(false);
                let _ = lk2.set_enabled(false);
            });

            let tray_icon =
                tauri::image::Image::from_bytes(include_bytes!("../icons/tray_icon.png"))
                    .unwrap_or_else(|_| app.default_window_icon().unwrap().clone());

            let tray = TrayIconBuilder::new()
                .icon(tray_icon)
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| {
                    let action = match event.id.as_ref() {
                        "scan-screen" => "scan-screen",
                        "scan-camera" => "scan-camera",
                        "settings" => "settings",
                        "lock" => "lock",
                        _ => return,
                    };
                    if let Some(win) = app.get_webview_window("popover") {
                        let _ =
                            win.move_window(tauri_plugin_positioner::Position::TrayBottomCenter);
                        let _ = win.show();
                        let _ = win.set_focus();
                        let _ = win.emit("tray-action", action);
                    }
                })
                .build(app)?;

            tray.on_tray_icon_event(|tray, event| {
                tauri_plugin_positioner::on_tray_event(tray.app_handle(), &event);
                if let TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } = event
                {
                    let app = tray.app_handle();
                    if let Some(win) = app.get_webview_window("popover") {
                        let visible = win.is_visible().unwrap_or(false);
                        if visible {
                            let _ = win.hide();
                        } else {
                            let _ = win
                                .move_window(tauri_plugin_positioner::Position::TrayBottomCenter);
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                }
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::Focused(false) = event {
                if window.label() == "popover" {
                    let _ = window.hide();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
