use std::io::{self, Stdout};
use std::path::{Path, PathBuf};
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Terminal;

use crate::core::apply::Shell;
use crate::core::color::ColorPalette;
use crate::core::theme::{PromptConfig, Theme};
use crate::error::{PrismError, PrismResult};
use crate::tui::components::terminal_frame::render_terminal;

pub fn edit_theme(theme: Theme, destination: &Path) -> PrismResult<()> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let result = ThemeEditorApp::new(theme, destination.to_path_buf()).run(&mut terminal);
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}

struct ThemeEditorApp {
    theme: Theme,
    fields: Vec<FieldKind>,
    selected: usize,
    mode: InputMode,
    input: String,
    message: Option<String>,
    destination: PathBuf,
}

impl ThemeEditorApp {
    fn new(theme: Theme, destination: PathBuf) -> Self {
        let fields = FieldKind::build(&theme);
        Self {
            theme,
            fields,
            selected: 0,
            mode: InputMode::Navigate,
            input: String::new(),
            message: Some("Use j/k to navigate, Enter to edit, s=save, a=apply, q=quit".into()),
            destination,
        }
    }

    fn run(mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> PrismResult<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            if crossterm::event::poll(Duration::from_millis(200))? {
                if let Event::Key(key) = event::read()? {
                    if self.handle_key(key.code)? {
                        break;
                    }
                }
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, code: KeyCode) -> PrismResult<bool> {
        match self.mode {
            InputMode::Navigate => match code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Char('j') | KeyCode::Down => self.next(),
                KeyCode::Char('k') | KeyCode::Up => self.prev(),
                KeyCode::Char('e') | KeyCode::Enter => self.begin_edit(),
                KeyCode::Char('s') => self.save()?,
                KeyCode::Char('a') => self.apply_current_theme()?,
                KeyCode::Char('<') => self.move_selected_segment(-1),
                KeyCode::Char('>') => self.move_selected_segment(1),
                _ => {}
            },
            InputMode::Edit => match code {
                KeyCode::Esc => {
                    self.mode = InputMode::Navigate;
                    self.input.clear();
                }
                KeyCode::Enter => self.commit_input()?,
                KeyCode::Backspace => {
                    self.input.pop();
                }
                KeyCode::Char(ch) => self.input.push(ch),
                _ => {}
            },
        }
        Ok(false)
    }

