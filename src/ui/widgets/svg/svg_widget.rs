use image::DynamicImage;
use ratatui::widgets::{Block, StatefulWidget, Widget};
use ratatui_image::{picker::Picker, StatefulImage};
use resvg::{tiny_skia, usvg};

use crate::ui::widgets::svg::{options::SVG_OPTS, svg_template::SVGTemplate};


/// SVG Widget, containing any static widget configuration. Rendering takes an [[SVGTemplate]] as
/// an input to describe what SVG to render.
#[derive(Default)]
#[allow(clippy::upper_case_acronyms)]
pub struct SVG {
    png_data: DynamicImage
}

impl SVG {
    pub fn new() -> Self { Self::default() }

    // this area should be pixel-sized, not font-sized, maybe I need a separate wrapper for rects?
    pub async fn update(&mut self, area: ratatui::prelude::Rect, state: &mut SVGTemplate) {
        // TODO: Error handling, lots of bare unwraps running around
        //
        tracing::info!("Preparing to render");

        // this should be something more like:
        // PNGRenderer::send(template, area) -> Return channel
        // on render, await the return channel to respond and then pull the data from png_data?


        let mut picker = Picker::from_fontsize((8,12));
        picker.set_protocol_type(ratatui_image::picker::ProtocolType::Kitty);
        let (width_adj, height_adj) = picker.font_size();

        // SVG TEMPLATE RENDERING
 
        tracing::debug!("Recieved rect {:?}", area);
        state.set_width(area.width * width_adj * 3);
        state.set_height(area.height * height_adj * 4);

        // NOTE: I think all this could be pre-prepped in the widget state? at least some of it.
        // Picker at least, probably bits of the tree, there's waste here.

        // RENDER PHASE
        let content = state.render();

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
        let png_data: Vec<u8> = pixmap.encode_png().unwrap_or_default();

        tracing::info!("Loading into Memory");
        let rendered_image = image::load_from_memory(&png_data).expect("Failed to load PNG from memory");
        self.png_data = rendered_image;
    }
}

impl StatefulWidget for &SVG {
    type State = SVGTemplate;

    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, _state: &mut Self::State) {
        // TODO: Make this support other terminals?
        let mut picker = Picker::from_fontsize((8,12));
        picker.set_protocol_type(ratatui_image::picker::ProtocolType::Kitty);

        // FIXME: non-ideal clone.
        let mut image = picker.new_resize_protocol(self.png_data.clone());
        let container = Block::new();
        let widget = StatefulImage::default();
        let container_area = container.inner(area);


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
