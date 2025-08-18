# Prehistory

Rough idea, this is something of a DM screen/note integration tool. Foundry is a very nice _display_ tool, it's not a
great _build_ tool; I want to export a 'view' of the underlying data to foundry (ideally via tiles), so that I can
incrementally expand the world.

Truncheon is there to manage all the math of encounter checking, hex populating, etc. The goal is to specify 'regions',
which are collections of 'fields' of hexes for specific purposes. So there is a 'terrain' field, an 'encounter' field, a
'weather' field, etc. these all get bundled up into a 'region' which is a bounded subset of axial coordinates on the
'world' map it inhabits. Regions can overlap, and being in a region can trigger effects on players, wandering enemies,
etc.

I think the way it may need to work is to control an assistant GM user via websocket. I do need to look at the rest API
module I saw though -- https://github.com/cclloyd/planeshift -- under it's hood is https://pptr.dev/ which may be an
alternative.

Ideally this gets compiled to WASM, and runs in-browser, I may be able to claw back the foundry api directly that way.

Ideally this tool stores abstract encounters (abstracting in scale, difficulty, etc), and manages all my usage die and
such.


##

TODO:

1. Polygon Shape for canvas, should draw each successive pair of points as a line.
2. Explore egui
3. Event loop for parsing commands from `input`
4. finish spiral impl.
5. Could look into writing out a .bmp or similar and then using ratatui-img to display it wholesale in terminal; this'd
   allow for in-terminal graphics but w/ higher fidelity/using image sets.
    - This would obviate the need for egui, but may be wildly more complicated. It also might be a fuckload easier, not
      sure.
    - Something like -- draw an SVG, render to bmp or whatever, display with terminal graphics.
    - I actually like that pipeline, take ratatui-image, render that instead of the canvas, the math still maths for an
      SVG but should be somewhat easier, then I just need to convert SVG -> some image format and load it.


The whole process can be wrapped up in a widget, the widget will render out an SVG based on changes to an underlying
datastructure, when the DS changes, it re-renders the image, and updates the UI accordingly. I'll want that to be async,
probably.

```rust
// example pulled from https://github.com/linebender/resvg/blob/main/crates/resvg/examples/custom_href_resolver.rs
// I mostly care about the render path and the templating step it does where it swaps out the image.
fn main() {
    let mut opt = usvg::Options::default();

    let ferris_image = std::sync::Arc::new(std::fs::read("./examples/ferris.png").unwrap());

    // We know that our SVG won't have DataUrl hrefs, just return None for such case.
    let resolve_data = Box::new(|_: &str, _: std::sync::Arc<Vec<u8>>, _: &usvg::Options| None);

    // Here we handle xlink:href attribute as string,
    // let's use already loaded Ferris image to match that string.
    let resolve_string = Box::new(move |href: &str, _: &usvg::Options| match href {
        "ferris_image" => Some(usvg::ImageKind::PNG(ferris_image.clone())),
        _ => None,
    });

    // Assign new ImageHrefResolver option using our closures.
    opt.image_href_resolver = usvg::ImageHrefResolver {
        resolve_data,
        resolve_string,
    };

    let svg_data = std::fs::read("./examples/custom_href_resolver.svg").unwrap();
    let tree = usvg::Tree::from_data(&svg_data, &opt).unwrap();

    let pixmap_size = tree.size().to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();

    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

    pixmap.save_png("custom_href_resolver.png").unwrap();
}
```

resvg claims some degree of speed, but I doubt I'll get extremely high framerates doing things this way, so egui may
still be the longterm play.


# 14-AUG-2025

## 1510

I got the svg pipeline working, it seems pretty snappy but I'm also not drawing it most of the time. Next step is to
extend the template stuff to allow for drawing shapes and whatnot.

# 17-AUG-2025

## 0130

Good progress. I factored out some caches, though the image one isn't working right just yet. I still need to review the
render function to make sure I got it wired up correctly.

I need to get the UI async for this approach to work. I don't need particularly high framerates for the map especially
(since it will be mostly static most of the time), and in fact a longer render time for the 'static' portion should make
the non-static portion easier to make it's own, transparent layer that gets dropped on top. Pulling the caches out to be
async also just makes sense, they're already tossing Arcs around with impugnity.

The image cache is currently caching the whole ImageKind, but I think it should actually cache the tag and data
separately, this would allow me to reconstruct the imagekind at the last minute, avoiding an extra Arc and an unpleasant
`.as_ref().clone()`

I'm still fighting to get it to fill the polyline hexagon with the image, but practically speaking I can also probably
just use `image` tags instead, and layer the hexgrid on top of it.

As it is, the process takes about 3s to render the svg as a png, I still can't actually see the image that's slowing it
down, but the cache works, I just eat a 3s render time.

Hey, it's down from the 53 minutes it took on the first run.

It also appears that the use of `defs` deeply slows the rendering down. So maybe just writing the `image` directly makes
sense. This is inline with what I want to do with `SVGTemplate` anyway, so I think that's the way I'll go.

I did notice that when working w/ the webp images, it would frequently miscalculate the size of the grid it should
render. I need to have this derive not from the SVG dimensions, but set the offset based on input values from the
widget, as well as the visible space and 'zoom' level of the widget. Some transform exists from the `ratatui-image`
widget's `Rect` and the SVG's native bounding box based on some scalar zoom level which would map coordinates and
heights and stuff. I'll need to figure that out and that should resolve the issue. It doesn't seem to happen with .pngs,
I assume because they are more strict about their bounding box. webp is a mystery to me, all file formats are mysteries,
really.

For now I'll just use pngs.

using images instead of the polyline does mean that I'll need to figure out how to convert from where-ever the `png` is
anchored to the center of the hex it contains. There may be a way to control where the drag point is in SVG, I don't
know.

## 1438

Definitely a render-box issue, I've added a hardcoded 100% rect to the template and it actually should have excluded
some stuff, it appears the width/height is controlled by parameters on the <svg> element.
controls what part of a larger document needs to be rendered, so I think that's also where general camera control logic
ends up.

https://www.w3.org/TR/SVG/coords.html#TransformProperty

Seems to be useful for this

# 18-AUG-2025

## 0921

The process for rendering a hexfield is going to involve the following parts:

1. Map from image dimensions (which I should look up on load and store in the cache) to nominal hex dimensions, scaling
   everything to the same scale. This'll require some math to determine the anchor points in the SVG for the image
2. Figure out the viewbox/viewport/svg size stuff. This might be just ensuring I'm not working near the SVG origin
   (since I think negative x/y values are not supported? Not sure. Might be able to solve this by making matching
    adjustments to the viewport offset when I select subsections).
        - Indeed, my example images have a big pile of surrounding transparency, the hexes are centered, though, so it
should be easy enough to find the center dynamically. which should allow placement relative to the center, it is
absolutely top-left anchored atm (by experimentation).
        - This may also explain why the `fill` wasn't working, it was, we were just only seeing the transparent section.
          I'll need to experiment to see if adding back the `defs` saves time once there are a lot of hexes being shown.
        - an upshot might be that I could make a _texture_ to apply to the hex, instead of a hex image; contiguous
regions of hexes could pull from coordinated parts of the texture and get semi-random changes to add variety.

3. Use existing hex math stuff to place all the hexes, this will probably need some more math to go from nominal hexes
   in radial space to the image dimensions which will be weird.

Before any of that I _have_ to make this shit async because it is so annoying to wait for the thing. It'd also be good
to get it to automatically rerender every few seconds or whatever.
