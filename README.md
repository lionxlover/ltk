# LTK — Lion Tool Kit · Phase 1 Foundation Layer

**The native UI framework of LionOS. A complete GTK replacement built in Rust.**

---

## What Is LTK?

LTK (Lion Tool Kit) is the official UI framework of LionOS. It replaces GTK on
Linux with a Rust-first, Wayland-native, declarative-first framework designed
to last 20+ years.

Phase 1 — the **Foundation Layer** — provides every architectural primitive
needed to build desktop applications: rendering, layout, input, events,
animation, state management, accessibility, i18n, theming, plugin engine,
and both OpenGL and Vulkan backends.

---

## Crate Map · 21 crates · 174 modules · 7,721 lines

| Crate | Files | Responsibility |
|-------|------:|----------------|
| `ltk-a11y` | 7 | AT-SPI2 bridge, a11y tree, screen reader, keyboard nav |
| `ltk-animation` | 10 | Timeline, spring/RK4, physics, stagger, motion-path, state machines |
| `ltk-core` | 15 | Shared types, IDs, geometry, color, sync primitives |
| `ltk-design` | 19 | Color tokens, typography, spacing, elevation, icons, motion, theme |
| `ltk-devtools` | 7 | Logger, crash handler, hot-reload, inspector, live preview |
| `ltk-event` | 7 | Event bus, signals, commands, undo/redo, actions |
| `ltk-gl` | 4 | OpenGL 4.6 / ES 3.2 rendering backend |
| `ltk-input` | 11 | Keyboard, mouse, touch, gesture, stylus, DnD, focus, shortcuts |
| `ltk-layout` | 13 | Flex, Grid, anchor, dock, split, virtual, responsive layout |
| `ltk-locale` | 8 | i18n/l10n, BiDi, date/number formatters, IME bridge |
| `ltk-perf` | 5 | Profiler, frame analyzer, memory analyzer, overdraw detector |
| `ltk-platform` | 5 | Linux/LionOS glue, D-Bus, XDG portals, filesystem watcher |
| `ltk-plugin` | 6 | Plugin system, WASM sandbox, component/service registry |
| `ltk-render` | 14 | Scene graph, canvas, damage tracking, compositor, GL/VK backends |
| `ltk-resources` | 7 | Asset manager, LRU caches, lazy loading |
| `ltk-security` | 4 | Permissions, Landlock sandbox, secure clipboard |
| `ltk-state` | 8 | Reactive signals, computed, effects, bindings, store, persistence |
| `ltk-testing` | 5 | Headless test harness, UI testing, snapshots, a11y assertions |
| `ltk-vk` | 6 | Vulkan 1.3 rendering backend |
| `ltk-wayland` | 7 | XDG shell, decoration, seat, output, DnD protocol |
| `ltk-window` | 6 | Window abstraction, display, monitor, DPI, cursor |

---

## Quick Start

### Dependencies

```toml
# Cargo.toml
[dependencies]
ltk-core      = { path = "crates/ltk-core" }
ltk-design    = { path = "crates/ltk-design" }
ltk-state     = { path = "crates/ltk-state" }
ltk-event     = { path = "crates/ltk-event" }
ltk-animation = { path = "crates/ltk-animation" }
ltk-a11y      = { path = "crates/ltk-a11y" }
ltk-render    = { path = "crates/ltk-render" }
ltk-gl        = { path = "crates/ltk-gl", features = ["live-gl"] }
ltk-vk        = { path = "crates/ltk-vk" }
ltk-wayland   = { path = "crates/ltk-wayland", features = ["live-wayland"] }
ltk-plugin    = { path = "crates/ltk-plugin",  features = ["wasm"] }
```

### Hello World (Rust)

```rust
use ltk_core::task::register_ui_thread;
use ltk_design::theme::Theme;
use ltk_design::color::ColorToken;
use ltk_design::motion::Easing;

fn main() {
    register_ui_thread();

    // Build the default dark theme at blue hue 213 degrees
    let theme = Theme::default_dark();

    let primary = theme.colors.get(ColorToken::Primary);
    let bg      = theme.colors.get(ColorToken::BgBase);

    // WCAG AA is guaranteed by the generator
    assert!(primary.is_wcag_aa(bg));
    println!("Primary:  {}", primary);
    println!("Contrast: {:.2}:1", primary.contrast_ratio(bg));

    // Spring easing
    let e = Easing::spring_standard();
    println!("Spring t=0.5: {:.4}", e.evaluate(0.5));
}
```

### Reactive State

```rust
use ltk_state::{Signal, computed};

fn main() {
    let name  = Signal::new("Lion".to_string());
    let upper = computed({
        let name = name.clone();
        move || name.get().to_uppercase()
    });

    println!("{}", upper.get());  // "LION"
    name.set("LionOS".into());
    upper.invalidate();
    println!("{}", upper.get());  // "LIONOS"
}
```

### Spring Physics Animation

