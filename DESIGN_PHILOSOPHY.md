# LTK Design Philosophy
## *Inspired by macOS & GNOME — Built for LionOS*

---

## 1. The North Star

LTK inherits the best of two great design traditions:

| macOS (Apple HIG) | GNOME (libadwaita / HIG 3) |
|-------------------|---------------------------|
| Spring-physics motion | Adaptive breakpoint layouts |
| Vibrancy & glass materials | Headerbar-first navigation |
| Depth through shadows | Session-aware color scheme |
| Fluid gesture continuity | Typographic information hierarchy |
| Squircle geometry | Preference for content over chrome |
| Contextual menus | Keyboard-first accessibility |
| Traffic-light window controls | GNOME application pattern |

LionOS blends these into a single coherent language: **Leonux**.

---

## 2. Materials

### Glass (macOS Vibrancy inspired)

Three levels of glass, each with a different blur radius and tint opacity.

```
Thin   — blur 8px,  tint 40%  — tooltips, badges, small overlays
Regular— blur 20px, tint 65%  — panels, sidebars, menus
Thick  — blur 40px, tint 80%  — modals, sheets, fullscreen overlays
```

The tint colour is always derived from `bg-surface` so the glass adapts
to both dark and light modes automatically.

### Acrylic (GNOME headerbar inspired)

Headerbar surfaces use a flatter "acrylic" material:
- No blur in light mode (fully opaque, high contrast)
- Subtle blur in dark mode (8px, 90% tint) for depth

---

## 3. Motion

Every interactive element uses spring physics. No cubic-bezier except for
page-level transitions.

### Spring Presets

| Name | Mass | Stiffness | Damping | Use case |
|------|------|-----------|---------|----------|
| `micro` | 1 | 600 | 38 | Checkbox, radio, toggle |
| `standard` | 1 | 280 | 26 | Buttons, menus, panels |
| `bouncy` | 1 | 200 | 14 | FAB, onboarding, celebrations |
| `gentle` | 1 | 120 | 20 | Sheet entrances, drawer |
| `stiff` | 1 | 500 | 40 | Drag snapping, resize |

### Duration as Side-Effect

Springs have no explicit duration — they settle when energy dissipates.
Named durations (`fast`, `base`, `slow`) are only used for:
- Opacity fade-ins / fade-outs (no spring makes sense)
- Progress bar fills
- Skeleton shimmer

---

## 4. Color

### Primary Hue: 213° (LionOS Blue)

LionOS ships with blue as its default accent, matching macOS "Blue" and
GNOME's default blue accent, but with higher vibrancy in OKLCH space.

### Five Accent Colors (macOS-inspired names, OKLCH values)

| Name | Hue | OKLCH L | OKLCH C |
|------|-----|---------|---------|
| Blue (default) | 213° | 0.64 | 0.18 |
| Purple | 280° | 0.68 | 0.17 |
| Pink | 340° | 0.68 | 0.19 |
| Red | 25° | 0.62 | 0.20 |
| Orange | 52° | 0.72 | 0.20 |
| Yellow | 85° | 0.78 | 0.19 |
| Green | 148° | 0.70 | 0.20 |
| Graphite | 230° | 0.55 | 0.05 |

### Surfaces (GNOME-inspired naming)

```
bg-base       = the "desktop" / base layer
bg-surface    = cards, panels (one step up)
bg-raised     = popovers, menus (two steps up)
bg-overlay    = sheets, modals (three steps up)
bg-headerbar  = window chrome (special case)
```

### State Colours

| State | macOS analogue | GNOME analogue | LTK token |
|-------|---------------|----------------|-----------|
| Success | Green system colour | --success-color | StateSuccess |
| Warning | Yellow system colour | --warning-color | StateWarning |
| Error | Red system colour | --error-color | StateError |
| Accent | System accent | --accent-color | Primary |

---

## 5. Typography

### Scale (macOS SF Pro inspired, libre fonts)

| Style | Family | Size | Weight | Line Height |
|-------|--------|------|--------|-------------|
| Large Title | Space Grotesk | 34px | Bold | 1.1 |
| Title 1 | Space Grotesk | 28px | Bold | 1.15 |
| Title 2 | Space Grotesk | 22px | Bold | 1.2 |
| Title 3 | Space Grotesk | 20px | Semibold | 1.25 |
| Headline | Inter | 17px | Semibold | 1.3 |
| Body | Inter | 17px | Regular | 1.5 |
| Callout | Inter | 16px | Regular | 1.45 |
| Subheadline | Inter | 15px | Regular | 1.4 |
| Footnote | Inter | 13px | Regular | 1.4 |
| Caption 1 | Inter | 12px | Regular | 1.4 |
| Caption 2 | Inter | 11px | Regular | 1.35 |
| Monospaced | JetBrains Mono | 13px | Regular | 1.6 |

