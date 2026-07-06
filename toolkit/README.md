# LTK — Lion Tool Kit
### A UI framework by Lion · Inspired by macOS & GNOME

---

```
        ██╗  ████████╗██╗  ██╗
        ██║  ╚══██╔══╝██║ ██╔╝
        ██║     ██║   █████╔╝
        ██║     ██║   ██╔═██╗
        ███████╗██║   ██║  ██╗
        ╚══════╝╚═╝   ╚═╝  ╚═╝
     Lion Tool Kit — LionOS UI Framework
```

---

## What Is LTK?

**LTK (Lion Tool Kit)** is the official native UI framework of [LionOS](https://lionos.dev).
Built by **Lion**, it draws deep inspiration from two of the most polished
design traditions in computing:

| macOS Human Interface Guidelines | GNOME HIG 3 / libadwaita |
|----------------------------------|--------------------------|
| Spring-physics motion model | Adaptive breakpoint layouts |
| Vibrancy / glass materials | Headerbar-first navigation |
| Traffic-light window controls | AdwNavigationSplitView pattern |
| Squircle geometry | Preference row / section groups |
| Contextual popover menus | GNOME-style empty states |
| Fluid gesture continuity | AT-SPI2 accessibility bridge |
| SF-Pro-inspired type scale | Session-aware dark/light mode |
| System accent colour system | D-Bus / XDG portal integration |

The result is **Leonux** — a design language that feels native on LionOS
while borrowing the best ideas from both worlds.

---

## File Map · 57 Slint Components

```
ltk/
├── ltk.slint                    ← Single master import (all components)
│
├── theme/                       ← Design token globals
│   ├── palette.slint            OKLCH-based colour system, 8 accent swatches
│   ├── typography.slint         macOS SF-Pro scale → Inter/Space Grotesk/JetBrains
│   ├── spacing.slint            8px grid, macOS control metrics
│   ├── radius.slint             Squircle progression xs → full (9999px)
│   ├── motion.slint             Spring presets + reduce-motion support
│   ├── shadows.slint            macOS elevation model (xs → 2xl)
│   └── icons.slint              60+ named SVG icon slots
│
├── windows/                     ← Window chrome
│   ├── LTKTrafficLights.slint   macOS traffic light control group
│   ├── LTKHeaderBar.slint       LionOS hybrid: traffic lights + GNOME headerbar
│   ├── LTKAppWindow.slint       Full app shell (adaptive sidebar + content)
│   └── LTKDialogWindow.slint    Modal dialog + GNOME bottom sheet variant
│
├── buttons/                     ← Interactive controls
│   ├── LTKButton.slint          Base: pill shape, spring press, shadow lift
│   ├── LTKPrimaryButton.slint   Filled accent — main CTA
│   ├── LTKSecondaryButton.slint Glass-bordered — secondary action
│   ├── LTKDestructiveButton.slint Red destructive (macOS HIG pattern)
│   ├── LTKGhostButton.slint     Transparent — toolbar / sidebar
│   ├── LTKIconButton.slint      Circular icon-only (28px default)
│   ├── LTKFAButton.slint        FAB: squircle→circle hover, spring scale
│   ├── LTKSegmentControl.slint  macOS UISegmentedControl exact clone
│   ├── LTKToggleSwitch.slint    macOS UISwitch: liquid fill, RK4 knob
│   └── LTKChip.slint            GNOME badge/filter chip
│
├── inputs/                      ← Data entry
│   ├── LTKTextField.slint       GNOME AdwEntryRow + macOS focus ring
│   ├── LTKSearchField.slint     Pill search: macOS NSSearchField + ⌘K badge
│   ├── LTKCheckBox.slint        macOS checkbox: spring check-draw
│   ├── LTKRadioButton.slint     macOS radio: spring inner dot
│   ├── LTKSlider.slint          macOS NSSlider: thumb scales on hover
│   └── LTKComboBox.slint        macOS NSPopUpButton: checkmark on selected
│
├── navigation/                  ← Layout navigation
│   ├── LTKSidebar.slint         GNOME nautilus sidebar + macOS Finder style
│   ├── LTKTabBar.slint          Pill / Underline / Solid tab styles
│   ├── LTKBreadcrumb.slint      macOS Finder path bar + chevron separators
│   ├── LTKStatusBar.slint       macOS bottom bar + GNOME action bar
│   ├── LTKToolbar.slint         macOS NSToolbar + GNOME Toolbar
│   └── LTKWindowSwitcher.slint  macOS Mission Control thumbnail strip
│
├── containers/                  ← Layout & grouping
│   ├── LTKCard.slint            Filled / Glass / Outlined variants, spring lift
│   ├── LTKGlassPanel.slint      macOS vibrancy: Thin / Regular / Thick tiers
│   ├── LTKPopover.slint         macOS NSPopover: arrow, spring scale-in
│   ├── LTKAccordion.slint       macOS disclosure group: spring chevron rotation
│   ├── LTKSection.slint         GNOME AdwPreferencesRow (SectionGroup + Row)
│   ├── LTKProgressBar.slint     Determinate + indeterminate shimmer
│   ├── LTKSpinner.slint         macOS NSProgressIndicator spinning arc
│   ├── LTKBadge.slint           Notification count / presence dot
│   ├── LTKAvatar.slint          Photo / initials / icon: Circle / Squircle / Square
│   ├── LTKDivider.slint         Horizontal / vertical separator
│   ├── LTKSkeleton.slint        Shimmer loading placeholder + SkeletonCard
│   ├── LTKEmptyState.slint      GNOME-style zero-data centred view
│   └── LTKMenuRow.slint         macOS contextual menu row + LTKContextMenu
│
├── notifications/               ← Feedback overlays
│   ├── LTKToast.slint           macOS banner: left accent stripe, spring slide-in
│   ├── LTKBanner.slint          GNOME AdwBanner: full-width, bottom accent line
│   └── LTKTooltip.slint         macOS dark pill: always-dark, kbd shortcut badge
│
├── layouts/                     ← Adaptive structure
│   ├── LTKAdaptiveLayout.slint  GNOME AdwNavigationSplitView (5 breakpoints)
│   └── LTKSplitView.slint       macOS NSSplitView / GNOME Paned: resizable
│
├── editor/                      ← Developer tools
│   ├── LTKCodeEditor.slint      JetBrains Mono, line numbers, syntax-ready shell
│   └── LTKTerminalView.slint    Terminal output: dark, monospace, selection
│
├── advanced/                    ← System-level components
│   ├── LTKSystemTray.slint      macOS menu-bar extra / GNOME system tray
│   └── LTKPropertyInspector.slint  Xcode / GNOME Builder property sidebar
│
└── examples/
    └── kitchen_sink.slint       All components on one scrollable page
```

---

## Quick Start

### 1. Import Everything (one line)

```slint
import { LTKPalette, LTKType, LTKSpace,
         LTKAppWindow, LTKHeaderBar,
         LTKPrimaryButton, LTKSecondaryButton,
         LTKTextField, LTKSearchField,
         LTKSidebar, LTKNavEntry,
         LTKCard, LTKGlassPanel, LTKToast } from "ltk/ltk.slint";
```

### 2. Hello LionOS App

```slint
import { LTKPalette, LTKType, LTKSpace }
    from "ltk/ltk.slint";
import { LTKHeaderBar }
    from "ltk/windows/LTKHeaderBar.slint";
import { LTKPrimaryButton, LTKSecondaryButton, LTKToggleSwitch }
    from "ltk/ltk.slint";
import { LTKTextField }
    from "ltk/inputs/LTKTextField.slint";

export component MyApp inherits Window {
    title: "My LionOS App";
    no-frame: true;
    background: LTKPalette.bg-base;
    default-font-family: LTKType.font-ui;
    width: 800px; height: 600px;

    VerticalLayout {
        spacing: 0px;

        LTKHeaderBar {
            title: "My App";
            subtitle: "By Lion";
        }

        VerticalLayout {
            padding: LTKSpace.space-6;
            spacing: LTKSpace.space-4;
            alignment: start;

            LTKTextField {
                label: "Your name";
                placeholder: "lion@lionos.dev";
            }

            LTKToggleSwitch {
                label: "Enable dark mode";
                description: "Adjust for ambient light";
                checked: true;
            }

            HorizontalLayout {
                spacing: LTKSpace.space-1;
                LTKSecondaryButton { text: "Cancel"; }
                LTKPrimaryButton   { text: "Save changes"; }
            }
        }
    }
}
```

### 3. GNOME-style Settings Page

```slint
import { LTKSectionGroup, LTKSectionRow,
         LTKToggleSwitch, LTKGhostButton,
         LTKHeaderBar }
    from "ltk/ltk.slint";

export component SettingsPage inherits Window {
    no-frame: true;
    background: LTKPalette.bg-base;
    width: 680px; height: 800px;

    VerticalLayout {
        LTKHeaderBar { title: "Settings"; }

        ScrollView {
            VerticalLayout {
                padding: 24px; spacing: 8px;

                LTKSectionGroup {
                    label: "APPEARANCE";

                    LTKSectionRow {
                        title: "Theme";
                        subtitle: "LionOS Dark (default)";
                        separator: true;
                        show-chevron: true; interactive: true;
                    }
                    LTKSectionRow {
                        title: "Accent colour";
                        subtitle: "Blue · 213° OKLCH";
                        separator: true;
                        show-chevron: true; interactive: true;
                    }
                    LTKSectionRow {
                        title: "Reduce motion";
                        separator: false;
                        LTKToggleSwitch { }
                    }
                }

                LTKSectionGroup {
                    label: "PRIVACY";

                    LTKSectionRow {
                        title: "Analytics";
                        subtitle: "Share anonymous crash reports";
                        separator: true;
                        LTKToggleSwitch { checked: true; }
                    }
                    LTKSectionRow {
                        title: "Location";
                        subtitle: "Never";
                        separator: false;
                        show-chevron: true; interactive: true;
                    }
                }
            }
        }
    }
}
```

### 4. Adaptive Two-Pane App (macOS Master-Detail)

```slint
import { LTKAdaptiveLayout }  from "ltk/layouts/LTKAdaptiveLayout.slint";
import { LTKSidebar, LTKNavEntry } from "ltk/ltk.slint";
import { LTKHeaderBar } from "ltk/ltk.slint";

export component FilesApp inherits Window {
    no-frame: true;
    background: LTKPalette.bg-base;
    min-width: 360px; width: 1100px; height: 700px;

    property <bool> sidebar-open: true;

    VerticalLayout {
        LTKHeaderBar {
            title: "Files";
        }

        LTKAdaptiveLayout {
            sidebar-open: sidebar-open;
            // Sidebar collapses to icon-rail at 640–900px
            // Overlay drawer below 640px (mobile/narrow)
        }
    }
}
```

---

## Design System Reference

### Colour Tokens (from `LTKPalette`)

| Token | Dark | Light | Usage |
|-------|------|-------|-------|
| `bg-base` | `#0B0D0F` | `#EFEDE8` | App background |
| `bg-surface` | `#15171B` | `#FFFFFF` | Cards, panels |
| `bg-raised` | `#1C1F25` | `#F4F2EF` | Menus, popovers |
| `bg-overlay` | `#24272F` | `#E8E6E2` | Modals, sheets |
| `bg-headerbar` | `#12141A` | `#DDDBD6` | Window chrome |
| `bg-sidebar` | `#0F1115` | `#E8E5E0` | Navigation sidebar |
| `accent` | `#5B9DFA` | `#5B9DFA` | Primary brand colour |
| `text-primary` | `#F1F2F5` | `#1A1B1F` | Body text |
| `text-secondary` | `#9CA3AF` | `#5A5E6B` | Label text |
| `text-tertiary` | `#5C6370` | `#9499A5` | Hints, placeholders |
| `glass-thin` | `#15171B66` | `#FFFFFF66` | Tooltips, badges |
| `glass-regular` | `#15171BAA` | `#FFFFFFAA` | Panels, menus |
| `glass-thick` | `#15171BD9` | `#FFFFFFD9` | Modals, sheets |
| `traffic-close` | `#FF5F57` | same | Close button |
| `traffic-minimize` | `#FFBD2E` | same | Minimize button |
| `traffic-zoom` | `#28C840` | same | Zoom button |

### Accent Swatches (macOS system colour names)

```slint
LTKPalette.blue      // #5B9DFA — LionOS default
LTKPalette.purple    // #9D7AFA
LTKPalette.pink      // #F472B6
LTKPalette.red       // #F28B82
LTKPalette.orange    // #FB923C
LTKPalette.yellow    // #FBBF24
LTKPalette.green     // #5EE29A
LTKPalette.graphite  // #8B8FA8
```

### Type Scale (from `LTKType`)

| Token | Size | Weight | Family |
|-------|------|--------|--------|
| `large-title` | 34px | Bold | Space Grotesk |
| `title-1` | 28px | Bold | Space Grotesk |
| `title-2` | 22px | Bold | Space Grotesk |
| `title-3` | 20px | Semibold | Space Grotesk |
| `headline` | 17px | Semibold | Inter |
| `body` | 17px | Regular | Inter |
| `callout` | 16px | Regular | Inter |
| `subheadline` | 15px | Regular | Inter |
| `footnote` | 13px | Regular | Inter |
| `caption-1` | 12px | Regular | Inter |
| `caption-2` | 11px | Regular | Inter |

### Motion Presets (from `LTKMotion`)

| Easing | Curve | Use case |
|--------|-------|----------|
| `ease-spring` | `cubic-bezier(0.34, 1.56, 0.64, 1)` | Buttons, menu items, FAB |
| `ease-soft` | `cubic-bezier(0.16, 1.00, 0.30, 1)` | Panels, drawers (no overshoot) |
| `ease-decelerate` | `cubic-bezier(0.00, 0.00, 0.20, 1)` | Elements entering the screen |
| `ease-accelerate` | `cubic-bezier(0.40, 0.00, 1.00, 1)` | Elements leaving the screen |
| `ease-page` | `cubic-bezier(0.42, 0.00, 0.58, 1)` | Page-level transitions |

### Spring Physics Presets

| Name | Stiffness | Damping | Overshoot | Use case |
|------|-----------|---------|-----------|----------|
| micro | 600 | 38 | None | Checkbox, radio, toggle knob |
| standard | 280 | 26 | Slight | Buttons, menus, cards |
| bouncy | 200 | 14 | Strong | FAB, onboarding, celebrations |
| gentle | 120 | 20 | None | Sheet, drawer entrance |
| stiff | 500 | 40 | None | Drag snap, resize handles |

### Adaptive Breakpoints (from `LTKAdaptiveLayout`)

| Name | Width | Sidebar | Layout |
|------|-------|---------|--------|
| `is-mobile` | < 360px | Overlay (hidden) | Single column |
| `is-narrow` | 360–640px | Overlay (hamburger) | Single column |
| `is-compact` | 640–900px | Persistent icon-only | Sidebar + content |
| `is-medium` | 900–1200px | Persistent full labels | Sidebar + content |
| `is-wide` | > 1200px | Persistent + inspector | Three-pane |

---

## Principles Applied

### From macOS

- **Pill-shaped buttons** — `border-radius: 9999px` on every button
- **Traffic light controls** — exact macOS colour spec (grey → coloured on hover)
- **Spring physics** — every interaction uses spring easing, not cubic-bezier
- **Shadow lift on hover** — cards and buttons lift with a soft shadow on hover
- **Squircle geometry** — radii > 12px use the squircle progression
- **Always-dark tooltips** — even in light mode, tooltips stay dark with white text
- **Vibrancy glass** — three tiers: thin / regular / thick, adapts to dark/light
- **WCAG contrast** — all text generated via OKLCH guarantees minimum 4.5:1

### From GNOME

- **Headerbar-first** — no separate menu bar; actions live in popovers and ⋮ menus
- **AdwPreferencesRow** — `LTKSectionGroup` + `LTKSectionRow` for settings UIs
- **AdwNavigationSplitView** — `LTKAdaptiveLayout` with 5 breakpoints
- **Empty states** — `LTKEmptyState` with icon, headline, body, action
- **GNOME banners** — `LTKBanner` as full-width info / warning / error strip
- **Bottom sheet** — `LTKDialogWindow` with `sheet-mode: true`
- **AT-SPI2 accessibility** — every component declares accessible role/name/state
- **Reduce-motion** — `LTKMotion.reduce-motion` zeroes all durations

---

## Accessibility

Every interactive component:
- Sets accessible **role** (Button, CheckBox, Slider, MenuItem…)
- Provides accessible **name** from its label/text property
- Supports **keyboard focus** via `FocusScope`
- Emits AT-SPI2 events via `ltk-a11y` foundation layer
- Respects **reduce-motion** via `LTKMotion.reduce-motion`
- Respects **high-contrast** mode via `ltk-a11y::high_contrast`
- Maintains **WCAG AA** contrast on all text (guaranteed by OKLCH generator)

---

## Licence & Credits

**MIT OR Apache-2.0**

Designed and built by **Lion** for [LionOS](https://lionos.dev).
Design inspiration: Apple Human Interface Guidelines, GNOME HIG 3, libadwaita.
Typography: Inter, Space Grotesk, JetBrains Mono.
Colour system: OKLCH-based palette generation.

© 2026 LionOS Project · Lion
