use ratatui::{self, DefaultTerminal};

fn controls (
    event_key: crossterm::event::KeyEvent,
    cursor_y: &mut i32,
    cursor_x: &mut i32,
    gcursor: &mut i32,
    splitted: &mut Vec<&str>
    ) -> std::io::Result<bool>
{
    match event_key.code 
    {
        crossterm::event::KeyCode::Char('h') => {
            if *cursor_x > 0 {
                *cursor_x -= 1;
                *gcursor -= 1;
            }
        }
        crossterm::event::KeyCode::Char('l') => {
            if *cursor_x < splitted[*cursor_y as usize].len() as i32 {
                *cursor_x += 1;
                *gcursor += 1;
            }
        }
        crossterm::event::KeyCode::Char('k') => {
            if *cursor_y > 0 {
                if *cursor_x > splitted[*cursor_y as usize - 1].len() as i32 {
                    *cursor_x = splitted[*cursor_y as usize -1 ].len() as i32;
                }
                *cursor_y -= 1;
                *gcursor = 0;
                for y in 0..*cursor_y as usize {
                    *gcursor += splitted[y].len() as i32 + 1;
                }
                *gcursor += *cursor_x;
            }
        }
        crossterm::event::KeyCode::Char('j') => {
            if *cursor_y < splitted.len() as i32 - 1 {
                if *cursor_x > splitted[*cursor_y as usize + 1].len() as i32 {
                    *cursor_x = splitted[*cursor_y as usize + 1 ].len() as i32;
                }
                *cursor_y += 1;
                *gcursor = 0;
                for y in 0..*cursor_y as usize {
                    *gcursor += splitted[y].len() as i32 + 1;
                }
                *gcursor += *cursor_x;
            }
        }

        crossterm::event::KeyCode::Left => {
            if *cursor_x > 0 {
                *cursor_x -= 1;
                *gcursor -= 1;
            }
        }
        crossterm::event::KeyCode::Right => {
            if *cursor_x < splitted[*cursor_y as usize].len() as i32 {
                *cursor_x += 1;
                *gcursor += 1;
            }
        }
        crossterm::event::KeyCode::Up => {
            if *cursor_y > 0 {
                if *cursor_x > splitted[*cursor_y as usize - 1].len() as i32 {
                    *cursor_x = splitted[*cursor_y as usize -1 ].len() as i32;
                }
                *cursor_y -= 1;
                *gcursor = 0;
                for y in 0..*cursor_y as usize {
                    *gcursor += splitted[y].len() as i32 + 1;
                }
                *gcursor += *cursor_x;
            }
        }
        crossterm::event::KeyCode::Down => {
            if *cursor_y < splitted.len() as i32 - 1 {
                if *cursor_x > splitted[*cursor_y as usize + 1].len() as i32 {
                    *cursor_x = splitted[*cursor_y as usize + 1 ].len() as i32;
                }
                *cursor_y += 1;
                *gcursor = 0;
                for y in 0..*cursor_y as usize {
                    *gcursor += splitted[y].len() as i32 + 1;
                }
                *gcursor += *cursor_x;
            }
        }
        _ => {return Ok(false);}
    }
    Ok(true)

}



pub fn normal_mode (
    terminal: &DefaultTerminal,
    event_key: crossterm::event::KeyEvent,
    mode: &mut i32,
    cursor_x: &mut i32,
    cursor_y: &mut i32,
    gcursor: &mut i32,
    splitted: &mut Vec<&str>
    ) -> std::io::Result<bool> {
    if controls(event_key, cursor_y, cursor_x, gcursor, splitted).unwrap() {
        return Ok(true);
    }
    match event_key.code {
        crossterm::event::KeyCode::Char('a') => {
            *mode = 1;
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
    input_box: &mut String,
    splitted: &mut Vec<&str>
    ) -> std::io::Result<bool> 
{
    match event_key.code 
    {
        crossterm::event::KeyCode::Esc => 
        {
            *mode = 0;
            return Ok(false);
        }

        crossterm::event::KeyCode::Char(c) => 
        {
            let width = terminal.size()?.width;
            if *cursor_x >= width.into()
            {
                input_box.insert(*gcursor as usize,'\n');
                *cursor_x = 0;
                *cursor_y += 1;
                *gcursor += 1;
            }
            
            input_box.insert(*gcursor as usize, c);
            *cursor_x += 1;
            *gcursor += 1;
        }

        crossterm::event::KeyCode::Enter => 
        {
            input_box.insert(*gcursor as usize, '\n');
            *cursor_x = 0;
            *cursor_y += 1;
            *gcursor += 1;
        }

        crossterm::event::KeyCode::Backspace => 
        {
            if *gcursor > 0 
            {
                *gcursor -= 1;
                input_box.remove(*gcursor as usize);
                
                if *cursor_x > 0 
                {
                    *cursor_x -= 1;
                }

                else if *cursor_y > 0
                {
                    *cursor_y -= 1;
                    *cursor_x = splitted[*cursor_y as usize].len() as i32;
                }
            }
        }
        crossterm::event::KeyCode::Left => {
            if *cursor_x > 0 {
                *cursor_x -= 1;
                *gcursor -= 1;
            }
        }
        crossterm::event::KeyCode::Right => {
            if *cursor_x < splitted[*cursor_y as usize].len() as i32 {
                *cursor_x += 1;
                *gcursor += 1;
            }
        }
        crossterm::event::KeyCode::Up => {
            if *cursor_y > 0 {
                if *cursor_x > splitted[*cursor_y as usize - 1].len() as i32 {
                    *cursor_x = splitted[*cursor_y as usize -1 ].len() as i32;
                }
                *cursor_y -= 1;
                *gcursor = 0;
                for y in 0..*cursor_y as usize {
                    *gcursor += splitted[y].len() as i32 + 1;
                }
                *gcursor += *cursor_x;
            }
        }
        crossterm::event::KeyCode::Down => {
            if *cursor_y < splitted.len() as i32 - 1 {
                if *cursor_x > splitted[*cursor_y as usize + 1].len() as i32 {
                    *cursor_x = splitted[*cursor_y as usize + 1 ].len() as i32;
                }
                *cursor_y += 1;
                *gcursor = 0;
                for y in 0..*cursor_y as usize {
                    *gcursor += splitted[y].len() as i32 + 1;
                }
                *gcursor += *cursor_x;
            }
        }
        _ => {}
    }
    Ok(true)
}
