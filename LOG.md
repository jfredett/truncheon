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

## 1024

Looking at the async stuff, it may be that I need to separate the drawing side of the `SVG` stuff and the display. The
widget can have an async 'update' function which stores the pngdata in the struct, the render just splats the pngdata to
the screen.

## 1113

I started to tease apart the update/render parts of the svg so I can async it. The `picker` part is a little tricky to
locate, and I've got a lot of state that needs managing, but I think I should be able to pass through the picker
information (perhaps as part of the state for SVG?).

It feels like there should be a trait for 'Async' widgets (similar to stateful widgets), which are widgets with state
that endure a separate update loop. Each would get all the current layout information, so that, for instance, I can
pre-render the next SVG frame to the correct size, but one would get called to manage FPS and the other for actual state
upate?

IDK, feels like there's a pattern here, haven't found it yet.

I definitely think the layout should get cached, maybe I just don't like immediate-mode UI? :) It's just a set of rects
that get calculated, it's another thing that I could stick in an async update loop and reference everywhere.

One crazy idea here would be to refactor so the _whole UI_ is done as an SVG. At that point I'm just sort of streaming
graphics over the terminal protocol so it's a little bit silly, but I suppose that's probably what, e.g., canvas is
doing in a browser over HTTP.

## 1319

I think I'm moving inexorably towards an event-loop style system with a bunch of asynchronous handlers with a
framerate-managed render loop. The current method does a loop like:

```
1. Check for exit
2. No exit, then handle events (sync)
3. Update (async)
4. synchronize
5. Draw
```

What would be ideal is if the startup ran two systems, the frontend UI which just does the `draw` step on a
fixed-framerate-target loop, and a 'backend' component which is asynchronous. The frontend would capture events and send
them to the backend, and request data from the backend when needed to render.

# 21-AUG-2025

## 0102

Still not super happy with the PNG rendering performance, but there is much to do to improve it, and it's mostly
asynchronous now, all the major issues are worked out with the wiring, just needs a bunch of polish and tests.

I dislike how I'm managing the layout on multiple levels, but it's the most flexible thing for right now, eventually it
probably makes sense to have it have a fixed initial size so that the render loop can start without a blank screen. I've
already hardcoded it to use the `kitty` protocol to avoid all the `picker` nonsense that broke input for quite a while.

## 0924

I decided to go to sleep mid thought when I realized what time it was.

The way the layout works right now, it drops everything into a (fresh) hashmap on each call to `build_layout`, which is
pretty frequent. This means that I've got strings littered everywhere. In this form, it's quite easy to get hard crashes
due to uncatchable typos, which is not ideal. The obvious solution is to use an enum, but that adds some overhead to
re-arranging sections while I work on it, so I think I'm going to tolerate the pain until I've firmed up the UI design,
then encode it.

Next step is to chase down the scaling issue, and then work to send the actual png generation as an async task, which I
think should make the UI stay responsive during rendering.

## 1028

I think I need to create an entirely separate thread that communicates over a channel. I've been trying to get
`tokio::task` to work, but I can't seem to find the right invocation to get it to be non-blocking. Instead I could start
up the `App` and `UI` as separate threads that communicate, alternatively, a PNG renderer that runs in a separate
thread, maybe using a shared memory space? Not quite sure how I want to proceed.

## 1145

https://stackoverflow.com/questions/61752896/how-to-create-a-dedicated-threadpool-for-cpu-intensive-work-in-tokio

Seems relevant.

I also need to do some kind of caching/don't re-render if the template hasn't changed. Probably a simple hash scheme'd
do it.

# 25-AUG-2025

## 1538

Apparently ratatui_image has something for separating rendering, might be able to use that as an example of how to
structure the SVG rendering thread. The current `HEAD` of ratatui_image has a tokio example as well.

# 26-AUG-2025

## 1313

I think I need to focus on building a separate svg renderer that does a better job of separating the update/render
stuff. The `SVG` widget should basically first-and-forget the SVG render, and the update should look it up after.
Probably the 'simplest' way to do that is to hash the SVG Template after compiling the template, and have the renderer
cache based on that hash. I'd push the `template.render()` step _up_ to the `update` call, then on the `render` call
look up based on the current hash -- so that I don't clone the png data, I just retrieve an ARC to it from the
renderer.
# 26-AUG-2025

