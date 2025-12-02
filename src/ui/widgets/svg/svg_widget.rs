use std::sync::{self, Arc};

use image::DynamicImage;
use resvg::{tiny_skia, usvg};
use ratatui::widgets::{Block, StatefulWidget, Widget};
use ratatui_image::{picker::Picker, StatefulImage};

use crate::{ui::widgets::svg::SVGTemplate, util::{image_cache::IMAGE_CACHE, options::Parameters}};


pub static SVG_OPTS : sync::LazyLock<usvg::Options> = sync::LazyLock::new(|| {
    let mut opt = usvg::Options::default();
    opt.fontdb_mut().load_system_fonts();
    opt.dpi = 32.0;
    opt.image_rendering = usvg::ImageRendering::OptimizeSpeed;
    let resolve_data = Box::new(|_: &str, _: Arc<Vec<u8>>, _: &usvg::Options| None);
    opt.image_href_resolver = usvg::ImageHrefResolver {
        resolve_data,
        resolve_string: Box::new(move |href, _| -> Option<usvg::ImageKind> {
            // FIXME: This is almost certainly extremely dangerous. Good thing I don't know what
            // I'm doing.
            // FIXME: Don't ever try to do this with a broken link, you monster.

            IMAGE_CACHE.add(href).map(|img| img.as_ref().clone())
        })
    };
    opt
});


/// SVG Widget, containing any static widget configuration. Rendering takes an [[SVGTemplate]] as
/// an input to describe what SVG to render. Caches the png_data after rendering the template.
#[derive(Default)]
#[allow(clippy::upper_case_acronyms)]
pub struct SVG {
    png_data: DynamicImage
}

impl SVG {
    pub fn new() -> Self { Self::default() }

    // this area should be pixel-sized, not font-sized, maybe I need a separate wrapper for rects?
    pub async fn update(&mut self, area: ratatui::prelude::Rect, state: &mut SVGTemplate, p: &Parameters) {
        // TODO: Error handling, lots of bare unwraps running around

        state.set_width(area.width * p.font_size.0);
        state.set_height(area.height * p.font_size.1);

        // RENDER PHASE
        let content = state.render();

        let result = tokio::task::spawn_blocking(move || {
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

            pixmap.encode_png().unwrap_or_default()
        }).await;
        // this should get back a one-shot channel to await on.
        self.png_data = image::load_from_memory(&result.unwrap()).expect("Failed to load image from memory");
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
