use ratatui::{self, DefaultTerminal};



pub fn normal_mode (
    // terminal: &DefaultTerminal,
    event_key: crossterm::event::KeyEvent,
    mode: &mut i32,
    cursor_x: &mut i32,
    gcursor: &mut i32,
    // cursor_y: &mut i32,
    // input_box: &mut String
    ) -> std::io::Result<bool> {
    match event_key.code {
        crossterm::event::KeyCode::Char('a') => {
            *mode = 1;
        }
        crossterm::event::KeyCode::Char('h') => {
            if *cursor_x > 0 {
                *cursor_x -= 1;
                *gcursor -= 1;
            }
        }
        crossterm::event::KeyCode::Char('q') => {
            return Ok(false);
        }
        _ => {}
    }
    Ok(true)
}

pub fn insert_mode(
    terminal: &DefaultTerminal,
    event_key: crossterm::event::KeyEvent,
    mode: &mut i32,
    cursor_x: &mut i32,
    cursor_y: &mut i32,
    gcursor: &mut i32,
    input_box: &mut String
    ) -> std::io::Result<bool> {
    match event_key.code {
        crossterm::event::KeyCode::Esc => {
            *mode = 0;
            return Ok(false);
        }
        crossterm::event::KeyCode::Char(c) => {
            let width = terminal.size()?.width;
            if *cursor_x >= width.into() {
                input_box.insert(*gcursor as usize,'\n');
                *cursor_x = 0;
                *cursor_y += 1;
                *gcursor += 1;
            }
            input_box.insert(*gcursor as usize, c);
            *cursor_x += 1;
            *gcursor += 1;
        }
        crossterm::event::KeyCode::Enter => {
            input_box.insert(*gcursor as usize, '\n');
            *cursor_x = 0;
            *cursor_y += 1;
            *gcursor += 1;
        }
        crossterm::event::KeyCode::Backspace => {
            if *gcursor > 0 {
                *gcursor -= 1;
                input_box.remove(*gcursor as usize);
                if *cursor_x > 0 {
                    *cursor_x -= 1;
                }
                else if *cursor_y > 0{
                    *cursor_y -= 1;
                    let last_newline = input_box.rfind('\n');
                    match last_newline {
                        Some(last_newline) => {
                            *cursor_x = input_box[last_newline + 1..].len() as i32;
                        }
                        None => {
                            *cursor_x = *gcursor;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    Ok(true)
}
