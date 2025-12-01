use std::env;
use std::io::{self, Stdout};
use std::path::Path;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Tabs};
use ratatui::Terminal;
use serde::Deserialize;

use crate::core::apply::Shell;
use crate::core::loader::ThemeCatalogEntry;
use crate::core::theme::Theme;
use crate::error::{PrismError, PrismResult};
use crate::tui::components::terminal_frame::render_terminal;
use crate::tui::editor;

const COMMUNITY_STUB: &str = include_str!("../../docs/gallery-stub.json");

pub fn run_preview(themes: Vec<ThemeCatalogEntry>) -> PrismResult<()> {
    if themes.is_empty() {
        return Err(PrismError::new("no themes available for preview"));
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let result = GalleryApp::new(themes).run(&mut terminal);
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}

struct GalleryApp {
    entries: Vec<ThemeCatalogEntry>,
    community: Vec<CommunityEntry>,
    tab: GalleryTab,
    selected: usize,
    message: Option<String>,
    search_query: String,
    search_mode: bool,
    shell: Shell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GalleryTab {
    Local,
    Community,
}

impl GalleryTab {
    fn titles() -> [&'static str; 2] {
        ["Local", "Community"]
    }

    fn index(self) -> usize {
        match self {
            GalleryTab::Local => 0,
            GalleryTab::Community => 1,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommunityEntry {
    name: String,
    author: String,
    description: String,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    source: Option<String>,
}

#[derive(Clone, Debug, Default)]
struct GalleryFilter {
    name_terms: Vec<String>,
    tag_terms: Vec<String>,
    author_terms: Vec<String>,
    general_terms: Vec<String>,
}

impl GalleryFilter {
    fn parse(query: &str) -> Self {
        let mut filter = Self::default();
        for token in query.split_whitespace() {
            let trimmed = token.trim();
            if trimmed.is_empty() {
                continue;
            }
            if let Some((field, value)) = trimmed.split_once(':') {
                let normalized_value = value.trim();
                if normalized_value.is_empty() {
                    continue;
                }
                let normalized_value = normalized_value.to_lowercase();
                match field.to_ascii_lowercase().as_str() {
                    "name" => filter.name_terms.push(normalized_value),
                    "author" => filter.author_terms.push(normalized_value),
                    "tag" | "tags" => filter.tag_terms.push(normalized_value),
                    _ => filter.general_terms.push(trimmed.to_lowercase()),
                }
            } else {
                filter.general_terms.push(trimmed.to_lowercase());
            }
        }
        filter
    }

    fn matches_theme(&self, theme: &Theme) -> bool {
        if self.is_empty() {
            return true;
        }
        if !self.general_terms.is_empty() {
            let haystack = format!(
                "{} {} {} {} {}",
                theme.metadata.name,
                theme.metadata.author,
                theme.metadata.description,
                theme.metadata.version,
                theme.metadata.tags.join(" ")
            )
            .to_lowercase();
            if self
                .general_terms
                .iter()
                .any(|term| !haystack.contains(term))
            {
                return false;
            }
        }
        let name = theme.metadata.name.to_lowercase();
        if self.name_terms.iter().any(|term| !name.contains(term)) {
            return false;
        }
        let author = theme.metadata.author.to_lowercase();
        if self.author_terms.iter().any(|term| !author.contains(term)) {
            return false;
        }
        if !self.tag_terms.is_empty() {
            let lower_tags: Vec<String> = theme
                .metadata
                .tags
                .iter()
                .map(|tag| tag.to_lowercase())
                .collect();
            for term in &self.tag_terms {
                if !lower_tags.iter().any(|tag| tag.contains(term)) {
                    return false;
                }
            }
        }
        true
    }

    fn matches_community(&self, entry: &CommunityEntry) -> bool {
        if self.is_empty() {
            return true;
        }
        if !self.general_terms.is_empty() {
            let haystack = format!(
                "{} {} {} {}",
                entry.name,
                entry.author,
                entry.description,
                entry.tags.join(" ")
            )
            .to_lowercase();
            if self
                .general_terms
                .iter()
                .any(|term| !haystack.contains(term))
            {
                return false;
            }
        }
        let name = entry.name.to_lowercase();
        if self.name_terms.iter().any(|term| !name.contains(term)) {
            return false;
        }
        let author = entry.author.to_lowercase();
        if self.author_terms.iter().any(|term| !author.contains(term)) {
            return false;
        }
        if !self.tag_terms.is_empty() {
            let lower_tags: Vec<String> = entry.tags.iter().map(|tag| tag.to_lowercase()).collect();
            for term in &self.tag_terms {
                if !lower_tags.iter().any(|tag| tag.contains(term)) {
                    return false;
                }
            }
        }
        true
    }

    fn is_empty(&self) -> bool {
        self.name_terms.is_empty()
            && self.tag_terms.is_empty()
            && self.author_terms.is_empty()
            && self.general_terms.is_empty()
    }
}

impl GalleryApp {
    fn new(entries: Vec<ThemeCatalogEntry>) -> Self {
        let community = serde_json::from_str(COMMUNITY_STUB).unwrap_or_default();
        Self {
            entries,
            community,
            tab: GalleryTab::Local,
            selected: 0,
            message: Some("Press Tab to switch to Community gallery".into()),
            search_query: String::new(),
            search_mode: false,
            shell: detect_shell(),
        }
    }

    fn run(mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> PrismResult<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            if event::poll(Duration::from_millis(200))? {
                match event::read()? {
                    Event::Key(key) => {
                        if self.handle_key(key.code, key.modifiers)? {
                            break;
                        }
                    }
                    Event::Resize(_, _) => {}
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, code: KeyCode, modifiers: KeyModifiers) -> PrismResult<bool> {
        match code {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Esc => {
                if self.search_mode {
                    self.search_mode = false;
                    self.message = Some("Exited filter mode".into());
                    return Ok(false);
                }
                return Ok(true);
            }
            KeyCode::Tab => {
                self.toggle_tab();
            }
            KeyCode::Char('/') => {
                self.search_mode = true;
                self.search_query.clear();
                self.message =
                    Some("Filter (use name:, author:, tag: prefixes) then Enter to lock".into());
            }
            KeyCode::Char('l') => {
                self.set_tab(GalleryTab::Local);
            }
            KeyCode::Char('c') => {
                self.set_tab(GalleryTab::Community);
            }
            KeyCode::Char('j') | KeyCode::Down => {
                self.next();
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.prev();
            }
            KeyCode::Char('a') | KeyCode::Char('A') | KeyCode::Char('e') => {
                self.apply_selected_theme();
            }
            KeyCode::Char('E') => {
                self.edit_selected_theme()?;
            }
            KeyCode::Enter => {
                if self.search_mode {
                    self.search_mode = false;
                    self.message = Some(
                        "Filter locked. Use prefixes name:/tag:/author: and press Enter to apply"
                            .into(),
                    );
                } else {
                    self.apply_selected_theme();
                }
            }
            KeyCode::Backspace => {
                if self.search_mode && !self.search_query.is_empty() {
                    self.search_query.pop();
                    self.message = Some(format!("filter: {query}", query = self.search_query));
                    self.clamp_selection();
                }
            }
            KeyCode::Char(ch) => {
                if self.search_mode && !modifiers.contains(KeyModifiers::CONTROL) {
                    self.search_query.push(ch);
                    self.message = Some(format!("filter: {query}", query = self.search_query));
                    self.clamp_selection();
                }
            }
            _ => {}
        }
        Ok(false)
    }

    fn toggle_tab(&mut self) {
        let next = if self.tab == GalleryTab::Local {
            GalleryTab::Community
        } else {
            GalleryTab::Local
        };
        self.set_tab(next);
    }

    fn set_tab(&mut self, tab: GalleryTab) {
        self.tab = tab;
        self.selected = 0;
        self.message = Some(match tab {
            GalleryTab::Local => "Browsing local themes".into(),
            GalleryTab::Community => "Community gallery (preview only)".into(),
        });
        self.clamp_selection();
    }

    fn next(&mut self) {
        let len = self.filtered_len();
        if len == 0 {
            self.selected = 0;
            return;
        }
        self.selected = (self.selected + 1) % len;
    }

    fn prev(&mut self) {
        let len = self.filtered_len();
        if len == 0 {
            self.selected = 0;
            return;
        }
        if self.selected == 0 {
            self.selected = len - 1;
        } else {
            self.selected -= 1;
        }
    }

    fn clamp_selection(&mut self) {
        let len = self.filtered_len();
        if len == 0 {
            self.selected = 0;
        } else if self.selected >= len {
            self.selected = len - 1;
        }
    }

    fn filtered_len(&self) -> usize {
        match self.tab {
            GalleryTab::Local => self.filtered_local_indices().len(),
            GalleryTab::Community => self.filtered_community_indices().len(),
        }
    }

    fn filtered_local_indices(&self) -> Vec<usize> {
        let filter = GalleryFilter::parse(&self.search_query);
        self.entries
            .iter()
            .enumerate()
            .filter(|(_, entry)| filter.matches_theme(&entry.theme))
            .map(|(idx, _)| idx)
            .collect()
    }

    fn filtered_community_indices(&self) -> Vec<usize> {
        let filter = GalleryFilter::parse(&self.search_query);
        self.community
            .iter()
            .enumerate()
            .filter(|(_, entry)| filter.matches_community(entry))
            .map(|(idx, _)| idx)
            .collect()
    }

    fn current_theme(&self) -> Option<&Theme> {
        let indices = self.filtered_local_indices();
        indices
            .get(self.selected)
            .map(|idx| &self.entries[*idx].theme)
    }

    fn current_community(&self) -> Option<&CommunityEntry> {
        let indices = self.filtered_community_indices();
        indices.get(self.selected).map(|idx| &self.community[*idx])
    }

    fn selected_local_index(&self) -> Option<usize> {
        self.filtered_local_indices().get(self.selected).copied()
    }

    fn display_row(&self) -> usize {
        let len = self.filtered_len();
        if len == 0 {
            0
        } else {
            let clamped = self.selected.min(len - 1);
            clamped + 1
        }
    }

    fn apply_selected_theme(&mut self) {
        if self.tab != GalleryTab::Local {
            self.message = Some("Community themes are preview-only (apply coming soon)".into());
            return;
        }
        if let Some(idx) = self.selected_local_index() {
            let entry = &self.entries[idx];
            let name = entry.theme.metadata.name.clone();
            match entry.theme.apply(self.shell) {
                Ok(_) => {
                    self.message = Some(format!("Applied '{name}' for {:?}", self.shell));
                }
                Err(err) => {
                    log::warn!("Failed to apply theme {name}: {err}");
                    self.message = Some(format!("Failed to apply {name}: {err}"));
                }
            }
        } else {
            self.message = Some("No local theme selected".into());
        }
    }

    fn edit_selected_theme(&mut self) -> PrismResult<()> {
        if self.tab != GalleryTab::Local {
            self.message = Some("Community themes are preview-only (editor coming soon)".into());
            return Ok(());
        }
        if let Some(idx) = self.selected_local_index() {
            let path = self.entries[idx].path.clone();
            let name = self.entries[idx].theme.metadata.name.clone();
            let theme_clone = self.entries[idx].theme.clone();
            editor::edit_theme(theme_clone, &path)?;
            let updated = Theme::load(&path)?;
            if let Some(entry) = self.entries.get_mut(idx) {
                entry.theme = updated;
            }
            self.message = Some(format!("Reloaded '{name}' after editing"));
        } else {
            self.message = Some("No local theme selected".into());
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut ratatui::Frame<'_>) {
        let outer_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(8),
                Constraint::Length(3),
            ])
            .split(frame.size());

        let header_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(24), Constraint::Min(20)])
            .split(outer_chunks[0]);

        let tabs = Tabs::new(
            GalleryTab::titles()
                .iter()
                .map(|label| Line::from(Span::raw(*label)))
                .collect::<Vec<_>>(),
        )
        .select(self.tab.index())
        .block(Block::default().borders(Borders::ALL).title("Gallery"))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );
        frame.render_widget(tabs, header_chunks[0]);

        let search_label = if self.search_mode {
            format!("/ {query}_", query = self.search_query)
        } else if self.search_query.is_empty() {
            "Filter: (press / to start)".into()
        } else {
            format!("Filter: {query}", query = self.search_query)
        };
        let search_paragraph = Paragraph::new(search_label)
            .block(Block::default().borders(Borders::ALL).title("Search"));
        frame.render_widget(search_paragraph, header_chunks[1]);

        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(42), Constraint::Min(30)])
            .split(outer_chunks[1]);

        let list_title = match self.tab {
            GalleryTab::Local => format!(
                "Local Themes ({}/{})",
                self.display_row(),
                self.filtered_len()
            ),
            GalleryTab::Community => format!(
                "Community Picks ({}/{})",
                self.display_row(),
                self.filtered_len()
            ),
        };
        let list_block = Block::default().borders(Borders::ALL).title(list_title);
        let list_items = match self.tab {
            GalleryTab::Local => self
                .filtered_local_indices()
                .iter()
                .enumerate()
                .map(|(row, idx)| self.theme_list_item(*idx, row == self.selected))
                .collect::<Vec<_>>(),
            GalleryTab::Community => self
                .filtered_community_indices()
                .iter()
                .enumerate()
                .map(|(row, idx)| self.community_list_item(*idx, row == self.selected))
                .collect::<Vec<_>>(),
        };
        let list = List::new(list_items).block(list_block).highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Cyan),
        );
        frame.render_widget(list, content_chunks[0]);

        let preview_block = Block::default().borders(Borders::ALL).title("Live Preview");
        let preview_area = content_chunks[1];
        if let Some(theme) = self.current_theme() {
            let preview = render_terminal(theme, preview_area);
            frame.render_widget(preview, preview_area);
        } else if let Some(entry) = self.current_community() {
            let mut lines = vec![
                Line::from(Span::styled(
                    entry.name.clone(),
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(Span::raw(format!("by {}", entry.author))),
                Line::from(Span::raw(entry.description.clone())),
            ];
            if !entry.tags.is_empty() {
                lines.push(Line::from(Span::styled(
                    format!("tags: {}", entry.tags.join(", ")),
                    Style::default().fg(Color::Yellow),
                )));
            }
            if let Some(source) = &entry.source {
                lines.push(Line::from(Span::styled(
                    format!("source: {source}"),
                    Style::default().fg(Color::Green),
                )));
            }
            let paragraph = Paragraph::new(Text::from(lines)).block(preview_block);
            frame.render_widget(paragraph, preview_area);
        } else {
            let paragraph = Paragraph::new(Text::from("No themes match the current filter."))
                .block(preview_block);
            frame.render_widget(paragraph, preview_area);
        }

        let footer_msg = self.message.clone().unwrap_or_else(|| {
            "q quit • Tab switch • / filter (name:/tag:/author:) • j/k navigate • Enter/a/e apply • E edit"
                .into()
        });
        let footer = Paragraph::new(Text::from(footer_msg))
            .block(Block::default().borders(Borders::ALL).title("Status"));
        frame.render_widget(footer, outer_chunks[2]);
    }

    fn theme_list_item(&self, idx: usize, selected: bool) -> ListItem<'_> {
        let entry = &self.entries[idx];
        let theme = &entry.theme;
        let mut lines = Vec::new();
        let title = Line::from(vec![
            Span::styled(
                &theme.metadata.name,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!(" v{}", theme.metadata.version)),
        ]);
        lines.push(title);
        let mut caption = format!("by {}", theme.metadata.author);
        if !theme.metadata.description.is_empty() {
            caption.push_str(" — ");
            caption.push_str(&theme.metadata.description);
        }
        if !theme.metadata.tags.is_empty() {
            caption.push_str(" [");
            caption.push_str(&theme.metadata.tags.join(", "));
            caption.push(']');
        }
        lines.push(Line::from(Span::styled(
            caption,
            Style::default().fg(Color::Gray),
        )));
        let style = if selected {
            Style::default().bg(Color::DarkGray)
        } else {
            Style::default()
        };
        ListItem::new(lines).style(style)
    }

    fn community_list_item(&self, idx: usize, selected: bool) -> ListItem<'_> {
        let entry = &self.community[idx];
        let mut lines = Vec::new();
        lines.push(Line::from(Span::styled(
            &entry.name,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::raw(format!("by {}", entry.author))));
        lines.push(Line::from(Span::raw(entry.description.clone())));
        if !entry.tags.is_empty() {
            lines.push(Line::from(Span::styled(
                format!("tags: {}", entry.tags.join(", ")),
                Style::default().fg(Color::Yellow),
            )));
        }
        let style = if selected {
            Style::default().bg(Color::DarkGray)
        } else {
            Style::default()
        };
        ListItem::new(lines).style(style)
    }
}

