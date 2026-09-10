use ratatui::{self, DefaultTerminal};
use crate::controls::{default_controls, controls};


pub fn normal_mode (
    terminal: &DefaultTerminal,
    event_key: crossterm::event::KeyEvent,
    mode: &mut i32,
    cursor_x: &mut i32,
    cursor_y: &mut i32,
    gcursor: &mut i32,
    // input_box: &mut String,
    splitted: &mut Vec<&str>,
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
        crossterm::event::KeyCode::Char('s') => 
        {
            *mode = 10;
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
    if default_controls(event_key, cursor_y, cursor_x, gcursor, splitted)?
    {
        return Ok(true);
    }
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
        _ => {}
    }
    Ok(true)
}



//////////////////////////////////////////// DEAD CODE BURIED HERE ////////////////////////////////////////////

// pub fn command_mode(
//     event_key: crossterm::event::KeyEvent,
//     mode: &mut i32,
//     the_command_line: &mut String
//     ) -> std::io::Result<bool>
// {
//     match mode {
//         10 => 
//         {
//
//         }
//     }
//     Ok(true)
// }
//
//
// pub fn execute_commands(
//     the_command_line: &mut String
// ) -> std::io::Result<bool>
// {
//     let mut parts: Vec<&str> = the_command_line.split_whitespace().collect();
//
//     let output = std::process::Command::new(parts[0])
//         .args(&parts[1..])
//         .output()
//         .expect("Failed to run the command");
//
//     println!("{:?}", output);
//     the_command_line.clear();
//     Ok(true)
// }
