use ratatui::{crossterm::event::{Event, KeyCode}, layout::{Constraint, Layout}, style::{Color, Style}, widgets::{StatefulWidget, Widget}, Frame};
use truncheon::hex::{coord::axial, field::Field};

use std::{collections::HashMap, fmt::Debug, path::Path, str::FromStr};

use tui_logger::{LevelFilter, TuiLoggerLevelOutput, TuiLoggerSmartWidget, TuiWidgetState};

use lazy_static::lazy_static;

use crate::ui::widgets::{hexmap::Hexmap, placeholder::Placeholder, svg::{SVGTemplate, SVG}};

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
        // todo!();
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
        // Used to show the travel 'trail', both the intended and actual
        let trail_slice = chunks[0];
        // Used to show the intended location of the players
        let player_map_slice = chunks[1];
        // Used to show the actual location of the players
        let gm_map_slice = chunks[2];


        let chunks = IO_SECTION_LAYOUT.split(io_section);
        let output_section = chunks[0];
        let input_section = chunks[1];

        let tlw = self.tui_logger_widget();

        Widget::render(tlw, log_section, frame.buffer_mut());


        // Should show a log of intended direction, actual, etc. Maybe add a logline beneath maps
        // to show expanded info.
        Widget::render(&Placeholder::for_section(trail_slice).text("TRAIL"), trail_slice, frame.buffer_mut());
        // A canvas, maybe extend placeholder first to do a dummy canvas.
        StatefulWidget::render(SVG::new(), player_map_slice, frame.buffer_mut(), &mut SVGTemplate::from_file(Path::new("./tests/fixtures/svg/template.svg")));
        StatefulWidget::render(Hexmap::default(), gm_map_slice, frame.buffer_mut(), &mut Field::<isize>::new());
        Widget::render(&Placeholder::for_section(output_section).text("OUTPUT"), output_section, frame.buffer_mut());
        Widget::render(&Placeholder::for_section(input_section).text("> INPUT"), input_section, frame.buffer_mut());
    }
}

lazy_static! {
    static ref PRIMARY_LAYOUT : Layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(30),
                Constraint::Percentage(20),
                Constraint::Min(1),
            ].as_ref());

    static ref UPPER_LAYOUT : Layout = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(45),
            Constraint::Percentage(45),
        ].as_ref());
    static ref IO_SECTION_LAYOUT : Layout = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(3)
        ].as_ref());

    static ref TEST_FIELD : Field<isize> = {
        let f = Field::new();
        f.insert(axial::Point::from_str("[0,0]").unwrap(), 0);
        f.insert(axial::Point::from_str("[0,1]").unwrap(), 1);
        f.insert(axial::Point::from_str("[1,0]").unwrap(), 2);
        f.insert(axial::Point::from_str("[1,1]").unwrap(), 3);
        f.insert(axial::Point::from_str("[0,-1]").unwrap(), -1);
        f.insert(axial::Point::from_str("[-1,0]").unwrap(), -2);
        f.insert(axial::Point::from_str("[-1,-1]").unwrap(), -3);
        f
    };
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
