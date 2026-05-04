mod commands;
mod state;

use state::AppState;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::Mutex;
use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Listener, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder,
};

/// If true, the popover does NOT auto-hide on focus loss. JS toggles this
/// around operations that legitimately steal focus (file pickers, camera
/// scan, screen capture) so the user doesn't see the popover vanish
/// mid-action.
static POPOVER_PINNED: AtomicBool = AtomicBool::new(false);

/// Latest tray click position in PHYSICAL screen pixels. Captured on
/// every tray event so the popover can open on whichever display the
/// user clicked, with no scale-factor conversion needed: Tauri reports
/// `TrayIconEvent::Click { position: PhysicalPosition<f64>, .. }` already
/// in absolute physical screen coordinates.
static TRAY_CLICK_X: AtomicI32 = AtomicI32::new(0);
static TRAY_CLICK_Y: AtomicI32 = AtomicI32::new(0);
static TRAY_CLICK_SET: AtomicBool = AtomicBool::new(false);

/// Toggle whether the popover stays open during focus-stealing operations.
/// Called from JS via `invoke('set_popover_pinned', { pinned: true|false })`.
#[tauri::command]
fn set_popover_pinned(pinned: bool) {
    POPOVER_PINNED.store(pinned, Ordering::Relaxed);
}

fn store_tray_click(x: f64, y: f64) {
    TRAY_CLICK_X.store(x as i32, Ordering::Relaxed);
    TRAY_CLICK_Y.store(y as i32, Ordering::Relaxed);
    TRAY_CLICK_SET.store(true, Ordering::Relaxed);
}

/// Move the popover's NSWindow top-left to the given Cocoa-screen point.
///
/// We bypass Tauri's `set_position` because in multi-display setups it
/// silently no-ops (returns Ok but the window stays put). Calling
/// `setFrameTopLeftPoint:` on the underlying NSWindow works reliably.
///
/// Cocoa screen coordinates: origin at the bottom-left of the *primary*
/// screen, x right, y UP, units are POINTS (not physical pixels).
#[cfg(target_os = "macos")]
fn set_window_top_left_cocoa(win: &WebviewWindow, x_pt: f64, y_pt: f64) {
    use objc2::rc::Retained;
    use objc2_app_kit::NSWindow;
    use objc2_foundation::NSPoint;

    let _ = win.with_webview(move |wv| unsafe {
        let ns_window = wv.ns_window() as *mut NSWindow;
        if let Some(window) = Retained::retain(ns_window) {
            window.setFrameTopLeftPoint(NSPoint::new(x_pt, y_pt));
        }
    });
}

/// Position the popover under the tray icon on whichever monitor contains
/// the click. Uses NSWindow directly to avoid Tauri's broken multi-display
/// `set_position`.
fn position_popover_under_tray(window: &WebviewWindow) {
    if !TRAY_CLICK_SET.load(Ordering::Relaxed) {
        if let Ok(Some(m)) = window.primary_monitor() {
            let mp = m.position();
            let ms = m.size();
            let scale = m.scale_factor();
            let primary_h_pt = ms.height as f64 / scale;
            // Centre on primary in Cocoa points.
            let cx_pt = (ms.width as f64 / scale) / 2.0 - 320.0 / 2.0;
            let cy_pt = primary_h_pt - 28.0; // 28 pt below the menu bar
            #[cfg(target_os = "macos")]
            set_window_top_left_cocoa(window, mp.x as f64 + cx_pt, cy_pt);
            let _ = window;
        }
        return;
    }

    // Tauri reports monitor positions and click position in POINTS but
    // monitor sizes in PHYSICAL PIXELS. Normalise size to points using
    // each monitor's scale factor before doing bounds arithmetic.
    let click_x_pt = TRAY_CLICK_X.load(Ordering::Relaxed) as f64;
    let click_y_pt = TRAY_CLICK_Y.load(Ordering::Relaxed) as f64;

    let monitors = window.available_monitors().unwrap_or_default();
    let monitor = monitors.iter().find(|m| {
        let p = m.position();
        let s = m.size();
        let scale = m.scale_factor();
        let w_pt = s.width as f64 / scale;
        let h_pt = s.height as f64 / scale;
        click_x_pt >= p.x as f64
            && click_x_pt < p.x as f64 + w_pt
            && click_y_pt >= p.y as f64
            && click_y_pt < p.y as f64 + h_pt
    });

    let primary_h_pt = window
        .primary_monitor()
        .ok()
        .flatten()
        .map(|m| m.size().height as f64 / m.scale_factor())
        .unwrap_or(0.0);

    if let Some(m) = monitor {
        let mp = m.position();
        let ms = m.size();
        let scale = m.scale_factor();
        let mw_pt = ms.width as f64 / scale;
        // 28 points below the top of the screen — macOS menu bar height.
        let target_top_y_pt = mp.y as f64 + 28.0;
        // Centre horizontally on the click, clamped to the monitor's bounds.
        let popover_w_pt = 320.0;
        let mut target_x_pt = click_x_pt - popover_w_pt / 2.0;
        let max_x_pt = mp.x as f64 + mw_pt - popover_w_pt;
        if target_x_pt < mp.x as f64 {
            target_x_pt = mp.x as f64;
        }
        if target_x_pt > max_x_pt {
            target_x_pt = max_x_pt;
        }

        // Convert Tauri (top-left, y-down, points) → Cocoa (bottom-left of
        // primary, y-up, points). Cocoa's y for the window's TOP edge is
        // primary_height_pt − tauri_top_y_pt.
        let cocoa_top_y_pt = primary_h_pt - target_top_y_pt;

        eprintln!(
            "[pos] click=({:.0},{:.0}) target_topleft_tauri=({:.0},{:.0}) → cocoa=({:.0},{:.0}) primary_h={:.0}",
            click_x_pt, click_y_pt, target_x_pt, target_top_y_pt, target_x_pt, cocoa_top_y_pt, primary_h_pt
        );

        #[cfg(target_os = "macos")]
        set_window_top_left_cocoa(window, target_x_pt, cocoa_top_y_pt);
        // Keep the function buildable on non-macOS platforms even though
        // we only ship the menu-bar app on macOS.
        #[cfg(not(target_os = "macos"))]
        {
            let _ = window.set_position(PhysicalPosition::new(
                (target_x_pt * scale) as i32,
                (target_top_y_pt * scale) as i32,
            ));
        }

        if let Ok(p) = window.outer_position() {
            eprintln!("[pos] outer_position after set = ({}, {})", p.x, p.y);
        }
    }
}

