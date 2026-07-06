# LTK Foundation Layer — Phase 1 Architecture

**LTK (Lion Tool Kit)** is the official native UI framework of LionOS.
Phase 1 delivers the complete Foundation Layer: 100+ modules organized into
20 crates, covering every concern from rendering to accessibility.

---

## 1. Mission & Scope

LTK replaces GTK on Linux with a Rust-first, Wayland-native, declarative-first
UI framework designed to last 20+ years.  It is simultaneously:

- A **runtime** (Wayland surfaces, GPU compositing, input routing)
- A **design system** (tokens, themes, motion, typography)
- A **component library** (via LTK Toolkit layer, Phase 2)
- An **accessibility platform** (AT-SPI2 bridge, full keyboard nav)
- A **developer platform** (hot-reload, inspector, plugin engine)

---

## 2. Guiding Principles

| Principle | Rule |
|-----------|------|
| **Modularity** | Every concern is a crate; nothing is monolithic |
| **Composition** | Trait objects + composition, never deep inheritance |
| **Zero unsafe by default** | Unsafe only in GPU/FFI boundary crates, audited |
| **Data-oriented design** | Flat arrays + indices over pointer trees |
| **Declarative first** | Slint `.slint` drives UI; Rust drives logic |
| **Accessibility is core** | AT-SPI2 tree built alongside visual tree |
| **Future-proof APIs** | Semver, versioned extension points everywhere |
| **Deterministic performance** | No hidden GC; explicit arenas; measured frames |

---

## 3. Crate Map

```
ltk-foundation/
├── crates/
│   ├── ltk-core          Shared types, IDs, errors, sync primitives
│   ├── ltk-design        Color, typography, spacing, elevation, icon, motion tokens
│   ├── ltk-layout        Measure/arrange/constraint solver, all layout algorithms
│   ├── ltk-render        Scene graph, GPU/CPU backends, compositor, canvas
│   ├── ltk-input         Keyboard, mouse, touch, gesture, drag-and-drop
│   ├── ltk-event         Event bus, signal/slot, commands, undo/redo
│   ├── ltk-animation     Timeline, physics, spring, easing, state machines
│   ├── ltk-state         Property system, reactivity, bindings, change detection
│   ├── ltk-resources     Asset manager, caches, lazy loading
│   ├── ltk-locale        i18n, l10n, BiDi, formatters
│   ├── ltk-a11y          AT-SPI2, accessibility tree, screen reader bridge
│   ├── ltk-window        Window abstraction, display/monitor/DPI management
│   ├── ltk-platform      Linux/LionOS platform glue, environment detection
│   ├── ltk-perf          Profiler, frame analyzer, memory analyzer
│   ├── ltk-security      Permissions, sandbox interface, secure clipboard
│   ├── ltk-devtools      Logger, crash handler, hot-reload, inspector
│   ├── ltk-testing       Test harness, UI testing, snapshot, a11y testing
│   ├── ltk-plugin        Plugin/extension system, WASM sandbox, service registry
│   ├── ltk-wayland       Wayland protocol implementations (XDG, decoration, etc.)
│   └── ltk-gl / ltk-vk   OpenGL 4.6 and Vulkan 1.3 rendering backends
```

---

## 4. Module Inventory (120 Modules)

### 4.1 ltk-core (15 modules)

| Module | Responsibility |
|--------|---------------|
| `core::id` | Globally unique typed IDs (WidgetId, LayerId, …) |
| `core::arena` | Typed arena allocators for widget trees |
| `core::slot_map` | SlotMap-based component storage |
| `core::geometry` | Point, Size, Rect, Insets, Transform2D, Transform3D |
| `core::color` | Color (sRGB, linear, P3, OKLCH) with alpha, conversion |
| `core::string` | SmallString, SharedString, localized-string wrapper |
| `core::path` | Filesystem path abstractions |
| `core::error` | LtkError enum, LtkResult, error context |
| `core::sync` | LtkMutex, LtkRwLock, LtkAtomic wrappers |
| `core::task` | Async task executor interface, spawn_ui, spawn_bg |
| `core::time` | Duration, Instant, FrameTime, monotonic clock |
| `core::version` | SemVer struct, VersionRange, compatibility checks |
| `core::flags` | Bitflag macro + typed flag collections |
| `core::callback` | Type-erased Fn callback, WeakCallback, CallbackList |
| `core::env` | Environment detection (display server, GPU, a11y) |