### Legibility Defaults (GNOME HIG)

- Minimum body text size: 11px (Caption 2)
- All interactive labels: minimum 13px (Footnote)
- Disabled text: 40% opacity, not grey colour override

---

## 6. Geometry

### Corner Radii (macOS squircle progression)

| Token | Radius | Applies to |
|-------|--------|-----------|
| `xs` | 3px | Tags, small badges |
| `sm` | 6px | Inputs, small cards |
| `md` | 10px | Buttons, standard cards |
| `lg` | 14px | Panels, sidebars |
| `xl` | 18px | Drawers, larger modals |
| `2xl` | 24px | Sheets, full-screen overlays |
| `3xl` | 32px | App icon shapes |
| `full` | 9999px | Pills, toggles, chips |

### Squircle Rule (macOS-inspired)

For radius > 12px: use CSS `border-radius` with `path()` squircle approximation.
For radius ≤ 12px: standard circular arc is fine.

---

## 7. Spacing

8px base grid (identical to GNOME HIG's 8pt grid). All spacing values are
multiples of 4px (half-steps allowed for tight internal padding only).

```
Micro  = 4px    — icon-to-label gap, badge padding
XSmall = 8px    — form field internal padding
Small  = 12px   — section internal padding
Base   = 16px   — standard element gap
Medium = 24px   — section gap
Large  = 32px   — view padding
XLarge = 48px   — hero sections
```

---

## 8. Iconography

- **Style**: SF Symbols–inspired weight-adaptive line icons
- **Weights**: 3 weights (thin, regular, bold) matching font weight
- **Sizes**: 16, 20, 24, 32, 48px
- **Format**: SVG with `currentColor` for automatic theme adaptation
- **Grid**: 24×24 bounding box, 20×20 optical area

---

## 9. Window Chrome

### macOS Traffic Light (left-side control group)

```
[●][●][●]  Close · Minimize · Zoom
```

- 12px circles, 6px gap
- Hover: coloured (red/yellow/green)
- Default: grey — becomes coloured only on hover or active window focus
- Keyboard: Cmd+W (close), Cmd+M (minimize), Ctrl+Cmd+F (fullscreen)

### GNOME Headerbar (single bar, no menu bar)

- Title centered (or left-aligned for document apps)
- Action buttons on right: ⋮ menu, window controls
- Headerbar doubles as drag region for window move
- No separate menu bar: actions live in popovers and ⋮ menus

### LionOS Hybrid

- Traffic light controls on the left (macOS muscle memory)
- Title centered (GNOME style)
- Application menu (⋮) on the right
- Unified headerbar (no separate titlebar + toolbar)

---

## 10. GNOME Adaptive Patterns

### Breakpoints

| Name | Width | Layout change |
|------|-------|--------------|
| `mobile` | < 360px | Single column, bottom nav |
| `narrow` | 360–640px | Single column, sidebar collapses to overlay |
| `compact` | 640–900px | Sidebar appears, content > 50% |
| `medium` | 900–1200px | Two-pane with split view |
| `wide` | > 1200px | Three-pane, inspector visible |

### AdwNavigationSplitView analogue

LTK implements the GNOME `AdwNavigationSplitView` pattern:
- On `narrow`: sidebar is an overlay (hamburger → slide in)
- On `compact+`: sidebar is persistent (always visible)
- On `wide`: both sidebar + detail + inspector visible

---

## 11. Accessibility (GNOME + macOS combined)

| Feature | Source | LTK implementation |
|---------|--------|--------------------|
| Full keyboard nav | Both | `ltk-input::focus::FocusEngine` |
| Screen reader | GNOME AT-SPI2 | `ltk-a11y::atspi::AtSpiAdapter` |
| High contrast | GNOME | `ltk-a11y::high_contrast` |
| Reduce motion | macOS | `ltk-design::motion::MotionTokens::reduce_motion` |
| Increase contrast | macOS | `ContrastEngine::ensure_contrast()` |
| Larger text | Both | `DpiManager::set_user_scale()` |
| Voice control | macOS | Accessible name on every interactive widget |

---

*LTK Design Philosophy v1.0 — LionOS · © 2026 Lion*
