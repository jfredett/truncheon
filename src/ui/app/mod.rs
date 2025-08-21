use ratatui::{crossterm::event::{Event, KeyCode}, layout::{Constraint, Layout, Rect}, style::{Color, Style}, Frame};
use truncheon::hex::{coord::axial, field::Field};

use std::{collections::HashMap, fmt::Debug, path::Path, str::FromStr};

use tui_logger::{LevelFilter, TuiLoggerLevelOutput, TuiLoggerSmartWidget, TuiWidgetState};

use lazy_static::lazy_static;

use crate::ui::{tui::Message, widgets::{hexmap::Hexmap, placeholder::Placeholder, svg::{SVGTemplate, SVG}}};

enum Mode {
    Insert,
    Command
}

pub struct UI {
    flags: HashMap<String, bool>,
    // layout: HashMap<String, Rect> // this gets async updated with the current layout whenever a resize event occurs.
    // UI
    mode: Mode,
    // NOTE: the update function is going to be used to render the SVG in the background, so I need
    // to remember the current Frame's area so I can grab the dimensions of the thing without
    // having the thing. This means that there may be a race when resizing, I think this could be
    // maintained via the resize event
    frame_area: Option<Rect>,
    layout: HashMap<String, Rect>,
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


impl UI {
    pub fn new() -> Self {
        Self {
            flags: HashMap::new(),
            mode: Mode::Command,
            tuiloggerstate: TuiWidgetState::new().set_default_display_level(LevelFilter::Trace),
            player_map: SVG::new(),
            // BUG: probably this should be set by something other than just the resize. Or maybe I
            // should just fire an initial resize somehow? results in a no-first-render bug.
            frame_area: None,
            layout: HashMap::new()
        }
    }

    // TODO: Pull this into some separate structure.
    // TODO: In addition, this should just specify what _should_ happen, not _do_ any of it, that's
    // for the Message to indicate. So message might be "Message:SwitchMode(Mode::Insert)" or
    // whatever.
    pub fn handle_events(&mut self, event: Event) -> Message {
        match event {
            Event::Resize(width, height) => { 
                tracing::info!("Caught resize old area: {:?}, new area {width},{height}", self.frame_area);
                let (x,y) = self.frame_area.map_or((0,0), |a| (a.x, a.y));
                self.frame_area = Some(Rect::new(x, y, width, height));
                self.build_layout();
            },
            Event::Key(key) => {
                match self.mode {
                    // Probably insert mode is just handled by the tile wholesale?
                    Mode::Insert => {
                        match key.code {
                            KeyCode::Esc => {
                                tracing::trace!("Switching to Command Mode");
                                self.mode = Mode::Command;
                            },
                            KeyCode::Char(c) => {
                                tracing::debug!("Recieved key: {:?}", c);
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
                                tracing::trace!("Switching to Insert Mode");
                                self.mode = Mode::Insert;
                            },
                            KeyCode::Char('q') => {
                                tracing::trace!("Quitting");
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

    pub fn build_layout(&mut self) {
        if let Some(rect) = self.frame_area {
            tracing::info!("Rebuilding Layout");
            let chunks = PRIMARY_LAYOUT.split(rect);
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

            self.layout = HashMap::from([
                ("trail_slice".to_string(), trail_slice),
                ("log_section".to_string(), log_section),
                ("gm_map_slice".to_string(), gm_map_slice),
                ("player_map_slice".to_string(), player_map_slice),
                ("output_section".to_string(), output_section),
                ("input_section".to_string(), input_section),
            ]);
        } else {
            tracing::debug!("Skipping layout since there is no frame yet");
        }
    }

    pub async fn update(&mut self) {
        if self.frame_area.is_none() { return; }
        // FIXME: This is non-ideal duplication.
        self.build_layout();
        // FIXME: This is also non-ideal duplication. I suppose I may want to swap out svgs during an
        // update, but this doesn't feel great as is
        self.player_map.update(self.layout["player_map_slice"], &mut SVGTemplate::from_file(Path::new("./tests/fixtures/svg/template.svg")));
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

    pub fn render(&self, frame: &mut Frame<'_>) {
        let tlw = self.tui_logger_widget();

        if self.frame_area.is_none() { return; }

        frame.render_widget(tlw, self.layout["log_section"]);

        frame.render_widget(&Placeholder::for_section(self.layout["trail_slice"]).text("TRAIL"), self.layout["trail_slice"]);
        frame.render_stateful_widget(&self.player_map, self.layout["player_map_slice"], &mut SVGTemplate::from_file(Path::new("./tests/fixtures/svg/template.svg")));
        frame.render_stateful_widget(Hexmap::default(), self.layout["gm_map_slice"], &mut Field::<isize>::new());
        frame.render_widget(&Placeholder::for_section(self.layout["output_section"]).text("OUTPUT"), self.layout["output_section"]);
        frame.render_widget(&Placeholder::for_section(self.layout["input_section"]).text("> INPUT"), self.layout["input_section"]);
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
