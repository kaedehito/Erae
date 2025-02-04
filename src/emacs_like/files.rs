use ratatui::style::Stylize;

use super::types::{self, Message};
use std::fs;
use std::path::Path;

pub fn save_file(real_name: &str, buffer: &[String], message: &mut Message) {
    let path: &Path = real_name.as_ref();
    let content = buffer.join("\n");
    fs::write(path, content).unwrap_or_else(|e| {
        message.text = format!(" Failed to write file: {e} ")
            .light_red()
            .to_string();
    });
    message.text = format!(" Writed to {} ", real_name)
        .light_yellow()
        .to_string();
    message.message_type = types::MessageType::Info;
}