```rust
use ltk_animation::spring::{SpringConfig, SpringSim};

fn main() {
    // Bouncy spring: mass=1, stiffness=280, damping=14
    let mut sim = SpringSim::new(SpringConfig::BOUNCY, 0.0, 1.0);
    let dt = 1.0 / 60.0; // 60 Hz

    for frame in 0..180 {
        sim.step(dt);
        let bar = "#".repeat((sim.position * 40.0) as usize);
        println!("Frame {:3}: [{:<40}] {:.4}", frame, bar, sim.position);
        if sim.is_at_rest(0.0005) {
            println!("Settled at frame {}", frame);
            break;
        }
    }
}
```

### RK4 Integration (Spring internals)

```rust
// SpringSim::step() uses 4th-order Runge-Kutta:
//
// f(p, v) = (-k*(p - target) - d*v) / m
//
// k0 = f(p,        v)
// k1 = f(p + v*dt/2, v + k0*dt/2)
// k2 = f(p + v*dt/2, v + k1*dt/2)
// k3 = f(p + v*dt,   v + k2*dt)
//
// v_new = v + dt/6 * (k0 + 2k1 + 2k2 + k3)
// p_new = p + dt/6 * (v0 + 2v1 + 2v2 + v3)
```

### WCAG Color Generation (OKLCH)

```rust
use ltk_design::color::{AccentGenerator, ColorToken};

fn main() {
    // Any hue 0-360: generates full dark + light semantic palette
    let purple = AccentGenerator::new(280.0);
    let dark   = purple.build_dark();
    let light  = purple.build_light();

    let primary_dark  = dark.get(ColorToken::Primary);
    let primary_light = light.get(ColorToken::Primary);
    let bg_dark       = dark.get(ColorToken::BgBase);
    let bg_light      = light.get(ColorToken::BgBase);

    println!("Dark  primary contrast: {:.2}:1", primary_dark.contrast_ratio(bg_dark));
    println!("Light primary contrast: {:.2}:1", primary_light.contrast_ratio(bg_light));
}
```

### Accessibility Tree

```rust
use ltk_a11y::tree::{AccessibilityTree, AccessibleNode, AccessibleRole, AccessibleState};
use ltk_a11y::engine::A11yEngine;
use ltk_core::id::WidgetId;

fn main() {
    let mut engine = A11yEngine::new(true /* enabled */);

    let dialog_id = WidgetId::new();
    let btn_id    = WidgetId::new();

    // Build nodes
    let mut dialog = AccessibleNode::new(dialog_id, AccessibleRole::Dialog);
    dialog.name    = Some("Confirm deletion".into());
    dialog.state  |= AccessibleState::MODAL | AccessibleState::VISIBLE;
    dialog.children.push(btn_id);
    engine.tree.insert(dialog);

    let mut btn = AccessibleNode::new(btn_id, AccessibleRole::Button);
    btn.name    = Some("Delete".into());
    btn.state  |= AccessibleState::FOCUSABLE | AccessibleState::ENABLED;
    btn.parent  = Some(dialog_id);
    engine.tree.insert(btn);
    engine.tree.set_root(dialog_id);

    // Flush to AT-SPI2 (emits D-Bus events to Orca)
    engine.flush();

    println!("a11y tree: {} nodes", engine.tree.node_count());
}
```

### Event Bus

```rust
use ltk_event::bus::EventBus;

#[derive(Debug, Clone)]
struct ThemeChanged { hue: f32 }

#[derive(Debug, Clone)]
struct WidgetClicked { id: u64 }

fn main() {
    let bus = EventBus::new();

    // Subscribe to typed events
    let _s1 = bus.subscribe::<ThemeChanged>(|e| {
        println!("Theme changed to hue {:.0}°", e.hue);
    });
    let _s2 = bus.subscribe::<WidgetClicked>(|e| {
        println!("Widget {} clicked", e.id);
    });

    // Publish
    bus.publish(ThemeChanged { hue: 280.0 });
    bus.publish(WidgetClicked { id: 42 });
    // No cross-talk between event types — fully typed
}
```

### Undo/Redo

```rust
use ltk_event::command::{Command, CommandContext, CommandResult};
use ltk_event::undo::UndoStack;
use std::sync::Arc;

struct RenameCommand { from: String, to: String }

impl Command for RenameCommand {
    fn name(&self) -> &str { "Rename" }
    fn execute(&self, _ctx: &mut CommandContext) -> CommandResult {
        println!("Renamed '{}' -> '{}'", self.from, self.to);
        Ok(())
    }
    fn undo(&self, _ctx: &mut CommandContext) -> CommandResult {
        println!("Undone: '{}' -> '{}'", self.to, self.from);
        Ok(())
    }
}

fn main() {
    let mut stack = UndoStack::new(50 /* max depth */);
    let mut ctx   = CommandContext::new();

    stack.execute(Arc::new(RenameCommand {
        from: "foo.txt".into(), to: "bar.txt".into()
    }), &mut ctx).unwrap();

    assert!(stack.can_undo());
    println!("Undo label: {}", stack.undo_label().unwrap());

    stack.undo(&mut ctx).unwrap();
    assert!(stack.can_redo());
    stack.redo(&mut ctx).unwrap();
}
```

