use ratatui::widgets::StatefulWidget;
use ratatui_image::{picker::Picker, StatefulImage};
use resvg::{tiny_skia, usvg};

use crate::ui::widgets::svg::svg_template::SVGTemplate;


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

        // SVG TEMPLATE RENDERING

        // Prep the content
        let content = state.render();

        // RENDER PHASE

        // default options
        let mut opt = usvg::Options::default();
        opt.fontdb_mut().load_system_fonts();

        // Create the SVG DOM
        let tree = usvg::Tree::from_str(&content, &opt).unwrap();

        // Build a pixmap to fill
        let pixmap_size = tree.size().to_int_size();
        let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();

        // Fill it
        resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

        let png_data = pixmap.encode_png().unwrap();

        // IMAGE TIME

        // avoids an issue during testing
        let picker = if cfg!(test) {
            Picker::from_fontsize((8, 12))
        } else {
            Picker::from_query_stdio().unwrap()
        };

        let rendered_image = image::load_from_memory(&png_data).unwrap();

        let mut image = picker.new_resize_protocol(rendered_image);

        let widget = StatefulImage::default();

        StatefulWidget::render(widget, area, buf, &mut image);
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
