# 🖌 egui: an easy-to-use GUI in pure Rust

[<img alt="github" src="https://img.shields.io/badge/github-emilk/egui-8da0cb?logo=github" height="20">](https://github.com/foxxcn/eguizh)
[![Latest version](https://img.shields.io/crates/v/egui.svg)](https://crates.io/crates/egui)
[![Documentation](https://docs.rs/egui/badge.svg)](https://docs.rs/egui)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![Build Status](https://github.com/foxxcn/eguizh/workflows/Rust/badge.svg)](https://github.com/foxxcn/eguizh/actions/workflows/rust.yml)
[![MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/foxxcn/eguizh/blob/main/LICENSE-MIT)
[![Apache](https://img.shields.io/badge/license-Apache-blue.svg)](https://github.com/foxxcn/eguizh/blob/main/LICENSE-APACHE)
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/foxxcn/eguizh)

---

👉 [Click to run the web demo](https://www.egui.rs/#demo) 👈

egui (pronounced "e-gooey") is a simple, fast, and highly portable immediate mode GUI library for Rust. egui runs on the web, natively, and [in your favorite game engine](#integrations).

egui aims to be the easiest-to-use Rust GUI library, and the simplest way to make a web app in Rust.

egui can be used anywhere you can draw textured triangles, which means you can easily integrate it into your game engine of choice.

[`eframe`](https://github.com/foxxcn/eguizh/tree/main/crates/eframe) is the official egui framework, which supports writing apps for Web, Linux, Mac, Windows, and Android.

## Example

```rust
ui.heading("My egui Application");
ui.horizontal(|ui| {
    ui.label("Your name: ");
    ui.text_edit_singleline(&mut name);
});
ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
if ui.button("Increment").clicked() {
    age += 1;
}
ui.label(format!("Hello '{name}', age {age}"));
ui.image(egui::include_image!("ferris.png"));
```

<img alt="Dark mode" src="https://github.com/user-attachments/assets/3b446d29-99d8-4c82-86bb-4d8ef0516017"> &nbsp; &nbsp; <img alt="Light mode" src="https://github.com/user-attachments/assets/a5e7da93-89a8-4ba0-86b8-0fa2228a4f62" height="278">

## Sections:

- [Example](#example)
- [Quick start](#quick-start)
- [Demo](#demo)
- [Goals](#goals)
- [State / features](#state)
- [Dependencies](#dependencies)
- [Who is egui for?](#who-is-egui-for)
- [Integrations](#integrations)
- [Why immediate mode](#why-immediate-mode)
- [FAQ](#faq)
- [Other](#other)
- [Credits](#credits)

## Quick start

There are simple examples in [the `examples/` folder](https://github.com/foxxcn/eguizh/blob/main/examples/). If you want to write a web app, then go to <https://github.com/emilk/eframe_template/> and follow the instructions. The official docs are at <https://docs.rs/egui>. For inspiration and more examples, check out the [the egui web demo](https://www.egui.rs/#demo) and follow the links in it to its source code.

If you want to integrate egui into an existing engine, go to the [Integrations](#integrations) section.

## Demo

[Click to run egui web demo](https://www.egui.rs/#demo) (works in any browser with Wasm and WebGL support). Uses [`eframe`](https://github.com/foxxcn/eguizh/tree/main/crates/eframe).

To test the demo app locally, run `cargo run --release -p egui_demo_app`.

The native backend is [`egui_glow`](https://github.com/foxxcn/eguizh/tree/main/crates/egui_glow) (using [`glow`](https://crates.io/crates/glow)) and should work out-of-the-box on Mac and Windows, but on Linux you need to first run:

`sudo apt-get install -y libclang-dev libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel`

**NOTE**: This is just for the demo app - egui itself is completely platform agnostic!

## Goals

- The easiest to use GUI library
- Responsive: target 60 Hz in debug build
- Friendly: difficult to make mistakes, and shouldn't panic
- Portable: the same code works on the web and as a native app
- Easy to integrate into any environment
- A simple 2D graphics API for custom painting ([`epaint`](https://docs.rs/epaint)).
- Pure immediate mode: no callbacks
- Extensible: [easy to write your own widgets for egui](https://github.com/foxxcn/eguizh/blob/main/crates/egui_demo_lib/src/demo/toggle_switch.rs)
- Modular: You should be able to use small parts of egui and combine them in new ways
- Safe: there is no `unsafe` code in egui
- Minimal dependencies

egui is _not_ a framework. egui is a library you call into, not an environment you program for.

**NOTE**: egui does not claim to have reached all these goals yet! egui is still work in progress.

### Non-goals

- Become the most powerful GUI library
- Native looking interface

## State

egui is in active development. It works well for what it does, but it lacks many features and the interfaces are still in flux. New releases will have breaking changes.

### Features

- Widgets: label, text button, hyperlink, checkbox, radio button, slider, draggable value, text editing, color picker, spinner
- Images
- Layouts: horizontal, vertical, columns, automatic wrapping
- Text editing: multiline, copy/paste, undo, emoji supports
- Windows: move, resize, name, minimize and close. Automatically sized and positioned.
- Regions: resizing, vertical scrolling, collapsing headers (sections), panels
- Rendering: Anti-aliased rendering of lines, circles, text and convex polygons.
- Tooltips on hover
- Accessibility via [AccessKit](https://accesskit.dev/)
- Label text selection
- And more!

Check out the [3rd party egui crates wiki](https://github.com/emilk/egui/wiki/3rd-party-egui-crates) for even more
widgets and features, maintained by the community.

<img src="https://github.com/user-attachments/assets/13e73b76-e456-42bd-8ec9-220802834268" width="50%">

Light Theme:

<img src="https://github.com/user-attachments/assets/2e38972c-a444-4894-b32f-47a2719cf369" width="50%">

## Dependencies

`egui` has a minimal set of default dependencies:

* [`ab_glyph`](https://crates.io/crates/ab_glyph)
* [`ahash`](https://crates.io/crates/ahash)
* [`bitflags`](https://crates.io/crates/bitflags)
* [`nohash-hasher`](https://crates.io/crates/nohash-hasher)
* [`parking_lot`](https://crates.io/crates/parking_lot)

Heavier dependencies are kept out of `egui`, even as opt-in.
All code in `egui` is Wasm-friendly (even outside a browser).

To load images into `egui` you can use the official [`egui_extras`](https://github.com/foxxcn/eguizh/tree/main/crates/egui_extras) crate.

[`eframe`](https://github.com/foxxcn/eguizh/tree/main/crates/eframe) on the other hand has a lot of dependencies, including [`winit`](https://crates.io/crates/winit), [`image`](https://crates.io/crates/image), graphics crates, clipboard crates, etc,

## Who is egui for?

egui aims to be the best choice when you want a simple way to create a GUI, or you want to add a GUI to a game engine.

If you are not using Rust, egui is not for you. If you want a GUI that looks native, egui is not for you. If you want something that doesn't break when you upgrade it, egui isn't for you (yet).

But if you are writing something interactive in Rust that needs a simple GUI, egui may be for you.

## Integrations

egui is built to be easy to integrate into any existing game engine or platform you are working on.
egui itself doesn't know or care on what OS it is running or how to render things to the screen - that is the job of the egui integration.

An integration needs to do the following each frame:

- **Input**: Gather input (mouse, touches, keyboard, screen size, etc) and give it to egui
- Call into the application GUI code
- **Output**: Handle egui output (cursor changes, paste, texture allocations, …)
- **Painting**: Render the triangle mesh egui produces (see [OpenGL example](https://github.com/foxxcn/eguizh/blob/main/crates/egui_glow/src/painter.rs))

### Official integrations

These are the official egui integrations:

- [`eframe`](https://github.com/foxxcn/eguizh/tree/main/crates/eframe) for compiling the same app to web/wasm and desktop/native. Uses `egui-winit` and `egui_glow` or `egui-wgpu`
- [`egui_glow`](https://github.com/foxxcn/eguizh/tree/main/crates/egui_glow) for rendering egui with [glow](https://github.com/grovesNL/glow) on native and web, and for making native apps
- [`egui-wgpu`](https://github.com/foxxcn/eguizh/tree/main/crates/egui-wgpu) for [wgpu](https://crates.io/crates/wgpu) (WebGPU API)
- [`egui-winit`](https://github.com/foxxcn/eguizh/tree/main/crates/egui-winit) for integrating with [winit](https://github.com/rust-windowing/winit)

### 3rd party integrations

Check the wiki to find [3rd party integrations](https://github.com/emilk/egui/wiki/3rd-party-integrations)
and [egui crates](https://github.com/emilk/egui/wiki/3rd-party-egui-crates).

### Writing your own egui integration

Missing an integration for the thing you're working on? Create one, it's easy!
See <https://docs.rs/egui/latest/egui/#integrating-with-egui>.

## Why immediate mode

`egui` is an [immediate mode GUI library](https://en.wikipedia.org/wiki/Immediate_mode_GUI), as opposed to a _retained mode_ GUI library. The difference between retained mode and immediate mode is best illustrated with the example of a button: In a retained GUI you create a button, add it to some UI and install some on-click handler (callback). The button is retained in the UI, and to change the text on it you need to store some sort of reference to it. By contrast, in immediate mode you show the button and interact with it immediately, and you do so every frame (e.g. 60 times per second). This means there is no need for any on-click handler, nor to store any reference to it. In `egui` this looks like this: `if ui.button("Save file").clicked() { save(file); }`.

A more detailed description of immediate mode can be found [in the `egui` docs](https://docs.rs/egui/latest/egui/#understanding-immediate-mode).

There are advantages and disadvantages to both systems.

The short of it is this: immediate mode GUI libraries are easier to use, but less powerful.

### Advantages of immediate mode

#### Usability

The main advantage of immediate mode is that the application code becomes vastly simpler:

- You never need to have any on-click handlers and callbacks that disrupts your code flow.
- You don't have to worry about a lingering callback calling something that is gone.
- Your GUI code can easily live in a simple function (no need for an object just for the UI).
- You don't have to worry about app state and GUI state being out-of-sync (i.e. the GUI showing something outdated), because the GUI isn't storing any state - it is showing the latest state _immediately_.

In other words, a whole lot of code, complexity and bugs are gone, and you can focus your time on something more interesting than writing GUI code.

### Disadvantages of immediate mode

#### Layout

The main disadvantage of immediate mode is it makes layout more difficult. Say you want to show a small dialog window in the center of the screen. To position the window correctly the GUI library must first know the size of it. To know the size of the window the GUI library must first layout the contents of the window. In retained mode this is easy: the GUI library does the window layout, positions the window, then checks for interaction ("was the OK button clicked?").

In immediate mode you run into a paradox: to know the size of the window, we must do the layout, but the layout code also checks for interaction ("was the OK button clicked?") and so it needs to know the window position _before_ showing the window contents. This means we must decide where to show the window _before_ we know its size!

This is a fundamental shortcoming of immediate mode GUIs, and any attempt to resolve it comes with its own downsides.

One workaround is to store the size and use it the next frame. This produces a frame-delay for the correct layout, producing occasional flickering the first frame something shows up. `egui` does this for some things such as windows and grid layouts.

The "first-frame jitter" can be covered up with an extra _pass_, which egui supports via `Context::request_discard`.
The downside of this is the added CPU cost of a second pass, so egui only does this in very rare circumstances (the majority of frames are single-pass).

For "atomic" widgets (e.g. a button) `egui` knows the size before showing it, so centering buttons, labels etc is possible in `egui` without any special workarounds.

See [this issue](https://github.com/emilk/egui/issues/4378) for more.

#### CPU usage

Since an immediate mode GUI does a full layout each frame, the layout code needs to be quick. If you have a very complex GUI this can tax the CPU. In particular, having a very large UI in a scroll area (with very long scrollback) can be slow, as the content needs to be laid out each frame.

If you design the GUI with this in mind and refrain from huge scroll areas (or only lay out the part that is in view) then the performance hit is generally pretty small. For most cases you can expect `egui` to take up 1-2 ms per frame, but `egui` still has a lot of room for optimization (it's not something I've focused on yet). `egui` only repaints when there is interaction (e.g. mouse movement) or an animation, so if your app is idle, no CPU is wasted.

If your GUI is highly interactive, then immediate mode may actually be more performant compared to retained mode. Go to any web page and resize the browser window, and you'll notice that the browser is very slow to do the layout and eats a lot of CPU doing it. Resize a window in `egui` by contrast, and you'll get smooth 60 FPS at no extra CPU cost.

#### IDs

There are some GUI state that you want the GUI library to retain, even in an immediate mode library such as `egui`. This includes position and sizes of windows and how far the user has scrolled in some UI. In these cases you need to provide `egui` with a seed of a unique identifier (unique within the parent UI). For instance: by default `egui` uses the window titles as unique IDs to store window positions. If you want two windows with the same name (or one window with a dynamic name) you must provide some other ID source to `egui` (some unique integer or string).

`egui` also needs to track which widget is being interacted with (e.g. which slider is being dragged). `egui` uses unique IDs for this as well, but in this case the IDs are automatically generated, so there is no need for the user to worry about it. In particular, having two buttons with the same name is no problem (this is in contrast with [`Dear ImGui`](https://github.com/ocornut/imgui)).

Overall, ID handling is a rare inconvenience, and not a big disadvantage.

## FAQ

Also see [GitHub Discussions](https://github.com/emilk/egui/discussions/categories/q-a).

### Can I use `egui` with non-latin characters?

Yes! But you need to install your own font (`.ttf` or `.otf`) using [`Context::set_fonts`](https://docs.rs/egui/latest/egui/struct.Context.html#method.set_fonts).

### Can I customize the look of egui?

Yes! You can customize the colors, spacing, fonts and sizes of everything using `Context::set_style`.

This is not yet as powerful as say CSS, [but this is going to improve](https://github.com/emilk/egui/issues/3284).

Here is an example (from https://github.com/a-liashenko/TinyPomodoro):

<img src="https://github.com/user-attachments/assets/e6107237-2547-41d6-996b-9a20ae0345ab" width="50%">

### How do I use egui with `async`?

If you call `.await` in your GUI code, the UI will freeze, which is very bad UX. Instead, keep the GUI thread non-blocking and communicate with any concurrent tasks (`async` tasks or other threads) with something like:

- Channels (e.g. [`std::sync::mpsc::channel`](https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html)). Make sure to use [`try_recv`](https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html#method.try_recv) so you don't block the gui thread!
- `Arc<Mutex<Value>>` (background thread sets a value; GUI thread reads it)
- [`poll_promise::Promise`](https://docs.rs/poll-promise)
- [`eventuals::Eventual`](https://docs.rs/eventuals/latest/eventuals/struct.Eventual.html)
- [`tokio::sync::watch::channel`](https://docs.rs/tokio/latest/tokio/sync/watch/fn.channel.html)

### How do I create a file dialog?

The async version of [rfd](https://docs.rs/rfd/latest/rfd/) supports both native and Wasm. See example app here https://github.com/woelper/egui_pick_file which also has a demo available via [gitub pages](https://woelper.github.io/egui_pick_file/).

### What about accessibility, such as screen readers?

egui includes optional support for [AccessKit](https://accesskit.dev/), which currently implements the native accessibility APIs on Windows and macOS. This feature is enabled by default in eframe. For platforms that AccessKit doesn't yet support, including web, there is an experimental built-in screen reader; in [the web demo](https://www.egui.rs/#demo) you can enable it in the "Backend" tab.

The original discussion of accessibility in egui is at <https://github.com/emilk/egui/issues/167>. Now that AccessKit support is merged, providing a strong foundation for future accessibility work, please open new issues on specific accessibility problems.

### What is the difference between [egui](https://docs.rs/egui) and [eframe](https://github.com/foxxcn/eguizh/tree/main/crates/eframe)?

`egui` is a 2D user interface library for laying out and interacting with buttons, sliders, etc.
`egui` has no idea if it is running on the web or natively, and does not know how to collect input or show things on screen.
That is the job of _the integration_ or _backend_.

It is common to use `egui` from a game engine (using e.g. [`bevy_egui`](https://docs.rs/bevy_egui)),
but you can also use `egui` stand-alone using `eframe`. `eframe` has integration for web and native, and handles input and rendering.
The _frame_ in `eframe` stands both for the frame in which your egui app resides and also for "framework" (`eframe` is a framework, `egui` is a library).

### How do I render 3D stuff in an egui area?

There are multiple ways to combine egui with 3D. The simplest way is to use a 3D library and have egui sit on top of the 3D view. See for instance [`bevy_egui`](https://github.com/mvlabat/bevy_egui) or [`three-d`](https://github.com/asny/three-d).

If you want to embed 3D into an egui view there are two options:

#### `Shape::Callback`

Example:
* <https://github.com/emilk/egui/blob/main/examples/custom_3d_glow/src/main.rs>

`Shape::Callback` will call your code when egui gets painted, to show anything using whatever the background rendering context is. When using [`eframe`](https://github.com/foxxcn/eguizh/tree/main/crates/eframe) this will be [`glow`](https://github.com/grovesNL/glow). Other integrations will give you other rendering contexts, if they support `Shape::Callback` at all.

#### Render-to-texture

You can also render your 3D scene to a texture and display it using [`ui.image(…)`](https://docs.rs/egui/latest/egui/struct.Ui.html#method.image). You first need to convert the native texture to an [`egui::TextureId`](https://docs.rs/egui/latest/egui/enum.TextureId.html), and how to do this depends on the integration you use.

Examples:

- Using [`egui-miniquad`](https://github.com/not-fl3/egui-miniquad): https://github.com/not-fl3/egui-miniquad/blob/master/examples/render_to_egui_image.rs

## Other

### Conventions and design choices

All coordinates are in screen space coordinates, with (0, 0) in the top left corner

All coordinates are in logical "points" which may consist of many physical pixels.

All colors have premultiplied alpha, unless otherwise stated.

egui uses the builder pattern for construction widgets. For instance: `ui.add(Label::new("Hello").text_color(RED));` I am not a big fan of the builder pattern (it is quite verbose both in implementation and in use) but until Rust has named, default arguments it is the best we can do. To alleviate some of the verbosity there are common-case helper functions, like `ui.label("Hello");`.

Instead of using matching `begin/end` style function calls (which can be error prone) egui prefers to use `FnOnce` closures passed to a wrapping function. Lambdas are a bit ugly though, so I'd like to find a nicer solution to this. More discussion of this at <https://github.com/emilk/egui/issues/1004#issuecomment-1001650754>.

egui uses a single `RwLock` for short-time locks on each access of `Context` data. This is to leave implementation simple and transactional and allow users to run their UI logic in parallel. Instead of creating mutex guards, egui uses closures passed to a wrapping function, e.g. `ctx.input(|i| i.key_down(Key::A))`. This is to make it less likely that a user would accidentally double-lock the `Context`, which would lead to a deadlock.

### Inspiration

The one and only [Dear ImGui](https://github.com/ocornut/imgui) is a great Immediate Mode GUI for C++ which works with many backends. That library revolutionized how I think about GUI code and turned GUI programming from something I hated to do to something I now enjoy.

### Name

The name of the library and the project is "egui" and pronounced as "e-gooey". Please don't write it as "EGUI".

The library was originally called "Emigui", but was renamed to "egui" in 2020.

## Credits

egui author and maintainer: Emil Ernerfeldt ([@emilk](https://github.com/emilk)).

Notable contributions by:

* [@n2](https://github.com/n2): [Mobile web input and IME support](https://github.com/emilk/egui/pull/253)
* [@optozorax](https://github.com/optozorax): [Arbitrary widget data storage](https://github.com/emilk/egui/pull/257)
* [@quadruple-output](https://github.com/quadruple-output): [Multitouch](https://github.com/emilk/egui/pull/306)
* [@EmbersArc](https://github.com/EmbersArc): [Plots](https://github.com/emilk/egui/pulls?q=+is%3Apr+author%3AEmbersArc)
* [@AsmPrgmC3](https://github.com/AsmPrgmC3): [Proper sRGBA blending for web](https://github.com/emilk/egui/pull/650)
* [@AlexApps99](https://github.com/AlexApps99): [`egui_glow`](https://github.com/emilk/egui/pull/685)
* [@mankinskin](https://github.com/mankinskin): [Context menus](https://github.com/emilk/egui/pull/543)
* [@KentaTheBugMaker](https://github.com/KentaTheBugMaker): [Port glow painter to web](https://github.com/emilk/egui/pull/868)
* [@danielkeller](https://github.com/danielkeller): [`Context` refactor](https://github.com/emilk/egui/pull/1050)
* [@MaximOsipenko](https://github.com/MaximOsipenko): [`Context` lock refactor](https://github.com/emilk/egui/pull/2625)
* [@mwcampbell](https://github.com/mwcampbell): [AccessKit](https://github.com/AccessKit/accesskit) [integration](https://github.com/emilk/egui/pull/2294)
* [@hasenbanck](https://github.com/hasenbanck), [@s-nie](https://github.com/s-nie), [@Wumpf](https://github.com/Wumpf): [`egui-wgpu`](https://github.com/emilk/egui/tree/main/crates/egui-wgpu)
* [@jprochazk](https://github.com/jprochazk): [egui image API](https://github.com/emilk/egui/issues/3291)
* And [many more](https://github.com/emilk/egui/graphs/contributors?type=a).

egui is licensed under [MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE).

- The flattening algorithm for the cubic bezier curve and quadratic bezier curve is from [lyon_geom](https://docs.rs/lyon_geom/latest/lyon_geom/)

Default fonts:

- `emoji-icon-font.ttf`: [Copyright (c) 2014 John Slegers](https://github.com/jslegers/emoji-icon-font) , MIT License
- `SourceCodePro-Regular.ttf`: <https://github.com/adobe-fonts/source-code-pro>, [SIL Open Font License](https://scripts.sil.org/cms/scripts/page.php?site_id=nrsi&id=OFL)
- `NotoEmoji-Regular.ttf`: [google.com/get/noto](https://google.com/get/noto), [SIL Open Font License](https://scripts.sil.org/cms/scripts/page.php?site_id=nrsi&id=OFL)
- `SourceHanSerifCN-VF.ttf OR SourceHanSansCN-VF.ttf`:<https://github.com/adobe-fonts/source-han-sans>, [SIL Open Font License](https://scripts.sil.org/cms/scripts/page.php?site_id=nrsi&id=OFL)

---
