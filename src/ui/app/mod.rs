use ratatui::{crossterm::event::{Event, KeyCode}, layout::{Constraint, Layout}, style::{Color, Style}, widgets::{StatefulWidget, Widget}, Frame};

use std::{collections::HashMap, fmt::Debug, sync::Mutex};

use tui_logger::{LevelFilter, TuiLoggerLevelOutput, TuiLoggerSmartWidget, TuiWidgetState};

use lazy_static::lazy_static;

use crate::ui::widgets::placeholder::Placeholder;

enum Mode {
    Insert,
    Command
}

pub struct UI {
    flags: HashMap<String, bool>,
    // UI
    mode: Mode,
    // State
    tuiloggerstate: TuiWidgetState,
}

impl Debug for UI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Hazel")
            .field("flags", &self.flags)
            .finish()
    }
}

// Workflow:
// 0. handle_events
//  - Just update inputs, don't calculate any new state
// 1. update
//  - Update all state simultaneously
// 2. render
//  - render the new view.
impl UI {
    pub async fn run() -> Self {
        Self {
            flags: HashMap::new(),
            mode: Mode::Command,
            tuiloggerstate: TuiWidgetState::new().set_default_display_level(LevelFilter::Trace),
        }
    }

    pub fn handle_events(&mut self, event: Event) {
        if let Event::Key(key) = event {
            match self.mode {
                // Probably insert mode is just handled by the tile wholesale?
                Mode::Insert => {
                    match key.code {
                        KeyCode::Esc => {
                            self.mode = Mode::Command;
                        },
                        KeyCode::Char(_c) => {
                            // self.tile.handle_input(c);
                        },
                        KeyCode::Backspace => {
                            // self.tile.handle_backspace();
                        },
                        KeyCode::Enter => {
                            // self.tile.handle_enter();
                        },
                        _ => {
                        }
                    }
                },
                // Command mode will eventually select the tile you want/start new tiles, etc.
                Mode::Command => {
                    match key.code {
                        KeyCode::Char('i') => {
                            self.mode = Mode::Insert;
                        },
                        KeyCode::Char('q') => {
                            self.set_flag("exit", true);
                        },
                        // KeyCode::Down => {
                        // }
                        // KeyCode::Up => {
                        // }
                        _ => {
                        }
                    }
                }
            }
        }
    }

    pub fn set_flag(&mut self, flag: &str, value: bool) {
        self.flags.insert(flag.to_string(), value);
    }

    pub fn check_flag(&self, flag: &str) -> bool {
        match self.flags.get(flag) {
            Some(value) => *value,
            None => false
        }
    }

    pub async fn update(&mut self) {
        todo!();
    }

    fn tui_logger_widget(&self) -> TuiLoggerSmartWidget {
        TuiLoggerSmartWidget::default()
            .style_error(Style::default().fg(Color::Red))
            .style_debug(Style::default().fg(Color::Green))
            .style_warn(Style::default().fg(Color::Yellow))
            .style_trace(Style::default().fg(Color::Magenta))
            .style_info(Style::default().fg(Color::Cyan))
            .output_separator(':')
            .output_timestamp(Some("%H:%M:%S".to_string()))
            .output_level(Some(TuiLoggerLevelOutput::Abbreviated))
            .output_target(true)
            .output_file(true)
            .output_line(true)
            .state(&self.tuiloggerstate)
    }

    pub fn render(&self, frame: &mut Frame<'_>) {
        let chunks = PRIMARY_LAYOUT.split(frame.area());
        let upper_section = chunks[0];
        let log_section = chunks[1];
        let io_section = chunks[2];

        let chunks = UPPER_LAYOUT.split(upper_section);
        let board_section = chunks[0];
        let tapereader_section = chunks[1];

        let chunks = BOARD_SECTION_LAYOUT.split(board_section);
        let _board_header = chunks[0];
        let board_field = chunks[1];
        let board_footer = chunks[2];

        let chunks = IO_SECTION_LAYOUT.split(io_section);
        let output_section = chunks[0];
        let input_section = chunks[1];

        let tlw = self.tui_logger_widget();

        Widget::render(tlw, log_section, frame.buffer_mut());
        Widget::render(&Placeholder::for_section(board_field), board_field, frame.buffer_mut());
        Widget::render(&Placeholder::for_section(board_footer).text("TEST"), board_footer, frame.buffer_mut());
    }
}

lazy_static! {
    static ref PRIMARY_LAYOUT : Layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(30),
                Constraint::Min(1),
            ].as_ref());

    static ref UPPER_LAYOUT : Layout = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ].as_ref());

    static ref BOARD_SECTION_LAYOUT : Layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ].as_ref());

    static ref IO_SECTION_LAYOUT : Layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(1)
        ].as_ref());
}


#[cfg(test)]
mod tests {
    use ratatui::backend::TestBackend;
    use insta::assert_debug_snapshot;
    use ratatui::Terminal;

    use super::*;

    #[tokio::test]
    async fn renders_as_expected() {
        let truncheon = UI::run().await;

        let mut t = Terminal::new(TestBackend::new(64, 32)).unwrap();
        let _ = t.draw(|f| {
            truncheon.render(f);
        });

        assert_debug_snapshot!(t.backend().buffer().clone());
    }
}
