use ratatui::{crossterm::event::{Event, KeyCode}, layout::{Constraint, Layout}, style::{Color, Style}, widgets::{StatefulWidget, Widget}, Frame};
use truncheon::hex::{coord::axial, field::Field};

use std::{collections::HashMap, fmt::Debug, path::Path, str::FromStr};

use tui_logger::{LevelFilter, TuiLoggerLevelOutput, TuiLoggerSmartWidget, TuiWidgetState};

use lazy_static::lazy_static;

use crate::ui::{tui::{FrameShape, Message}, widgets::{hexmap::Hexmap, placeholder::Placeholder, svg::{SVGTemplate, SVG}}};

enum Mode {
    Insert,
    Command
}

pub struct UI {
    flags: HashMap<String, bool>,
    // layout: HashMap<String, Rect> // this gets async updated with the current layout whenever a resize event occurs.
    // UI
    mode: Mode,
    // State
    tuiloggerstate: TuiWidgetState,
    player_map: SVG,
}

impl Debug for UI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Truncheon")
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
    pub fn new() -> Self {
        Self {
            flags: HashMap::new(),
            mode: Mode::Command,
            tuiloggerstate: TuiWidgetState::new().set_default_display_level(LevelFilter::Trace),
            player_map: SVG::new(),
        }
    }

    // Pull this into some separate structure.
    pub fn handle_events(&mut self, event: Event) -> Message {
        match event {
            Event::Resize(_width, _height) => { 
                tracing::info!("Caught resize");
                // update on UI the current width/height
                // re-acq the picker and set relevant values

            },
            Event::Key(key) => {
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
                                return Message::Quit;
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
            },
            _ => {}
            // Event::FocusGained => todo!(),
            // Event::FocusLost => todo!(),
            // Event::Mouse(_mouse_event) => todo!(),
            // Event::Paste(_) => todo!(),
        }

        Message::Noop
    }

    pub async fn update(&mut self) {
        tracing::info!("In UI::update");
    }

    fn tui_logger_widget(&self) -> TuiLoggerSmartWidget {
        TuiLoggerSmartWidget::default()
            .style_error(Style::default().fg(Color::Red))
            .style_debug(Style::default().fg(Color::Green))
            .style_warn(Style::default().fg(Color::Yellow))
            .style_trace(Style::default().fg(Color::Magenta))
            .style_info(Style::default().fg(Color::Cyan))
            .output_separator('|')
            .output_timestamp(Some("%H:%M:%S".to_string()))
            .output_level(Some(TuiLoggerLevelOutput::Long))
            .output_target(true)
            .output_file(true)
            .output_line(true)
            .state(&self.tuiloggerstate)
    }

    pub fn render(&self, frame: &mut Frame<'_>, _frame_shape: &FrameShape) {
        tracing::info!("In UI::render");
        let chunks = PRIMARY_LAYOUT.split(frame.area());
        let upper_section = chunks[0];
        let log_section = chunks[1];
        let io_section = chunks[2];

        let chunks = UPPER_LAYOUT.split(upper_section);
        // Used to show the travel 'trail', both the intended and actual
        let trail_slice = chunks[0];
        // Used to show the intended location of the players -- NOTE: Should this be more like,
        // "svg_viewport_a" and have it's use determined by the SVGTemplate we pass it? That might
        // be better overall
        let player_map_slice = chunks[1];
        // Used to show the actual location of the players
        let gm_map_slice = chunks[2];


        let chunks = IO_SECTION_LAYOUT.split(io_section);
        let output_section = chunks[0];
        let input_section = chunks[1];

        let tlw = self.tui_logger_widget();

        frame.render_widget(tlw, log_section);


        frame.render_widget(&Placeholder::for_section(trail_slice).text("TRAIL"), trail_slice);
        frame.render_stateful_widget(&self.player_map, player_map_slice, &mut SVGTemplate::from_file(Path::new("./tests/fixtures/svg/template.svg")));
        frame.render_stateful_widget(Hexmap::default(), gm_map_slice,&mut Field::<isize>::new());
        frame.render_widget(&Placeholder::for_section(output_section).text("OUTPUT"), output_section);
        frame.render_widget(&Placeholder::for_section(input_section).text("> INPUT"), input_section);
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
