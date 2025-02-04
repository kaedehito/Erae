use super::files::save_file;
use super::scroll::update_scroll;
use super::types::Cursor;
use super::types::Message;
use super::types::MessageType;
use super::types::YOrNState;
use crossterm::event::{self, KeyCode, KeyModifiers};
use crossterm::terminal::size;
use ratatui::style::Stylize;
use std::io;

pub fn handle_key_event(
    content: String,
    key: event::KeyEvent,
    buffer: &mut Vec<String>,
    cursor: &mut Cursor,
    message: &mut Message,
    y_or_n_state: &mut YOrNState,
    real_name: &str,
    scroll_offset: &mut usize,
    saved: &mut bool,
    should_exit: &mut bool,
) -> io::Result<()> {
    let (_, height) = size()?;
    let screen_height = (height as usize).saturating_sub(2); // 上下の枠を考慮

    match key.code {
        KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            let old = buffer.join("\n");
            if content != old && !*saved {
                y_or_n_state.is_active = true;
                y_or_n_state.prompt = format!(
                    "The file {} is not saved! Do you really want to exit?",
                    real_name
                );
            } else {
                *should_exit = true;
            }
        }
        KeyCode::Char('b') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            cursor.x = cursor.x.saturating_sub(1);
        }
        KeyCode::Char('f') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if cursor.x < buffer[cursor.y].len() {
                cursor.x += 1;
            }
        }
        KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if cursor.y < buffer.len() - 1 {
                cursor.y += 1;
                update_scroll(&mut cursor.y, screen_height, scroll_offset);
                cursor.x = cursor.x.min(buffer[cursor.y].len());
            }
        }

        KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if cursor.y > 0 {
                cursor.y -= 1;
                update_scroll(&mut cursor.y, screen_height, scroll_offset);
                cursor.x = cursor.x.min(buffer[cursor.y].len());
            }
        }

        KeyCode::Char('o') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            buffer.insert(cursor.y + 1, "".to_string());
            cursor.y += 1;
            cursor.x = 0;
        }
        KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if cursor.x == 0 && buffer[cursor.y].len() == 0{
                // **行削除処理**
                buffer.remove(cursor.y);
                if cursor.y > 0 {
                    cursor.y -= 1;
                }
                cursor.x = 0;
            } else {
                // 行の中の文字を削除
                if cursor.x < buffer[cursor.y].len() {
                    buffer[cursor.y].remove(cursor.x);
                }
            }
        }
        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            save_file(real_name, buffer, message);
            *saved = true;
        }
        KeyCode::Char(p) if key.modifiers.contains(KeyModifiers::CONTROL) => {
            message.text = format!(" C-{p}: Not a Command ").light_red().to_string();
            message.displayed = true;
        }
        KeyCode::Char(c) => {
            buffer[cursor.y].insert(cursor.x, c);
            cursor.x += 1;
            *saved = false;
        }
        KeyCode::Backspace => {
            handle_backspace(buffer, cursor);
        }
        KeyCode::Enter => {
            let remaining_text = buffer[cursor.y].split_off(cursor.x);
            buffer.insert(cursor.y + 1, remaining_text);
            cursor.y += 1;
            cursor.x = 0;
        }
        KeyCode::Left => {
            display_deprecated_message(" Arrow keys are deprecated, use C-b ", message);
            cursor.x = cursor.x.saturating_sub(1);
        }
        KeyCode::Right => {
            display_deprecated_message(" Arrow keys are deprecated, use C-f ", message);
            if cursor.x < buffer[cursor.y].len() {
                cursor.x += 1;
            }
        }
        KeyCode::Up => {
            display_deprecated_message(" Arrow keys are deprecated, use C-p ", message);
            if cursor.y > 0 {
                cursor.y -= 1;
                cursor.x = cursor.x.min(buffer[cursor.y].len());
            }
        }
        KeyCode::Down => {
            display_deprecated_message(" Arrow keys are deprecated, use C-n ", message);
            if cursor.y < buffer.len() - 1 {
                cursor.y += 1;
                cursor.x = cursor.x.min(buffer[cursor.y].len());
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_backspace(buffer: &mut Vec<String>, cursor: &mut Cursor) {
    if cursor.x > 0 {
        buffer[cursor.y].remove(cursor.x - 1);
        cursor.x -= 1;
    } else if cursor.y > 0 {
        let removed_line = buffer.remove(cursor.y);
        cursor.y -= 1;
        cursor.x = buffer[cursor.y].len();
        buffer[cursor.y].push_str(&removed_line);
    }
}

fn display_deprecated_message(text: &str, message: &mut Message) {
    message.text = text
        .to_string()
        .light_red()
        .to_string()
        .yellow()
        .to_string();
    message.message_type = MessageType::Error;
}