---

## Architecture

### 10-Phase Frame Loop (60 Hz)

```
┌─────────────────────────────── Frame N ─────────────────────────────────┐
│  1  INPUT      Collect Wayland/libinput events, normalize, route        │
│  2  EVENT      Dispatch to focused/hovered widgets, fire bus callbacks  │
│  3  STATE      Batch-propagate reactive Signal changes                  │
│  4  ANIMATION  Advance springs (RK4), timelines, physics by Δt         │
│  5  LAYOUT     Measure+arrange dirty subtrees (flex, grid, virtual)     │
│  6  SCENE      Update scene graph transforms, clips, blend modes        │
│  7  PAINT      Record draw commands for damaged regions only            │
│  8  COMMIT     Upload to GPU, present wl_surface / eglSwapBuffers       │
│  9  A11Y       Diff accessibility tree, emit AT-SPI2 D-Bus events       │
│ 10  DEBUG      Flush profiler spans, update debug overlay               │
└─────────────────────────────────────────────────────────────────────────┘
```

### Dependency Graph (strictly acyclic)

```
ltk-core  ◄────────────────────────── (all crates depend on this)
  │
  ├─► ltk-design
  │     └─► ltk-layout
  │           └─► ltk-render ──► ltk-gl  (OpenGL 4.6 / ES 3.2)
  │                          └─► ltk-vk  (Vulkan 1.3)
  │
  ├─► ltk-event ──► ltk-input
  │
  ├─► ltk-state ──► ltk-animation
  │
  ├─► ltk-resources
  │
  ├─► ltk-locale
  │
  ├─► ltk-a11y
  │
  ├─► ltk-platform ──► ltk-wayland ──► ltk-window
  │
  ├─► ltk-perf
  ├─► ltk-security
  ├─► ltk-devtools
  ├─► ltk-plugin
  └─► ltk-testing  (test-only, depends on all)
```

### Theme Token Resolution Flow

```
User picks accent hue (e.g. 213° = LionOS blue)
  │
  ▼ ltk_design::color::AccentGenerator::build_dark()
Generate full OKLCH palette (38 semantic color slots)
  │
  ▼ ltk_design::theme::Theme::default_dark()
Merge with typography scale, spacing, radius, motion tokens
  │
  ▼ ltk_state::Signal<Arc<Theme>>
Subscribers invalidate (components, renderers, accessibility)
  │
  ▼ Next frame: re-paint affected widgets
```

---

## Feature Flags

| Crate | Flag | Activates |
|-------|------|-----------|
| `ltk-gl`      | `live-gl`      | Real EGL/OpenGL context (requires GPU driver) |
| `ltk-wayland` | `live-wayland` | Real Wayland socket connection |
| `ltk-plugin`  | `wasm`         | WASM sandbox via Wasmtime |
| `ltk-core`    | `serde`        | Serialize IDs and geometry types |

Without feature flags, every crate compiles and passes `cargo check` without
any GPU or display server present — ideal for CI and headless testing.

---

## Building

```bash
# Ubuntu 24 / LionOS prerequisites
sudo apt install build-essential clang lld pkg-config \
     libwayland-dev libxkbcommon-dev libvulkan-dev libegl-dev \
     libdbus-1-dev libatspi2.0-dev

# Fast workspace check (no GPU, no display server needed)
cargo check --workspace

# Run all unit tests (headless)
cargo test --workspace

# Production build
cargo build --workspace --release

# Enable all live backends (requires GPU + Wayland session)
cargo build --workspace --release \
  --features ltk-gl/live-gl,ltk-wayland/live-wayland,ltk-plugin/wasm
```

---

## Design Principles

| Principle | How It's Achieved |
|-----------|-------------------|
| **Modularity** | 21 independent crates; each has one job |
| **No unsafe by default** | `#![forbid(unsafe_code)]` on all non-GPU crates |
| **Composition over inheritance** | Trait objects everywhere; no class hierarchies |
| **Accessibility is not an afterthought** | AT-SPI2 tree built in the same pass as the visual tree |
| **Wayland-native** | No X11 in the hot path; X11 support via XWayland only |
| **Deterministic performance** | Arena allocators, slot maps, explicit frame budgets |
| **Testable without hardware** | Full headless test harness; snapshot testing; a11y assertions |
| **OKLCH color math** | Perceptually uniform palettes, auto WCAG AA compliance |
| **Extensible** | Plugin WASM sandbox, service DI container, extension points |

---

## Phase 2 Roadmap

Phase 1 is the Foundation. Phase 2 will add:

- **ltk-widgets** — 150+ native components (Button, Input, Calendar, Kanban…)
- **ltk-shell** — LionOS shell integration (taskbar, launcher, notifications)
- **ltk-compositor** — Wayland compositor bridge for the LionOS window manager
- **ltk-remote** — Render UI over network transport (for remote desktop)
- **ltk-ai** — AI-assisted adaptive layout, LLM tool-calling UI hooks

---

## License

**MIT OR Apache-2.0** — your choice.

© 2026 LionOS Project · Lion
