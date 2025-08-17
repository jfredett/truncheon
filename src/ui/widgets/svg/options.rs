use std::sync::{self, Arc};

use resvg::usvg;

use crate::util::image_cache::IMAGE_CACHE;


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
