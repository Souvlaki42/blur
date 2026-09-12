// helpers.rs — add these to what's already there
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{ThemeSet, Theme};
use syntect::easy::HighlightLines;
use syntect::util::LinesWithEndings;
use ratatui::text::{Line, Span};
use ratatui::style::{Color, Style};

pub struct Highlighter {
    syntax_set: SyntaxSet,
    theme: Theme,
}

impl Highlighter {
    pub fn new() -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();
        let theme = theme_set.themes["base16-ocean.dark"].clone();
        Highlighter { syntax_set, theme }
    }

    pub fn highlight<'a>(&self, input_box: &str, file_name: &str) -> Vec<Line<'a>> {
        if file_name.is_empty() {
            return input_box
                .split('\n')
                .map(|line| Line::from(Span::styled(
                    line.to_string(),
                    Style::default().fg(Color::White),
                )))
                .collect();
        }

        let syntax = self.syntax_set
            .find_syntax_for_file(file_name)
            .ok()
            .flatten()
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        let mut h = HighlightLines::new(syntax, &self.theme);

        LinesWithEndings::from(input_box)
            .map(|line| {
                let ranges = h.highlight_line(line, &self.syntax_set).unwrap_or_default();
                let spans: Vec<Span> = ranges
                    .into_iter()
                    .map(|(style, text)| {
                        let fg = style.foreground;
                        Span::styled(
                            text.trim_end_matches('\n').to_string(),
                            Style::default().fg(Color::Rgb(fg.r, fg.g, fg.b)),
                        )
                    })
                    .collect();
                Line::from(spans)
            })
            .collect()
    }
}

//////////////////////////////////////////////////////////////////////////////

pub struct Tab {
    pub file_name: String,
    pub input_box: String,
    pub cursor_x: i32,
    pub cursor_y: i32,
    pub gcursor: i32
}

impl Tab {
    pub fn new() -> Self
    {
        Self { 
            file_name: String::from(""),
            input_box : String::new(),
            cursor_x : 0,
            cursor_y : 0,
            gcursor : 0
        }
    }
}



