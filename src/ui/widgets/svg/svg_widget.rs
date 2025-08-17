use std::{collections::HashMap, fs, path::PathBuf, sync::{self, Arc, RwLock}};

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


pub type Image = Arc<usvg::ImageKind>;

#[derive(Default)]
pub struct ImageCache {
    content: RwLock<HashMap<PathBuf, Image>>
}

impl ImageCache {
    pub fn get(&self, path: &std::path::Path) -> Option<Image> {
        let cache = self.content.read().unwrap();
        let img_path = fs::canonicalize(path).expect("File not found");

        tracing::info!("Cache keys are {:?}", cache);
        tracing::info!("Checking cache for {}", img_path.display());

        cache.get(&img_path).cloned()
    }

    pub fn add(&self, path: &str) -> Option<Image> {
        let img_path = fs::canonicalize(path).expect("File not found");

        if let Some(x) = self.get(&img_path) {
            tracing::info!("Cache hit for {}", img_path.display());
            return Some(x)
        }

        tracing::info!("looking at {}", img_path.display());
        let image_data = fs::read(&img_path).expect("Failed to read");
        let len = image_data.len();

        let img = Arc::new(image_data);

        let entry = Arc::new(if path.ends_with("png") {
            tracing::info!("found png, loaded, {} bytes wide", len);
            usvg::ImageKind::PNG(img.clone())
        } else if path.ends_with("webp") {
            tracing::info!("found webp, loaded, {} bytes wide", len);
            usvg::ImageKind::WEBP(img.clone())
        } else {
            tracing::error!("unrecognized file format");
            return None
        });

        { // write to the cache
            let mut cache = self.content.write().unwrap();
            cache.insert(img_path.into(), entry.clone());
        }

        Some(entry)
    }

}

pub static IMAGE_CACHE : sync::LazyLock<ImageCache> = sync::LazyLock::new(|| {
    ImageCache::default()
});

pub static SVG_OPTS : sync::LazyLock<usvg::Options> = sync::LazyLock::new(|| {
    let mut opt = usvg::Options::default();
    opt.fontdb_mut().load_system_fonts();
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

impl StatefulWidget for SVG {
    type State = SVGTemplate;

    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        // TODO: Error handling, lots of bare unwraps running around
        //
        tracing::info!("Preparing to render");

        // SVG TEMPLATE RENDERING

        // Prep the content
        let content = state.render();

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

        // avoids an issue during testing
        let picker = if cfg!(test) {
            Picker::from_fontsize((8, 12))
        } else {
            Picker::from_query_stdio().unwrap()
        };

        let mut image = picker.new_resize_protocol(rendered_image);

        let widget = StatefulImage::default();

        tracing::info!("Rendering to ratatui screen");
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