### 4.2 ltk-design (26 modules)

#### Color System (8)
| Module | Responsibility |
|--------|---------------|
| `design::color_tokens` | Named semantic color tokens (primary, surface, error…) |
| `design::color_generator` | Algorithmic palette from seed hue (OKLCH-based) |
| `design::dynamic_colors` | Runtime color overrides, wallpaper-extracted accents |
| `design::accent_colors` | User-chosen accent propagation through token tree |
| `design::semantic_colors` | Role → token mapping (success, warning, danger…) |
| `design::contrast_engine` | WCAG AA/AAA contrast checking, auto color adjustment |
| `design::palette_manager` | Palette CRUD, import/export, built-in palettes |
| `design::gradient_system` | Linear/radial/conical/mesh gradient descriptors |

#### Typography System (7)
| Module | Responsibility |
|--------|---------------|
| `design::font_registry` | Font database, family lookup, style matching |
| `design::font_loader` | FreeType/fontconfig/system font loading |
| `design::font_manager` | Active font lifecycle, preloading, priority |
| `design::font_fallback` | Script-aware fallback chains per language |
| `design::text_metrics` | Glyph advance, line height, cap-height, x-height |
| `design::text_scaling` | DPI-aware text sizing, user font-size preference |
| `design::variable_fonts` | OpenType variable axis controls (wght, wdth, ital…) |

#### Spacing & Elevation (5)
| Module | Responsibility |
|--------|---------------|
| `design::spacing_tokens` | 8px-base scale: space-1…space-32, semantic aliases |
| `design::layout_tokens` | Grid columns, gutter, max-width, breakpoints |
| `design::shadow_engine` | Elevation → shadow parameters, colored shadows |
| `design::blur_engine` | Gaussian blur passes, backdrop blur descriptors |
| `design::glass_engine` | Glass surface: blur + tint + border + specular |

#### Icon & Illustration (3)
| Module | Responsibility |
|--------|---------------|
| `design::icon_registry` | Named icon → SVG path lookup, theme variants |
| `design::icon_loader` | On-demand icon loading, batch prefetch |
| `design::svg_engine` | SVG parse, layout, colorize, rasterize cache |

#### Motion Design (3)
| Module | Responsibility |
|--------|---------------|
| `design::easing_library` | 40+ named easings + cubic-bezier + spring curves |
| `design::motion_tokens` | Duration scale (fast/base/slow/xslow), easing aliases |
| `design::theme_engine` | Master token tree, dark/light/custom, token resolution |

### 4.3 ltk-layout (14 modules)

| Module | Responsibility |
|--------|---------------|
| `layout::measure` | Measure pass: intrinsic/min/max size queries |
| `layout::arrange` | Arrange pass: assign final Rect to every node |
| `layout::constraint` | Cassowary-style constraint solver for anchor layout |
| `layout::box_model` | CSS box model: margin, border, padding, content |
| `layout::flex` | Full CSS Flexbox algorithm (RFC 9346 compliant) |
| `layout::grid` | CSS Grid Level 2 (tracks, areas, subgrid) |
| `layout::flow` | Normal flow, inline flow, block formatting context |
| `layout::wrap` | Multi-line wrap layout, justify-content support |
| `layout::anchor` | Anchor constraints (attach to parent/sibling edges) |
| `layout::dock` | Dock layout: top/bottom/left/right/fill zones |
| `layout::split` | Resizable split pane, drag-handle physics |
| `layout::stack` | Z-ordered stack, absolute positioning |
| `layout::responsive` | Breakpoint system, adaptive layout rules |
| `layout::virtual_layout` | Virtualized lists/grids: only layout visible items |

### 4.4 ltk-render (16 modules)