    fn draw(&mut self, frame: &mut ratatui::Frame<'_>) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(10),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(frame.size());

        let body = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(45), Constraint::Percentage(55)].as_ref())
            .split(layout[1]);

        frame.render_widget(self.instructions(), layout[0]);
        frame.render_widget(self.field_list(), body[0]);
        frame.render_widget(self.preview(body[1]), body[1]);
        frame.render_widget(self.status_bar(), layout[2]);
    }

    fn instructions(&self) -> Paragraph<'static> {
        let mut text = match self.mode {
            InputMode::Navigate => {
                "Navigate: j/k or arrows • Enter/e=edit • s=save • a=apply • q=quit".to_string()
            }
            InputMode::Edit => "Editing… type value, Enter=commit, Esc=cancel".to_string(),
        };
        if matches!(
            self.current_field(),
            Some(FieldKind::SegmentField { .. }) if self.mode == InputMode::Navigate
        ) {
            text.push_str(" • < / > reorder segments");
        }
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Controls"))
    }

    fn field_list(&self) -> List<'_> {
        let items: Vec<_> = self
            .fields
            .iter()
            .enumerate()
            .map(|(idx, field)| {
                let mut line = format!("{}: {}", field.label(), field.value(&self.theme));
                if self.mode == InputMode::Edit && idx == self.selected {
                    line.push_str(&format!(" → {}", self.input));
                }
                let mut style = Style::default();
                if idx == self.selected {
                    style = style.fg(Color::Cyan).add_modifier(Modifier::BOLD);
                }
                ListItem::new(Line::from(Span::styled(line, style)))
            })
            .collect();

        List::new(items).block(
            Block::default()
                .title(format!(
                    "Fields ({} saved at {})",
                    self.theme.metadata.name,
                    self.destination.display()
                ))
                .borders(Borders::ALL),
        )
    }

    fn preview(&self, area: Rect) -> Paragraph<'static> {
        let mut preview = render_terminal(&self.theme, area);
        preview = preview.block(
            Block::default()
                .title("Preview")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Gray)),
        );
        preview
    }

    fn status_bar(&self) -> Paragraph<'static> {
        let message = self.message.clone().unwrap_or_else(|| "Ready.".into());
        Paragraph::new(message).block(Block::default().borders(Borders::ALL).title("Status"))
    }

    fn current_field(&self) -> Option<&FieldKind> {
        self.fields.get(self.selected)
    }

    fn selected_segment_name(&self) -> Option<String> {
        match self.current_field() {
            Some(FieldKind::SegmentField { segment, .. }) => Some(segment.clone()),
            _ => None,
        }
    }

    fn move_selected_segment(&mut self, delta: isize) {
        if let Some(segment) = self.selected_segment_name() {
            if self.theme.prompt.move_segment(&segment, delta) {
                self.rebuild_fields(Some(segment));
            }
        }
    }

    fn rebuild_fields(&mut self, focus: Option<String>) {
        self.fields = FieldKind::build(&self.theme);
        if let Some(name) = focus {
            if let Some(idx) = self.fields.iter().position(|field| {
                matches!(field, FieldKind::SegmentField { segment, .. } if segment == &name)
            }) {
                self.selected = idx;
            }
        }
    }

    fn next(&mut self) {
        if self.fields.is_empty() {
            return;
        }
        self.selected = (self.selected + 1) % self.fields.len();
    }

    fn prev(&mut self) {
        if self.fields.is_empty() {
            return;
        }
        if self.selected == 0 {
            self.selected = self.fields.len() - 1;
        } else {
            self.selected -= 1;
        }
    }

    fn begin_edit(&mut self) {
        if let Some(field) = self.fields.get(self.selected) {
            self.input = field.value(&self.theme);
            self.mode = InputMode::Edit;
        }
    }

    fn commit_input(&mut self) -> PrismResult<()> {
        if let Some(field) = self.fields.get(self.selected).cloned() {
            let trimmed = self.input.trim().to_string();
            field.validate(&trimmed).map_err(PrismError::new)?;
            field.set_value(&mut self.theme, trimmed);
            self.mode = InputMode::Navigate;
            self.input.clear();
            self.message = Some(format!("Updated {}", field.label()));
        }
        Ok(())
    }

    fn save(&mut self) -> PrismResult<()> {
        if let Some(parent) = self.destination.parent() {
            std::fs::create_dir_all(parent)?;
        }
        self.theme.validate()?;
        let payload = toml::to_string_pretty(&self.theme)?;
        std::fs::write(&self.destination, payload)?;
        self.message = Some(format!("Saved theme to {}", self.destination.display()));
        Ok(())
    }

    fn apply_current_theme(&mut self) -> PrismResult<()> {
        self.theme.apply(Shell::Zsh)?;
        self.message = Some("Applied theme to Zsh (use --shell to apply via CLI)".into());
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq)]
enum InputMode {
    Navigate,
    Edit,
}

#[derive(Clone)]
enum FieldKind {
    MetadataName,
    MetadataAuthor,
    MetadataDescription,
    PromptSeparator,
    Color(ColorField),
    PromptFlag(PromptFlagField),
    SegmentField {
        segment: String,
        property: SegmentProperty,
    },
}

impl FieldKind {
    fn build(theme: &Theme) -> Vec<Self> {
        let mut fields = vec![
            FieldKind::MetadataName,
            FieldKind::MetadataAuthor,
            FieldKind::MetadataDescription,
            FieldKind::PromptSeparator,
            FieldKind::PromptFlag(PromptFlagField::User),
            FieldKind::PromptFlag(PromptFlagField::Host),
            FieldKind::PromptFlag(PromptFlagField::Time),
            FieldKind::PromptFlag(PromptFlagField::Git),
        ];
        fields.extend(ColorField::all().into_iter().map(FieldKind::Color));
        for name in theme.prompt.segments.keys() {
            fields.push(FieldKind::SegmentField {
                segment: name.clone(),
                property: SegmentProperty::Bg,
            });
            fields.push(FieldKind::SegmentField {
                segment: name.clone(),
                property: SegmentProperty::Fg,
            });
            fields.push(FieldKind::SegmentField {
                segment: name.clone(),
                property: SegmentProperty::Icon,
            });
        }
        fields
    }

