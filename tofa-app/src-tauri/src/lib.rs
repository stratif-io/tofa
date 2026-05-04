mod commands;
mod state;

use state::AppState;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::Mutex;
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Listener, Manager, PhysicalPosition, PhysicalSize, WebviewUrl, WebviewWindow,
    WebviewWindowBuilder,
};

/// If true, the popover does NOT auto-hide on focus loss. JS toggles this
/// around operations that legitimately steal focus (file pickers, camera
/// scan, screen capture) so the user doesn't see the popover vanish
/// mid-action.
static POPOVER_PINNED: AtomicBool = AtomicBool::new(false);

/// Latest tray-icon screen rectangle, captured on every tray event. Used
/// to position the popover under the tray on the correct monitor, even
/// when the menu bar is on a secondary display. Stored as four atomics
/// (x, y, w, h) plus a "set" flag to avoid a Mutex on the focus-loss path.
static TRAY_RECT_X: AtomicI32 = AtomicI32::new(0);
static TRAY_RECT_Y: AtomicI32 = AtomicI32::new(0);
static TRAY_RECT_W: AtomicI32 = AtomicI32::new(0);
static TRAY_RECT_H: AtomicI32 = AtomicI32::new(0);
static TRAY_RECT_SET: AtomicBool = AtomicBool::new(false);

/// Toggle whether the popover stays open during focus-stealing operations.
/// Called from JS via `invoke('set_popover_pinned', { pinned: true|false })`.
#[tauri::command]
fn set_popover_pinned(pinned: bool) {
    POPOVER_PINNED.store(pinned, Ordering::Relaxed);
}

fn store_tray_rect(x: f64, y: f64, w: f64, h: f64) {
    TRAY_RECT_X.store(x as i32, Ordering::Relaxed);
    TRAY_RECT_Y.store(y as i32, Ordering::Relaxed);
    TRAY_RECT_W.store(w as i32, Ordering::Relaxed);
    TRAY_RECT_H.store(h as i32, Ordering::Relaxed);
    TRAY_RECT_SET.store(true, Ordering::Relaxed);
}

/// Position the popover horizontally centred under the tray icon, on the
/// monitor that contains the tray. Falls back to centring on the primary
/// monitor if no tray rect has been captured yet.
fn position_popover_under_tray(window: &WebviewWindow) {
    let win_size = window.outer_size().unwrap_or(PhysicalSize::new(320, 480));

    if !TRAY_RECT_SET.load(Ordering::Relaxed) {
        if let Ok(Some(m)) = window.primary_monitor() {
            let mp = m.position();
            let ms = m.size();
            let x = mp.x + (ms.width as i32 - win_size.width as i32) / 2;
            let y = mp.y + 30;
            let _ = window.set_position(PhysicalPosition::new(x, y));
        }
        return;
    }

    let tx = TRAY_RECT_X.load(Ordering::Relaxed);
    let ty = TRAY_RECT_Y.load(Ordering::Relaxed);
    let tw = TRAY_RECT_W.load(Ordering::Relaxed);
    let tray_centre_x = tx + tw / 2;

    // Find the monitor containing the tray icon (multi-display support).
    let monitors = window.available_monitors().unwrap_or_default();
    let monitor = monitors.iter().find(|m| {
        let p = m.position();
        let s = m.size();
        tx >= p.x && tx < p.x + s.width as i32 && ty >= p.y && ty < p.y + s.height as i32
    });

    if let Some(m) = monitor {
        let mp = m.position();
        let ms = m.size();
        // Below the menu bar — 30 px is a safe value across recent macOS.
        let y = mp.y + 30;
        let mut x = tray_centre_x - win_size.width as i32 / 2;
        let max_x = mp.x + ms.width as i32 - win_size.width as i32;
        x = x.clamp(mp.x, max_x);
        let _ = window.set_position(PhysicalPosition::new(x, y));
    }
}

/// Show the popover under the tray on the right monitor, then focus it.
fn show_popover_under_tray(win: &WebviewWindow) {
    position_popover_under_tray(win);
    let _ = win.show();
    let _ = win.set_focus();
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(Mutex::new(AppState::new()))
        .invoke_handler(tauri::generate_handler![
            set_popover_pinned,
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

            // Make the window follow the active Space instead of switching to the Space it lives in
            #[cfg(target_os = "macos")]
            {
                use objc2::rc::Retained;
                use objc2_app_kit::{NSWindow, NSWindowCollectionBehavior};
                let _ = _window.with_webview(|wv| unsafe {
                    let ns_window = wv.ns_window() as *mut NSWindow;
                    if let Some(win) = Retained::retain(ns_window) {
                        win.setCollectionBehavior(
                            NSWindowCollectionBehavior::MoveToActiveSpace
                                | NSWindowCollectionBehavior::Transient,
                        );
                    }
                });
            }

            let item_scan_screen =
                MenuItem::with_id(app, "scan-screen", "Scan Screen", false, None::<&str>)?;
            let item_scan_camera =
                MenuItem::with_id(app, "scan-camera", "Scan Camera", false, None::<&str>)?;
            let item_lock = MenuItem::with_id(app, "lock", "Lock", false, None::<&str>)?;
            let item_quit = MenuItem::with_id(app, "quit", "Quit Tofa", true, Some("CmdOrCtrl+Q"))?;

            let menu = Menu::with_items(
                app,
                &[
                    &item_scan_screen,
                    &item_scan_camera,
                    &MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?,
                    &PredefinedMenuItem::separator(app)?,
                    &item_lock,
                    &PredefinedMenuItem::separator(app)?,
                    &item_quit,
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
                        "quit" => {
                            app.exit(0);
                            return;
                        }
                        _ => return,
                    };
                    if let Some(win) = app.get_webview_window("popover") {
                        show_popover_under_tray(&win);
                        let _ = win.emit("tray-action", action);
                    }
                })
                .build(app)?;

            tray.on_tray_icon_event(|tray, event| {
                // Capture the tray rect on every event so positioning stays
                // correct even after the user moves the menu bar between
                // displays in System Settings.
                if let TrayIconEvent::Click { rect, .. } = &event {
                    // tauri::Rect contains Position/Size enums (Physical or
                    // Logical). Normalise to physical pixels using the
                    // popover window's scale factor so positioning math works
                    // consistently regardless of the variant Tauri reports.
                    if let Some(win) = tray.app_handle().get_webview_window("popover") {
                        let scale = win.scale_factor().unwrap_or(1.0);
                        let pp: tauri::PhysicalPosition<f64> = rect.position.to_physical(scale);
                        let ps: tauri::PhysicalSize<f64> = rect.size.to_physical(scale);
                        store_tray_rect(pp.x, pp.y, ps.width, ps.height);
                    }
                }
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
                            show_popover_under_tray(&win);
                        }
                    }
                }
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::Focused(false) = event {
                if window.label() == "popover" && !POPOVER_PINNED.load(Ordering::Relaxed) {
                    let _ = window.hide();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
