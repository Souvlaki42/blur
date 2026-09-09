mod modes;
mod controls;


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
    let mut gcursor = 0;
    let mut mode = 0;
    loop {
        terminal.draw(|frame| renderer(frame, &input_box, cursor_x, cursor_y, gcursor, mode))?;

        let event = crossterm::event::read()?;
        let the_text = &input_box.clone();
        let mut splitted: Vec<_> = the_text.split('\n').collect();
        if let crossterm::event::Event::Key(event_key) = event {
            match mode {
                0 => { ////////////////////// NORMAL MODE ////////////////////////
                       if !modes::normal_mode(terminal, event_key, &mut mode, &mut cursor_x, &mut cursor_y, &mut gcursor, &mut splitted).unwrap() {
                           break;
                       }
                }
                1 => { /////////////////////// INSERT MODE /////////////////////////
                       if !modes::insert_mode(terminal, event_key, &mut mode, &mut cursor_x, &mut cursor_y, &mut gcursor, &mut input_box, &mut splitted).unwrap(){
                           continue;
                       }
                }
                // 2 => { ////////////////////// COMMAND MODE ////////////////////////////////
                //         if !modes::command_mode(event_key, &mut mode, &mut the_command_line).unwrap()
                //         {
                //             break;
                //         }
                // }
                _ => {}
            }
        }
    }
    Ok(())
}


fn renderer(frame: &mut Frame, input_box: &str, cursor_x: i32, cursor_y: i32, gcursor: i32, mode: i32){
    let areas = ratatui::layout::Layout::vertical([
        ratatui::layout::Constraint::Min(0),
        ratatui::layout::Constraint::Length(1)
    ]).split(frame.area());

    let footer_text: String;
    let bottom_chunk = ratatui::layout::Layout::horizontal([
        ratatui::layout::Constraint::Percentage(50),
        ratatui::layout::Constraint::Percentage(50)
    ]).split(areas[1]);

    match mode {
        0 => {
            footer_text = format!("NORMAL");
        }
        1 => {
            footer_text = format!("INSERT");
        }
        // 2 => {
        //     footer_text = format!("{}", the_command_line);
        // }
        _ => {footer_text = "SOME ERRORS, try to relaunch the program                   BLUR V0.1".to_string();}
    }

    let footer_mode = ratatui::widgets::Paragraph::new(footer_text)
                .alignment(ratatui::layout::Alignment::Left)
                .style(ratatui::style::Style::default()
                    .fg(Black)
                    .bg(White));
    let footer_copyrights = ratatui::widgets::Paragraph::new(format!("{}, {}   BLUR V0.1" , cursor_x, gcursor))
                .alignment(ratatui::layout::Alignment::Right)
                .style(ratatui::style::Style::default()
                    .fg(Black)
                    .bg(White));

    let input = ratatui::text::Text::from(input_box);
    frame.render_widget(input, areas[0]);
    frame.render_widget(footer_mode, bottom_chunk[0]);
    frame.render_widget(footer_copyrights, bottom_chunk[1]);

    frame.set_cursor_position((
            areas[0].x + cursor_x as u16,
            areas[0].y + cursor_y as u16
            ));

}

