use ratatui::{style::Color, widgets::{canvas::{Canvas, Rectangle}, StatefulWidget, Widget}};
use truncheon::hex::field::Field;

#[derive(Debug, Default, Clone)]
pub struct Hexmap {
    // store style info only.
}

impl StatefulWidget for Hexmap {
    // TODO: Temporarily hardcode field, isize
    type State = Field<isize>;

    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let widget = Canvas::default()
            .x_bounds([-100.0, 100.0])
            .y_bounds([-100.0, 100.0])
            .paint(|ctx| {
                ctx.draw(&Rectangle {
                    x: 10.0,
                    y: 20.0,
                    width: 10.0,
                    height: 10.0,
                    color: Color::Red,
                });
            });

        Widget::render(&widget, area, buf);
    }
}


