// Taken and adapted from the original https://github.com/d-holguin/async-ratatui/blob/main/src/lib.rs
// used under the offered MIT License, thus copyright by 2024 d.holguin. Thanks Daniel!


use ratatui::backend::CrosstermBackend;
use ratatui::crossterm::event::{
    DisableMouseCapture, EnableMouseCapture, Event,
};
use ratatui::crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::{crossterm, Terminal};
use tokio_stream::StreamExt;
use tui_logger::{LevelFilter, TuiLoggerFile, TuiLoggerLevelOutput};
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::time;

use crate::ui::app::UI;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Message {
    Noop,
    Quit,
    Tick,
    Render
}

pub struct Tui {
    pub terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    pub frame_rate: f64,
    pub tick_rate: f64,
    pub event_tx: UnboundedSender<Message>,
    pub event_rx: UnboundedReceiver<Message>,
    // TODO: UI -> generic 'Engine' which takes the update/event/render, can be shared.
    pub app: Arc<RwLock<UI>>
}

#[derive(Clone, Debug)]
pub enum UpdateCommand {
    None,
    Quit,
}

impl Tui {
    pub fn new(app: Arc<RwLock<UI>>, frame_rate: f64, tick_rate: f64) -> Result<Self> {
        let terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        Ok(Self {
            terminal,
            frame_rate,
            tick_rate,
            event_tx,
            event_rx,
            app
        })
    }

    fn enter(&mut self) -> Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        if crossterm::terminal::is_raw_mode_enabled()? {
            self.terminal.flush()?;
            crossterm::execute!(std::io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
            crossterm::terminal::disable_raw_mode()?;
            self.terminal.show_cursor()?;
            println!("Terminal exited.");
        }
        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        use tracing_subscriber::prelude::*;

        // Set up the Tracing layer
        tracing_subscriber::registry()
            .with(tui_logger::TuiTracingSubscriberLayer)
            .init();

        // Set the log files
        tui_logger::init_logger(LevelFilter::Trace)?;
        tui_logger::set_default_level(LevelFilter::Trace);
        // prepare the log directory and file.
        let mut dir = env::temp_dir();

        // TODO: Replace with something like "app.name()"
        dir.push("truncheon.log");
        let file_options = TuiLoggerFile::new(dir.to_str().unwrap())
            .output_level(Some(TuiLoggerLevelOutput::Abbreviated))
            .output_file(true)
            .output_separator(':');

        tui_logger::set_log_file(file_options);

        // TODO: Set target dynamically as well
        tracing::debug!(target:"truncheon::ui", "Logging to {}", dir.to_str().unwrap());

        // FIXME: A bunch of this should probably live in like, an `init` function on app.

        self.enter()?;
        assert!(crossterm::terminal::is_raw_mode_enabled()?, "Raw mode not enabled!");

        tracing::debug!(target:"truncheon::ui", "Entered terminal");
        self.terminal.clear()?;

        let tick_rate = Duration::from_secs_f64(self.tick_rate);
        let frame_rate = Duration::from_secs_f64(self.frame_rate);
        let mut tick_interval = time::interval(tick_rate);
        let mut frame_interval = time::interval(frame_rate);

        let mut event_stream = crossterm::event::EventStream::new();

        loop {
            tokio::select! {
                _tick = tick_interval.tick() => {
                    if let Err(e) = self.event_tx.send(Message::Tick) {
                        return Err(format!("Failed to tick: {:?}", e).into());
                    }
                }
                _frame = frame_interval.tick() => {
                    if let Err(e) = self.event_tx.send(Message::Render) {
                        return Err(format!("Failed to render frame: {:?}", e).into());
                    }
                }

                Some(Ok(event)) = event_stream.next() => {
                    if let Err(e) = self.handle_event(event).await {
                        tracing::error!("Failed to handle event: {:?}", e);
                        return Err(format!("Failed to handle event: {:?}", e).into());
                    }
                }

                // Meta events (call to quit, etc).
                Some(message) = self.event_rx.recv() => {
                    match self.update(message).await? {
                        UpdateCommand::Quit => return {
                            self.exit()?;
                            Ok(())
                        },
                        UpdateCommand::None => continue,
                    }
                }
            }
        }
    }

    // TODO: Inline, make the app#handle_events return a result
    async fn handle_event(&self, event: Event) -> Result<()> {
        let mut app = self.app.write().await;
        let response = app.handle_events(event);
        self.event_tx.send(response)?;
        Ok(())
    }

    async fn update(&mut self, message: Message) -> Result<UpdateCommand> {
        match message {
            Message::Noop => {},
            Message::Quit => { return Ok(UpdateCommand::Quit) },
            Message::Tick => {
                let app = self.app.clone();
                tokio::task::spawn(async move {
                    let mut guard = app.write().await;
                    guard.update().await;
                });
            },
            Message::Render => {
                self.view().await?;
            },
        }

        Ok(UpdateCommand::None)
    }

    async fn view(&mut self) -> Result<()> {
        // BUG: This picker shit is killing the party.
        // let picker = if cfg!(test) {
        //     // avoids an issue during testing by fixing the fontsize, normally this is unset for
        //     // the test
        //     Picker::from_fontsize((7, 12))
        // } else {
        //     Picker::from_query_stdio().unwrap_or(Picker::from_fontsize((8,12)))
        // };


        let app = self.app.read().await;
        self.terminal.draw(|f| {
            app.render(f)
        })?;

        Ok(())
    }
}

impl Drop for Tui {
    fn drop(&mut self) {
        self.exit().expect("Failed to end terminal mode");
    }
}

