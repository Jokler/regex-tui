use crate::app::{App, AppResult, CurrentField};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use regex::Regex;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if key_event.modifiers == KeyModifiers::CONTROL {
        match key_event.code {
            KeyCode::Char('u') | KeyCode::Char('U') => {
                match app.current_input {
                    CurrentField::Regex => {
                        app.regex.clear();
                        app.cursor_pos.x = 0;
                    }
                    CurrentField::Text => {
                        app.text[app.cursor_pos.y as usize].clear();
                        app.cursor_pos.x = 0;
                    }
                }
                return Ok(());
            }
            _ => (),
        }
    }
    match key_event.code {
        // exit application on ESC
        KeyCode::Esc => {
            app.running = false;
        }
        KeyCode::Char(c) => {
            match app.current_input {
                CurrentField::Regex => {
                    app.regex.insert(app.cursor_pos.x as usize, c);
                    app.cursor_pos.x += 1;
                }
                CurrentField::Text => {
                    app.text[app.cursor_pos.y as usize].insert(app.cursor_pos.x as usize, c);
                    app.cursor_pos.x += 1;
                }
            };
            update_output(app);
        }
        KeyCode::Enter => {
            if app.current_input.is_text() {
                app.text.push(String::new());
                app.cursor_pos.y += 1;
                app.cursor_pos.x = 0;
            }
            update_output(app);
        }
        KeyCode::Tab => {
            app.current_input.next();
            match app.current_input {
                CurrentField::Regex => app.cursor_pos.x = app.regex.len() as u16,
                CurrentField::Text => {
                    app.cursor_pos.x = app.text[app.cursor_pos.y as usize].len() as u16
                }
            }
        }
        KeyCode::Backspace => match app.current_input {
            CurrentField::Regex => {
                if app.cursor_pos.x > 0 {
                    app.regex.remove(app.cursor_pos.x as usize - 1);
                    app.cursor_pos.x -= 1;
                }
                update_output(app);
            }
            CurrentField::Text => {
                let line = &mut app.text[app.cursor_pos.y as usize];
                if app.cursor_pos.x == 0 {
                    if app.cursor_pos.y != 0 {
                        app.text.remove(app.cursor_pos.y as usize);
                        app.cursor_pos.y -= 1;
                        app.cursor_pos.x = app.text[app.cursor_pos.y as usize].len() as u16;
                    }
                } else {
                    line.remove(app.cursor_pos.x as usize - 1);
                    app.cursor_pos.x -= 1;
                }
                update_output(app);
            }
        },
        KeyCode::Up => {
            if app.current_input.is_text() && app.cursor_pos.y > 0 {
                app.cursor_pos.y -= 1;
                app.cursor_pos.x = app.text[app.cursor_pos.y as usize].len() as u16;
            }
        }
        KeyCode::Down => {
            if app.current_input.is_text() && app.cursor_pos.y < app.text.len() as u16 - 1 {
                app.cursor_pos.y += 1;
                app.cursor_pos.x = app.text[app.cursor_pos.y as usize].len() as u16;
            }
        }
        KeyCode::Left => {
            if app.cursor_pos.x > 0 {
                app.cursor_pos.x -= 1;
            }
        }
        KeyCode::Right => {
            let line = match app.current_input {
                CurrentField::Regex => &app.regex,
                CurrentField::Text => &app.text[app.cursor_pos.y as usize],
            };

            if app.cursor_pos.x < line.len() as u16 {
                app.cursor_pos.x += 1;
            }
        }
        _ => {}
    }

    Ok(())
}

pub fn update_output(app: &mut App) {
    if app.current_input == CurrentField::Regex {
        if !app.regex.is_empty() {
            match Regex::new(&app.regex) {
                Ok(re) => app.re = Some(re),
                Err(e) => {
                    app.output = e.to_string();
                    return;
                }
            }
        } else {
            app.re = None;
            app.output.clear();
        }
    }

    if let Some(re) = &app.re {
        let mut new_output = String::new();

        let joined = app.text.join("\n");
        let captures = re.captures_iter(&joined);
        for (i, capture) in captures.enumerate() {
            new_output.push_str(&format!("{}.\n", i));

            let mut capture_names = re.capture_names();
            for (i, m) in capture.iter().enumerate() {
                let m = m.unwrap();
                match capture_names.next().unwrap() {
                    Some(name) => new_output.push_str(&format!("  {}: {}\n", name, m.as_str())),
                    None => new_output.push_str(&format!("  {}: {}\n", i, m.as_str())),
                }
            }
        }

        app.output = new_output;
    }
}