fn detect_shell() -> Shell {
    if let Ok(value) = env::var("PRISM_PREVIEW_SHELL") {
        if let Some(shell) = parse_shell_hint(&value) {
            return shell;
        }
    }
    if let Ok(value) = env::var("SHELL") {
        if let Some(shell) = parse_shell_hint(&value) {
            return shell;
        }
    }
    Shell::Zsh
}

fn parse_shell_hint(value: &str) -> Option<Shell> {
    let leaf = Path::new(value)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(value)
        .trim_start_matches('.');
    match leaf.to_ascii_lowercase().as_str() {
        "zsh" => Some(Shell::Zsh),
        "bash" => Some(Shell::Bash),
        "fish" => Some(Shell::Fish),
        _ => None,
    }
}

pub mod bench_support {
    use super::*;
    use crate::core::color::ColorPalette;
    use crate::core::theme::{ContextRules, PromptConfig, ThemeMetadata, WidgetConfig};

    pub fn make_theme(name: &str, author: &str, tags: &[&str]) -> Theme {
        Theme {
            metadata: ThemeMetadata {
                name: name.into(),
                author: author.into(),
                version: "1.0.0".into(),
                description: String::new(),
                tags: tags.iter().map(|tag| tag.to_string()).collect(),
            },
            colors: ColorPalette::default(),
            prompt: PromptConfig::default(),
            widgets: WidgetConfig::default(),
            context_rules: ContextRules::default(),
        }
    }

