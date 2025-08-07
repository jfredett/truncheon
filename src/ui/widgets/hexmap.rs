use ratatui::{style::Color, widgets::{canvas::{Canvas, Context, Line, Rectangle}, StatefulWidget, Widget}};
use truncheon::hex::{coord::axial, field::Field};

#[derive(Debug, Clone)]
pub struct Hexmap {
    // store widget-display/style info only.
    center: axial::Point
}

impl Hexmap {
    pub fn draw(&self, ctx: &mut Context<'_>, state: &Field<isize>) {
        // starting from current origin in the center, spiral outward and render each hex
        // incrementally.
        for ax in axial::spiral() {
            let shifted_ax = ax + (self.center - axial::Point::origin());
            // convert ax -> screenspace coords, [0,0] in the center of the canvas
            let (p_x, p_y) = ax.to_flattop_pixel();
            ctx.draw(&Line {
                x1: p_x,
                y1: p_y,
                x2: 2.0 * p_x,
                y2: 2.0 * p_y,
                color: Color::Blue
            });

            // draw a hex of specific size
            // write shifted coords at bottom of hex
            // write {} of content to center of hex
        }
    }
}

impl Default for Hexmap {
    fn default() -> Self {
        Self { center: axial::Point::new(0, 0) }
    }
}

impl StatefulWidget for Hexmap {
    // TODO: Temporarily hardcode field, isize
    type State = Field<isize>;


    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let widget = Canvas::default()
            .x_bounds([-100.0, 100.0])
            .y_bounds([-100.0, 100.0])
            .paint(|ctx| self.draw(ctx, state));

        Widget::render(&widget, area, buf);
    }
}
