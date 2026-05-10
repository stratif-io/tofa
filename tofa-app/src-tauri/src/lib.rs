mod about_window;
mod commands;
mod state;

use state::AppState;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use tauri::{
    menu::{IsMenuItem, Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Listener, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder, Wry,
};

/// If true, the popover does NOT auto-hide on focus loss. JS toggles this
/// around operations that legitimately steal focus (file pickers, camera
/// scan, screen capture) so the user doesn't see the popover vanish
/// mid-action.
static POPOVER_PINNED: AtomicBool = AtomicBool::new(false);

/// Toggle whether the popover stays open during focus-stealing operations.
/// Called from JS via `invoke('set_popover_pinned', { pinned: true|false })`.
#[tauri::command]
fn set_popover_pinned(pinned: bool) {
    POPOVER_PINNED.store(pinned, Ordering::Relaxed);
}

/// Position the popover flush below the tray icon on whichever screen was
/// clicked. Uses NSScreen and NSEvent directly to avoid Tauri's broken
/// multi-display coordinate conversions.
///
/// Strategy:
/// - `NSEvent::mouseLocation()` gives the cursor in Cocoa points (y-up from
///   bottom-left of the primary screen), which is the same coordinate system
///   as NSScreen.frame, so no conversion is needed.
/// - `NSScreen.visibleFrame.maxY` is exactly the bottom edge of the menu bar
///   on each screen — that is where the popover top belongs.
/// - We bypass Tauri's `set_position` (which silently no-ops on secondary
///   screens) and call `setFrameTopLeftPoint:` on the underlying NSWindow.
#[cfg(target_os = "macos")]
fn position_popover_under_tray(window: &WebviewWindow) {
    let _ = window.with_webview(|wv| unsafe {
        use objc2::rc::Retained;
        use objc2_app_kit::{NSEvent, NSScreen, NSWindow};
        use objc2_foundation::NSPoint;

        let ns_window = wv.ns_window() as *mut NSWindow;
        let win = match Retained::retain(ns_window) {
            Some(w) => w,
            None => return,
        };

        // Cursor in Cocoa screen coords — same system as NSScreen.frame.
        let loc = NSEvent::mouseLocation();

        // MainThreadMarker is safe here: with_webview runs on the main thread.
        use objc2::MainThreadMarker;
        let mtm = MainThreadMarker::new_unchecked();

        const POPOVER_W: f64 = 320.0;

        // Compute (target_x, target_y) from the screen that contains the cursor.
        // visibleFrame excludes the menu bar at the top, so its maxY is the
        // Cocoa y of the menu bar's bottom edge — exactly where the popover top sits.
        let compute_pos = |s: &NSScreen| -> (f64, f64) {
            let visible = s.visibleFrame();
            let y = visible.origin.y + visible.size.height;
            let frame = s.frame();
            let x = (loc.x - POPOVER_W / 2.0)
                .max(frame.origin.x)
                .min(frame.origin.x + frame.size.width - POPOVER_W);
            (x, y)
        };

        // Find the screen whose frame contains the cursor.
        let screens = NSScreen::screens(mtm);
        let mut pos: Option<(f64, f64)> = None;
        for i in 0..screens.count() {
            let s = screens.objectAtIndex(i);
            let f = s.frame();
            if loc.x >= f.origin.x
                && loc.x < f.origin.x + f.size.width
                && loc.y >= f.origin.y
                && loc.y < f.origin.y + f.size.height
            {
                pos = Some(compute_pos(&s));
                break;
            }
        }

        // Fall back to the screen that owns the menu bar.
        let (target_x, target_y) = pos
            .or_else(|| NSScreen::mainScreen(mtm).as_deref().map(compute_pos))
            .unwrap_or((0.0, 0.0));

        win.setFrameTopLeftPoint(NSPoint::new(target_x, target_y));
    });
}

#[cfg(not(target_os = "macos"))]
fn position_popover_under_tray(_window: &WebviewWindow) {}

/// Show the popover under the tray on the right monitor, then focus it.
fn show_popover_under_tray(win: &WebviewWindow) {
    position_popover_under_tray(win);
    let _ = win.show();
    let _ = win.set_focus();
    // Re-position once visible — some macOS state transitions can shift
    // the frame on show.
    position_popover_under_tray(win);
}

fn build_tray_menu(
    app: &tauri::AppHandle,
    item_about: &MenuItem<Wry>,
    item_scan_screen: &MenuItem<Wry>,
    item_scan_camera: &MenuItem<Wry>,
    item_lock: &MenuItem<Wry>,
    item_quit: &MenuItem<Wry>,
    update_item: Option<&MenuItem<Wry>>,
) -> tauri::Result<Menu<Wry>> {
    let sep1 = PredefinedMenuItem::separator(app)?;
    let settings_item = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
    let sep2 = PredefinedMenuItem::separator(app)?;
    let sep3 = PredefinedMenuItem::separator(app)?;

    let mut items: Vec<&dyn IsMenuItem<Wry>> = vec![item_about];
    if let Some(u) = update_item {
        items.push(u);
    }
    items.push(&sep1);
    items.push(item_scan_screen);
    items.push(item_scan_camera);
    items.push(&settings_item);
    items.push(&sep2);
    items.push(item_lock);
    items.push(&sep3);
    items.push(item_quit);

    Menu::with_items(app, &items)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(Mutex::new(AppState::new()))
        .manage(crate::commands::PendingDownload::default())
        .invoke_handler(tauri::generate_handler![
            set_popover_pinned,
            commands::get_versions,
            commands::check_for_updates,
            commands::download_and_install,
            commands::take_pending_download,
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
            commands::pick_and_import_file,
            commands::import_file,
            commands::generate_entry_qr,
            commands::generate_selection_qr,
            commands::generate_otpauth_list,
            commands::save_qr_png,
            commands::save_qr_zip,
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
            let item_quit = MenuItem::with_id(app, "quit", "Quit TOFA", true, Some("CmdOrCtrl+Q"))?;

            let item_about = MenuItem::with_id(app, "about", "About Tofa", true, None::<&str>)?;
            let menu = build_tray_menu(
                app.handle(),
                &item_about,
                &item_scan_screen,
                &item_scan_camera,
                &item_lock,
                &item_quit,
                None,
            )?;

            static ICON_LOCKED: &[u8] = include_bytes!("../icons/tray_icon_locked.png");
            static ICON_OPEN: &[u8] = include_bytes!("../icons/tray_icon_open.png");

            let tray_icon = tauri::image::Image::from_bytes(ICON_LOCKED)
                .unwrap_or_else(|_| app.default_window_icon().unwrap().clone());

            let tray = TrayIconBuilder::new()
                .icon(tray_icon)
                .menu(&menu)
                .tooltip(if cfg!(debug_assertions) {
                    "TOFA DEV"
                } else {
                    "TOFA"
                })
                .show_menu_on_left_click(false)
                .on_menu_event(move |app, event| {
                    let action = match event.id.as_ref() {
                        "update-available" => {
                            use std::sync::atomic::Ordering;
                            let pending = app.state::<crate::commands::PendingDownload>();
                            pending.0.store(true, Ordering::Relaxed);
                            crate::about_window::show_or_focus(app);
                            let _ = app.emit("trigger-download", ());
                            return;
                        }
                        "about" => {
                            crate::about_window::show_or_focus(app);
                            return;
                        }
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

            let tray_id = tray.id().clone();

            // Clones the listener uses to rebuild the menu when an update arrives.
            let item_about_l = item_about.clone();
            let item_scan_screen_l = item_scan_screen.clone();
            let item_scan_camera_l = item_scan_camera.clone();
            let item_lock_l = item_lock.clone();
            let item_quit_l = item_quit.clone();
            let app_for_event = app.handle().clone();
            let tray_id_for_event = tray_id.clone();

            // Enable scan/lock items when unlocked, disable when locked
            let ss = item_scan_screen.clone();
            let sc = item_scan_camera.clone();
            let lk = item_lock.clone();
            let app_unlock = app.handle().clone();
            let tray_id_unlock = tray_id.clone();
            app.listen("session-unlocked", move |_| {
                let _ = ss.set_enabled(true);
                let _ = sc.set_enabled(true);
                let _ = lk.set_enabled(true);
                if let Some(tray) = app_unlock.tray_by_id(&tray_id_unlock) {
                    if let Ok(icon) = tauri::image::Image::from_bytes(ICON_OPEN) {
                        let _ = tray.set_icon(Some(icon));
                    }
                }
            });

            let ss2 = item_scan_screen.clone();
            let sc2 = item_scan_camera.clone();
            let lk2 = item_lock.clone();
            let app_lock = app.handle().clone();
            let tray_id_lock = tray_id.clone();
            app.listen("session-locked", move |_| {
                let _ = ss2.set_enabled(false);
                let _ = sc2.set_enabled(false);
                let _ = lk2.set_enabled(false);
                if let Some(tray) = app_lock.tray_by_id(&tray_id_lock) {
                    if let Ok(icon) = tauri::image::Image::from_bytes(ICON_LOCKED) {
                        let _ = tray.set_icon(Some(icon));
                    }
                }
            });

            tray.on_tray_icon_event(|tray, event| {
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

            app.listen("update-available", move |evt| {
                #[derive(serde::Deserialize)]
                struct UpdatePayload {
                    version: String,
                }

                let payload: UpdatePayload = match serde_json::from_str(evt.payload()) {
                    Ok(p) => p,
                    Err(_) => return,
                };
                let label = format!("Update available — v{}", payload.version);
                let update_item = match MenuItem::with_id(
                    &app_for_event,
                    "update-available",
                    &label,
                    true,
                    None::<&str>,
                ) {
                    Ok(i) => i,
                    Err(_) => return,
                };
                let new_menu = match build_tray_menu(
                    &app_for_event,
                    &item_about_l,
                    &item_scan_screen_l,
                    &item_scan_camera_l,
                    &item_lock_l,
                    &item_quit_l,
                    Some(&update_item),
                ) {
                    Ok(m) => m,
                    Err(_) => return,
                };
                if let Some(tray) = app_for_event.tray_by_id(&tray_id_for_event) {
                    let _ = tray.set_menu(Some(new_menu));
                }
            });

            // Background update check: once on launch, then every 24 hours.
            let bg_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                use std::time::Duration;
                use tauri_plugin_updater::UpdaterExt;

                async fn run_once(app: &tauri::AppHandle) {
                    let updater = match app.updater() {
                        Ok(u) => u,
                        Err(_) => return,
                    };
                    if let Ok(Some(update)) = updater.check().await {
                        let _ = app.emit(
                            "update-available",
                            serde_json::json!({ "version": update.version }),
                        );
                    }
                }

                run_once(&bg_handle).await;
                let mut ticker = tokio::time::interval(Duration::from_secs(24 * 60 * 60));
                ticker.tick().await; // first tick fires immediately; we already ran above
                loop {
                    ticker.tick().await;
                    run_once(&bg_handle).await;
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
