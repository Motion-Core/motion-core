use indicatif::{ProgressBar, ProgressStyle};
use owo_colors::{DynColors, OwoColorize};
use std::time::Duration;

pub const BRAND_COLOR: DynColors = DynColors::Rgb(0xFF, 0x69, 0x00);

pub fn brand(text: impl AsRef<str>) -> String {
    format!("{}", text.as_ref().color(BRAND_COLOR))
}

pub fn heading(text: impl AsRef<str>) -> String {
    format!("{}", text.as_ref().bold().color(BRAND_COLOR))
}

pub fn muted(text: impl AsRef<str>) -> String {
    format!("{}", text.as_ref().dimmed())
}

pub fn success(text: impl AsRef<str>) -> String {
    format!("{}", text.as_ref().green().bold())
}

pub fn warning(text: impl AsRef<str>) -> String {
    format!("{}", text.as_ref().yellow())
}

pub fn danger(text: impl AsRef<str>) -> String {
    format!("{}", text.as_ref().red().bold())
}

pub fn create_spinner(message: impl Into<String>) -> ProgressBar {
    let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏", "✔"];
    let tinted: Vec<String> = frames
        .iter()
        .map(|frame| format!("\x1b[38;2;255;105;0m{}\x1b[0m", frame))
        .collect();
    let tinted_refs: Vec<&str> = tinted.iter().map(|s| s.as_str()).collect();

    let style = ProgressStyle::with_template("{spinner} {msg}")
        .unwrap()
        .tick_strings(&tinted_refs);

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(style);
    spinner.enable_steady_tick(Duration::from_millis(80));
    spinner.set_message(message.into());
    spinner
}