## 1313

I think I need to focus on building a separate svg renderer that does a better job of separating the update/render
stuff. The `SVG` widget should basically first-and-forget the SVG render, and the update should look it up after.
Probably the 'simplest' way to do that is to hash the SVG Template after compiling the template, and have the renderer
cache based on that hash.

# 10-OCT-2025

## 0033

I think I've found the actual issue with the svg rendered, it's blocking. It needs to run in it's own thread and the UI
talk to it, separate from `tokio`, started independently and globally available. `tokio` can run the UI and provide
requests to the renderer, updating as new frames are available. This _may_ be what `spawn_blocking` does, I haven't read
the docs enough to grok it yet. I grepped for it and it's not in current use anyway, so worth looking through, an old
stackoverflow [here](https://stackoverflow.com/questions/76965631/how-do-i-spawn-possibly-blocking-async-tasks-in-tokio)
also points toward it. My initial thought, however, was to either spawn a separate thread with an async channel, then
start tokio, obviating the `#[tokio::main]` handle and thus DIYing it with this additional thread. The renderer side
would also have to directly manage the Future I think, that seems messy.

I could also just try to implement Future for the svg rendering step itself, so that it's callable in the async context
directly. I think the implementation would just be a `spawn_blocking` call.

I may need to grant some access to the tokio runtime to do this?

https://tokio.rs/tokio/topics/bridging#spawning-things-on-a-runtime

Seems relevant.


# 20-OCT-2025

## 1352

I don't know that it fixed it, but it is slightly less laggy now. I need to hook up console or something to get data
out, but I think the issue is that `svg` is fundamentally going to be slow because it's just a long, unaccelerated
pipeline and ultimately it's not even really necessary; I could just work directly on the image buffer.

The upside of svg is that it is lightweight for a render-once-clientside application, which isn't my intent; and it's
nice to convert to other image formats and resolutions. The downside is that it is unlikely that I'll ever be able to
render it quickly enough to 'work'. I think next step is to start sticking probes around it and seeing if I can get some
timing data out, I suspect it will quickly show more blocking processes that need to be handled; and it will also
probably show that each of those blocking processes still take a long time to render. Low framerates for the thing are
tolerable, but dismal ones won't be, so I might still go for a 'render via ratatui-image' approach, but just manage a
DynamicImage (or some wrapping structure) which I copy out when rendering each frame. Then I can have a process
continually buffer a new frame and just asynchronously ask for the current one each actual rendered frame. Should be
significantly simpler and avoids me having to write a template file.

## 1425

I hooked up tokio_console and found some interesting info. In particular there are, as expected, quite a few blocking
functions. In particular there seem to be 4 copies of the render step running all the time, Only one task seems to ever
actually be running; which shouldn't be the case, and because of this I seem to remain pretty far down in the queue. I
think tokio is basically shoving blocking tasks onto threads, those tasks take a while to return so it creates another
one w/o cancelling the first, and so it quickly exhausts all it's available worker threads and spins waiting for all
it's stuff to finish. Trying to feed people in half the time by cooking twice as much food.

It seems that the problem 'should' be solved with messages, as I had been moving towards before. Sending a message can
be ignored if the render is already in progress, so you just have a little state machine on the far side and if it's
rendering, it adds another return address to the queue.

I want to benchmark the actual SVG rendering time for a reasonably complicated SVG, so I think I'm going to create a
separate map renderer that doesn't do any of the ratatui stuff, then I can figure out a best-possible render time; if
that is acceptable, then I like the idea of trying to keep the SVG-based approach since being able to keep the actual
content of the game in pure xml/text seems nice from a longevity point of view. Easy to store and compress, easy to
build another interpreter for, etc. If it's not, then using `image` and writing a simple rasterizer (I think that's the
right term, a thing that takes abstract model of space and turns it into pixels is what I mean, I'm not a graphics guy)
should be the path, since I'll have all the control I can manage and can make the pipeline async-throughout, which would
hopefully resolve the issue as well. In any case I think it's a good opportunity to use the `state-machine` port.

# 26-OCT-2025

## 1244

I had an idea last night that I think squares everything up. Instead of pulling images in and rendering 'on-demand', the
renderer should instead run as an independent process that allows 'subscribers'

The pipeline would then be for an internal model to update, which the renderer can observe; the renderer creates a new
frame and sends a pointer to each subscriber, the UI is a subscriber. On application start, the model is booted, then
the renderer, then the UI 'subscribes' to the renderer and passes the imageref to whatever widgets it likes.

Ideally it's a queue, so the UI event loop runs, places the new image in the 'right slot', but only renders the actual
UI every `1/f` seconds (f = framerate), So if the renderer ends up generating 3 frames before the render call happens,
it should only copy the image for the last of those frames, the rest just being an Arc/other ref to the image data. This
also means the renderer can be 'smart' in the sense that I can create an SVG of the whole map, and dynamically grab
portions of it after rendering it once. I can also have the renderer manage multiple layers and other tweaks by making
the image reference a little smarter. This nets me the SVG-based approach I wanted (for transportability) and the speed
I'm looking for (since the map should change pretty rarely).

# 18-NOV-2025

## 1050

EU V has been eating my whole life. Factorio too.

I've also been thinking about this and what I want to do. Ultimately, and I recognize this is a function of having been
playing EU V pretty much nonstop since it came out, I think I want to push truncheon in the direction of having some
pop-based simulatation component. It's not exactly EU, but it's definitely adjacent.

The idea would be to have a few classes of 'pop's, which would be user definable with 'promotion rules' between them. So
for my D&D campaign I might have two general classes of pops, "Mundane" and "Sorted", with no promotion between them
(generally), and within "Sorted" I might have a subclass for each level with promotion rules about how muse "Applied
Vesper" the Sorted has acquired. As they acquire enough, they promote to the next level.

I can then make arbitrary rules for how those pops behave, move around the world, etc. Mundane pops would tend to
congregate with other Mundanes, give them logic to form villages and stuff; and have the sorted run around. Maybe
another class for Monsters, etc.

Then I give them a big hex grid to hang around in and let them do their thing; I can still use it to track a party in
the same world, except now as they move, so does everything else.

The world would have a bunch of `Good`s as well, produced by the various pop classes by doing `Job`s. Jobs produce goods
or alter a `Locality`, of which a `Region` may have many. `Regions` are fields of `Hex`es associated with a certain
`Scale`. Regions exist within a `World`, and are connected to other regions within a specific hex, and may be contained
within other regions. The connections are arbitrary scale.

## 1105

Another way of approaching this that might be interesting. Hexes are nice for rendering, but representation as a fixed
field is a little limiting. In particular, it would be nice to just think in terms of topology, and then maybe
superimpose some hex system when it's appropriate?

## 1231

I roughed out some ideas on paper, but rapidly realized I'll also need some sense of a 'timeline' for each party, as
different places within the worlds I manage may move at different rates.

# 19-NOV-2025

## 1220

I think I have a theory of how it should work.

Each Pop and Locality is equipped with a Timeline. Timelines are sequences of a shared pool of Events which are
_Simultaneous_ across those timelines but not necessarily across others. When an event occurs, all those present for it
mark it in their timeline as an event.

Each Locality is part of a Region, multiple Regions are part of a World, and a World has a special timeline that all
inhabitants of that World share. The World Timeline ("Worldline") contains every event that happens in the world on a
shared reference clock; but other worlds can proceed at different paces to it. So for my setup I currently have 4
worlds:

1. Eret
2. Ynn
3. Stygia
4. The IM's lair

Time moves differently in Ynn, but is synced with Eret in the other cases. Each Worldline advances according to GM Fiat;
generally, instead, timelines calculate _where they should be on the other timeline_ when travelling between worlds.
Take this simpler example:


| Ratio    | World A | World B |
|----------|---------|---------|
| World A  |   1     |   2     |
| World B  |   0.5   |   1     |


So if a Pop is in World A, then enters World B and stays for 1 hour on world B, then returns to World A, they will
return at `T1 = T0 + 0.5h`, where `T0` is the time of the previous event on world A. If they wait an hour on A, then return to B,
they will arrive at `T1+2h`, or `T0+2.5h`.

So long as we know the sequence of events undertaken by a Pop, we can always calculate where we should arrive on the
target timeline.

The goal is to just keep a consistent history of each Pop, the timeline of each world is just a function of the
relationship between two timeline's clock-speed and the events which cause them to co-align. Events get tagged with the
`Locality` they occur in, and the Worldlines are thus calculated. An event has a `Start` time local to its worldline,
which will account for gaps.

This creates a final structure like this:

- Universe
    - Eret
        - Upper Carpathia
        - Ulm
        - Edom
        - Corinth
    - Ynn
        - 3x Eret
    - Stygia
        - 1x Eret
    - The IM's Lair
        - 1x Eret

With connections such that Eret is the Hub, and the others are spokes, which simplifies the matrix above.

It is possible, likely even, that this will allow for paradoxes. That's intentional. When a pardox occurs, the worlds
should split into new, independent worlds, with that Event representing the branching point. Any timeline can split.
This occurs naturally as people travel across different worlds.

This model also allows for pseudorelativistic effects between worlds, since travel is 'instant' inasmuch as there is no
space to travel, but the two worlds run their clocks at 'different rates' so things change slightly as you move 'further
away'. The relative clock speed on Mercury, for instance, is somewhat slower than an equivalent clock on Earth, losing
about 1 minute every 50 years or so, but varying considerably in the exact rate of exchange between an terran minute and
a mercurial one. This would model that as a fixed ratio (though in actuality it is not fixed, but based on the relative
positions of the various involved bodies. A future enhancement might turn the relativity matrix above into a dynamically
calculated function, but that'd require additional work to make the various 'worlds' more like physical objects.

# 20-NOV-2025

## 1441

All of this does nothing in the face of the actual current problem which is rendering slowness, but it has been fun to
dive down the rabbit hole a bit.

# 1-DEC-2025

## 1334

I think I have settled in on what this is, it is a campaign system. A GM of a TTRPG chooses a TTRPG system, and creates
maps/players/etc and then tracks their goings on via this system, which manages hex/depth crawls, and tracks multiple
parties of both PCs and NPCs and generates "Decisions" which end up assigned to different players. Most will be left to
the GM, who can use a scripting language to make decisions automatically or intervene to make specific decisions when
they care to. Others get assigned to the players and the GM presents those decisions organically as a part of play. The
point of the system is to track the timelines of PCs and NPCs so that any individual's history is inspectable by the GM
so that the GM can have a full understanding of how things occurred within the game world. The idea is that it provides
tools to track the tedious parts of the game, separate from any particular representation of the game. It is system
agnostic, it still requires a GM to intervene and encode the effects of decisions that players make, but it tries to
tuck away all the die rolling to generate encounters, terrain, etc, and helps track the overall difficulty of the world
and accrue details on what players are doing as they do it.

I think I'm going to switch to using Bevy with a Ratatui frontend. I still want this to be usable via CLI, I still want
it to integrate with Foundry, though I'm less concerned with the SVG output-based approach and more interested in the
already existing bevy_ratatui and bevy_ratatui_camera, which I think cover my needs nicely.

# 2-DEC-2025

## 1220

I'm working on getting everything organized. I decided to just drop the current state into a commit, and then start
cutting through things till I have something functional again. I'm going to rip out all the SVG stuff after that commit
as I don't intend to use it, and I think the more I pare down the better time I'm going to have.

# 4-DEC-2025

## 1457

Feeling a bit frustrated trying to cajole `bevy_ratatui_camera` into `ratatui_image` and get something sensible to pop
out. I suspect many things, but I've also been thinking a bit about how to approach this if I can't get it working at
the CLI level. In particular, getting it to run in a browser is appealing from a delivery perspective, it would be
significantly simpler to embed it as a WebGL/WSGL + WASM binary, at which point I could ostensibly have hot-reloading as
well; it minimizes the testability to some extent (`insta` probably won't work), but it would mean I could just use
`bevy` as intended, rather than trying to force it into a `ratatui_image` block.

Another alternative is to move development to be directly on dragon rather than over SSH. I like the idea of trying to
get something that uses kitty's image protocol to do real 'graphics' over SSH, but I also just want the thing to work so
I can get on with the actual point of this project.

# 8-DEC-2025

## 1419

Fighting with nixos and dioxus and bevy and friends. Makes me think of building a VM or container for just dev. NixOS is
a fantastic thing for static environments, but it's very frustrating to use in a dev environment.

An Arch box w/ home-manager may be in the future.