| Module | Responsibility |
|--------|---------------|
| `render::scene_graph` | Node tree: visual properties, transforms, clips |
| `render::layer_manager` | Layer compositing order, opacity, blend modes |
| `render::surface_manager` | Wayland wl_surface lifecycle, damage tracking |
| `render::frame_scheduler` | VSync, adaptive frame timing, frame budget |
| `render::dirty_tracking` | Per-node dirty flags, invalidation propagation |
| `render::damage_tracking` | Damaged region accumulation, minimal repaints |
| `render::compositor` | Multi-layer alpha compositing, shader dispatch |
| `render::canvas` | 2D drawing API: paths, fills, strokes, clips |
| `render::paint` | Fill/stroke style: colors, gradients, patterns |
| `render::vector` | Bezier curves, path boolean ops, stroke expand |
| `render::texture_manager` | GPU texture atlas, mip-maps, eviction policy |
| `render::glyph_cache` | Signed-distance-field glyph atlas |
| `render::image_renderer` | Decode + upload: PNG/JPEG/WebP/AVIF/SVG |
| `render::backend_api` | RenderBackend trait: submit, present, resize |
| `render::gl_backend` | OpenGL 4.6 + ES 3.2 implementation |
| `render::vk_backend` | Vulkan 1.3 + VK_KHR_surface implementation |

### 4.5 ltk-input (10 modules)

| Module | Responsibility |
|--------|---------------|
| `input::manager` | Central input dispatcher, device registry |
| `input::keyboard` | Key events, key codes, modifiers, repeat timing |
| `input::mouse` | Pointer events, button state, scroll delta |
| `input::touch` | Touch points, pressure, multi-touch tracking |
| `input::gesture` | Tap, double-tap, long-press, pinch, swipe, pan |
| `input::stylus` | Pen tilt, pressure, barrel button, eraser |
| `input::gamepad` | Gamepad axes, buttons, rumble (for accessibility) |
| `input::focus` | Focus owner, focus scope, tab order graph |
| `input::shortcut` | Keyboard shortcut matching, conflict detection |
| `input::dnd` | Drag-and-drop: source, target, MIME negotiation |

### 4.6 ltk-event (8 modules)

| Module | Responsibility |
|--------|---------------|
| `event::bus` | Typed publish/subscribe event bus |
| `event::queue` | Per-thread event queue, priority lanes |
| `event::signal` | Type-safe signal/slot connections |
| `event::observer` | Weak-ref observer pattern, auto-disconnect |
| `event::dispatcher` | Route events: capture → target → bubble |
| `event::command` | Command trait, executor, named commands |
| `event::undo` | UndoStack: push/undo/redo/branch |
| `event::action` | Action: label + icon + shortcut + enabled state |

### 4.7 ltk-animation (9 modules)

| Module | Responsibility |
|--------|---------------|
| `anim::engine` | Animation registry, tick driver, frame coupling |
| `anim::timeline` | Keyframe timeline: property → value curve |
| `anim::transition` | Enter/exit/style transitions on state change |
| `anim::spring` | Damped spring (mass, stiffness, damping) |
| `anim::physics` | 2D physics: gravity, friction, collision |
| `anim::stagger` | Sequential/offset animations for lists |
| `anim::motion_path` | Animate element along SVG/Bezier path |
| `anim::state_machine` | Finite-state animation: states + transitions |
| `anim::frame_sync` | Sync animations to display VSync signal |

### 4.8 ltk-state (7 modules)

| Module | Responsibility |
|--------|---------------|
| `state::property` | Observable typed property with change callbacks |
| `state::computed` | Derived computed property with dependency tracking |
| `state::binding` | One-way and two-way property binding engine |
| `state::reactive` | Fine-grained reactive system (Signal/Effect/Memo) |
| `state::store` | Centralized state store with selectors |
| `state::history` | Time-travel state snapshots (dev + undo support) |
| `state::persistence` | Persist state to disk (config, preferences) |

### 4.9 ltk-resources (6 modules)

| Module | Responsibility |
|--------|---------------|
| `res::manager` | Named resource registry, load/unload lifecycle |
| `res::asset_cache` | LRU cache with size budget and eviction |
| `res::image_cache` | Decoded image storage, format negotiation |
| `res::font_cache` | Shaped glyph cache, shaped text reuse |
| `res::theme_cache` | Compiled token tree cache per theme |
| `res::lazy` | Lazy<T>, OnDemand<T>, AsyncLazy<T> |

### 4.10 ltk-locale (7 modules)

| Module | Responsibility |
|--------|---------------|
| `locale::engine` | String lookup table, .ftl / .po / JSON backends |
| `locale::lang_manager` | Locale detection, priority list, fallback chain |
| `locale::plural` | ICU-compatible plural rules engine |
| `locale::date_fmt` | Date formatting per locale + custom patterns |
| `locale::num_fmt` | Number/currency formatting, grouping separators |
| `locale::bidi` | Unicode BiDi algorithm, RTL layout mirroring |
| `locale::ime` | Input method editor bridge (IBus/Fcitx5) |

