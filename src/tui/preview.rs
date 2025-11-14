use std::io::{self, Stdout};
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Terminal;

use crate::core::theme::Theme;
use crate::error::{PrismError, PrismResult};
use crate::tui::components::terminal_frame::render_terminal;

pub fn run_preview(themes: Vec<Theme>) -> PrismResult<()> {
    if themes.is_empty() {
        return Err(PrismError::new("no themes available for preview"));
    }

    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let result = PreviewApp::new(themes).run(&mut terminal);
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}

struct PreviewApp {
    themes: Vec<Theme>,
    index: usize,
    message: Option<String>,
}

impl PreviewApp {
    fn new(themes: Vec<Theme>) -> Self {
        Self {
            themes,
            index: 0,
            message: Some("Use j/k to navigate, a=apply, q=quit".into()),
        }
    }

    fn run(mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> PrismResult<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            if crossterm::event::poll(Duration::from_millis(200))? {
                match event::read()? {
                    Event::Key(key) => match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('j') | KeyCode::Down => self.next(),
                        KeyCode::Char('k') | KeyCode::Up => self.prev(),
                        KeyCode::Char('a') => {
                            let name = self.current().metadata.name.clone();
                            self.message = Some(format!(
                                "Queued apply for {name}. Run `prism apply {name}`."
                            ));
                        }
                        _ => {}
                    },
                    Event::Resize(_, _) => {}
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut ratatui::Frame<'_>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(5),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(frame.size());

        let list_items: Vec<_> = self
            .themes
            .iter()
            .enumerate()
            .map(|(idx, theme)| {
                let mut line = theme.metadata.name.clone();
                if !theme.metadata.description.is_empty() {
                    line.push_str(&format!(" — {}", theme.metadata.description));
                }
                let style = if idx == self.index {
                    Style::default().fg(Color::Cyan)
                } else {
                    Style::default()
                };
                ListItem::new(Line::from(Span::styled(line, style)))
            })
            .collect();

        let list =
            List::new(list_items).block(Block::default().title("Themes").borders(Borders::ALL));
        frame.render_widget(list, chunks[0]);

        let preview = render_terminal(self.current(), chunks[1]);
        frame.render_widget(preview, chunks[1]);

        let footer = Paragraph::new(
            self.message
                .clone()
                .unwrap_or_else(|| "Press q to exit".into()),
        )
        .block(Block::default().borders(Borders::ALL).title("Status"));
        frame.render_widget(footer, chunks[2]);
    }

    fn next(&mut self) {
        self.index = (self.index + 1) % self.themes.len();
    }

    fn prev(&mut self) {
        if self.index == 0 {
            self.index = self.themes.len() - 1;
        } else {
            self.index -= 1;
        }
    }

    fn current(&self) -> &Theme {
        &self.themes[self.index]
    }
}