/// Show the popover under the tray on the right monitor, then focus it.
fn show_popover_under_tray(win: &WebviewWindow) {
    position_popover_under_tray(win);
    let _ = win.show();
    let _ = win.set_focus();
    // Re-position once visible — some macOS state transitions can shift
    // the frame on show.
    position_popover_under_tray(win);
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
                // DEBUG: dump everything Tauri reports about a tray click so
                // we can pinpoint multi-monitor coordinate-system issues.
                // Remove before merging to main.
                if let TrayIconEvent::Click { position, rect, .. } = &event {
                    eprintln!(
                        "[tray] click position = ({:.1}, {:.1})",
                        position.x, position.y
                    );
                    eprintln!("[tray] rect = {:?}", rect);
                    if let Some(win) = tray.app_handle().get_webview_window("popover") {
                        let scale = win.scale_factor().unwrap_or(1.0);
                        eprintln!("[tray] popover.scale_factor = {}", scale);
                        if let Ok(monitors) = win.available_monitors() {
                            eprintln!("[tray] available_monitors:");
                            for (i, m) in monitors.iter().enumerate() {
                                let p = m.position();
                                let s = m.size();
                                eprintln!(
                                    "  [{}] name={:?} pos=({},{}) size=({}x{}) scale={}",
                                    i,
                                    m.name(),
                                    p.x,
                                    p.y,
                                    s.width,
                                    s.height,
                                    m.scale_factor()
                                );
                            }
                        }
                        let cx = position.x as i32;
                        let cy = position.y as i32;
                        let monitors = win.available_monitors().unwrap_or_default();
                        let matched = monitors.iter().enumerate().find(|(_, m)| {
                            let p = m.position();
                            let s = m.size();
                            cx >= p.x
                                && cx < p.x + s.width as i32
                                && cy >= p.y
                                && cy < p.y + s.height as i32
                        });
                        match matched {
                            Some((i, _)) => eprintln!("[tray] matched monitor index = {}", i),
                            None => eprintln!(
                                "[tray] NO MONITOR MATCHED click ({},{}) — fallback to primary",
                                cx, cy
                            ),
                        }
                    }
                }

                // Capture the click position on every tray event. `position`
                // is already PhysicalPosition<f64> in absolute screen pixels,
                // so we don't need to know which monitor the click was on
                // (or its scale factor) to record it.
                if let TrayIconEvent::Click { position, .. } = &event {
                    store_tray_click(position.x, position.y);
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
                            // DEBUG: what position did we end up at?
                            if let Ok(p) = win.outer_position() {
                                eprintln!("[tray] popover positioned at ({}, {})", p.x, p.y);
                            }
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