### 4.11 ltk-a11y (6 modules)

| Module | Responsibility |
|--------|---------------|
| `a11y::engine` | Master accessibility controller |
| `a11y::tree` | Accessibility tree mirroring widget tree |
| `a11y::atspi` | AT-SPI2 D-Bus bridge (roles, states, events) |
| `a11y::keyboard_nav` | Focus traversal, skip links, landmark nav |
| `a11y::high_contrast` | HC theme detection, forced-color overrides |
| `a11y::announcer` | Live region announcements to screen readers |

### 4.12 ltk-window (5 modules)

| Module | Responsibility |
|--------|---------------|
| `window::abstraction` | Window trait: title, size, state, cursor, icon |
| `window::display` | X11/Wayland display connection lifecycle |
| `window::monitor` | Monitor enumeration, geometry, refresh rate |
| `window::dpi` | DPI detection, scale factor, logical↔physical px |
| `window::cursor` | Cursor shape management, custom cursors |

### 4.13 ltk-platform (4 modules)

| Module | Responsibility |
|--------|---------------|
| `platform::detection` | Detect OS, display server, capabilities |
| `platform::fs` | Async filesystem watcher, XDG dirs |
| `platform::dbus` | D-Bus connection pool, method calls |
| `platform::portal` | XDG Desktop Portal: file, screenshot, settings |

### 4.14 ltk-perf (4 modules)

| Module | Responsibility |
|--------|---------------|
| `perf::profiler` | Instrumented spans, flame graph data |
| `perf::frame_analyzer` | Per-frame breakdown: measure/arrange/paint/commit |
| `perf::memory_analyzer` | Arena usage, cache hit rates, leak detection |
| `perf::render_optimizer` | Batch detection, overdraw analysis |

### 4.15 ltk-security (3 modules)

| Module | Responsibility |
|--------|---------------|
| `security::permissions` | Capability system, per-app permission grants |
| `security::sandbox` | Landlock/seccomp interface for sandboxed widgets |
| `security::clipboard` | Secure clipboard: type filtering, access control |

### 4.16 ltk-devtools (6 modules)

| Module | Responsibility |
|--------|---------------|
| `devtools::logger` | Structured log (tracing), per-crate filters |
| `devtools::crash_handler` | Signal handler, stack trace, crash report |
| `devtools::debug_overlay` | In-app FPS counter, layout bounds, repaint flash |
| `devtools::inspector` | Live widget tree inspector (like GTK Inspector) |
| `devtools::hot_reload` | File watcher → diff → hot-patch live UI |
| `devtools::live_preview` | Standalone preview runner for `.slint` files |

### 4.17 ltk-testing (4 modules)

| Module | Responsibility |
|--------|---------------|
| `testing::harness` | Headless test runtime, tick/pump helpers |
| `testing::ui_test` | Simulate clicks/keys/gestures in tests |
| `testing::snapshot` | Visual snapshot comparison with diff output |
| `testing::a11y_test` | Assert accessibility tree shape/roles/labels |

### 4.18 ltk-plugin (5 modules)

| Module | Responsibility |
|--------|---------------|
| `plugin::system` | Plugin manifest, load/unload, version checks |
| `plugin::wasm` | WASM sandbox via Wasmtime for untrusted plugins |
| `plugin::component_registry` | Register custom Slint components at runtime |
| `plugin::service_registry` | Named service discovery + DI container |
| `plugin::extension_manager` | Extension point system: hooks + contributions |

---

## 5. Public API Surface

### Core Types (used everywhere)

