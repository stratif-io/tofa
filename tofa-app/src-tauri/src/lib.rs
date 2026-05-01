mod commands;
mod state;

use std::sync::Mutex;
use state::AppState;
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, WebviewUrl, WebviewWindowBuilder,
};
use tauri_plugin_positioner::WindowExt;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_positioner::init())
        .manage(Mutex::new(AppState::new()))
        .invoke_handler(tauri::generate_handler![
            commands::unlock,
            commands::get_entries,
            commands::copy_code,
            commands::get_settings,
            commands::save_settings,
            commands::scan_screen,
            commands::scan_image_data,
            commands::add_from_uri,
            commands::delete_entry,
        ])
        .setup(|app| {
            let _window = WebviewWindowBuilder::new(
                app,
                "popover",
                WebviewUrl::App("index.html".into()),
            )
            .decorations(false)
            .always_on_top(true)
            .resizable(false)
            .visible(false)
            .inner_size(320.0, 480.0)
            .build()?;

            let menu = Menu::with_items(app, &[
                &MenuItem::with_id(app, "scan-screen", "Scan Screen", true, None::<&str>)?,
                &MenuItem::with_id(app, "scan-camera", "Scan Camera", true, None::<&str>)?,
                &MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?,
                &PredefinedMenuItem::separator(app)?,
                &PredefinedMenuItem::quit(app, None)?,
            ])?;

            let tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .menu_on_left_click(false)
                .on_menu_event(|app, event| {
                    let action = match event.id.as_ref() {
                        "scan-screen" => "scan-screen",
                        "scan-camera" => "scan-camera",
                        "settings" => "settings",
                        _ => return,
                    };
                    if let Some(win) = app.get_webview_window("popover") {
                        let _ = win.move_window(tauri_plugin_positioner::Position::TrayBottomCenter);
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
                            let _ = win.move_window(
                                tauri_plugin_positioner::Position::TrayBottomCenter,
                            );
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
