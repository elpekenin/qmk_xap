use active_win_pos_rs::get_active_window;

pub fn active_window() -> Option<String> {
    let window = get_active_window().ok()?;

    Some(window.process_name)
}
