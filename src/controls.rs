pub fn default_controls (
    event_key: crossterm::event::KeyEvent,
    cursor_y: &mut i32,
    cursor_x: &mut i32,
    gcursor: &mut i32,
    splitted: &mut Vec<&str>
    ) -> std::io::Result<bool>
{
    match event_key.code
    {
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

pub fn controls (
    event_key: crossterm::event::KeyEvent,
    cursor_y: &mut i32,
    cursor_x: &mut i32,
    gcursor: &mut i32,
    splitted: &mut Vec<&str>
    ) -> std::io::Result<bool>
{
    if default_controls(event_key, cursor_y, cursor_x, gcursor, splitted)?
    {
        return Ok(true);
    }
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
        _ => {return Ok(false);}
    }
    Ok(true)

}
