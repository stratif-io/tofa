use tofa::tui::state::{AppState, Screen};

#[test]
fn initial_screen_is_unlock() {
    let state = AppState::new();
    assert_eq!(state.screen, Screen::Unlock);
}

#[test]
fn clear_add_form_resets_fields() {
    let mut state = AppState::new();
    state.add_name = "GitHub".to_string();
    state.add_secret_input = "SECRETKEY".to_string();
    state.add_focused_field = 1;

    state.clear_add_form();

    assert!(state.add_name.is_empty());
    assert!(state.add_secret_input.is_empty());
    assert_eq!(state.add_focused_field, 0);
}

#[test]
fn selected_index_default_is_zero() {
    let state = AppState::new();
    assert_eq!(state.selected_index, 0);
}
