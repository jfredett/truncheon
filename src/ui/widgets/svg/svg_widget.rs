use ratatui::widgets::{Block, StatefulWidget, Widget};
use ratatui_image::{picker::Picker, StatefulImage};
use resvg::{tiny_skia, usvg};

use crate::ui::widgets::svg::{options::SVG_OPTS, svg_template::SVGTemplate};


/// SVG Widget, containing any static widget configuration. Rendering takes an [[SVGTemplate]] as
/// an input to describe what SVG to render.
#[derive(Default)]
#[allow(clippy::upper_case_acronyms)]
pub struct SVG { }

impl SVG {
    pub fn new() -> Self { Self { } }
}

impl StatefulWidget for SVG {
    type State = SVGTemplate;

    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        // TODO: Error handling, lots of bare unwraps running around
        //
        tracing::info!("Preparing to render");

        // SVG TEMPLATE RENDERING
 

        // Get the current font size and other info about the terminal
        let picker = if cfg!(test) {
            // avoids an issue during testing by fixing the fontsize, normally this is unset for
            // the test
            Picker::from_fontsize((8, 12))
        } else {
            Picker::from_query_stdio().unwrap()
        };

        // Prep the content

        // set the SVG width/height to match the container the widget lives in.
        // This may need to be more complicated to manage margins and other such. For now
        // this works pretty alright
        let (f_w, f_h) = picker.font_size();
        state.set_width(area.width * f_w);
        state.set_height(area.height * f_h);
        let content = state.render();


        // NOTE: I think all this could be pre-prepped in the widget state? at least some of it.
        // Picker at least, probably bits of the tree, there's waste here.

        // RENDER PHASE

        // Create the SVG DOM
        let tree = usvg::Tree::from_str(&content, &SVG_OPTS).unwrap();

        // Build a pixmap to fill
        let pixmap_size = tree.size().to_int_size();
        tracing::info!("{pixmap_size:?}");
        let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();

        // Fill it
        tracing::info!("Rendering SVG");
        resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());
        tracing::info!("Encoding as PNG");
        let png_data = pixmap.encode_png().unwrap();
        tracing::info!("Loading into Memory");
        let rendered_image = image::load_from_memory(&png_data).unwrap();

        // IMAGE TIME


        let mut image = picker.new_resize_protocol(rendered_image);
        tracing::debug!("picker: {:?}", picker);

        let container = Block::new();
        let widget = StatefulImage::default();
        let container_area = container.inner(area);

        tracing::info!("Rendering to ratatui screen");
        Widget::render(container, area, buf);
        StatefulWidget::render(widget, container_area, buf, &mut image);
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use ratatui::{buffer::Buffer, layout::Rect, style::{Color, Style}};
    use rstest::*;
    use super::*;


    #[fixture]
    fn example_svg() -> SVGTemplate {
        SVGTemplate::new(std::fs::read_to_string("./tests/fixtures/svg/example.svg").unwrap())
    }

    #[rstest]
    fn renders_example(mut example_svg: SVGTemplate) {
        let rect = Rect::new(0, 0, 8, 8);
        let mut buffer = Buffer::empty(rect);
        buffer.set_style(rect, Style::default().fg(Color::White).bg(Color::Black));

        let svg = SVG::new();

        svg.render(rect, &mut buffer, &mut example_svg);

        assert_debug_snapshot!(buffer);
    }
}
