use bevy::{input::keyboard::KeyboardInput, prelude::*};
use ratatui::{layout::{Constraint, Direction, Layout}, widgets::{Block, Borders, Paragraph, Widget}};


#[derive(Default)]
struct Input {
    // TODO: should be a smallvec
    buf: String
}

impl Input {
    pub fn push(&mut self, s: &str) {
        self.buf.push_str(s);
    }
}

#[derive(Default)]
struct History {
    // TODO: should be a smallvec
    buf: Vec<String>
}


impl History {
    pub fn append(&mut self, entry: String) {
        // TODO: Smallvec and drop oldest from history on push
        self.buf.push(entry);
    }
}

#[derive(Component, Default)]
pub struct IOWidget {
    input: Input,
    history: History
}

// The idea is:
//
// Insert a component for each widget.
//
// One widget can be marked 'focused' at a time, some can be marked 'recieves input'
//
// Capture every keystroke, if the IO widget is focused, send the keystroke to the Input buffer of
// the widget, where it is held, if and only if it's marked as 'recieves input'
//
// When it's not focused, or when a component that is focused is not marked to recieve input, then
// input is ignored.
//
// Focus is removed from the current element and placed on the 'CommandMode' element upon pressing
// `esc`
//
// Should the plugin be the whole UI, or just this widget? I think it's the UI. Maybe the UI has
// these widgets as plugins?
//
// Each widget is a plugin that bundles all the functionality and adds the widget as a component,
// then the UI is the actual render system, layout, etc.
//


impl Plugin for IOWidget {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_keyboard_io)
           .add_systems(Startup, add_io_widget)
           .add_message::<CommandEvent>()
           .add_observer(log_on_execute);
    }
}

pub fn add_io_widget(
    mut commands: Commands,
) {
    commands.spawn((
        IOWidget::default(),
        Focus
    ));
}

impl Widget for &mut IOWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) where Self: Sized {
        // we're given an area, we'll lay it out ourselves to what we want
        //
        let layout = Layout::new(
            Direction::Vertical,
            [Constraint::Min(1), Constraint::Length(1)]
        ).split(area);
        let history_section = layout[0];
        let input_line = layout[1];

        let history_block = Block::bordered().borders(Borders::TOP | Borders::LEFT | Borders::RIGHT).title("History");
        let history_content = Paragraph::new(self.history.buf.join("\n")).block(history_block);

        let input_line_block = Block::bordered().borders(Borders::LEFT | Borders::RIGHT);
        let input_line_content = Paragraph::new(self.input.buf.clone()).block(input_line_block);

        history_content.render(history_section, buf);
        input_line_content.render(input_line, buf);
    }
}


/// RO Struct containing the entered command
#[derive(Message, Event, Deref, Clone, Debug)]
pub struct CommandEvent(String);


impl IOWidget {
    pub fn record(&mut self, cmd: String) {
        self.history.append(cmd);
    }

    // accept an incoming character
    pub fn send_input(&mut self, s: &str) {
        self.input.push(s);
    }

    // Send an event that signals a command has been entered.
    pub fn execute(&mut self,
        commands: &mut Commands,
    ) {
        let s = self.input.buf.clone();
        self.input = Input::default();
        commands.trigger(CommandEvent(s));
    }
}

pub fn log_on_execute(
    event: On<CommandEvent>,
    mut io_widget: Single<&mut IOWidget>
) {
    let cmd = (*event).clone();
    trace!("{:?}", cmd);
    if true { // TODO: this should depend on if the command should be echoed or not
        io_widget.record(cmd.to_string());
    }
}


// NOTE: This is explicitly future planning, I don't expect to need to move this around, or
// exactly how I'm going to use this, but for now I'm adding a marker component to the IOWidget
// Component, even though I don't expect it'll ever actually lose focus in a way that matters
// (needing to send keyboard inputs somewhere other than the IO box).
//
// Eventually I'd like to have a 'ratatui modal input' plugin, that's why this is here.
#[derive(Component)]
pub struct Focus;

// NOTE: See above
// trait CanFocus {
//     fn send_input_char(ch: u8);
//     fn on_defocus(&mut self) {}
//     fn on_focus(&mut self) {}
// }
//
//#[derive(Resource)]
//struct CommandMode;

// // FIXME: Replace with a bevy-level input handling, this is very slow, and I hope bevy's thing is
// // better tied into the event loop.
// pub fn handle_keyboard_io(
//     mut commands: Commands,
//     mut focus: Single<&mut IOWidget, With<Focus>>,
//     mut messages: MessageReader<KeyMessage>,
// ) {
//     for message in messages.read() {
//         match message.code {
//             ratatui::crossterm::event::KeyCode::Enter => {
//                 trace!("executing command");
//                 focus.execute(&mut commands);
//             },
//             ratatui::crossterm::event::KeyCode::Char(c) => {
//                 trace!("recording character");
//                 focus.send_input_char(c);
//             },
//             _ => {}
//         }
//     }
//     // capture input, find the current focused element, if the element is marked for accepting
//     // input, send it along, unless it's `escape`, which means you should remove focus on the
//     // focused element and place it on the CommandMode element.

//     // if there is no focused element, then the keystrokes are sent to the CommandMode handler,
//     // where they can be buffered as needed for parsing.
// }

pub fn handle_keyboard_io(
    mut commands: Commands,
    mut keyboard_events: EventReader<KeyboardInput>,
    mut focus: Single<&mut IOWidget, With<Focus>>,
) {
    for event in keyboard_events.read() {
        trace!("Key pressed: {:?}, logical key: {:?}", event.key_code, event.logical_key);
        use bevy::input::keyboard::Key;
        match &event.logical_key {
            Key::Character(s) => {
                trace!("recording character");
                focus.send_input(&s.clone());
            }
            Key::Enter => {
                trace!("executing command");
                focus.execute(&mut commands);
            }
            _ => {}
        }
    }
}