```rust
// Widget identity
pub struct WidgetId(u64);
pub struct LayerId(u32);
pub struct WindowId(u32);

// Geometry (logical pixels unless stated)
pub struct Point  { pub x: f32, pub y: f32 }
pub struct Size   { pub width: f32, pub height: f32 }
pub struct Rect   { pub origin: Point, pub size: Size }
pub struct Insets { pub top: f32, pub right: f32, pub bottom: f32, pub left: f32 }

// Color (all internal math in linear f32)
pub struct Color  { r: f32, g: f32, b: f32, a: f32 }  // linear sRGB
impl Color {
    pub fn from_srgb(r: u8, g: u8, b: u8) -> Self;
    pub fn from_hex(hex: &str) -> Result<Self>;
    pub fn from_oklch(l: f32, c: f32, h: f32, a: f32) -> Self;
    pub fn with_alpha(self, a: f32) -> Self;
    pub fn blend(self, other: Self, t: f32) -> Self;
    pub fn contrast_ratio(self, other: Self) -> f32;
    pub fn is_wcag_aa(self, bg: Self) -> bool;
}
```

### Design Token API

```rust
// Token tree access
pub trait TokenProvider: Send + Sync {
    fn color(&self, token: ColorToken) -> Color;
    fn spacing(&self, token: SpaceToken) -> f32;
    fn radius(&self, token: RadiusToken) -> f32;
    fn shadow(&self, token: ElevationToken) -> ShadowSpec;
    fn font(&self, token: FontToken) -> FontSpec;
    fn duration(&self, token: DurationToken) -> Duration;
    fn easing(&self, token: EasingToken) -> Easing;
}

pub struct ThemeManager {
    pub fn current() -> Arc<dyn TokenProvider>;
    pub fn set_theme(theme: Theme);
    pub fn set_dark_mode(dark: bool);
    pub fn set_accent(hue: f32);
    pub fn subscribe(cb: impl Fn(ThemeChangeEvent) + 'static) -> SubscriptionId;
}
```

### Layout API

```rust
pub trait LayoutEngine: Send {
    fn measure(&self, node: LayoutNodeId, constraint: SizeConstraint) -> Size;
    fn arrange(&self, node: LayoutNodeId, available: Rect);
    fn invalidate(&self, node: LayoutNodeId);
}

pub struct SizeConstraint {
    pub min: Size,
    pub max: Size,       // f32::INFINITY = unbounded
    pub definite: Option<Size>,
}

pub struct FlexConfig {
    pub direction:    FlexDirection,
    pub wrap:         FlexWrap,
    pub align_items:  AlignItems,
    pub justify:      JustifyContent,
    pub gap:          f32,
}

pub struct GridConfig {
    pub columns:      Vec<TrackSize>,
    pub rows:         Vec<TrackSize>,
    pub areas:        Option<GridAreas>,
    pub gap:          (f32, f32),
}
```

### Render API

```rust
pub trait RenderBackend: Send + Sync {
    fn begin_frame(&mut self) -> FrameContext;
    fn submit_layer(&mut self, layer: &LayerCommand);
    fn end_frame(&mut self, ctx: FrameContext);
    fn resize(&mut self, physical_size: Size);
    fn present(&mut self);
    fn capabilities(&self) -> BackendCapabilities;
}

pub struct Canvas<'b> {
    pub fn save(&mut self);
    pub fn restore(&mut self);
    pub fn translate(&mut self, x: f32, y: f32);
    pub fn scale(&mut self, sx: f32, sy: f32);
    pub fn rotate(&mut self, radians: f32);
    pub fn clip_rect(&mut self, rect: Rect, radius: f32);
    pub fn fill_rect(&mut self, rect: Rect, paint: &Paint);
    pub fn stroke_rect(&mut self, rect: Rect, paint: &Paint, width: f32);
    pub fn fill_path(&mut self, path: &Path, paint: &Paint);
    pub fn stroke_path(&mut self, path: &Path, paint: &Paint, width: f32);
    pub fn draw_image(&mut self, image: &Texture, dst: Rect, src: Option<Rect>);
    pub fn draw_text(&mut self, shaped: &ShapedText, x: f32, y: f32, paint: &Paint);
    pub fn fill_shadow(&mut self, rect: Rect, spec: &ShadowSpec);
    pub fn fill_blur(&mut self, rect: Rect, radius: f32);
}
```

### Event API

```rust
pub enum LtkEvent {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Touch(TouchEvent),
    Gesture(GestureEvent),
    Window(WindowEvent),
    Custom(Box<dyn Any + Send>),
}

pub struct EventBus {
    pub fn subscribe<E: 'static>(&self, cb: impl Fn(&E) + 'static) -> SubscriptionId;
    pub fn publish<E: 'static>(&self, event: E);
    pub fn unsubscribe(&self, id: SubscriptionId);
}

pub trait Command: Send + 'static {
    fn name(&self) -> &str;
    fn execute(&self, ctx: &mut CommandContext) -> CommandResult;
    fn undo(&self, ctx: &mut CommandContext) -> CommandResult;
    fn description(&self) -> Option<&str> { None }
    fn icon(&self) -> Option<IconName> { None }
}
```

