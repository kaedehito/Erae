use super::{
    handle_key_event,
    scroll::update_scroll,
    types::{self, Cursor, Message, YOrNState},
};
use crossterm::{
    cursor::{self, MoveTo, Show},
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::Margin,
    prelude::CrosstermBackend,
    style::{Color, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph},
    Terminal,
};
use std::{fs, io, path::Path, time::Duration};

pub fn run(content: String, file_name: &str, real_name: &str) -> io::Result<()> {
    setup_terminal()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.show_cursor()?; // `ratatui` でカーソルを有効化

    let mut should_exit = false;
    let mut buffer = initialize_buffer(&content);
    let mut cursor = Cursor::default();
    let mut message = Message::default();
    let mut y_or_n_state = YOrNState::default();
    let mut scroll_offset = 0usize;
    let mut saved = false;

    while !should_exit {
        message.count += 1;
        draw_terminal(
            &mut terminal,
            &buffer.clone(),
            &mut cursor,
            &mut message,
            &mut y_or_n_state,
            file_name,
            scroll_offset,
        )?;

        if y_or_n_state.is_active {
            handle_y_or_n_event(
                &mut should_exit,
                &mut y_or_n_state,
                &buffer.join("\n"),
                real_name,
            )?;
            continue;
        }

        clear_message_after_timeout(&mut message);

        execute!(
            io::stdout(),
            Show,
            MoveTo((cursor.x + 1) as u16, (cursor.y - scroll_offset + 1) as u16)
        )?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                handle_key_event::handle_key_event(
                    content.clone(),
                    key,
                    &mut buffer,
                    &mut cursor,
                    &mut message,
                    &mut y_or_n_state,
                    real_name,
                    &mut scroll_offset,
                    &mut saved,
                    &mut should_exit,
                )?;
            }
        }
    }

    cleanup_terminal()?;
    Ok(())
}

fn setup_terminal() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen, cursor::Show)?;
    Ok(())
}

fn initialize_buffer(content: &str) -> Vec<String> {
    if content.is_empty() {
        vec![String::new()]
    } else {
        content.lines().map(String::from).collect()
    }
}

fn draw_terminal(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    buffer: &[String],
    cursor: &mut Cursor,
    message: &mut Message,
    y_or_n_state: &mut YOrNState,
    file_name: &str,
    mut scroll_offset: usize,
) -> io::Result<()> {
    terminal
        .draw(|frame| {
            let size = frame.area();
            let screen_height = size.height as usize - 2;

            update_scroll(&mut cursor.y, screen_height, &mut scroll_offset);

            let visible_lines: Vec<String> = buffer
                .iter()
                .skip(scroll_offset)
                .take(screen_height)
                .cloned()
                .collect();
            let buf_str = visible_lines.join("\n");

            let text = Line::from(vec![
                " Ln: ".into(),
                format!("{} | ", cursor.y + 1).into(),
                "Col: ".into(),
                format!("{} ", cursor.x + 1).into(),
            ]);

            let window_name = Line::from(format!(" {} ", file_name));

            let mut block = Block::bordered()
                .title_top(window_name.light_green().right_aligned())
                .title_bottom(text.clone().left_aligned())
                .border_set(border::THICK);

            if !message.text.is_empty() {
                let message_line = Line::from(message.text.clone());
                block = match message.message_type {
                    types::MessageType::Error => {
                        block.title_bottom(message_line.light_red().right_aligned())
                    }
                    types::MessageType::Info => {
                        block.title_bottom(message_line.light_green().right_aligned())
                    }
                    _ => block,
                };
            }

            let paragraph = Paragraph::new(buf_str.clone())
                .block(block)
                .style(Style::default().fg(Color::White));

            frame.render_widget(paragraph, size);
        })
        .map(|_| ())?;

    if y_or_n_state.is_active {
        terminal.clear().unwrap();
        terminal
            .draw(|frame| {
                let area = frame.area().inner(Margin::new(10, 10));
                let lin = Line::from(" Confirm ");

                let bottom_title = Line::from(vec![
                    " Save and Exit".into(),
                    " <S> ".yellow().bold(), // アクションの後にショートカットを配置
                    " Exit Anyway".into(),
                    " <Q> ".yellow().bold(), // 同じく
                    " Cancel".into(),
                    " <N> ".yellow().bold(), // ここも配置を調整
                ]);

                let prompt = Line::from(y_or_n_state.prompt.clone());

                let block = Block::bordered()
                    .title(lin.centered())
                    .title_bottom(bottom_title.centered())
                    .style(Style::default().fg(ratatui::style::Color::White))
                    .border_set(border::THICK);

                // 背景色を黒に設定（必要に応じて他の色に変更可能）
                let paragraph =
                    Paragraph::new(prompt.alignment(ratatui::layout::Alignment::Center))
                        .block(block)
                        .style(Style::default().fg(ratatui::style::Color::White))
                        .style(Style::default().bg(ratatui::style::Color::Black)); // 背景を黒でクリア

                frame.render_widget(paragraph, area);
            })
            .map(|_| ())?;
    }

    Ok(())
}

fn handle_y_or_n_event(
    should_exit: &mut bool,
    y_or_n_state: &mut YOrNState,
    content: &str,
    path: &str,
) -> io::Result<()> {
    if let Event::Key(k) = event::read()? {
        match k.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                *should_exit = true;
                y_or_n_state.is_active = false;
            }

            KeyCode::Char('s') | KeyCode::Char('S') => {
                let path: &Path = path.as_ref();

                fs::write(path, content).unwrap_or_else(|e| {
                    *should_exit = true;
                    eprintln!("Failed to write file: {e}");
                });

                *should_exit = true;
            }

            KeyCode::Char('n') => {
                y_or_n_state.is_active = false;
            }
            _ => {}
        }
    }
    Ok(())
}

fn clear_message_after_timeout(message: &mut Message) {
    if message.count >= 120 && message.displayed {
        message.text.clear();
        message.displayed = false;
    } else if !message.displayed {
        message.count = 0;
    }
}

fn cleanup_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