    fn label(&self) -> String {
        match self {
            FieldKind::MetadataName => "metadata.name".into(),
            FieldKind::MetadataAuthor => "metadata.author".into(),
            FieldKind::MetadataDescription => "metadata.description".into(),
            FieldKind::PromptSeparator => "prompt.separator".into(),
            FieldKind::Color(field) => field.label().into(),
            FieldKind::PromptFlag(flag) => flag.label().into(),
            FieldKind::SegmentField { segment, property } => property.label(segment),
        }
    }

    fn value(&self, theme: &Theme) -> String {
        match self {
            FieldKind::MetadataName => theme.metadata.name.clone(),
            FieldKind::MetadataAuthor => theme.metadata.author.clone(),
            FieldKind::MetadataDescription => theme.metadata.description.clone(),
            FieldKind::PromptSeparator => theme.prompt.separator.clone(),
            FieldKind::Color(field) => field.value(&theme.colors).to_string(),
            FieldKind::PromptFlag(flag) => flag.value(&theme.prompt).to_string(),
            FieldKind::SegmentField { segment, property } => property.value(segment, theme),
        }
    }

    fn set_value(&self, theme: &mut Theme, value: String) {
        match self {
            FieldKind::MetadataName => theme.metadata.name = value,
            FieldKind::MetadataAuthor => theme.metadata.author = value,
            FieldKind::MetadataDescription => theme.metadata.description = value,
            FieldKind::PromptSeparator => theme.prompt.separator = value,
            FieldKind::Color(field) => field.set_value(&mut theme.colors, value),
            FieldKind::PromptFlag(flag) => flag.set_value(&mut theme.prompt, value),
            FieldKind::SegmentField { segment, property } => {
                property.set_value(segment, theme, value)
            }
        }
    }

    fn validate(&self, value: &str) -> Result<(), &'static str> {
        match self {
            FieldKind::MetadataName | FieldKind::MetadataAuthor => {
                if value.trim().is_empty() {
                    Err("Value cannot be empty")
                } else {
                    Ok(())
                }
            }
            FieldKind::PromptSeparator => {
                if value.trim().is_empty() {
                    Err("Separator cannot be empty")
                } else {
                    Ok(())
                }
            }
            FieldKind::MetadataDescription => Ok(()),
            FieldKind::Color(_) => {
                if is_hex_color(value) {
                    Ok(())
                } else {
                    Err("Expected hex color like #aabbcc")
                }
            }
            FieldKind::PromptFlag(_) => {
                if matches_ignore_case(value, "true") || matches_ignore_case(value, "false") {
                    Ok(())
                } else {
                    Err("Expected true or false")
                }
            }
            FieldKind::SegmentField { property, .. } => match property {
                SegmentProperty::Icon => Ok(()),
                _ => {
                    if is_hex_color(value) {
                        Ok(())
                    } else {
                        Err("Expected hex color like #aabbcc")
                    }
                }
            },
        }
    }
}

#[derive(Clone, Copy)]
enum ColorField {
    Background,
    Foreground,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl ColorField {
    fn all() -> Vec<Self> {
        vec![
            Self::Background,
            Self::Foreground,
            Self::Black,
            Self::Red,
            Self::Green,
            Self::Yellow,
            Self::Blue,
            Self::Magenta,
            Self::Cyan,
            Self::White,
            Self::BrightBlack,
            Self::BrightRed,
            Self::BrightGreen,
            Self::BrightYellow,
            Self::BrightBlue,
            Self::BrightMagenta,
            Self::BrightCyan,
            Self::BrightWhite,
        ]
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Background => "colors.background",
            Self::Foreground => "colors.foreground",
            Self::Black => "colors.black",
            Self::Red => "colors.red",
            Self::Green => "colors.green",
            Self::Yellow => "colors.yellow",
            Self::Blue => "colors.blue",
            Self::Magenta => "colors.magenta",
            Self::Cyan => "colors.cyan",
            Self::White => "colors.white",
            Self::BrightBlack => "colors.bright.black",
            Self::BrightRed => "colors.bright.red",
            Self::BrightGreen => "colors.bright.green",
            Self::BrightYellow => "colors.bright.yellow",
            Self::BrightBlue => "colors.bright.blue",
            Self::BrightMagenta => "colors.bright.magenta",
            Self::BrightCyan => "colors.bright.cyan",
            Self::BrightWhite => "colors.bright.white",
        }
    }