### Animation API

```rust
pub struct AnimationBuilder {
    pub fn property<T: Lerp>(self, getter: ..., setter: ...) -> Self;
    pub fn from(self, from: impl Into<AnimValue>) -> Self;
    pub fn to(self, to: impl Into<AnimValue>) -> Self;
    pub fn duration(self, d: Duration) -> Self;
    pub fn easing(self, e: Easing) -> Self;
    pub fn delay(self, d: Duration) -> Self;
    pub fn repeat(self, times: RepeatCount) -> Self;
    pub fn on_complete(self, cb: impl Fn() + 'static) -> Self;
    pub fn build(self) -> AnimationHandle;
}

pub struct SpringConfig {
    pub mass:      f32,   // default 1.0
    pub stiffness: f32,   // default 200.0
    pub damping:   f32,   // default 20.0
}

pub fn spring_to(target: f32, cfg: SpringConfig) -> SpringHandle;
```

### State / Reactivity API

```rust
// Signal<T>: reactive value
pub struct Signal<T: Clone + 'static>(/* ... */);
impl<T: Clone> Signal<T> {
    pub fn new(value: T) -> Self;
    pub fn get(&self) -> T;
    pub fn set(&self, value: T);
    pub fn update(&self, f: impl FnOnce(T) -> T);
    pub fn subscribe(cb: impl Fn(&T) + 'static) -> SubscriptionId;
}

// Computed<T>: derived reactive value
pub fn computed<T: Clone>(f: impl Fn() -> T + 'static) -> Computed<T>;

// Effect: run side effects when signals change
pub fn effect(f: impl Fn() + 'static) -> EffectHandle;

// Binding: two-way property sync
pub fn bind<T: Clone>(a: &Signal<T>, b: &Signal<T>) -> BindingHandle;
```

### Accessibility API

```rust
pub struct AccessibleNode {
    pub id:          WidgetId,
    pub role:        AriaRole,
    pub name:        Option<SharedString>,
    pub description: Option<SharedString>,
    pub value:       Option<AccessibleValue>,
    pub states:      AccessibleStates,
    pub actions:     Vec<AccessibleAction>,
    pub children:    Vec<WidgetId>,
}

pub trait Accessible {
    fn accessible_node(&self) -> AccessibleNode;
    fn on_accessible_action(&mut self, action: AccessibleAction);
}
```

---

## 6. Internal Architecture

### 6.1 Widget Tree → Render Pipeline

```
Widget Tree (Slint AST)
        │
        ▼ ltk-layout
Layout Tree (LayoutNode + Rect)
        │
        ▼ ltk-render::scene_graph
Scene Graph (visual props, transforms, clips)
        │
        ├──▶ ltk-render::damage_tracking
        │           → minimal dirty regions
        │
        ▼ ltk-render::compositor
Layer Commands (sorted by z, batched by material)
        │
        ├──▶ ltk-render::gl_backend  (OpenGL 4.6)
        └──▶ ltk-render::vk_backend  (Vulkan 1.3)
                │
                ▼ Wayland wl_surface + wl_buffer
              Display
```

### 6.2 Input → Event → State → Render Loop

```
Wayland input events (libinput)
        │
        ▼ ltk-input::manager
InputEvent (normalized)
        │
        ├──▶ ltk-input::gesture   (recognize gesture)
        ├──▶ ltk-input::focus     (route to focused widget)
        ├──▶ ltk-input::shortcut  (check global shortcuts)
        └──▶ ltk-input::dnd       (drag-and-drop protocol)
                │
                ▼ ltk-event::bus
        Widget receives LtkEvent
                │
                ▼ Widget updates ltk-state::Signal<T>
        Reactive graph propagates
                │
                ▼ Dirty flags set on affected LayoutNode/SceneNode
        Next frame: layout → render
```

### 6.3 Theme Resolution

