use bevy::prelude::*;
use bevy_ratatui::event::KeyMessage;
use ratatui::widgets::Widget;


#[derive(Default)]
struct Input {
    // TODO: should be a smallvec
    buf: String
}

impl Input {
    pub fn push(&mut self, ch: char) {
        self.buf.push(ch);
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
        app.add_systems(Update, handle_keyboard_io);
    }
}


impl Widget for IOWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) where Self: Sized {
        todo!()
    }
}


/// RO Struct containing the entered command
#[derive(Event, Deref, Clone, Debug)]
pub struct CommandEvent(String);


impl IOWidget {
    pub fn create(
        mut commands: Commands,
    ) {
        commands.spawn((
            IOWidget::default(),
        ));
    }

    pub fn record(&mut self, cmd: String) {
        self.history.append(cmd);
    }

    // accept an incoming character
    pub fn send_input_char(&mut self, ch: char) {
        self.input.push(ch);
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
struct Focus;

// NOTE: See above
// trait CanFocus {
//     fn send_input_char(ch: u8);
//     fn on_defocus(&mut self) {}
//     fn on_focus(&mut self) {}
// }
//
//#[derive(Resource)]
//struct CommandMode;


pub fn handle_keyboard_io(
    mut commands: Commands,
    mut focus: Single<&mut IOWidget, With<Focus>>,
    mut messages: MessageReader<KeyMessage>,
    mut exit: MessageWriter<AppExit>,
) {
    for message in messages.read() {
        match message.code {
            ratatui::crossterm::event::KeyCode::Enter => {
                focus.execute(&mut commands);
            },
            ratatui::crossterm::event::KeyCode::Char(c) => {
                focus.send_input_char(c);
            },
            _ => {}
        }
    }
    // capture input, find the current focused element, if the element is marked for accepting
    // input, send it along, unless it's `escape`, which means you should remove focus on the
    // focused element and place it on the CommandMode element.

    // if there is no focused element, then the keystrokes are sent to the CommandMode handler,
    // where they can be buffered as needed for parsing.
}
