use crate::error::{AppError, Result};
use std::process::Command;

pub fn capture_screen(monitor: &str, output_path: &str) -> Result<()> {
    let status = Command::new("grim")
        .args(["-o", monitor, output_path])
        .status()
        .map_err(|e| AppError::Command(format!("Failed to execute grim: {}", e)))?;

    if !status.success() {
        Command::new("grim")
            .arg(output_path)
            .status()
            .map_err(|e| AppError::Command(format!("Grim fallback failed: {}", e)))?;
    }

    Ok(())
}

pub fn run_ocr(image_path: &str) -> Result<String> {
    let output = Command::new("tesseract")
        .args([image_path, "stdout"])
        .output()
        .map_err(|e| AppError::Command(format!("Failed to run tesseract: {}", e)))?;

    Ok(String::from_utf8_lossy(&output.stdout).to_lowercase())
}