    pub fn make_community_entry(name: &str, author: &str, tags: &[&str]) -> CommunityEntry {
        CommunityEntry {
            name: name.into(),
            author: author.into(),
            description: String::new(),
            tags: tags.iter().map(|tag| tag.to_string()).collect(),
            source: None,
        }
    }

    pub fn sample_themes(count: usize) -> Vec<Theme> {
        (0..count)
            .map(|idx| {
                let name = format!("Theme-{idx:04}");
                let author = format!("Author-{}", idx % 7);
                let tags = match idx % 3 {
                    0 => vec!["neon", "glow"],
                    1 => vec!["minimal", "mono"],
                    _ => vec!["solar", "warm"],
                };
                make_theme(&name, &author, &tags)
            })
            .collect()
    }

    pub fn sample_community(count: usize) -> Vec<CommunityEntry> {
        (0..count)
            .map(|idx| {
                let name = format!("Community-{idx:04}");
                let author = format!("Contributor-{}", idx % 5);
                let tags = match idx % 2 {
                    0 => vec!["purple", "beta"],
                    _ => vec!["green", "stable"],
                };
                make_community_entry(&name, &author, &tags)
            })
            .collect()
    }

    #[derive(Clone)]
    pub struct CompiledGalleryFilter(GalleryFilter);

