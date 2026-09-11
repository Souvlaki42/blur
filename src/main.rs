mod modes;
mod controls;
mod helpers;


use ratatui::{self, DefaultTerminal, Frame, style::Color::{Black, White}};
use helpers::Tab;


fn main() -> std::io::Result<()> 
{
    ratatui::run(app)?;
    Ok(())
}


fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut tab = Tab::new();
    let mut mode = 0;
    let mut the_command_line = String::new();
    loop {
        terminal.draw(|frame| renderer(frame, &tab, mode, &mut the_command_line))?;

        let event = crossterm::event::read()?;
        let the_text = &tab.input_box.clone();
        let mut splitted: Vec<_> = the_text.split('\n').collect();
        if let crossterm::event::Event::Key(event_key) = event {
            match mode {
                0 => { ////////////////////// NORMAL MODE ////////////////////////
                       if !modes::normal_mode(terminal, &mut tab, event_key, &mut mode, &mut the_command_line, &mut splitted).unwrap()
                       {
                           break;
                       }
                }
                1 => { /////////////////////// INSERT MODE /////////////////////////
                       if !modes::insert_mode(terminal, &mut tab, event_key, &mut mode, &mut splitted).unwrap()
                       {
                           continue;
                       }
                }
                10 => { ////////////////////// COMMAND MODE ////////////////////////////////
                        if !modes::save_mode(&mut tab, event_key, &mut the_command_line, &mut mode).unwrap()
                        {
                            break;
                        }
                }
                _ => {}
            }
        }
    }
    Ok(())
}


fn renderer(frame: &mut Frame, tab: &Tab, mode: i32, the_command_line: &str){
    let areas = ratatui::layout::Layout::vertical([
        ratatui::layout::Constraint::Min(0),
        ratatui::layout::Constraint::Length(1)
    ]).split(frame.area());

    let footer_text: String;
    let bottom_chunk = ratatui::layout::Layout::horizontal([
        ratatui::layout::Constraint::Percentage(50),
        ratatui::layout::Constraint::Percentage(25),
        ratatui::layout::Constraint::Percentage(25)
    ]).split(areas[1]);

    match mode {
        0 => {
            footer_text = format!("NORMAL");
        }
        1 => {
            footer_text = format!("INSERT");
        }
        10 => {
            footer_text = format!("Save file into : {}", the_command_line);
        }
        _ => {footer_text = "SOME ERRORS, try to relaunch the program                   BLUR V0.1".to_string();}
    }

    let footer_mode = ratatui::widgets::Paragraph::new(footer_text.clone())
                .alignment(ratatui::layout::Alignment::Left)
                .style(ratatui::style::Style::default()
                    .fg(Black)
                    .bg(White));
    let footer_file_name = ratatui::widgets::Paragraph::new(format!("{}", tab.file_name))
                .alignment(ratatui::layout::Alignment::Left)
                .style(ratatui::style::Style::default()
                    .fg(Black)
                    .bg(White));
    let footer_copyrights = ratatui::widgets::Paragraph::new(format!("{}, {}   BLUR V0.1" , tab.cursor_x, tab.gcursor))
                .alignment(ratatui::layout::Alignment::Right)
                .style(ratatui::style::Style::default()
                    .fg(Black)
                    .bg(White));

    let input = ratatui::text::Text::from(tab.input_box.clone());
    frame.render_widget(input, areas[0]);
    frame.render_widget(footer_mode, bottom_chunk[0]);
    frame.render_widget(footer_file_name, bottom_chunk[1]);
    frame.render_widget(footer_copyrights, bottom_chunk[2]);

    frame.set_cursor_position((
            areas[0].x + tab.cursor_x as u16,
            areas[0].y + tab.cursor_y as u16
            ));
    if mode == 10 {
        frame.set_cursor_position((
                areas[1].x + footer_text.len() as u16 + 1,
                areas[1].y
                ));
    }

}