```
User sets accent hue (e.g. 213°)
        │
        ▼ ltk-design::color_generator
Generate OKLCH palette (primary, containers, on-*, etc.)
        │
        ▼ ltk-design::theme_engine
Merge with base token tree (dark/light surfaces, typography…)
        │
        ▼ ltk-state::Signal<Arc<TokenTree>>
All token consumers invalidate
        │
        ▼ ltk-render dirty flag → next frame repaints
```

### 6.4 Accessibility Sync

```
Widget tree mutation
        │
        ▼ ltk-a11y::tree
AccessibleNode tree updated (structural diff)
        │
        ▼ ltk-a11y::atspi
AT-SPI2 D-Bus events emitted:
  - object:children-changed
  - object:state-changed
  - object:property-change:accessible-name
        │
        ▼ Orca / other AT consumes
```

---

## 7. Dependency Graph

```
ltk-core  ◀───────────────────────────────── (all crates)
    │
ltk-design  ←── ltk-core
ltk-layout  ←── ltk-core, ltk-design
ltk-render  ←── ltk-core, ltk-design, ltk-layout
ltk-input   ←── ltk-core, ltk-event
ltk-event   ←── ltk-core
ltk-anim    ←── ltk-core, ltk-design, ltk-state, ltk-render
ltk-state   ←── ltk-core
ltk-res     ←── ltk-core, ltk-render
ltk-locale  ←── ltk-core
ltk-a11y    ←── ltk-core, ltk-event, ltk-state
ltk-window  ←── ltk-core, ltk-wayland, ltk-render, ltk-input
ltk-platform←── ltk-core
ltk-perf    ←── ltk-core, ltk-render, ltk-layout
ltk-security←── ltk-core, ltk-platform
ltk-devtools←── ltk-core, ltk-render, ltk-layout, ltk-state
ltk-testing ←── (all crates, test-only)
ltk-plugin  ←── ltk-core, ltk-event, ltk-state
ltk-wayland ←── ltk-core, ltk-platform
ltk-gl      ←── ltk-core, ltk-render
ltk-vk      ←── ltk-core, ltk-render
```

**No cycles.** Dependency direction is always upward (higher-level → lower-level).

---

## 8. Data Flow

### Frame Loop (60 Hz target, adaptive)

```
┌─────────────────────────── Frame N ──────────────────────────────┐
│                                                                   │
│  1. INPUT PHASE      Collect + route all pending input events     │
│  2. EVENT PHASE      Dispatch to widgets, fire signal callbacks   │
│  3. STATE PHASE      Propagate reactive changes (batch)           │
│  4. ANIMATION PHASE  Advance all active animations by Δt         │
│  5. LAYOUT PHASE     Measure + arrange dirty subtrees only        │
│  6. SCENE PHASE      Update scene graph nodes (transforms/clips)  │
│  7. PAINT PHASE      Record draw calls for damaged regions only   │
│  8. COMMIT PHASE     Upload to GPU, present wl_surface            │
│  9. A11Y PHASE       Sync accessibility tree deltas               │
│ 10. DEBUG PHASE      Profiler flush, debug overlay update         │
│                                                                   │
└───────────────────────────────────────────────────────────────────┘
```

---

## 9. Folder Structure