    pub fn compile_filter(query: &str) -> CompiledGalleryFilter {
        CompiledGalleryFilter(GalleryFilter::parse(query))
    }

    pub fn count_theme_matches_compiled(filter: &CompiledGalleryFilter, themes: &[Theme]) -> usize {
        themes
            .iter()
            .filter(|theme| filter.0.matches_theme(theme))
            .count()
    }

    pub fn count_community_matches_compiled(
        filter: &CompiledGalleryFilter,
        entries: &[CommunityEntry],
    ) -> usize {
        entries
            .iter()
            .filter(|entry| filter.0.matches_community(entry))
            .count()
    }

    pub fn count_theme_matches(query: &str, themes: &[Theme]) -> usize {
        let filter = compile_filter(query);
        count_theme_matches_compiled(&filter, themes)
    }

    pub fn count_community_matches(query: &str, entries: &[CommunityEntry]) -> usize {
        let filter = compile_filter(query);
        count_community_matches_compiled(&filter, entries)
    }
}

#[cfg(test)]
mod tests {
    use super::bench_support::{make_community_entry, make_theme};
    use super::*;
    use crate::core::loader;
    use crate::error::PrismResult;
    use std::collections::HashSet;

    #[test]
    fn filter_matches_specific_fields() {
        let filter = GalleryFilter::parse("name:aurora author:wolf tag:neon");
        let matching = make_theme("Aurora Borealis", "GlassWolf", &["neon", "green"]);
        let nonmatching = make_theme("Velvet Midnight", "Nova", &["purple"]);
        assert!(filter.matches_theme(&matching));
        assert!(!filter.matches_theme(&nonmatching));
    }

