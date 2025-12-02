use resvg::{tiny_skia, usvg};
use tokio::sync::mpsc;
use std::sync::{self, Arc};

use crate::util::image_cache::IMAGE_CACHE;
use crate::util::svg_template::SVGTemplate;


/*
* A concrete design:
*
* This is a process that accepts a message with an SASE, the machine exists in three states:
*
* If the machine is "Delta", it is ready to render a new frame and has detected a change, when it
* receives a _RenderRequest_, at which point it will trigger a blocking render run, when this
* happens it enters the "Rendering" state and starts a timer. The machine has a configurable
* maximum framerate, if the render completes before the timer does, it enters the "Wait" state, if
* a RenderRequest comes in during the "Wait" state it gets a cached copy of the most recent frame.
*
* A parent machine can manage these machines and cache templates and whatever, alternatively each
* machine can be equipped with an internal representation and then handle the SVG side on it's own?
*
* 
*
* This would make is so the process is an instant-response "message recieved" from the requestor,
* which would then await the return message.
*
* The SASE could point to the 'main' ui, which then recieves a simple 'blit this thing to this
* spot', or it could communicate with the widget directly.
*
*
* Alternately, I could have it produce a `watch` channel that can be used on render to grab the
* latest frame?
*/


// pub static SVG_OPTS : sync::LazyLock<usvg::Options> = sync::LazyLock::new(|| {
//     let mut opt = usvg::Options::default();
//     opt.fontdb_mut().load_system_fonts();
//     opt.dpi = 32.0;
//     opt.image_rendering = usvg::ImageRendering::OptimizeSpeed;
//     let resolve_data = Box::new(|_: &str, _: Arc<Vec<u8>>, _: &usvg::Options| None);
//     opt.image_href_resolver = usvg::ImageHrefResolver {
//         resolve_data,
//         resolve_string: Box::new(move |href, _| -> Option<usvg::ImageKind> {
//             // FIXME: This is almost certainly extremely dangerous. Good thing I don't know what
//             // I'm doing.
//             // FIXME: Don't ever try to do this with a broken link, you monster.

//             IMAGE_CACHE.add(href).map(|img| img.as_ref().clone=))
//         })
//     };
//     opt
// });

// pub static RENDERER : sync::LazyLock<Arc<ImageRenderer>> = sync::LazyLock::new(Arc::new(ImageRenderer::new()));




////FIXME: This is duped from tui.rs, extract.
//pub type Error = Box<dyn std::error::Error>;
//pub type Result<T> = std::result::Result<T, Error>;

//pub struct ImageRenderer {
//    in_rx: mpsc::Receiver<SVGTemplate>,
//    // sends back the PNG bytestream
//    out_tx: mpsc::Sender<Vec<u8>>
//}

//impl ImageRenderer {
//    pub fn new() -> (Self, mpsc::Sender<SVGTemplate>, mpsc::Receiver<Vec<u8>>) {
//        // NOTE: this can own the SVG OPTS instead of static?

//        // TODO: Is unbounded right here?
//        let (in_tx, in_rx) = mpsc::channel(1_000);
//        let (out_tx, out_rx) = mpsc::channel(1_000);
//        (Self { in_rx, out_tx }, in_tx, out_rx)
//    }

//    pub async fn start(&mut self) -> Result<()> {
//        loop {
//            tokio::select! {
//                Some(template) = self.in_rx.recv() => {
//                    // process the template
//                    let res = self.render(template).await;
//                    // send the result
//                    self.out_tx.send(res).await
//                }
//            }?;
//        }
//    }

//    pub async fn render(&self, template: SVGTemplate) -> Vec<u8> {
//        // SVG TEMPLATE RENDERING

//        // RENDER PHASE
//        let content = template.render();

//        let res = tokio::task::spawn_blocking(move || {
//            // Create the SVG DOM
//            let tree = usvg::Tree::from_str(&content, &SVG_OPTS).unwrap();

//            // Build a pixmap to fill
//            let pixmap_size = tree.size().to_int_size();
//            tracing::info!("{pixmap_size:?}");
//            let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();

//            // Fill it
//            tracing::info!("Rendering SVG");
//            resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());
//            tracing::info!("Encoding as PNG");

//            pixmap.encode_png().unwrap_or_default()
//        }).await;

//        res.unwrap()
//    }
//}

