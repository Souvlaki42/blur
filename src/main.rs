use ratatui::{self, DefaultTerminal, Frame, style::Color::{Black, White}};

fn main() -> std::io::Result<()> 
{
    ratatui::run(app)?;
    Ok(())
}


fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {

    let mut input_box = String::new();
    let mut cursor_x = 0;
    let mut cursor_y = 0;
    let mut mode = 0;
    loop {
        terminal.draw(|frame| renderer(frame, &input_box, cursor_x, cursor_y, mode))?;

        let event = crossterm::event::read()?;
        if let crossterm::event::Event::Key(event_key) = event {
            match mode {
                0 => { ////////////////////// NORMAL MODE ////////////////////////
                    match event_key.code {
                        crossterm::event::KeyCode::Char('a') => {mode = 1; continue;}
                        crossterm::event::KeyCode::Char('q') => {break;}
                        _ => {}
                    }
                }
                1 => { /////////////////////// INSERT MODE /////////////////////////
                    match event_key.code {
                        crossterm::event::KeyCode::Esc => {mode = 0; continue;}
                        crossterm::event::KeyCode::Char(c) => {
                            let width = terminal.size()?.width;
                            if cursor_x >= width.into() {
                                input_box.push('\n');
                                cursor_x = 0;
                                cursor_y += 1;
                            }
                            input_box.push(c);
                            cursor_x += 1;
                        }
                        crossterm::event::KeyCode::Enter => {
                            input_box.push('\n');
                            cursor_x = 0;
                            cursor_y += 1;
                        }
                        crossterm::event::KeyCode::Backspace => {
                            input_box.pop();
                            if cursor_x > 0 {
                                cursor_x -= 1;
                            }
                            else if cursor_y > 0{
                                cursor_y -= 1;
                                let last_newline = input_box.rfind('\n');
                                match last_newline {
                                    Some(last_newline) => {cursor_x = input_box[last_newline + 1..].len() as i32;}
                                    None => {cursor_x = input_box.len() as i32;}
                                }

                            }
                        }
                        _ => {}
                    }
                } /////////////////// END OF INSERT MODE /////////////////////////
                _ => {}
            }
        }
    }
    Ok(())
}



fn renderer(frame: &mut Frame, input_box: &str, cursor_x: i32, cursor_y: i32, mode: i32){
    let areas = ratatui::layout::Layout::vertical([
        ratatui::layout::Constraint::Min(0),
        ratatui::layout::Constraint::Length(1)
    ]).split(frame.area());

    let footer_text: String;

    match mode {
        0 => {footer_text = "NORMAL                                                                                                                                             BLUR V0.1".to_string()}
        1 => {footer_text = "INSERT                                                                                                                                             BLUR V0.1".to_string();}
        _ => {footer_text = "SOME ERRORS, try to relaunch the program                   BLUR V0.1".to_string();}
    }

    let footer = ratatui::widgets::Paragraph::new(footer_text)
                .alignment(ratatui::layout::Alignment::Left)
                .style(ratatui::style::Style::default()
                    .fg(Black)
                    .bg(White));

    let input = ratatui::text::Text::from(input_box);
    frame.render_widget(input, areas[0]);
    frame.render_widget(footer, areas[1]);

    frame.set_cursor_position((
            areas[0].x + cursor_x as u16,
            areas[0].y + cursor_y as u16
            ));

}