    fn value<'a>(&self, palette: &'a ColorPalette) -> &'a str {
        match self {
            Self::Background => &palette.background,
            Self::Foreground => &palette.foreground,
            Self::Black => &palette.black,
            Self::Red => &palette.red,
            Self::Green => &palette.green,
            Self::Yellow => &palette.yellow,
            Self::Blue => &palette.blue,
            Self::Magenta => &palette.magenta,
            Self::Cyan => &palette.cyan,
            Self::White => &palette.white,
            Self::BrightBlack => &palette.bright.black,
            Self::BrightRed => &palette.bright.red,
            Self::BrightGreen => &palette.bright.green,
            Self::BrightYellow => &palette.bright.yellow,
            Self::BrightBlue => &palette.bright.blue,
            Self::BrightMagenta => &palette.bright.magenta,
            Self::BrightCyan => &palette.bright.cyan,
            Self::BrightWhite => &palette.bright.white,
        }
    }

    fn set_value(&self, palette: &mut ColorPalette, value: String) {
        match self {
            Self::Background => palette.background = value,
            Self::Foreground => palette.foreground = value,
            Self::Black => palette.black = value,
            Self::Red => palette.red = value,
            Self::Green => palette.green = value,
            Self::Yellow => palette.yellow = value,
            Self::Blue => palette.blue = value,
            Self::Magenta => palette.magenta = value,
            Self::Cyan => palette.cyan = value,
            Self::White => palette.white = value,
            Self::BrightBlack => palette.bright.black = value,
            Self::BrightRed => palette.bright.red = value,
            Self::BrightGreen => palette.bright.green = value,
            Self::BrightYellow => palette.bright.yellow = value,
            Self::BrightBlue => palette.bright.blue = value,
            Self::BrightMagenta => palette.bright.magenta = value,
            Self::BrightCyan => palette.bright.cyan = value,
            Self::BrightWhite => palette.bright.white = value,
        }
    }
}

#[derive(Clone, Copy)]
enum PromptFlagField {
    User,
    Host,
    Time,
    Git,
}

impl PromptFlagField {
    fn label(&self) -> &'static str {
        match self {
            Self::User => "prompt.show_user",
            Self::Host => "prompt.show_host",
            Self::Time => "prompt.show_time",
            Self::Git => "prompt.show_git",
        }
    }

    fn value(&self, config: &PromptConfig) -> bool {
        match self {
            Self::User => config.show_user,
            Self::Host => config.show_host,
            Self::Time => config.show_time,
            Self::Git => config.show_git,
        }
    }

    fn set_value(&self, config: &mut PromptConfig, value: String) {
        let flag = matches_ignore_case(&value, "true");
        match self {
            Self::User => config.show_user = flag,
            Self::Host => config.show_host = flag,
            Self::Time => config.show_time = flag,
            Self::Git => config.show_git = flag,
        }
    }
}

#[derive(Clone, Copy)]
enum SegmentProperty {
    Bg,
    Fg,
    Icon,
}

impl SegmentProperty {
    fn label(&self, name: &str) -> String {
        match self {
            SegmentProperty::Bg => format!("prompt.segments.{name}.bg"),
            SegmentProperty::Fg => format!("prompt.segments.{name}.fg"),
            SegmentProperty::Icon => format!("prompt.segments.{name}.icon"),
        }
    }

    fn value(&self, segment: &str, theme: &Theme) -> String {
        let entry = theme
            .prompt
            .segments
            .get(segment)
            .cloned()
            .unwrap_or_default();
        match self {
            SegmentProperty::Bg => entry.bg,
            SegmentProperty::Fg => entry.fg,
            SegmentProperty::Icon => entry.icon.unwrap_or_default(),
        }
    }

    fn set_value(&self, segment: &str, theme: &mut Theme, value: String) {
        let entry = theme
            .prompt
            .segments
            .entry(segment.to_string())
            .or_default();
        match self {
            SegmentProperty::Bg => entry.bg = value,
            SegmentProperty::Fg => entry.fg = value,
            SegmentProperty::Icon => {
                entry.icon = if value.trim().is_empty() {
                    None
                } else {
                    Some(value)
                }
            }
        }
    }
}

fn is_hex_color(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.len() != 7 || !trimmed.starts_with('#') {
        return false;
    }
    trimmed[1..].chars().all(|ch| ch.is_ascii_hexdigit())
}

fn matches_ignore_case(input: &str, expected: &str) -> bool {
    input.eq_ignore_ascii_case(expected)
}
