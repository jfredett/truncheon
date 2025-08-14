use ratatui::widgets::{canvas::{Canvas, Context}, Block, StatefulWidget, Widget};
use tracing::info;
use truncheon::hex::{coord::{axial, pixel}, field::Field};

#[derive(Debug, Clone)]
pub struct Hexmap {
    // store widget-display/style info only.
    center: axial::Point
}


impl Hexmap {
    pub fn draw(&self, _aspect: f64, ctx: &mut Context<'_>, _state: &Field<isize>) {
        // starting from current origin in the center, spiral outward and render each hex
        // incrementally.
        for ax in axial::spiral() {
            info!("Drawing hex: {}", ax);
            // Fixme:: hardcoded
            let r = 8.0;
            let ax_vec = ax - axial::Point::origin();

            // FIXME: Ugly
            let shifted_ax = ax_vec + (self.center - axial::Point::origin());

           
            // FIXME: Ugly
            // convert ax -> screenspace coords, [0,0] in the center of the canvas
            let p_unscaled = shifted_ax.to_flattop_pixel();
            let p = pixel::Point::new(
                p_unscaled.x() * r,
                p_unscaled.y() * r
            );

            // draw six lines from -- relative to the center point at `p`, scaled to the size
            //
            // -r/2, h  <-> r/2, h
            // r/2, h   <-> r,0
            // r,0      <-> r/2, -h
            // r/2, -h  <-> -r/2, -h
            // -r/2, -h <-> -r, 0
            // -r, 0    <-> -r/2, h
            //
            // where `r` is the radius of the hex
            // h = r * sqrt(3)/2


            // this should probably not live here
            let h = (3f64).sqrt() / 2.0;

            let v1 = r * pixel::Vector::new(-0.5 , h);
            let v2 = r * pixel::Vector::new(0.5  , h);
            let v3 = r * pixel::Vector::new(1.0  , 0.0);
            let v4 = r * pixel::Vector::new(0.5  , -h);
            let v5 = r * pixel::Vector::new(-0.5 , -h);
            let v6 = r * pixel::Vector::new(-1.0 , 0.0);

            ctx.draw(&pixel::Point::line(p + v1, p + v2));
            ctx.draw(&pixel::Point::line(p + v2, p + v3));
            ctx.draw(&pixel::Point::line(p + v3, p + v4));
            ctx.draw(&pixel::Point::line(p + v4, p + v5));
            ctx.draw(&pixel::Point::line(p + v5, p + v6));
            ctx.draw(&pixel::Point::line(p + v6, p + v1));


            // finish layer

            ctx.layer();

            // write shifted coords at bottom of hex
            // write {} of content to center of hex
            //

            // finish layer
            
            ctx.layer();
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
        let aspect = area.width as f64 / area.height as f64;
        let widget = Canvas::default()
            .x_bounds([-100.0, 100.0])
            .y_bounds([-100.0, 100.0])
            .block(Block::bordered().title("Hexmap"))
            .paint(|ctx| self.draw(aspect, ctx, state));

        Widget::render(&widget, area, buf);
    }
}