```
ltk-foundation/
├── Cargo.toml                    Workspace
├── ARCHITECTURE.md               This document
├── LICENSE-MIT
├── LICENSE-APACHE
├── README.md
│
├── crates/
│   ├── ltk-core/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── id.rs
│   │       ├── arena.rs
│   │       ├── geometry.rs
│   │       ├── color.rs
│   │       ├── string.rs
│   │       ├── error.rs
│   │       ├── sync.rs
│   │       ├── task.rs
│   │       ├── time.rs
│   │       ├── version.rs
│   │       ├── flags.rs
│   │       ├── callback.rs
│   │       └── env.rs
│   │
│   ├── ltk-design/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── color/
│   │       │   ├── mod.rs
│   │       │   ├── tokens.rs
│   │       │   ├── generator.rs
│   │       │   ├── dynamic.rs
│   │       │   ├── semantic.rs
│   │       │   ├── contrast.rs
│   │       │   └── palette.rs
│   │       ├── typography/
│   │       │   ├── mod.rs
│   │       │   ├── registry.rs
│   │       │   ├── loader.rs
│   │       │   ├── manager.rs
│   │       │   ├── fallback.rs
│   │       │   ├── metrics.rs
│   │       │   ├── scaling.rs
│   │       │   └── variable.rs
│   │       ├── spacing.rs
│   │       ├── elevation/
│   │       │   ├── mod.rs
│   │       │   ├── shadow.rs
│   │       │   ├── blur.rs
│   │       │   └── glass.rs
│   │       ├── icon/
│   │       │   ├── mod.rs
│   │       │   ├── registry.rs
│   │       │   ├── loader.rs
│   │       │   └── svg.rs
│   │       ├── motion/
│   │       │   ├── mod.rs
│   │       │   ├── easing.rs
│   │       │   └── tokens.rs
│   │       └── theme/
│   │           ├── mod.rs
│   │           ├── engine.rs
│   │           ├── loader.rs
│   │           └── serializer.rs
│   │
│   ├── ltk-layout/
│   ├── ltk-render/
│   ├── ltk-input/
│   ├── ltk-event/
│   ├── ltk-animation/
│   ├── ltk-state/
│   ├── ltk-resources/
│   ├── ltk-locale/
│   ├── ltk-a11y/
│   ├── ltk-window/
│   ├── ltk-platform/
│   ├── ltk-perf/
│   ├── ltk-security/
│   ├── ltk-devtools/
│   ├── ltk-testing/
│   ├── ltk-plugin/
│   ├── ltk-wayland/
│   ├── ltk-gl/
│   └── ltk-vk/
│
├── tools/
│   ├── ltk-inspector/            Standalone GTK-Inspector-like tool
│   ├── ltk-preview/              Slint live preview runner
│   └── ltk-theme-editor/         Visual token editor
│
├── docs/
│   ├── modules/                  Per-module deep dives
│   ├── guides/                   How-to guides
│   └── adr/                      Architecture Decision Records
│
└── tests/
    ├── integration/
    ├── snapshot/
    └── accessibility/
```

---

## 10. Design Rationale

### Why Rust?
Memory safety without GC, deterministic performance, superb FFI for Wayland/OpenGL/AT-SPI2, growing Linux ecosystem.

### Why Slint?
Declarative `.slint` syntax compiles to native Rust, supports live preview, generates AT-SPI2 metadata, targets embedded → desktop with the same codebase.

### Why OKLCH for color math?
Perceptually uniform: equal Δc feels equally different across hues. Generates harmonious palettes algorithmically. Better dark-mode derivation than HSL.

### Why Cassowary for anchor layout?
Proven (used in Apple Auto Layout, Cocoa). Handles circular constraints gracefully, efficient incremental re-solve.

### Why virtual layout?
Lists with 100,000 rows must scroll at 60 fps. Virtualization is non-negotiable for system file managers, mail clients, IDE trees.

### Why AT-SPI2 (not IAccessible2)?
AT-SPI2 is the Linux/Wayland accessibility standard, supported by Orca, Accerciser, Qt and GTK. D-Bus based, language-agnostic.

### Why WASM for plugins?
Sandboxed execution, language-agnostic, capability-based security. Plugin crashes cannot take down the host.

### Why 20 crates?
Each crate has a single responsibility and a clean public API boundary. Teams can own individual crates. Breaking changes are localized. Build times improve via parallel compilation.

---

## 11. Future Expansion Points

Every crate exposes extension traits and registries designed for Phase 2+:

| Expansion | Mechanism |
|-----------|-----------|
| Custom layout algorithms | `impl LayoutEngine for MyLayout` |
| Custom render backends (Metal on Mac) | `impl RenderBackend for MetalBackend` |
| Custom input devices | `impl InputDevice for BrailleDisplay` |
| AI-assisted layout | Hook into `ltk-layout::responsive` breakpoint resolver |
| Remote UI rendering | `ltk-render::backend_api` over network transport |
| Custom animation curves | `impl EasingFn for NeuralEasing` |
| Custom accessibility roles | `AriaRole::Custom(String)` fallback |
| Plugin-provided components | `ComponentRegistry::register(name, factory)` |
| Theme marketplace | `ThemeLoader::from_url()` + signature verification |
| WASM UI components | `ltk-plugin::wasm` sandboxed component factory |

---

*LTK Phase 1 Foundation — Architecture Document v1.0*
*© 2026 LionOS Project · MIT / Apache-2.0*
