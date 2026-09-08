use crossterm::style::Color::{Black, White};
use ratatui::{self, DefaultTerminal, Frame};

fn main() -> std::io::Result<()> 
{
    ratatui::run(app)?;
    Ok(())
}


fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {

    let mut input_box = String::new();
    loop {
        terminal.draw(|frame| renderer(frame, &input_box))?;
        let event = crossterm::event::read()?;

        if let crossterm::event::Event::Key(event_key) = event {
            match event_key.code {
                crossterm::event::KeyCode::Esc => {break;}
                crossterm::event::KeyCode::Char(c) => {input_box.push(c);}
                crossterm::event::KeyCode::Backspace => {input_box.pop();}
                _ => {}
            }
        }
    }
    Ok(())
}



fn renderer(frame: &mut Frame, input_box: &str){
    let areas = ratatui::layout::Layout::vertical([
        ratatui::layout::Constraint::Min(0),
        ratatui::layout::Constraint::Length(1)
    ]).split(frame.area());

    let footer = ratatui::widgets::Paragraph::new("BLUR V0.1")
        .alignment(ratatui::layout::Alignment::Center)
        .style(ratatui::style::Style::default())
        .fg(Black)
        .bg(White);
    frame.render_widget(input_box, areas[0]);
    frame.render_widget(footer, areas[1]);

}

