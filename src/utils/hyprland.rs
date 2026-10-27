use crate::models::monitor::Monitor;
use std::process::Command;

pub fn get_active_monitor_name() -> Option<String> {
    let output = Command::new("hyprctl")
        .arg("monitors")
        .arg("-j")
        .output()
        .ok()?;

    let monitors: Vec<Monitor> = serde_json::from_slice(&output.stdout).ok()?;

    monitors
        .into_iter()
        .find(|monitor| monitor.focused)
        .map(|monitor| monitor.name)
}