    #[test]
    fn filter_handles_general_terms_for_community() {
        let filter = GalleryFilter::parse("velvet author:nova");
        let entry = make_community_entry("Velvet Midnight", "Nova", &["purple"]);
        let other = make_community_entry("Solar Flare", "Sol", &["orange"]);
        assert!(filter.matches_community(&entry));
        assert!(!filter.matches_community(&other));
    }

    #[test]
    fn gallery_filter_matches_tag_queries() {
        let filter = GalleryFilter::parse("tag:neon");
        let with_tag = make_theme("Aurora", "Nova", &["neon", "glow"]);
        let without = make_theme("Plain", "Nova", &["minimal"]);
        assert!(filter.matches_theme(&with_tag));
        assert!(!filter.matches_theme(&without));
    }

    #[test]
    fn parse_shell_hint_detects_shells() {
        assert_eq!(parse_shell_hint("/bin/bash"), Some(Shell::Bash));
        assert_eq!(parse_shell_hint("fish"), Some(Shell::Fish));
        assert_eq!(parse_shell_hint("custom"), None);
    }

    #[test]
    fn gallery_reflects_loader_catalog() -> PrismResult<()> {
        let entries = loader::list_available()?;
        let loader_names: HashSet<String> = entries
            .iter()
            .map(|entry| entry.theme.metadata.name.clone())
            .collect();
        let app = GalleryApp::new(entries.clone());
        let gallery_names: HashSet<String> = app
            .entries
            .iter()
            .map(|entry| entry.theme.metadata.name.clone())
            .collect();
        for name in loader_names {
            assert!(
                gallery_names.contains(&name),
                "gallery missing catalog entry {}",
                name
            );
        }
        Ok(())
    }
}
