mod editor;
mod theme;
mod icons;
mod filebrowser;
mod tabs;
mod project;
mod git;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs as RatatuiTabs, Wrap},
    Frame, Terminal,
};
use std::env;
use std::io::{stdout, Stdout};

use theme::{MaterialTheme, ThemeColors};
use icons::MaterialIcons;
use filebrowser::FileBrowser;
use tabs::TabManager;
use project::{Project, FrameworkType};

enum AppMode {
    Normal,
    Insert,
    Command,
}

struct PhpSunshine {
    should_quit: bool,
    mode: AppMode,
    theme: MaterialTheme,
    tab_manager: TabManager,
    file_browser: Option<FileBrowser>,
    show_file_browser: bool,
    project: Option<Project>,
    status_message: String,
    command_input: String,
}

impl PhpSunshine {
    fn new() -> Result<Self> {
        let current_dir = env::current_dir()?;
        let project = Project::detect(&current_dir).ok();
        let file_browser = FileBrowser::new(&current_dir).ok();
        
        let framework_name = project
            .as_ref()
            .map(|p| match p.framework() {
                FrameworkType::PrestaShop => "PrestaShop",
                FrameworkType::Laravel => "Laravel",
                FrameworkType::Symfony => "Symfony",
                FrameworkType::Generic => "PHP",
            })
            .unwrap_or("PHP");
        
        Ok(Self {
            should_quit: false,
            mode: AppMode::Normal,
            theme: MaterialTheme::Ocean,
            tab_manager: TabManager::new(),
            file_browser,
            show_file_browser: true,
            project,
            status_message: format!("PHP Sunshine IDE - {} Project Ready", framework_name),
            command_input: String::new(),
        })
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                self.handle_key(key)?;
            }
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        match self.mode {
            AppMode::Normal => self.handle_normal_mode(key)?,
            AppMode::Insert => self.handle_insert_mode(key)?,
            AppMode::Command => self.handle_command_mode(key)?,
        }
        Ok(())
    }

    fn handle_normal_mode(&mut self, key: KeyEvent) -> Result<()> {
        let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
        
        match key.code {
            KeyCode::Char('q') if !ctrl => self.should_quit = true,
            KeyCode::Char('c') if ctrl => self.should_quit = true,
            KeyCode::Char('i') => {
                self.mode = AppMode::Insert;
                self.status_message = "-- INSERT --".to_string();
            }
            KeyCode::Char(':') => {
                self.mode = AppMode::Command;
                self.command_input.clear();
            }
            KeyCode::Char('e') if ctrl => {
                self.show_file_browser = !self.show_file_browser;
            }
            KeyCode::Char('s') if ctrl => {
                if let Err(e) = self.tab_manager.save_current() {
                    self.status_message = format!("Error saving: {}", e);
                } else {
                    self.status_message = "File saved".to_string();
                }
            }
            KeyCode::Char('t') if ctrl => {
                self.tab_manager.new_tab();
                self.status_message = "New tab created".to_string();
            }
            KeyCode::Char('w') if ctrl => {
                self.tab_manager.close_current();
            }
            KeyCode::Tab if ctrl => {
                self.tab_manager.next_tab();
            }
            KeyCode::BackTab => {
                self.tab_manager.prev_tab();
            }
            // Cursor movement in normal mode
            KeyCode::Up | KeyCode::Char('k') => {
                if self.show_file_browser {
                    if let Some(browser) = &mut self.file_browser {
                        browser.move_up();
                    }
                } else {
                    self.tab_manager.active_buffer().move_cursor_up();
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.show_file_browser {
                    if let Some(browser) = &mut self.file_browser {
                        browser.move_down();
                    }
                } else {
                    self.tab_manager.active_buffer().move_cursor_down();
                }
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.tab_manager.active_buffer().move_cursor_left();
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.tab_manager.active_buffer().move_cursor_right();
            }
            KeyCode::Enter => {
                if self.show_file_browser {
                    if let Some(browser) = &mut self.file_browser {
                        if let Some(entry) = browser.selected_entry() {
                            if entry.is_dir {
                                browser.toggle_selected()?;
                            } else {
                                // Open file
                                if let Err(e) = self.tab_manager.open_file(&entry.path) {
                                    self.status_message = format!("Error opening file: {}", e);
                                } else {
                                    self.status_message = format!("Opened: {}", entry.name);
                                    self.show_file_browser = false;
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_insert_mode(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.mode = AppMode::Normal;
                self.status_message = "Normal mode".to_string();
            }
            KeyCode::Char(c) => {
                self.tab_manager.active_buffer().insert_char(c);
            }
            KeyCode::Enter => {
                self.tab_manager.active_buffer().insert_char('\n');
            }
            KeyCode::Backspace => {
                self.tab_manager.active_buffer().backspace();
            }
            KeyCode::Delete => {
                self.tab_manager.active_buffer().delete_char();
            }
            KeyCode::Up => self.tab_manager.active_buffer().move_cursor_up(),
            KeyCode::Down => self.tab_manager.active_buffer().move_cursor_down(),
            KeyCode::Left => self.tab_manager.active_buffer().move_cursor_left(),
            KeyCode::Right => self.tab_manager.active_buffer().move_cursor_right(),
            _ => {}
        }
        Ok(())
    }

    fn handle_command_mode(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.mode = AppMode::Normal;
                self.command_input.clear();
            }
            KeyCode::Enter => {
                self.execute_command();
                self.mode = AppMode::Normal;
            }
            KeyCode::Char(c) => {
                self.command_input.push(c);
            }
            KeyCode::Backspace => {
                self.command_input.pop();
            }
            _ => {}
        }
        Ok(())
    }

    fn execute_command(&mut self) {
        let command = self.command_input.clone();
        let parts: Vec<&str> = command.split_whitespace().collect();
        self.command_input.clear();
        
        if parts.is_empty() {
            return;
        }

        match parts[0] {
            "q" | "quit" => self.should_quit = true,
            "w" | "write" => {
                if let Err(e) = self.tab_manager.save_current() {
                    self.status_message = format!("Error: {}", e);
                } else {
                    self.status_message = "Saved".to_string();
                }
            }
            "wq" => {
                let _ = self.tab_manager.save_current();
                self.should_quit = true;
            }
            "theme" => {
                if parts.len() > 1 {
                    self.change_theme(parts[1]);
                } else {
                    self.status_message = "Available: ocean, palenight, darker, lighter, deepocean".to_string();
                }
            }
            _ => {
                self.status_message = format!("Unknown command: {}", parts[0]);
            }
        }
    }

    fn change_theme(&mut self, name: &str) {
        let new_theme = match name.to_lowercase().as_str() {
            "ocean" => Some(MaterialTheme::Ocean),
            "palenight" => Some(MaterialTheme::Palenight),
            "darker" => Some(MaterialTheme::Darker),
            "lighter" => Some(MaterialTheme::Lighter),
            "deepocean" | "deep" => Some(MaterialTheme::DeepOcean),
            _ => None,
        };
        
        if let Some(theme) = new_theme {
            self.theme = theme;
            self.status_message = format!("Theme changed to: {}", self.theme.name());
        } else {
            self.status_message = "Invalid theme name".to_string();
        }
    }

    fn render(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        let colors = self.theme.colors();
        
        terminal.draw(|frame| {
            self.render_ui(frame, &colors);
        })?;
        Ok(())
    }

    fn render_ui(&mut self, frame: &mut Frame, colors: &ThemeColors) {
        let size = frame.area();
        
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),   // Header
                Constraint::Length(2),   // Tabs
                Constraint::Min(0),      // Main content
                Constraint::Length(2),   // Status bar
            ])
            .split(size);

        // Render header
        self.render_header(frame, main_chunks[0], colors);
        
        // Render tabs
        self.render_tabs(frame, main_chunks[1], colors);
        
        // Render main content (file browser + editor)
        let content_chunks = if self.show_file_browser {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
                .split(main_chunks[2]);
            chunks.to_vec()
        } else {
            vec![main_chunks[2]]
        };

        if self.show_file_browser && content_chunks.len() > 1 {
            self.render_file_browser(frame, content_chunks[0], colors);
            self.render_editor(frame, content_chunks[1], colors);
        } else {
            self.render_editor(frame, content_chunks[0], colors);
        }
        
        // Render status bar
        self.render_status_bar(frame, main_chunks[3], colors);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect, colors: &ThemeColors) {
        let framework_icon = self.project.as_ref().map(|p| match p.framework() {
            FrameworkType::PrestaShop => MaterialIcons::prestashop(),
            FrameworkType::Laravel => MaterialIcons::laravel(),
            FrameworkType::Symfony => MaterialIcons::symfony(),
            FrameworkType::Generic => "󰌟",
        }).unwrap_or("󰌟");

        let header = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("☀️  PHP Sunshine IDE", Style::default().fg(colors.accent).add_modifier(Modifier::BOLD)),
                Span::raw(" | "),
                Span::raw(framework_icon),
                Span::raw(" "),
                Span::styled(self.project.as_ref().map(|p| match p.framework() {
                    FrameworkType::PrestaShop => "PrestaShop",
                    FrameworkType::Laravel => "Laravel",
                    FrameworkType::Symfony => "Symfony",
                    FrameworkType::Generic => "PHP",
                }).unwrap_or("PHP"), Style::default().fg(colors.accent_light)),
                Span::raw(" | Theme: "),
                Span::styled(self.theme.name(), Style::default().fg(colors.fg_secondary)),
            ]),
        ])
        .style(Style::default().bg(colors.bg_secondary).fg(colors.fg_primary))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(colors.border)));
        
        frame.render_widget(header, area);
    }

    fn render_tabs(&self, frame: &mut Frame, area: Rect, colors: &ThemeColors) {
        let titles: Vec<String> = self.tab_manager.tabs().iter().map(|t| t.title()).collect();
        let tabs = RatatuiTabs::new(titles)
            .block(Block::default().borders(Borders::BOTTOM).border_style(Style::default().fg(colors.border)))
            .select(self.tab_manager.active_index())
            .style(Style::default().bg(colors.bg_secondary).fg(colors.fg_secondary))
            .highlight_style(Style::default().bg(colors.bg_tertiary).fg(colors.accent).add_modifier(Modifier::BOLD));
        
        frame.render_widget(tabs, area);
    }

    fn render_file_browser(&mut self, frame: &mut Frame, area: Rect, colors: &ThemeColors) {
        if let Some(browser) = &mut self.file_browser {
            browser.update_scroll(area.height.saturating_sub(2) as usize);
            let entries = browser.get_visible_entries();
            let selected = browser.selected_index();
            let scroll = browser.scroll_offset();
            
            let items: Vec<ListItem> = entries
                .iter()
                .skip(scroll)
                .take(area.height.saturating_sub(2) as usize)
                .map(|entry| {
                    let icon = if entry.is_dir {
                        if entry.is_expanded {
                            MaterialIcons::folder_open()
                        } else {
                            MaterialIcons::folder_special(&entry.name)
                        }
                    } else {
                        MaterialIcons::for_file(&entry.path)
                    };
                    
                    let indent = "  ".repeat(entry.depth);
                    let text = format!("{}{} {}", indent, icon, entry.name);
                    ListItem::new(text)
                })
                .collect();
            
            let list = List::new(items)
                .block(Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(colors.border))
                    .title(format!(" {} Explorer ", MaterialIcons::folder())))
                .style(Style::default().bg(colors.bg_primary).fg(colors.fg_primary))
                .highlight_style(Style::default().bg(colors.selection).fg(colors.accent).add_modifier(Modifier::BOLD));
            
            frame.render_stateful_widget(list, area, &mut ratatui::widgets::ListState::default().with_selected(Some(selected - scroll)));
        }
    }

    fn render_editor(&mut self, frame: &mut Frame, area: Rect, colors: &ThemeColors) {
        let buffer = self.tab_manager.active_buffer();
        let viewport_height = area.height.saturating_sub(2) as usize;
        buffer.update_scroll(viewport_height);
        
        let scroll = buffer.scroll_offset();
        let line_count = buffer.len_lines();
        
        let mut lines = Vec::new();
        for i in scroll..(scroll + viewport_height).min(line_count) {
            let line_num = format!("{:>4} ", i + 1);
            let line_content = buffer.line(i);
            
            lines.push(Line::from(vec![
                Span::styled(line_num, Style::default().fg(colors.line_number)),
                Span::styled(line_content, Style::default().fg(colors.fg_primary)),
            ]));
        }
        
        let text = Text::from(lines);
        let editor = Paragraph::new(text)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(colors.border))
                .title(format!(" {} {} ", MaterialIcons::for_file(buffer.file_path().unwrap_or(std::path::Path::new("untitled"))), buffer.file_name())))
            .style(Style::default().bg(colors.bg_primary))
            .wrap(Wrap { trim: false });
        
        frame.render_widget(editor, area);
    }

    fn render_status_bar(&self, frame: &mut Frame, area: Rect, colors: &ThemeColors) {
        let mode_text = match self.mode {
            AppMode::Normal => " NORMAL ",
            AppMode::Insert => " INSERT ",
            AppMode::Command => &format!(" :{} ", self.command_input),
        };
        
        let mode_color = match self.mode {
            AppMode::Normal => colors.info,
            AppMode::Insert => colors.success,
            AppMode::Command => colors.warning,
        };
        
        let buffer = &self.tab_manager.tabs()[self.tab_manager.active_index()].buffer;
        let cursor = buffer.cursor();
        let position = format!("{}:{} ", cursor.line + 1, cursor.column + 1);
        
        let status = Paragraph::new(Line::from(vec![
            Span::styled(mode_text, Style::default().bg(mode_color).fg(colors.bg_primary).add_modifier(Modifier::BOLD)),
            Span::raw(" "),
            Span::styled(&self.status_message, Style::default().fg(colors.fg_primary)),
            Span::raw(" ".repeat(area.width.saturating_sub(self.status_message.len() as u16 + mode_text.len() as u16 + position.len() as u16 + 2) as usize)),
            Span::styled(position, Style::default().fg(colors.fg_secondary)),
        ]))
        .style(Style::default().bg(colors.bg_secondary));
        
        frame.render_widget(status, area);
    }
}

fn main() -> Result<()> {
    env_logger::init();
    
    // Setup terminal
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut app = PhpSunshine::new()?;

    // Main loop
    let result = (|| -> Result<()> {
        while !app.should_quit {
            app.render(&mut terminal)?;
            app.handle_events()?;
        }
        Ok(())
    })();

    // Cleanup
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    } else {
        println!("Thanks for using PHP Sunshine! 🌞");
    }
    
    Ok(())
}
