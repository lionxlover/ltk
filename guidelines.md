# Lion Toolkit (LTK) Human Interface Guidelines & Master Design System Handbook
## A Complete Technical Guide to Liquid Layouts, Liquid Typography, and High-Aesthetic UI Engineering in Slint & Rust

---

## Table of Contents
1. [Executive Summary & Design Foundations](#1-executive-summary--design-foundations)
   - 1.1 Core Philosophy: Fluidity, Deference, Tactile Physics, Accessibility
   - 1.2 The Mathematics of Liquid Layouts vs Static Pixel Layouts
   - 1.3 Mathematical Fluid Typography Equations
   - 1.4 The 8-Point Spatial Grid System & Tokenization
2. [Design Tokens & Theme Architecture (`Theme.slint`)](#2-design-tokens--theme-architecture-themeslint)
   - 2.1 Color Palette Architecture & Surface Tiers
   - 2.2 Typographic Scaling Tokens
   - 2.3 Spacing, Padding, and Structural Tokens
   - 2.4 Corner Radii, Elevation Shadows, and Opacity Tiers
   - 2.5 Motion Easing & Duration Curves
   - 2.6 Full `Theme.slint` Source Code Definition
3. [Liquid Layout Architecture & Spatial Rules](#3-liquid-layout-architecture--spatial-rules)
   - 3.1 The Rule of Zero Hardcoded Dimensions
   - 3.2 Constraint-Based Layout Engines in Slint
   - 3.3 Stretch Factors and Relative Bounds
   - 3.4 Adaptive Container Breakpoints & Responsive Media Queries
   - 3.5 Shell Architecture: Headerbars, Sidebars, Viewports, and Floating Bars
   - 3.6 Scrollable Liquid Viewports (`Flickable` Best Practices)
4. [Liquid Typography & Font Scaling Mathematics](#4-liquid-typography--font-scaling-mathematics)
   - 4.1 The 10-Tier Typographic Scale
   - 4.2 Fluid Font Calculation Formulas
   - 4.3 Line Height, Character Spacing, and Vertical Rhythm
   - 4.4 Multi-Line Word Wrapping, Truncation, and Elision
   - 4.5 Baseline Alignment Mathematics (`y: (parent.height - self.height) / 2`)
5. [Complete Component Blueprints (Exhaustive Library Reference)](#5-complete-component-blueprints-exhaustive-library-reference)
   - 5.1 Buttons & Action Controls (38 Complete Component Codes)
   - 5.2 Input Controls & Selection Mechanics (18 Complete Component Codes)
   - 5.3 Data Display Surfaces & Cards (25 Complete Component Codes)
   - 5.4 Feedback, Overlays, and Status Indicators (10 Complete Component Codes)
   - 5.5 Navigation Components & Layout Structures (10 Complete Component Codes)
6. [Motion, Transitions & Micro-Interactions](#6-motion-transitions--micro-interactions)
   - 6.1 Easing Curves and Physics Simulation
   - 6.2 Interactive State Transitions (Hover, Press, Focus, Active, Disabled)
   - 6.3 Micro-Animations in Slint
7. [Rust Backend Integration & Asynchronous State Synchronization](#7-rust-backend-integration--asynchronous-state-synchronization)
   - 7.1 Coupling Slint Views with Rust Handlers
   - 7.2 Asynchronous Tokio Event Loop Pattern
   - 7.3 D-Bus & XDG Desktop Portal Theme Synchronization
8. [Complete End-to-End Liquid Reference Applications](#8-complete-end-to-end-liquid-reference-applications)
   - 8.1 Application 1: LTK Liquid Studio & Control Center
   - 8.2 Application 2: LTK Media Workbench & Asset Inspector
   - 8.3 Application 3: Complete Rust Backend Code (`main.rs` & `thememanager.rs`)

---

# 1. Executive Summary & Design Foundations

The **Lion Toolkit (LTK)** is a modern, ultra-high-performance UI component library engineered with **Slint** and **Rust**. LTK is designed from first principles to provide desktop and embedded applications with fluid responsiveness, stunning visual aesthetics, and resilient desktop integration.

Inspired by the design philosophies of the **Apple Human Interface Guidelines (HIG)** and the **GNOME Human Interface Guidelines (HIG)**, LTK provides developers with a comprehensive blueprint for constructing applications that adapt gracefully across all display geometries—from compact 720p windows to 4K desktop displays and tiling window managers (Sway, Hyprland, i3, KWin).

---

## 1.1 Core Philosophy: Fluidity, Deference, Tactile Physics, Accessibility

### I. Fluidity & Liquid Adaptability
An LTK interface is not rigid; it behaves like a liquid inside a container. Containers expand, shrink, reflow, and recalculate child positioning dynamically. Fixed pixel widths on top-level views are strictly forbidden.

### II. Deference & Content First
Visual chrome must never compete with user content. LTK uses layered translucent surfaces (`Theme.bg-base`, `Theme.bg-surface`, `Theme.bg-raised`, `Theme.bg-overlay`), subtle 1px structural borders, and calibrated drop-shadow elevations to establish clear spatial depth.

### III. Tactile Physics & Micro-Feedback
Every user action receives immediate visual confirmation. Buttons recess slightly on click, hover states transition smoothly over 150ms using soft cubic-bezier easing curves, and focus rings illuminate seamlessly.

### IV. Universal Accessibility
All interactive controls maintain a minimum target size of 36px–40px. Text contrast ratios satisfy WCAG AAA standards across both dark and light modes.

---

## 1.2 The Mathematics of Liquid Layouts vs Static Pixel Layouts

In legacy UI development, layouts were constructed using static pixel coordinates:

$$\text{Position}_{\text{legacy}} = (x_0, y_0, w_{\text{fixed}}, h_{\text{fixed}})$$

This static approach causes massive visual bugs: empty white gaps on large screens, scrollbars inside small windows, and clipped text labels.

LTK uses **Constraint-Based Liquid Equations**:

$$\text{Width}_{\text{element}} = \max\left(w_{\min}, \min\left(w_{\max}, w_{\text{parent}} \cdot \alpha + \beta\right)\right)$$

$$\text{Position}_{y,\text{centered}} = \frac{H_{\text{parent}} - h_{\text{child}}}{2}$$

By expressing element boundaries as continuous functions of window dimensions, LTK layouts automatically adapt to any screen geometry.

---

## 1.3 Mathematical Fluid Typography Equations

Fluid typography dynamically interpolates font size between a lower bound $f_{\min}$ at window width $w_{\min}$ and an upper bound $f_{\max}$ at window width $w_{\max}$:

$$f(w) = \text{clamp}\left(f_{\min},\; f_{\min} + (f_{\max} - f_{\min}) \cdot \frac{w - w_{\min}}{w_{\max} - w_{\min}},\; f_{\max}\right)$$

In Slint expression syntax, this fluid font equation is implemented as:

```slint
font-size: Math.max(min-size, Math.min(max-size, min-size + (max-size - min-size) * (root.width - min-width) / (max-width - min-width)));
```

For simplified responsive scaling, LTK provides linear proportion clamping:

```slint
font-size: Math.max(16px, Math.min(32px, root.width / 24));
```

---

## 1.4 The 8-Point Spatial Grid System & Tokenization

LTK enforces strict spatial rhythm using an **8-point grid** (with 4px micro-steps):

```
+-----------------------------------------------------------------------+
|  Theme.sp-1  (4px)   | Micro-spacing (badges, icon offsets)           |
|  Theme.sp-2  (8px)   | Tight inline gaps (icon to text)               |
|  Theme.sp-3  (12px)  | Standard control group spacing                 |
|  Theme.sp-4  (16px)  | Content padding inside cards & containers     |
|  Theme.sp-6  (24px)  | Main section padding                           |
|  Theme.sp-8  (32px)  | Major structural section breaks                |
|  Theme.sp-12 (48px)  | Page hero header spacing                       |
+-----------------------------------------------------------------------+
```

---

# 2. Design Tokens & Theme Architecture (`Theme.slint`)

The foundation of LTK is the central `Theme.slint` singleton module. All components draw their styling properties from `Theme`, enabling dynamic light/dark mode switching and global theme customization.

```slint
// slint/core/Theme.slint

export global Theme {
    // --- COLOR PALETTE & SURFACES ---
    in-out property <bool> dark-mode: true;

    // Background Layers
    out property <color> bg-base: dark-mode ? #0B0D0F : #F4F5F7;
    out property <color> bg-surface: dark-mode ? #15171B : #FFFFFF;
    out property <color> bg-raised: dark-mode ? #1E2228 : #F0F2F5;
    out property <color> bg-overlay: dark-mode ? #282D35 : #E4E7ED;

    // Interactive Surface States
    out property <color> surface-hover: dark-mode ? #262B33 : #E8EBEF;
    out property <color> surface-pressed: dark-mode ? #303742 : #DCDFE5;
    out property <color> surface-selected: dark-mode ? #1E3A8A : #DBEAFE;

    // Primary Accents
    out property <color> accent: #5B9DFA;
    out property <color> accent-hover: #4B8EEB;
    out property <color> accent-pressed: #3A7CDC;
    out property <color> accent-subtle: dark-mode ? #172554 : #EFF6FF;

    // Text & Content Hierarchy
    out property <color> text-primary: dark-mode ? #F3F4F6 : #111827;
    out property <color> text-secondary: dark-mode ? #9CA3AF : #4B5563;
    out property <color> text-tertiary: dark-mode ? #6B7280 : #9CA3AF;
    out property <color> text-disabled: dark-mode ? #4B5563 : #D1D5DB;

    // State Colors
    out property <color> success: #22C55E;
    out property <color> warning: #F59E0B;
    out property <color> error: #EF4444;
    out property <color> info: #3B82F6;

    out property <color> state-success: #22C55E;
    out property <color> state-warning: #F59E0B;
    out property <color> state-error: #EF4444;
    out property <color> state-info: #3B82F6;

    // Button Disabled Tokens
    out property <color> btn-disabled-bg: dark-mode ? #1F2937 : #E5E7EB;
    out property <color> btn-disabled-text: dark-mode ? #4B5563 : #9CA3AF;
    out property <color> btn-primary-bg-hover: #4B8EEB;
    out property <color> btn-primary-bg-pressed: #3A7CDC;

    // Borders
    out property <color> border-subtle: dark-mode ? #1F242D : #E5E7EB;
    out property <color> border-base: dark-mode ? #2B313D : #D1D5DB;
    out property <color> border-strong: dark-mode ? #3D4656 : #9CA3AF;

    // --- TYPOGRAPHY SCALE ---
    out property <string> font-ui: "Inter, system-ui, -apple-system, sans-serif";
    out property <string> font-display: "Outfit, Inter, system-ui, sans-serif";
    out property <string> font-mono: "JetBrains Mono, Fira Code, monospace";

    out property <length> text-xs: 11px;
    out property <length> text-sm: 12px;
    out property <length> text-base: 14px;
    out property <length> text-lg: 16px;
    out property <length> text-xl: 18px;
    out property <length> text-2xl: 22px;
    out property <length> text-3xl: 28px;
    out property <length> text-4xl: 36px;

    out property <int> weight-light: 300;
    out property <int> weight-regular: 400;
    out property <int> weight-medium: 500;
    out property <int> weight-semibold: 600;
    out property <int> weight-bold: 700;

    // --- SPACING SCALE ---
    out property <length> sp-1: 4px;
    out property <length> sp-2: 8px;
    out property <length> sp-3: 12px;
    out property <length> sp-4: 16px;
    out property <length> sp-5: 20px;
    out property <length> sp-6: 24px;
    out property <length> sp-8: 32px;
    out property <length> sp-10: 40px;
    out property <length> sp-12: 48px;

    // --- CORNER RADII ---
    out property <length> radius-xs: 4px;
    out property <length> radius-sm: 6px;
    out property <length> radius-md: 8px;
    out property <length> radius-lg: 12px;
    out property <length> radius-xl: 16px;
    out property <length> radius-full: 9999px;

    // --- CONTROL SIZES ---
    out property <length> button-height-sm: 32px;
    out property <length> button-height-md: 40px;
    out property <length> button-height-lg: 48px;

    out property <length> input-padding-x: 14px;
    out property <length> icon-sm: 14px;
    out property <length> icon-md: 16px;
    out property <length> icon-lg: 20px;

    // --- MOTION & ANIMATION ---
    out property <duration> dur-fast: 150ms;
    out property <duration> dur-normal: 250ms;
    out property <duration> dur-slow: 400ms;

    out property <easing> ease-soft: cubic-bezier(0.16, 1, 0.3, 1);
    out property <easing> ease-standard: cubic-bezier(0.2, 0, 0, 1);

    // --- ELEVATIONS & SHADOWS ---
    out property <length> elevation-1-y: 2px;
    out property <length> elevation-1-blur: 4px;
    out property <color> elevation-1-color: dark-mode ? #00000040 : #0000001A;

    out property <length> elevation-2-y: 6px;
    out property <length> elevation-2-blur: 12px;
    out property <color> elevation-2-color: dark-mode ? #00000060 : #00000020;

    out property <float> opacity-disabled: 0.45;
}
```

---

# 3. Liquid Layout Architecture & Spatial Rules

Liquid layout design ensures that UI elements adapt dynamically to any window size.

```
+--------------------------------------------------------------------------------+
|  LIQUID WINDOW BOUNDS (Window width & height are flexible)                     |
|                                                                                |
|  +----------------+  +------------------------------------------------------+  |
|  | SIDEBAR        |  | MAIN CONTENT WORKSPACE (stretch: 1)                  |  |
|  |                |  |                                                      |  |
|  | width: 220px   |  |  +------------------------------------------------+  |  |
|  | (fixed column) |  |  | FLUID HEADER (width: 100%)                       |  |  |
|  |                |  |  +------------------------------------------------+  |  |
|  |                |  |                                                      |  |
|  | vertical-      |  |  +------------------------------------------------+  |  |
|  | stretch: 1     |  |  | FLICKABLE VIEWPORT                             |  |  |
|  |                |  |  |  +------------------------------------------+  |  |  |
|  |                |  |  |  | Content Stack (preferred-height)           |  |  |  |
|  |                |  |  |  +------------------------------------------+  |  |  |
|  |                |  |  +------------------------------------------------+  |  |
|  +----------------+  +------------------------------------------------------+  |
+--------------------------------------------------------------------------------+
```

---

## 3.1 The Rule of Zero Hardcoded Dimensions

> [!CAUTION]
> **ANTI-PATTERN**: Hardcoding static sizes like `width: 800px; height: 600px;` on root test harnesses or page components breaks responsive scaling.

```slint
// ANTI-PATTERN: DO NOT DO THIS
export component RigidPage inherits Rectangle {
    width: 800px;   // WRONG! Clips on small screens, leaves blank space on large screens.
    height: 600px;  // WRONG! Disables vertical expansion.
}

// LIQUID PATTERN: DO THIS INSTEAD
export component LiquidPage inherits Rectangle {
    background: Theme.bg-base;
    // Fills 100% of parent width and height dynamically!
    
    Flickable {
        width: 100%;
        height: 100%;
        viewport-height: content.preferred-height;
        
        content := VerticalLayout {
            padding: Theme.sp-6;
            spacing: Theme.sp-4;
            // Page items...
        }
    }
}
```

---

## 3.2 Constraint-Based Layout Engines in Slint

Slint provides four primary layout primitives:

1. `HorizontalLayout`: Arranges child items in a horizontal row.
2. `VerticalLayout`: Arranges child items in a vertical column stack.
3. `GridBox` / `GridLayout`: Positions elements in a two-dimensional grid.
4. `Flickable`: Enables touch/wheel scrolling for content exceeding viewport dimensions.

---

## 3.3 Stretch Factors and Relative Bounds

Stretch factors specify how extra spatial room is allocated among siblings:

```slint
HorizontalLayout {
    spacing: Theme.sp-3;

    // Search bar expands to take all remaining horizontal room
    SearchInput {
        placeholder: "Search items...";
        horizontal-stretch: 1;
    }

    // Filter button stays at its natural content size
    SecondaryButton {
        text: "Filters";
        horizontal-stretch: 0;
    }
}
```

---

## 3.4 Adaptive Container Breakpoints & Responsive Media Queries

LTK components can declare internal breakpoint properties based on `root.width`:

```slint
export component AdaptiveWorkspace inherits Rectangle {
    property <bool> is-compact: root.width < 640px;
    property <bool> is-tablet: root.width >= 640px && root.width < 1024px;
    property <bool> is-desktop: root.width >= 1024px;

    // Compact layout (Mobile/Small Window)
    if is-compact: VerticalLayout {
        spacing: Theme.sp-3;
        MobileTopBar {}
        MainContent { vertical-stretch: 1; }
        MobileBottomNav {}
    }

    // Expanded layout (Desktop Window)
    if !is-compact: HorizontalLayout {
        spacing: Theme.sp-4;
        DesktopSidebar { width: 220px; }
        MainContent { horizontal-stretch: 1; }
    }
}
```

---

# 4. Liquid Typography & Font Scaling Mathematics

Liquid typography ensures text labels remain readable across all window dimensions.

---

## 4.1 The 10-Tier Typographic Scale

```
Display 1  (36px / Bold)     -> Banners & Hero Headlines
Display 2  (28px / Bold)     -> Major Page Titles
Title 1    (22px / SemiBold) -> Section Headers & Modal Titles
Title 2    (18px / SemiBold) -> Card Titles & Group Headers
Heading    (16px / SemiBold) -> Sub-headers & Input Labels
Body Large (16px / Regular)  -> Featured Paragraphs
Body Base  (14px / Regular)  -> Default Control Text & Body Text
Body Small (12px / Regular)  -> Secondary Descriptions
Caption    (11px / Medium)   -> Timestamps & Status Badges
Monospace  (13px / Code)     -> Code Blocks & Terminal Log Streams
```

---

## 4.2 Baseline Alignment Mathematics (`y: (parent.height - self.height) / 2`)

> [!IMPORTANT]
> **Baseline Alignment Formula**: In any row where an icon (SVG or `FaIcon`) sits next to text inside a layout container, the icon MUST be explicitly centered vertically using `y: (parent.height - self.height) / 2;`.

```slint
HorizontalLayout {
    alignment: center;
    spacing: 8px;

    Rectangle {
        width: 16px;
        height: 16px;
        y: (parent.height - self.height) / 2; // Pixel-perfect vertical centering

        Image {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/check.svg");
            width: 16px;
            height: 16px;
            colorize: Theme.accent;
        }
    }

    Text {
        text: "Confirmed";
        color: Theme.text-primary;
        font-size: Theme.text-base;
        vertical-alignment: center;
    }
}
```

---

# 5. Complete Component Blueprints (Exhaustive Library Reference)

This section provides complete, verified Slint source code implementations for every component tier in LTK.

---

## 5.1 Buttons & Action Controls (38 Complete Component Codes)

### 1. Primary Button (`PrimaryButton.slint`)
```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component PrimaryButton inherits Rectangle {
    in-out property <string> text: "Primary";
    in-out property <bool> enabled: true;

    callback clicked();

    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? Theme.btn-primary-bg-pressed : (ta.has-hover ? Theme.btn-primary-bg-hover : Theme.accent));
    border-radius: Theme.radius-md;
    height: Theme.button-height-md;
    horizontal-stretch: 1;

    animate background { duration: Theme.dur-fast; easing: Theme.ease-soft; }

    HorizontalLayout {
        alignment: center;
        spacing: Theme.sp-2;

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/check.svg");
            fa-size: Theme.icon-sm;
            fa-color: enabled ? #FFFFFF : Theme.btn-disabled-text;
            y: (parent.height - self.height) / 2;
        }

        Text {
            text: root.text;
            color: enabled ? #FFFFFF : Theme.btn-disabled-text;
            font-family: Theme.font-ui;
            font-size: Theme.text-base;
            font-weight: Theme.weight-semibold;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

### 2. Secondary Button (`SecondaryButton.slint`)
```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component SecondaryButton inherits Rectangle {
    in-out property <string> text: "Secondary";
    in-out property <bool> enabled: true;

    callback clicked();

    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? Theme.surface-pressed : (ta.has-hover ? Theme.surface-hover : Theme.bg-raised));
    border-radius: Theme.radius-md;
    border-width: 1px;
    border-color: Theme.border-base;
    height: Theme.button-height-md;
    horizontal-stretch: 1;

    animate background { duration: Theme.dur-fast; easing: Theme.ease-soft; }

    HorizontalLayout {
        alignment: center;
        spacing: Theme.sp-2;

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/sliders.svg");
            fa-size: Theme.icon-sm;
            fa-color: enabled ? Theme.text-primary : Theme.btn-disabled-text;
            y: (parent.height - self.height) / 2;
        }

        Text {
            text: root.text;
            color: enabled ? Theme.text-primary : Theme.btn-disabled-text;
            font-family: Theme.font-ui;
            font-size: Theme.text-base;
            font-weight: Theme.weight-semibold;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

### 3. Ghost Button (`GhostButton.slint`)
```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component GhostButton inherits Rectangle {
    in-out property <string> text: "Ghost";
    in-out property <bool> enabled: true;

    callback clicked();

    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? Theme.surface-pressed : (ta.has-hover ? Theme.surface-hover : transparent));
    border-radius: Theme.radius-md;
    height: Theme.button-height-md;
    horizontal-stretch: 1;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        alignment: center;
        spacing: Theme.sp-2;

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/ghost.svg");
            fa-size: Theme.icon-sm;
            fa-color: enabled ? Theme.text-secondary : Theme.btn-disabled-text;
            y: (parent.height - self.height) / 2;
        }

        Text {
            text: root.text;
            color: enabled ? Theme.text-secondary : Theme.btn-disabled-text;
            font-size: Theme.text-base;
            font-weight: Theme.weight-semibold;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

### 4. Outlined Button (`OutlinedButton.slint`)
```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component OutlinedButton inherits Rectangle {
    in-out property <string> text: "Outlined";
    in-out property <bool> enabled: true;

    callback clicked();

    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? Theme.surface-pressed : (ta.has-hover ? Theme.surface-hover : transparent));
    border-radius: Theme.radius-md;
    border-width: 1px;
    border-color: Theme.border-base;
    height: Theme.button-height-md;
    horizontal-stretch: 1;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        alignment: center;
        spacing: Theme.sp-2;

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/pen-to-square.svg");
            fa-size: Theme.icon-sm;
            fa-color: enabled ? Theme.text-primary : Theme.btn-disabled-text;
            y: (parent.height - self.height) / 2;
        }

        Text {
            text: root.text;
            color: enabled ? Theme.text-primary : Theme.btn-disabled-text;
            font-size: Theme.text-base;
            font-weight: Theme.weight-semibold;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

### 5. Danger Button (`DangerButton.slint`)
```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component DangerButton inherits Rectangle {
    in-out property <string> text: "Delete";
    in-out property <bool> enabled: true;

    callback clicked();

    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? Theme.error.with-alpha(0.8) : (ta.has-hover ? Theme.error.with-alpha(0.9) : Theme.error));
    border-radius: Theme.radius-md;
    height: Theme.button-height-md;
    horizontal-stretch: 1;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        alignment: center;
        spacing: Theme.sp-2;

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/trash.svg");
            fa-size: Theme.icon-sm;
            fa-color: enabled ? #FFFFFF : Theme.btn-disabled-text;
            y: (parent.height - self.height) / 2;
        }

        Text {
            text: root.text;
            color: enabled ? #FFFFFF : Theme.btn-disabled-text;
            font-size: Theme.text-base;
            font-weight: Theme.weight-semibold;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

### 6. Success Button (`SuccessButton.slint`)
```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component SuccessButton inherits Rectangle {
    in-out property <string> text: "Confirm";
    in-out property <bool> enabled: true;

    callback clicked();

    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? Theme.success.with-alpha(0.8) : (ta.has-hover ? Theme.success.with-alpha(0.9) : Theme.success));
    border-radius: Theme.radius-md;
    height: Theme.button-height-md;
    horizontal-stretch: 1;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        alignment: center;
        spacing: Theme.sp-2;

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/circle-check.svg");
            fa-size: Theme.icon-sm;
            fa-color: enabled ? #FFFFFF : Theme.btn-disabled-text;
            y: (parent.height - self.height) / 2;
        }

        Text {
            text: root.text;
            color: enabled ? #FFFFFF : Theme.btn-disabled-text;
            font-size: Theme.text-base;
            font-weight: Theme.weight-semibold;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

### 7. Warning Button (`WarningButton.slint`)
```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component WarningButton inherits Rectangle {
    in-out property <string> text: "Warning";
    in-out property <bool> enabled: true;

    callback clicked();

    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? Theme.warning.with-alpha(0.8) : (ta.has-hover ? Theme.warning.with-alpha(0.9) : Theme.warning));
    border-radius: Theme.radius-md;
    height: Theme.button-height-md;
    horizontal-stretch: 1;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        alignment: center;
        spacing: Theme.sp-2;

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/triangle-exclamation.svg");
            fa-size: Theme.icon-sm;
            fa-color: enabled ? #000000 : Theme.btn-disabled-text;
            y: (parent.height - self.height) / 2;
        }

        Text {
            text: root.text;
            color: enabled ? #000000 : Theme.btn-disabled-text;
            font-size: Theme.text-base;
            font-weight: Theme.weight-semibold;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

### 8. Reaction Button (`ReactionButton.slint`)
```slint
import { Theme } from "../core/Theme.slint";

export component ReactionButton inherits Rectangle {
    in-out property <int> count: 12;
    in-out property <bool> active: false;

    callback clicked();

    height: 32px;
    background: root.active ? Theme.accent-subtle : Theme.bg-surface;
    border-radius: 16px;
    border-width: 1px;
    border-color: root.active ? Theme.accent : Theme.border-base;
    horizontal-stretch: 0;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        padding-left: 10px;
        padding-right: 10px;
        spacing: 6px;
        alignment: center;

        Image {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/thumbs-up.svg");
            width: 14px;
            height: 14px;
            y: (parent.height - self.height) / 2;
            colorize: root.active ? Theme.accent : Theme.text-secondary;
        }

        Text {
            text: root.count;
            color: root.active ? Theme.accent : Theme.text-secondary;
            font-size: Theme.text-sm;
            font-weight: Theme.weight-medium;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        clicked => {
            root.active = !root.active;
            root.clicked();
        }
    }
}
```

### 9. Submit Button (`SubmitButton.slint`)
```slint
import { Theme } from "../core/Theme.slint";

export component SubmitButton inherits Rectangle {
    in-out property <string> text: "Submit";
    in-out property <bool> loading: false;
    in-out property <bool> enabled: true;

    callback clicked();

    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? Theme.btn-primary-bg-pressed : (ta.has-hover ? Theme.btn-primary-bg-hover : Theme.accent));
    border-radius: Theme.radius-md;
    height: Theme.button-height-md;
    horizontal-stretch: 1;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        alignment: center;
        spacing: Theme.sp-2;

        Rectangle {
            width: 14px;
            height: 14px;
            y: (parent.height - self.height) / 2;

            Image {
                source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/paper-plane.svg");
                width: 14px;
                height: 14px;
                visible: !root.loading;
                colorize: enabled ? #FFFFFF : Theme.btn-disabled-text;
            }

            Image {
                source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/spinner.svg");
                width: 14px;
                height: 14px;
                visible: root.loading;
                colorize: enabled ? #FFFFFF : Theme.btn-disabled-text;
            }
        }

        Text {
            text: root.text;
            color: enabled ? #FFFFFF : Theme.btn-disabled-text;
            font-size: Theme.text-base;
            font-weight: Theme.weight-semibold;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

### 10. Swipe To Confirm (`SwipeToConfirm.slint`)
```slint
import { Theme } from "../core/Theme.slint";

export component SwipeToConfirm inherits Rectangle {
    in-out property <bool> confirmed: false;

    callback clicked();

    background: Theme.bg-raised;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    height: 56px;
    horizontal-stretch: 1;

    HorizontalLayout {
        alignment: center;
        spacing: 8px;

        Rectangle {
            width: 16px;
            height: 16px;
            y: (parent.height - self.height) / 2;

            Image {
                source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/arrow-right.svg");
                width: 16px;
                height: 16px;
                visible: !root.confirmed;
                colorize: Theme.text-secondary;
            }

            Image {
                source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/check.svg");
                width: 16px;
                height: 16px;
                visible: root.confirmed;
                colorize: Theme.state-success;
            }
        }

        Text {
            text: root.confirmed ? "Confirmed!" : "Swipe to confirm";
            color: root.confirmed ? Theme.state-success : Theme.text-secondary;
            font-size: 14px;
            font-weight: Theme.weight-medium;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        clicked => {
            root.confirmed = !root.confirmed;
            root.clicked();
        }
    }
}
```

---

## 5.2 Input Controls & Selection Mechanics

### 11. Search Input (`SearchInput.slint`)
```slint
import { Theme } from "../core/Theme.slint";

export component SearchInput inherits Rectangle {
    in-out property <string> text: "";
    in-out property <string> placeholder: "Search...";
    in-out property <bool> enabled: true;

    callback edited(string);

    height: Theme.button-height-md;
    background: Theme.bg-raised;
    border-radius: Theme.radius-md;
    border-width: 1px;
    border-color: input.has-focus ? Theme.accent : Theme.border-base;

    animate border-color { duration: Theme.dur-fast; }

    HorizontalLayout {
        padding-left: Theme.input-padding-x;
        padding-right: Theme.input-padding-x;
        spacing: Theme.sp-2;

        Image {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/magnifying-glass.svg");
            width: Theme.icon-md;
            height: Theme.icon-md;
            y: (parent.height - self.height) / 2;
            colorize: Theme.text-tertiary;
        }

        input := TextInput {
            text <=> root.text;
            enabled: root.enabled;
            color: Theme.text-primary;
            font-size: Theme.text-base;
            vertical-alignment: center;
            horizontal-stretch: 1;

            edited => { root.edited(self.text); }
        }
    }
}
```

### 12. Text Area (`TextArea.slint`)
```slint
import { Theme } from "../core/Theme.slint";

export component TextArea inherits Rectangle {
    in-out property <string> text: "";
    in-out property <string> placeholder: "Enter multi-line text...";
    in-out property <bool> enabled: true;

    callback edited(string);

    min-height: 100px;
    background: Theme.bg-raised;
    border-radius: Theme.radius-md;
    border-width: 1px;
    border-color: input.has-focus ? Theme.accent : Theme.border-base;

    animate border-color { duration: Theme.dur-fast; }

    VerticalLayout {
        padding: Theme.input-padding-x;

        input := TextEdit {
            text <=> root.text;
            enabled: root.enabled;
            color: Theme.text-primary;
            font-family: Theme.font-ui;
            font-size: Theme.text-base;
            wrap: word-wrap;
            vertical-stretch: 1;

            edited => { root.edited(self.text); }
        }
    }
}
```

### 13. Radio Button (`RadioButton.slint`)
```slint
import { Theme } from "../core/Theme.slint";

export component RadioButton inherits Rectangle {
    in-out property <string> text: "";
    in-out property <string> label: text;
    in-out property <bool> checked: false;
    in-out property <bool> selected: checked;
    in property <bool> enabled: true;

    callback clicked();

    height: Theme.button-height-md;
    opacity: enabled ? 1.0 : Theme.opacity-disabled;

    HorizontalLayout {
        spacing: Theme.sp-2;
        alignment: start;

        Rectangle {
            width: 18px;
            height: 18px;
            y: (parent.height - self.height) / 2;
            horizontal-stretch: 0;

            border-radius: Theme.radius-full;
            border-width: 2px;
            border-color: !enabled ? Theme.text-disabled : ((checked || selected) ? Theme.accent : (ta.has-hover ? Theme.border-strong : Theme.border-base));
            background: ((checked || selected) && enabled) ? Theme.accent-subtle : #00000000;

            if (checked || selected): Rectangle {
                width: 8px;
                height: 8px;
                border-radius: Theme.radius-full;
                background: enabled ? Theme.accent : Theme.text-disabled;
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
            }
        }

        if (text != "" || label != ""): Text {
            text: text != "" ? text : label;
            color: enabled ? Theme.text-primary : Theme.text-disabled;
            font-family: Theme.font-ui;
            font-size: Theme.text-base;
            font-weight: Theme.weight-regular;
            vertical-alignment: center;
            horizontal-stretch: 1;
        }
    }

    ta := TouchArea {
        clicked => { root.clicked(); }
    }
}
```

---

## 5.3 Data Display Surfaces & Cards

### 14. Stat Card (`StatCard.slint`)
```slint
import { Theme } from "../core/Theme.slint";

export component StatCard inherits Rectangle {
    in property <string> label: "METRIC";
    in property <string> value: "0";
    in property <string> subtitle: "";
    in property <color> stat-color: Theme.accent;

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-subtle;
    horizontal-stretch: 1;

    VerticalLayout {
        padding: Theme.sp-4;
        spacing: Theme.sp-2;

        Text {
            text: root.label;
            color: Theme.text-secondary;
            font-size: Theme.text-xs;
            font-weight: Theme.weight-bold;
        }

        Text {
            text: root.value;
            color: root.stat-color;
            font-family: Theme.font-display;
            font-size: Theme.text-3xl;
            font-weight: Theme.weight-bold;
        }

        if root.subtitle != "": Text {
            text: root.subtitle;
            color: Theme.text-tertiary;
            font-size: Theme.text-sm;
        }
    }
}
```

### 15. Standard Card (`Card.slint`)
```slint
import { Theme } from "../core/Theme.slint";

export component Card inherits Rectangle {
    in property <string> title: "";
    in property <string> subtitle: "";

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-subtle;

    drop-shadow-offset-x: 0px;
    drop-shadow-offset-y: Theme.elevation-1-y;
    drop-shadow-blur: Theme.elevation-1-blur;
    drop-shadow-color: Theme.elevation-1-color;

    VerticalLayout {
        padding: Theme.sp-5;
        spacing: Theme.sp-3;

        if root.title != "": Text {
            text: root.title;
            color: Theme.text-primary;
            font-family: Theme.font-display;
            font-size: Theme.text-xl;
            font-weight: Theme.weight-bold;
        }

        if root.subtitle != "": Text {
            text: root.subtitle;
            color: Theme.text-secondary;
            font-size: Theme.text-sm;
        }

        @children
    }
}
```

### 16. List Item (`ListItem.slint`)
```slint
import { Theme } from "../core/Theme.slint";

export component ListItem inherits Rectangle {
    in property <string> title: "Item Title";
    in property <string> subtitle: "";
    in property <bool> active: false;

    callback clicked();

    min-height: 52px;
    background: active ? Theme.surface-selected : (ta.pressed ? Theme.surface-pressed : (ta.has-hover ? Theme.surface-hover : transparent));
    border-radius: Theme.radius-md;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        padding-left: Theme.sp-4;
        padding-right: Theme.sp-4;
        spacing: Theme.sp-3;

        // Icon Box (36x36) centered vertically
        Rectangle {
            width: 36px;
            height: 36px;
            y: (parent.height - self.height) / 2;
            background: Theme.bg-raised;
            border-radius: Theme.radius-sm;

            Image {
                source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/layer-group.svg");
                width: Theme.icon-md;
                height: Theme.icon-md;
                colorize: Theme.text-secondary;
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
            }
        }

        // Title and Subtitle Vertical Layout centered vertically
        VerticalLayout {
            y: (parent.height - self.height) / 2;
            alignment: center;
            spacing: 2px;
            horizontal-stretch: 1;

            Text {
                text: root.title;
                color: Theme.text-primary;
                font-size: Theme.text-base;
                font-weight: Theme.weight-medium;
                overflow: elide;
            }

            if root.subtitle != "": Text {
                text: root.subtitle;
                color: Theme.text-tertiary;
                font-size: Theme.text-xs;
                overflow: elide;
            }
        }
    }

    ta := TouchArea {
        clicked => { root.clicked(); }
    }
}
```

---



### 5.1 Buttons & Action Controls Code Manual

#### `BackNavButton` ([slint/buttons/BackNavButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/BackNavButton.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component BackNavButton inherits Rectangle {
    in-out property <string> direction: "back";
    in-out property <bool> enabled: true;

    callback clicked();

    width: 40px;
    height: 40px;
    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? Theme.surface-pressed : (ta.has-hover ? Theme.surface-hover : Theme.bg-raised));
    border-radius: Theme.radius-md;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 0;

    animate background { duration: Theme.dur-fast; }

    Rectangle {
        width: 16px;
        height: 16px;
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;

        Image {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/arrow-left.svg");
            width: 16px;
            height: 16px;
            visible: root.direction == "back";
            colorize: enabled ? Theme.text-primary : Theme.btn-disabled-text;
        }

        Image {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/arrow-up.svg");
            width: 16px;
            height: 16px;
            visible: root.direction != "back";
            colorize: enabled ? Theme.text-primary : Theme.btn-disabled-text;
        }
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

#### `BackToTopButton` ([slint/buttons/BackToTopButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/BackToTopButton.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component BackToTopButton inherits Rectangle {
    width: 48px;
    height: 48px;
    background: Theme.bg-raised;
    border-radius: 24px;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 0;

    FaIcon {
        source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/arrow-up.svg");
        fa-size: 18px;
        fa-color: Theme.text-primary;
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
    }

    TouchArea {
        clicked => { }
    }
}
```

#### `BookmarkButton` ([slint/buttons/BookmarkButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/BookmarkButton.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component BookmarkButton inherits Rectangle {
    in-out property <bool> saved: false;

    width: 32px;
    height: 32px;
    background: transparent;
    horizontal-stretch: 0;

    FaIcon {
        source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/bookmark.svg");
        fa-size: 18px;
        fa-color: root.saved ? Theme.warning : Theme.text-tertiary;
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
    }

    TouchArea {
        clicked => { root.saved = !root.saved; }
    }
}
```

#### `ChipFilterButton` ([slint/buttons/ChipFilterButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/ChipFilterButton.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component ChipFilterButton inherits Rectangle {
    in-out property <string> text: "Filter";
    in-out property <bool> active: false;

    background: root.active ? Theme.accent-subtle : Theme.bg-surface;
    border-radius: 16px;
    border-width: 1px;
    border-color: root.active ? Theme.accent : Theme.border-base;
    height: 32px;
    horizontal-stretch: 0;

    HorizontalLayout {
        alignment: center;
        spacing: 6px;

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/check.svg");
            fa-size: 10px;
            fa-color: root.active ? Theme.accent : transparent;
        }

        Text {
            text: root.text;
            color: root.active ? Theme.accent : Theme.text-secondary;
            font-size: 13px;
            font-weight: Theme.weight-medium;
            vertical-alignment: center;
        }
    }

    TouchArea {
        clicked => { root.active = !root.active; }
    }
}
```

#### `CloseDismissButton` ([slint/buttons/CloseDismissButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/CloseDismissButton.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component CloseDismissButton inherits Rectangle {
    in-out property <bool> enabled: true;

    width: 28px;
    height: 28px;
    background: Theme.bg-raised;
    border-radius: 14px;
    horizontal-stretch: 0;

    FaIcon {
        source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/xmark.svg");
        fa-size: 14px;
        fa-color: Theme.text-tertiary;
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
    }

    TouchArea {
        clicked => { }
    }
}
```

#### `CopyButton` ([slint/buttons/CopyButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/CopyButton.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component CopyButton inherits Rectangle {
    in-out property <bool> copied: false;
    in-out property <bool> enabled: true;

    callback clicked();

    width: 32px;
    height: 32px;
    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? Theme.surface-pressed : (ta.has-hover ? Theme.surface-hover : Theme.bg-raised));
    border-radius: Theme.radius-sm;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 0;

    animate background { duration: Theme.dur-fast; }

    Rectangle {
        width: 14px;
        height: 14px;
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;

        Image {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/copy.svg");
            width: 14px;
            height: 14px;
            visible: !root.copied;
            colorize: enabled ? Theme.text-tertiary : Theme.btn-disabled-text;
        }

        Image {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/check.svg");
            width: 14px;
            height: 14px;
            visible: root.copied;
            colorize: Theme.state-success;
        }
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => {
            root.copied = !root.copied;
            root.clicked();
        }
    }
}
```

#### `CtaHeroButton` ([slint/buttons/CtaHeroButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/CtaHeroButton.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component CtaHeroButton inherits Rectangle {
    in-out property <string> text: "Get Started";
    in-out property <bool> enabled: true;

    callback clicked();

    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? Theme.btn-primary-bg-pressed : (ta.has-hover ? Theme.btn-primary-bg-hover : Theme.accent));
    border-radius: Theme.radius-lg;
    height: 56px;
    horizontal-stretch: 1;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        alignment: center;
        spacing: 12px;

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/rocket.svg");
            fa-size: 20px;
            fa-color: enabled ? #ffffff : Theme.btn-disabled-text;
            y: (parent.height - self.height) / 2;
        }

        Text {
            text: root.text;
            color: enabled ? #ffffff : Theme.btn-disabled-text;
            font-size: 18px;
            font-weight: Theme.weight-bold;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

#### `DangerButton` ([slint/buttons/DangerButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/DangerButton.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component DangerButton inherits Rectangle {
    in-out property <string> text: "Delete";
    in-out property <bool> enabled: true;

    callback clicked();

    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? Theme.error.with-alpha(0.8) : (ta.has-hover ? Theme.error.with-alpha(0.9) : Theme.error));
    border-radius: Theme.radius-md;
    height: 40px;
    horizontal-stretch: 1;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        alignment: center;
        spacing: 8px;

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/trash.svg");
            fa-size: 14px;
            fa-color: enabled ? #ffffff : Theme.btn-disabled-text;
            y: (parent.height - self.height) / 2;
        }

        Text {
            text: root.text;
            color: enabled ? #ffffff : Theme.btn-disabled-text;
            font-size: 14px;
            font-weight: Theme.weight-semibold;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

#### `DownloadExportButton` ([slint/buttons/DownloadExportButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/DownloadExportButton.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component DownloadExportButton inherits Rectangle {
    in-out property <string> text: "Download";
    in-out property <bool> enabled: true;

    callback clicked();

    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? Theme.surface-pressed : (ta.has-hover ? Theme.surface-hover : Theme.bg-raised));
    border-radius: Theme.radius-md;
    border-width: 1px;
    border-color: Theme.border-base;
    height: 40px;
    horizontal-stretch: 1;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        alignment: center;
        spacing: 8px;

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/download.svg");
            fa-size: 14px;
            fa-color: enabled ? Theme.text-primary : Theme.btn-disabled-text;
            y: (parent.height - self.height) / 2;
        }

        Text {
            text: root.text;
            color: enabled ? Theme.text-primary : Theme.btn-disabled-text;
            font-size: 14px;
            font-weight: Theme.weight-semibold;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

#### `DropdownButton` ([slint/buttons/DropdownButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/DropdownButton.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component DropdownButton inherits Rectangle {
    in-out property <string> text: "Options";
    in-out property <bool> enabled: true;

    callback clicked();

    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? Theme.surface-pressed : (ta.has-hover ? Theme.surface-hover : Theme.bg-raised));
    border-radius: Theme.radius-md;
    border-width: 1px;
    border-color: Theme.border-base;
    height: 40px;
    horizontal-stretch: 1;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        padding-left: 16px;
        padding-right: 12px;

        Text {
            text: root.text;
            color: enabled ? Theme.text-primary : Theme.btn-disabled-text;
            font-size: 14px;
            font-weight: Theme.weight-medium;
            vertical-alignment: center;
            horizontal-stretch: 1;
        }

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/chevron-down.svg");
            fa-size: 12px;
            fa-color: enabled ? Theme.text-tertiary : Theme.btn-disabled-text;
            y: (parent.height - self.height) / 2;
        }
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

#### `Fab` ([slint/buttons/Fab.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/Fab.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component Fab inherits Rectangle {
    in-out property <bool> enabled: true;

    callback clicked();

    width: 56px;
    height: 56px;
    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? Theme.btn-primary-bg-pressed : (ta.has-hover ? Theme.btn-primary-bg-hover : Theme.accent));
    border-radius: 16px;
    horizontal-stretch: 0;

    animate background { duration: Theme.dur-fast; }

    Image {
        source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/plus.svg");
        width: 24px;
        height: 24px;
        colorize: enabled ? #ffffff : Theme.btn-disabled-text;
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

#### `FabExtended` ([slint/buttons/FabExtended.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/FabExtended.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component FabExtended inherits Rectangle {
    in-out property <string> text: "New item";
    in-out property <bool> enabled: true;

    callback clicked();

    height: 56px;
    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? Theme.btn-primary-bg-pressed : (ta.has-hover ? Theme.btn-primary-bg-hover : Theme.accent));
    border-radius: 16px;
    horizontal-stretch: 1;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        padding-left: 20px;
        padding-right: 20px;
        spacing: 10px;

        Image {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/plus.svg");
            width: 20px;
            height: 20px;
            y: (parent.height - self.height) / 2;
            colorize: enabled ? #ffffff : Theme.btn-disabled-text;
        }

        Text {
            text: root.text;
            color: enabled ? #ffffff : Theme.btn-disabled-text;
            font-size: 15px;
            font-weight: Theme.weight-semibold;
            vertical-alignment: center;
            horizontal-stretch: 1;
        }
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

#### `FabMini` ([slint/buttons/FabMini.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/FabMini.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component FabMini inherits Rectangle {
    in-out property <string> icon: "plus";
    in-out property <bool> enabled: true;

    callback clicked();

    width: 40px;
    height: 40px;
    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? Theme.btn-primary-bg-pressed : (ta.has-hover ? Theme.btn-primary-bg-hover : Theme.accent));
    border-radius: 12px;
    horizontal-stretch: 0;

    animate background { duration: Theme.dur-fast; }

    Image {
        source: icon == "pen" ? @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/pen.svg")
            : icon == "paperclip" ? @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/paperclip.svg")
            : icon == "camera" ? @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/camera.svg")
            : @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/plus.svg");
        width: 16px;
        height: 16px;
        colorize: enabled ? #ffffff : Theme.btn-disabled-text;
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

#### `GhostButton` ([slint/buttons/GhostButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/GhostButton.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component GhostButton inherits Rectangle {
    in-out property <string> text: "Ghost";
    in-out property <bool> enabled: true;

    callback clicked();

    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? Theme.surface-pressed : (ta.has-hover ? Theme.surface-hover : transparent));
    border-radius: Theme.radius-md;
    height: 40px;
    horizontal-stretch: 1;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        alignment: center;
        spacing: 8px;

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/ghost.svg");
            fa-size: 14px;
            fa-color: enabled ? Theme.text-secondary : Theme.btn-disabled-text;
            y: (parent.height - self.height) / 2;
        }

        Text {
            text: root.text;
            color: enabled ? Theme.text-secondary : Theme.btn-disabled-text;
            font-size: 14px;
            font-weight: Theme.weight-semibold;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

#### `IconButtonCircle` ([slint/buttons/IconButtonCircle.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/IconButtonCircle.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component IconButtonCircle inherits Rectangle {
    in-out property <bool> enabled: true;
    in property <image> icon: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/heart.svg");

    callback clicked();

    width: 40px;
    height: 40px;
    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? Theme.surface-pressed : (ta.has-hover ? Theme.surface-hover : Theme.bg-raised));
    border-radius: 20px;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 0;

    animate background { duration: Theme.dur-fast; }

    Image {
        source: root.icon;
        width: 16px;
        height: 16px;
        colorize: enabled ? Theme.text-primary : Theme.text-disabled;
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

#### `IconButtonSquare` ([slint/buttons/IconButtonSquare.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/IconButtonSquare.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component IconButtonSquare inherits Rectangle {
    in-out property <bool> enabled: true;
    in property <image> icon: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/plus.svg");

    callback clicked();

    width: 40px;
    height: 40px;
    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? Theme.surface-pressed : (ta.has-hover ? Theme.surface-hover : Theme.bg-raised));
    border-radius: Theme.radius-md;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 0;

    animate background { duration: Theme.dur-fast; }

    Image {
        source: root.icon;
        width: 16px;
        height: 16px;
        colorize: enabled ? Theme.text-primary : Theme.text-disabled;
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

#### `LikeHeartButton` ([slint/buttons/LikeHeartButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/LikeHeartButton.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component LikeHeartButton inherits Rectangle {
    in-out property <bool> liked: false;

    width: 32px;
    height: 32px;
    background: transparent;
    horizontal-stretch: 0;

    FaIcon {
        source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/heart.svg");
        fa-size: 18px;
        fa-color: root.liked ? #E63950 : Theme.text-tertiary;
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
    }

    TouchArea {
        clicked => { root.liked = !root.liked; }
    }
}
```

#### `LoadingButton` ([slint/buttons/LoadingButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/LoadingButton.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component LoadingButton inherits Rectangle {
    in-out property <string> text: "Loading...";
    in-out property <bool> loading: true;
    in-out property <bool> enabled: true;

    background: Theme.accent;
    border-radius: Theme.radius-md;
    height: 40px;
    horizontal-stretch: 1;

    HorizontalLayout {
        alignment: center;
        spacing: 8px;

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/spinner.svg");
            fa-size: 14px;
            fa-color: #ffffff;
        }

        Text {
            text: root.loading ? root.text : "Done";
            color: #ffffff;
            font-size: 14px;
            font-weight: Theme.weight-semibold;
            vertical-alignment: center;
        }
    }

    TouchArea {
        clicked => { }
    }
}
```

#### `LongPressButton` ([slint/buttons/LongPressButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/LongPressButton.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component LongPressButton inherits Rectangle {
    in-out property <string> text: "Hold to confirm";
    in-out property <bool> pressed: false;

    background: root.pressed ? Theme.success : Theme.bg-raised;
    border-radius: Theme.radius-md;
    border-width: 1px;
    border-color: root.pressed ? Theme.success : Theme.border-base;
    height: 40px;
    horizontal-stretch: 1;

    HorizontalLayout {
        alignment: center;
        spacing: 8px;

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/hand-pointer.svg");
            fa-size: 14px;
            fa-color: root.pressed ? #ffffff : Theme.text-primary;
        }

        Text {
            text: root.pressed ? "Done" : root.text;
            color: root.pressed ? #ffffff : Theme.text-primary;
            font-size: 14px;
            font-weight: Theme.weight-semibold;
            vertical-alignment: center;
        }
    }

    TouchArea {
        clicked => { root.pressed = !root.pressed; }
    }
}
```

#### `MuteUnmuteButton` ([slint/buttons/MuteUnmuteButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/MuteUnmuteButton.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component MuteUnmuteButton inherits Rectangle {
    in-out property <bool> muted: false;

    width: 40px;
    height: 40px;
    background: Theme.bg-raised;
    border-radius: Theme.radius-md;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 0;

    Rectangle {
        width: 16px;
        height: 16px;
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;

        Image {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/volume-high.svg");
            width: 16px;
            height: 16px;
            visible: !root.muted;
            colorize: Theme.text-primary;
        }

        Image {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/volume-xmark.svg");
            width: 16px;
            height: 16px;
            visible: root.muted;
            colorize: Theme.text-primary;
        }
    }

    TouchArea {
        clicked => { root.muted = !root.muted; }
    }
}
```

#### `OutlinedButton` ([slint/buttons/OutlinedButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/OutlinedButton.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component OutlinedButton inherits Rectangle {
    in-out property <string> text: "Outlined";
    in-out property <bool> enabled: true;

    callback clicked();

    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? Theme.surface-pressed : (ta.has-hover ? Theme.surface-hover : transparent));
    border-radius: Theme.radius-md;
    border-width: 1px;
    border-color: Theme.border-base;
    height: 40px;
    horizontal-stretch: 1;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        alignment: center;
        spacing: 8px;

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/pen-to-square.svg");
            fa-size: 14px;
            fa-color: enabled ? Theme.text-primary : Theme.btn-disabled-text;
            y: (parent.height - self.height) / 2;
        }

        Text {
            text: root.text;
            color: enabled ? Theme.text-primary : Theme.btn-disabled-text;
            font-size: 14px;
            font-weight: Theme.weight-semibold;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

#### `PillButton` ([slint/buttons/PillButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/PillButton.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component PillButton inherits Rectangle {
    in-out property <string> text: "Pill";

    background: Theme.accent;
    border-radius: 20px;
    height: 36px;
    horizontal-stretch: 0;

    Text {
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
        text: root.text;
        color: #ffffff;
        font-size: 13px;
        font-weight: Theme.weight-semibold;
        vertical-alignment: center;
        horizontal-alignment: center;
    }

    TouchArea {
        clicked => { }
    }
}
```

#### `PlayPauseButton` ([slint/buttons/PlayPauseButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/PlayPauseButton.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component PlayPauseButton inherits Rectangle {
    in-out property <bool> playing: false;

    width: 48px;
    height: 48px;
    background: Theme.accent;
    border-radius: 24px;
    horizontal-stretch: 0;

    Rectangle {
        width: 20px;
        height: 20px;
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;

        Image {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/play.svg");
            width: 20px;
            height: 20px;
            visible: !root.playing;
            colorize: #ffffff;
        }

        Image {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/pause.svg");
            width: 20px;
            height: 20px;
            visible: root.playing;
            colorize: #ffffff;
        }
    }

    TouchArea {
        clicked => { root.playing = !root.playing; }
    }
}
```

#### `PrimaryButton` ([slint/buttons/PrimaryButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/PrimaryButton.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component PrimaryButton inherits Rectangle {
    in-out property <string> text: "Primary";
    in-out property <bool> enabled: true;

    callback clicked();

    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? Theme.btn-primary-bg-pressed : (ta.has-hover ? Theme.btn-primary-bg-hover : Theme.accent));
    border-radius: Theme.radius-md;
    height: 40px;
    horizontal-stretch: 1;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        alignment: center;
        spacing: 8px;

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/check.svg");
            fa-size: 14px;
            fa-color: enabled ? #ffffff : Theme.btn-disabled-text;
            y: (parent.height - self.height) / 2;
        }

        Text {
            text: root.text;
            color: enabled ? #ffffff : Theme.btn-disabled-text;
            font-size: 14px;
            font-weight: Theme.weight-semibold;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

#### `ReactionButton` ([slint/buttons/ReactionButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/ReactionButton.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component ReactionButton inherits Rectangle {
    in-out property <int> count: 12;
    in-out property <bool> active: false;

    callback clicked();

    height: 32px;
    background: root.active ? Theme.accent-subtle : Theme.bg-surface;
    border-radius: 16px;
    border-width: 1px;
    border-color: root.active ? Theme.accent : Theme.border-base;
    horizontal-stretch: 0;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        padding-left: 10px;
        padding-right: 10px;
        spacing: 6px;
        alignment: center;

        Image {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/thumbs-up.svg");
            width: 14px;
            height: 14px;
            y: (parent.height - self.height) / 2;
            colorize: root.active ? Theme.accent : Theme.text-secondary;
        }

        Text {
            text: root.count;
            color: root.active ? Theme.accent : Theme.text-secondary;
            font-size: 12px;
            font-weight: Theme.weight-medium;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        clicked => {
            root.active = !root.active;
            root.clicked();
        }
    }
}
```

#### `RecordButton` ([slint/buttons/RecordButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/RecordButton.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component RecordButton inherits Rectangle {
    in-out property <bool> recording: false;

    width: 48px;
    height: 48px;
    background: root.recording ? Theme.error : Theme.bg-raised;
    border-radius: 24px;
    border-width: 2px;
    border-color: root.recording ? Theme.error : Theme.border-base;
    horizontal-stretch: 0;

    FaIcon {
        source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/circle.svg");
        fa-size: 16px;
        fa-color: #ffffff;
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
    }

    TouchArea {
        clicked => { root.recording = !root.recording; }
    }
}
```

#### `ScrollAnchorButton` ([slint/buttons/ScrollAnchorButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/ScrollAnchorButton.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component ScrollAnchorButton inherits Rectangle {
    in-out property <string> text: "Section 1";

    background: transparent;
    border-radius: Theme.radius-sm;
    height: 32px;
    horizontal-stretch: 1;

    HorizontalLayout {
        spacing: 8px;

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/chevron-right.svg");
            fa-size: 10px;
            fa-color: Theme.text-link;
        }

        Text {
            text: root.text;
            color: Theme.text-link;
            font-size: 13px;
            font-weight: Theme.weight-medium;
            vertical-alignment: center;
        }
    }

    TouchArea {
        clicked => { }
    }
}
```

#### `SecondaryButton` ([slint/buttons/SecondaryButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/SecondaryButton.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component SecondaryButton inherits Rectangle {
    in-out property <string> text: "Secondary";
    in-out property <bool> enabled: true;

    callback clicked();

    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? Theme.surface-pressed : (ta.has-hover ? Theme.surface-hover : Theme.bg-raised));
    border-radius: Theme.radius-md;
    border-width: 1px;
    border-color: Theme.border-base;
    height: 40px;
    horizontal-stretch: 1;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        alignment: center;
        spacing: 8px;

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/sliders.svg");
            fa-size: 14px;
            fa-color: enabled ? Theme.text-primary : Theme.btn-disabled-text;
            y: (parent.height - self.height) / 2;
        }

        Text {
            text: root.text;
            color: enabled ? Theme.text-primary : Theme.btn-disabled-text;
            font-size: 14px;
            font-weight: Theme.weight-semibold;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

#### `ShareButton` ([slint/buttons/ShareButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/ShareButton.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component ShareButton inherits Rectangle {
    in-out property <string> text: "Share";
    in-out property <bool> enabled: true;

    callback clicked();

    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? Theme.surface-pressed : (ta.has-hover ? Theme.surface-hover : Theme.bg-raised));
    border-radius: Theme.radius-md;
    border-width: 1px;
    border-color: Theme.border-base;
    height: 40px;
    horizontal-stretch: 1;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        alignment: center;
        spacing: 8px;

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/share-nodes.svg");
            fa-size: 14px;
            fa-color: enabled ? Theme.text-primary : Theme.btn-disabled-text;
            y: (parent.height - self.height) / 2;
        }

        Text {
            text: root.text;
            color: enabled ? Theme.text-primary : Theme.btn-disabled-text;
            font-size: 14px;
            font-weight: Theme.weight-medium;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

#### `SocialAuthApple` ([slint/buttons/SocialAuthApple.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/SocialAuthApple.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component SocialAuthApple inherits Rectangle {
    in-out property <string> text: "Continue with Apple";
    in-out property <bool> enabled: true;

    callback clicked();

    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? #1c1c1e : (ta.has-hover ? #2c2c2e : #000000));
    border-radius: Theme.radius-md;
    height: 44px;
    horizontal-stretch: 1;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        alignment: center;
        spacing: 12px;

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/brands/apple.svg");
            fa-size: 18px;
            fa-color: enabled ? #ffffff : Theme.btn-disabled-text;
            y: (parent.height - self.height) / 2;
        }

        Text {
            text: root.text;
            color: enabled ? #ffffff : Theme.btn-disabled-text;
            font-family: Theme.font-ui;
            font-size: 14px;
            font-weight: Theme.weight-medium;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

#### `SocialAuthFacebook` ([slint/buttons/SocialAuthFacebook.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/SocialAuthFacebook.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component SocialAuthFacebook inherits Rectangle {
    in-out property <string> text: "Continue with Facebook";
    in-out property <bool> enabled: true;

    callback clicked();

    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? #1464cc : (ta.has-hover ? #166fe5 : #1877F2));
    border-radius: Theme.radius-md;
    height: 44px;
    horizontal-stretch: 1;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        alignment: center;
        spacing: 12px;

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/brands/facebook.svg");
            fa-size: 18px;
            fa-color: enabled ? #ffffff : Theme.btn-disabled-text;
            y: (parent.height - self.height) / 2;
        }

        Text {
            text: root.text;
            color: enabled ? #ffffff : Theme.btn-disabled-text;
            font-family: Theme.font-ui;
            font-size: 14px;
            font-weight: Theme.weight-medium;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

#### `SocialAuthGitHub` ([slint/buttons/SocialAuthGitHub.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/SocialAuthGitHub.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component SocialAuthGitHub inherits Rectangle {
    in-out property <string> text: "Continue with GitHub";
    in-out property <bool> enabled: true;

    callback clicked();

    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? #1b1f23 : (ta.has-hover ? #2c3137 : #24292e));
    border-radius: Theme.radius-md;
    height: 44px;
    horizontal-stretch: 1;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        alignment: center;
        spacing: 12px;

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/brands/github.svg");
            fa-size: 18px;
            fa-color: enabled ? #ffffff : Theme.btn-disabled-text;
            y: (parent.height - self.height) / 2;
        }

        Text {
            text: root.text;
            color: enabled ? #ffffff : Theme.btn-disabled-text;
            font-family: Theme.font-ui;
            font-size: 14px;
            font-weight: Theme.weight-medium;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

#### `SocialAuthGoogle` ([slint/buttons/SocialAuthGoogle.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/SocialAuthGoogle.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component SocialAuthGoogle inherits Rectangle {
    in-out property <string> text: "Continue with Google";
    in-out property <bool> enabled: true;

    callback clicked();

    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? #f1f3f4 : (ta.has-hover ? #f8f9fa : #ffffff));
    border-radius: Theme.radius-md;
    border-width: 1px;
    border-color: #dadce0;
    height: 44px;
    horizontal-stretch: 1;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        alignment: center;
        spacing: 12px;

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/brands/google.svg");
            fa-size: 18px;
            fa-color: #4285F4;
            y: (parent.height - self.height) / 2;
        }

        Text {
            text: root.text;
            color: enabled ? #3c4043 : Theme.btn-disabled-text;
            font-family: Theme.font-ui;
            font-size: 14px;
            font-weight: Theme.weight-medium;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

#### `SocialAuthTwitter` ([slint/buttons/SocialAuthTwitter.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/SocialAuthTwitter.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component SocialAuthTwitter inherits Rectangle {
    in-out property <string> text: "Continue with X";
    in-out property <bool> enabled: true;

    callback clicked();

    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? #1c1c1e : (ta.has-hover ? #2c2c2e : #000000));
    border-radius: Theme.radius-md;
    height: 44px;
    horizontal-stretch: 1;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        alignment: center;
        spacing: 12px;

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/brands/twitter.svg");
            fa-size: 18px;
            fa-color: enabled ? #ffffff : Theme.btn-disabled-text;
            y: (parent.height - self.height) / 2;
        }

        Text {
            text: root.text;
            color: enabled ? #ffffff : Theme.btn-disabled-text;
            font-family: Theme.font-ui;
            font-size: 14px;
            font-weight: Theme.weight-medium;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

#### `SpeedDial` ([slint/buttons/SpeedDial.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/SpeedDial.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";
import { FabMini } from "FabMini.slint";

export component SpeedDial inherits Rectangle {
    horizontal-stretch: 0;

    VerticalLayout {
        spacing: 12px;

        FabMini { icon: "pen"; }
        FabMini { icon: "paperclip"; }
        FabMini { icon: "camera"; }
        FabMini { icon: "plus"; }
    }
}
```

#### `SplitButton` ([slint/buttons/SplitButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/SplitButton.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component SplitButton inherits Rectangle {
    in-out property <string> text: "Save";
    in-out property <bool> enabled: true;

    callback main-clicked();
    callback dropdown-clicked();

    height: 40px;
    horizontal-stretch: 1;

    Rectangle {
        x: 0px;
        y: 0px;
        width: parent.width - 36px;
        height: parent.height;
        background: !enabled ? Theme.btn-disabled-bg : (ta1.pressed ? Theme.btn-primary-bg-pressed : (ta1.has-hover ? Theme.btn-primary-bg-hover : Theme.accent));
        border-radius: Theme.radius-md;

        animate background { duration: Theme.dur-fast; }

        HorizontalLayout {
            alignment: center;
            spacing: 8px;

            FaIcon {
                source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/floppy-disk.svg");
                fa-size: 14px;
                fa-color: enabled ? #ffffff : Theme.btn-disabled-text;
                y: (parent.height - self.height) / 2;
            }

            Text {
                text: root.text;
                color: enabled ? #ffffff : Theme.btn-disabled-text;
                font-size: 14px;
                font-weight: Theme.weight-semibold;
                vertical-alignment: center;
            }
        }

        ta1 := TouchArea {
            enabled: root.enabled;
            clicked => { root.main-clicked(); }
        }
    }

    Rectangle {
        x: parent.width - 36px;
        y: 0px;
        width: 36px;
        height: parent.height;
        background: !enabled ? Theme.btn-disabled-bg : (ta2.pressed ? Theme.btn-primary-bg-pressed : (ta2.has-hover ? Theme.btn-primary-bg-hover : Theme.accent));
        border-radius: Theme.radius-md;

        animate background { duration: Theme.dur-fast; }

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/chevron-down.svg");
            fa-size: 12px;
            fa-color: enabled ? #ffffff : Theme.btn-disabled-text;
            x: (parent.width - self.width) / 2;
            y: (parent.height - self.height) / 2;
        }

        ta2 := TouchArea {
            enabled: root.enabled;
            clicked => { root.dropdown-clicked(); }
        }
    }
}
```

#### `SubmitButton` ([slint/buttons/SubmitButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/SubmitButton.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component SubmitButton inherits Rectangle {
    in-out property <string> text: "Submit";
    in-out property <bool> loading: false;
    in-out property <bool> enabled: true;

    callback clicked();

    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? Theme.btn-primary-bg-pressed : (ta.has-hover ? Theme.btn-primary-bg-hover : Theme.accent));
    border-radius: Theme.radius-md;
    height: 40px;
    horizontal-stretch: 1;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        alignment: center;
        spacing: 8px;

        Rectangle {
            width: 14px;
            height: 14px;
            y: (parent.height - self.height) / 2;

            Image {
                source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/paper-plane.svg");
                width: 14px;
                height: 14px;
                visible: !root.loading;
                colorize: enabled ? #ffffff : Theme.btn-disabled-text;
            }

            Image {
                source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/spinner.svg");
                width: 14px;
                height: 14px;
                visible: root.loading;
                colorize: enabled ? #ffffff : Theme.btn-disabled-text;
            }
        }

        Text {
            text: root.text;
            color: enabled ? #ffffff : Theme.btn-disabled-text;
            font-size: 14px;
            font-weight: Theme.weight-semibold;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

#### `SuccessButton` ([slint/buttons/SuccessButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/SuccessButton.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component SuccessButton inherits Rectangle {
    in-out property <string> text: "Confirm";
    in-out property <bool> enabled: true;

    callback clicked();

    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? Theme.success.with-alpha(0.8) : (ta.has-hover ? Theme.success.with-alpha(0.9) : Theme.success));
    border-radius: Theme.radius-md;
    height: 40px;
    horizontal-stretch: 1;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        alignment: center;
        spacing: 8px;

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/circle-check.svg");
            fa-size: 14px;
            fa-color: enabled ? #ffffff : Theme.btn-disabled-text;
            y: (parent.height - self.height) / 2;
        }

        Text {
            text: root.text;
            color: enabled ? #ffffff : Theme.btn-disabled-text;
            font-size: 14px;
            font-weight: Theme.weight-semibold;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```

#### `SwipeToConfirm` ([slint/buttons/SwipeToConfirm.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/SwipeToConfirm.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component SwipeToConfirm inherits Rectangle {
    in-out property <bool> confirmed: false;

    callback clicked();

    background: Theme.bg-raised;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    height: 56px;
    horizontal-stretch: 1;

    HorizontalLayout {
        alignment: center;
        spacing: 8px;

        Rectangle {
            width: 16px;
            height: 16px;
            y: (parent.height - self.height) / 2;

            Image {
                source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/arrow-right.svg");
                width: 16px;
                height: 16px;
                visible: !root.confirmed;
                colorize: Theme.text-secondary;
            }

            Image {
                source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/check.svg");
                width: 16px;
                height: 16px;
                visible: root.confirmed;
                colorize: Theme.state-success;
            }
        }

        Text {
            text: root.confirmed ? "Confirmed!" : "Swipe to confirm";
            color: root.confirmed ? Theme.state-success : Theme.text-secondary;
            font-size: 14px;
            font-weight: Theme.weight-medium;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        clicked => {
            root.confirmed = !root.confirmed;
            root.clicked();
        }
    }
}
```

#### `TextLinkButton` ([slint/buttons/TextLinkButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/TextLinkButton.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component TextLinkButton inherits Rectangle {
    in-out property <string> text: "Learn more";
    in-out property <bool> enabled: true;

    background: transparent;
    height: 32px;
    horizontal-stretch: 1;

    HorizontalLayout {
        alignment: center;
        spacing: 6px;

        Text {
            text: root.text;
            color: Theme.text-link;
            font-size: 14px;
            font-weight: Theme.weight-medium;
            vertical-alignment: center;
        }

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/arrow-up-right-from-square.svg");
            fa-size: 11px;
            fa-color: Theme.text-link;
        }
    }

    TouchArea {
        clicked => { }
    }
}
```

#### `ToggleButton` ([slint/buttons/ToggleButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/ToggleButton.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component ToggleButton inherits Rectangle {
    in-out property <string> text: "Toggle";
    in-out property <bool> active: false;
    in-out property <bool> enabled: true;

    callback clicked();

    background: !enabled ? Theme.btn-disabled-bg : (active ? Theme.accent : (ta.pressed ? Theme.surface-pressed : (ta.has-hover ? Theme.surface-hover : Theme.bg-raised)));
    border-radius: Theme.radius-md;
    border-width: 1px;
    border-color: root.active ? Theme.accent : Theme.border-base;
    height: 40px;
    horizontal-stretch: 1;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        alignment: center;
        spacing: 8px;

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/power-off.svg");
            fa-size: 14px;
            fa-color: !enabled ? Theme.btn-disabled-text : (root.active ? #ffffff : Theme.text-primary);
            y: (parent.height - self.height) / 2;
        }

        Text {
            text: root.text;
            color: !enabled ? Theme.btn-disabled-text : (root.active ? #ffffff : Theme.text-primary);
            font-size: 14px;
            font-weight: Theme.weight-semibold;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => {
            if (root.enabled) {
                root.active = !root.active;
                root.clicked();
            }
        }
    }
}
```

#### `WarningButton` ([slint/buttons/WarningButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/buttons/WarningButton.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component WarningButton inherits Rectangle {
    in-out property <string> text: "Warning";
    in-out property <bool> enabled: true;

    callback clicked();

    background: !enabled ? Theme.btn-disabled-bg : (ta.pressed ? Theme.warning.with-alpha(0.8) : (ta.has-hover ? Theme.warning.with-alpha(0.9) : Theme.warning));
    border-radius: Theme.radius-md;
    height: 40px;
    horizontal-stretch: 1;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        alignment: center;
        spacing: 8px;

        FaIcon {
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/triangle-exclamation.svg");
            fa-size: 14px;
            fa-color: enabled ? #000000 : Theme.btn-disabled-text;
            y: (parent.height - self.height) / 2;
        }

        Text {
            text: root.text;
            color: enabled ? #000000 : Theme.btn-disabled-text;
            font-size: 14px;
            font-weight: Theme.weight-semibold;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        enabled: root.enabled;
        clicked => { root.clicked(); }
    }
}
```


### 5.2 Input Controls & Selection Mechanics Code Manual

#### `FishCheckBox` ([slint/inputs/FishCheckBox.slint](file:///home/lion/Documents/GitHub/ltk/slint/inputs/FishCheckBox.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component FishCheckBox inherits Rectangle {
    in-out property <bool> checked: false;
    in property <string> text: "";
    in property <bool> enabled: true;

    callback toggled(bool);

    width: indicator.width + Theme.sp-3 + label.preferred-width;
    height: max(indicator.height, label.preferred-height);
    opacity: enabled ? 1.0 : Theme.opacity-disabled;

    indicator := Rectangle {
        x: 0;
        y: (parent.height - self.height) / 2;
        width: Theme.checkbox-size-md;
        height: Theme.checkbox-size-md;
        border-radius: Theme.radius-xs;
        border-width: 2px;
        border-color: checked ? Theme.accent : Theme.text-tertiary;
        background: checked ? Theme.accent : transparent;

        animate background { duration: Theme.dur-faster; easing: Theme.ease-spring; }
        animate border-color { duration: Theme.dur-faster; easing: Theme.ease-spring; }

        if checked: Text {
            x: 0;
            y: 0;
            width: 100%;
            height: 100%;
            text: "✓";
            color: Theme.on-accent;
            font-size: 12px;
            font-weight: Theme.weight-bold;
            horizontal-alignment: center;
            vertical-alignment: center;
        }
    }

    label := Text {
        x: indicator.x + indicator.width + Theme.sp-3;
        y: (parent.height - self.height) / 2;
        text: root.text;
        color: Theme.text-primary;
        font-family: Theme.font-ui;
        font-size: Theme.text-base;
        vertical-alignment: center;
        overflow: elide;
    }

    ta := TouchArea {
        clicked => {
            root.checked = !root.checked;
            root.toggled(root.checked);
        }
    }
}
```

#### `FishLabel` ([slint/inputs/FishLabel.slint](file:///home/lion/Documents/GitHub/ltk/slint/inputs/FishLabel.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component FishLabel inherits Text {
    in property <bool> enabled: true;
    color: Theme.text-primary;
    font-family: Theme.font-ui;
    font-size: Theme.text-base;
    opacity: self.enabled ? 1 : Theme.opacity-disabled;
}
```

#### `FishSlider` ([slint/inputs/FishSlider.slint](file:///home/lion/Documents/GitHub/ltk/slint/inputs/FishSlider.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component FishSlider inherits Rectangle {
    in property <float> minimum: 0;
    in property <float> maximum: 100;
    in-out property <float> value: 0;
    in property <bool> enabled: true;
    in property <bool> horizontal: true;
    in property <length> track-height: 5px;

    callback changed(float);

    background: transparent;

    track-bg := Rectangle {
        x: horizontal ? 0 : (parent.width - self.width) / 2;
        y: horizontal ? (parent.height - self.height) / 2 : 0;
        width: horizontal ? parent.width : track-height;
        height: horizontal ? track-height : parent.height;
        border-radius: self.width / 2;
        background: Theme.toggle-track-off;
    }

    filled-track := Rectangle {
        x: track-bg.x;
        y: track-bg.y;
        width: horizontal ? (value - minimum) / (maximum - minimum) * track-bg.width : track-height;
        height: horizontal ? track-height : (value - minimum) / (maximum - minimum) * track-bg.height;
        border-radius: self.width / 2;
        background: Theme.accent;
    }

    handle := Rectangle {
        x: horizontal
            ? track-bg.x + (value - minimum) / (maximum - minimum) * (track-bg.width - self.width)
            : (parent.width - self.width) / 2;
        y: horizontal
            ? (parent.height - self.height) / 2
            : track-bg.y + (value - minimum) / (maximum - minimum) * (track-bg.height - self.height);
        width: 22px;
        height: 22px;
        border-radius: self.height / 2;
        background: Theme.dark-mode ? #E8EAED : #FFFFFF;
        border-width: 1px;
        border-color: Theme.dark-mode ? rgba(255, 255, 255, 0.15) : rgba(0, 0, 0, 0.12);

        drop-shadow-offset-x: 0px;
        drop-shadow-offset-y: 0px;
        drop-shadow-blur: 8px;
        drop-shadow-color: ta.pressed ? rgba(0, 0, 0, 0.25) : rgba(0, 0, 0, 0.15);
    }

    ta := TouchArea {
        moved => {
            if (horizontal) {
                root.value = clamp(
                    root.minimum + (self.pressed-x / track-bg.width) * (root.maximum - root.minimum),
                    root.minimum,
                    root.maximum
                );
            } else {
                root.value = clamp(
                    root.minimum + (self.pressed-y / track-bg.height) * (root.maximum - root.minimum),
                    root.minimum,
                    root.maximum
                );
            }
            root.changed(root.value);
        }
    }
}
```

#### `FishSwitch` ([slint/inputs/FishSwitch.slint](file:///home/lion/Documents/GitHub/ltk/slint/inputs/FishSwitch.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component FishSwitch inherits Rectangle {
    in-out property <bool> checked: false;
    in property <string> text: "";
    in property <bool> enabled: true;

    callback toggled(bool);

    width: Theme.toggle-width + (text != "" ? Theme.sp-3 + label.preferred-width : 0px);
    height: Theme.toggle-height + Theme.sp-2;
    opacity: enabled ? 1.0 : Theme.opacity-disabled;

    track := Rectangle {
        x: 0;
        y: (parent.height - self.height) / 2;
        width: Theme.toggle-width;
        height: Theme.toggle-height;
        border-radius: self.height / 2;
        background: enabled
            ? (checked ? Theme.accent : Theme.toggle-track-off)
            : Theme.toggle-track-disabled;

        animate background { duration: Theme.dur-medium; easing: Theme.ease-standard; }
    }

    handle := Rectangle {
        x: max(Theme.sp-0-5, min(
            track.width - self.width,
            (checked ? track.width - self.width - Theme.sp-0-5 : Theme.sp-0-5)
        ));
        y: (parent.height - self.height) / 2;
        width: Theme.toggle-thumb;
        height: Theme.toggle-thumb;
        border-radius: self.height / 2;
        background: enabled ? (Theme.dark-mode ? #E8EAED : #FFFFFF) : Theme.text-tertiary;
        opacity: ta.pressed ? Theme.opacity-pressed : 1.0;

        animate opacity { duration: Theme.dur-faster; easing: Theme.ease-standard; }
        animate x { duration: Theme.dur-medium; easing: Theme.ease-soft; }

        drop-shadow-offset-x: 0px;
        drop-shadow-offset-y: 1px;
        drop-shadow-blur: 3px;
        drop-shadow-color: Theme.border-default;
    }

    label := Text {
        x: Theme.toggle-width + Theme.sp-3;
        y: (parent.height - self.height) / 2;
        text: root.text;
        color: Theme.text-primary;
        font-family: Theme.font-ui;
        font-size: Theme.text-base;
        vertical-alignment: center;
        overflow: elide;
        visible: text != "";
    }

    ta := TouchArea {
        clicked => {
            root.checked = !root.checked;
            root.toggled(root.checked);
        }
    }
}
```

#### `FishTextField` ([slint/inputs/FishTextField.slint](file:///home/lion/Documents/GitHub/ltk/slint/inputs/FishTextField.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component FishTextField inherits Rectangle {
    in-out property <string> text: "";
    in property <string> placeholder: "";
    in property <bool> enabled: true;
    in-out property <bool> has-focus: input.has-focus;

    callback edited(string);
    callback accepted(string);

    width: max(220px, placeholder-label.preferred-width + Theme.input-height-md);
    height: Theme.input-height-md;
    border-radius: Theme.radius-input;
    background: Theme.bg-raised;
    border-width: 1px;
    border-color: has-focus ? Theme.border-focus : Theme.border-default;

    animate border-color { duration: Theme.dur-fastest; easing: Theme.ease-standard; }

    placeholder-label := Text {
        x: Theme.input-padding-x;
        y: (parent.height - self.height) / 2;
        text: root.placeholder;
        color: Theme.text-tertiary;
        font-family: Theme.font-ui;
        font-size: Theme.text-base;
        visible: root.text == "" && !input.has-focus;
        overflow: elide;
    }

    input := TextInput {
        x: Theme.input-padding-x;
        y: (parent.height - self.height) / 2;
        width: parent.width - Theme.input-padding-x * 2;
        text: root.text;
        color: enabled ? Theme.text-primary : Theme.text-disabled;
        font-family: Theme.font-ui;
        font-size: Theme.text-base;
        vertical-alignment: center;
        selection-background-color: Theme.selection-bg;
        selection-foreground-color: Theme.selection-text;
        enabled: root.enabled;

        edited => { root.edited(self.text); }
        accepted => { root.accepted(self.text); }
    }
}
```

#### `NumberInput` ([slint/inputs/NumberInput.slint](file:///home/lion/Documents/GitHub/ltk/slint/inputs/NumberInput.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component NumberInput inherits Rectangle {
    in property <string> placeholder: "0";
    in-out property <float> value: 0;
    in property <float> min-value: 0;
    in property <float> max-value: 100;
    in property <float> step: 1;
    in property <bool> enabled: true;

    callback changed(float);

    height: Theme.input-height-md;
    opacity: enabled ? 1.0 : Theme.opacity-disabled;

    bg := Rectangle {
        border-radius: Theme.radius-input;
        border-width: 1px;
        border-color: ta.has-hover && enabled ? Theme.border-strong : Theme.border-base;
        background: enabled ? Theme.bg-surface : Theme.btn-disabled-bg;
        animate border-color { duration: Theme.dur-fast; }
    }

    value-text := Text {
        x: Theme.input-padding-x;
        y: (parent.height - self.height) / 2;
        text: root.value;
        color: Theme.text-primary;
        font-family: Theme.font-ui;
        font-size: Theme.text-sm;
        vertical-alignment: center;
        horizontal-stretch: 1;
    }

    // Decrement
    Rectangle {
        x: parent.width - 60px;
        width: 30px;
        height: parent.height;
        background: dec-ta.pressed ? Theme.surface-pressed
            : dec-ta.has-hover ? Theme.surface-hover : transparent;
        border-radius: 0px;

        Text {
            x: (parent.width - self.width) / 2;
            y: (parent.height - self.height) / 2;
            text: "−";
            color: Theme.text-secondary;
            font-size: Theme.text-lg;
            vertical-alignment: center;
        }

        dec-ta := TouchArea {
            enabled: root.enabled;
            clicked => {
                root.value = Math.max(root.min-value, root.value - root.step);
                root.changed(root.value);
            }
        }
    }

    // Separator
    Rectangle {
        x: parent.width - 30px;
        width: 1px;
        height: parent.height * 0.5;
        y: parent.height * 0.25;
        background: Theme.border-subtle;
    }

    // Increment
    Rectangle {
        x: parent.width - 30px;
        width: 30px;
        height: parent.height;
        background: inc-ta.pressed ? Theme.surface-pressed
            : inc-ta.has-hover ? Theme.surface-hover : transparent;
        border-radius: 0px;

        Text {
            x: (parent.width - self.width) / 2;
            y: (parent.height - self.height) / 2;
            text: "+";
            color: Theme.text-secondary;
            font-size: Theme.text-lg;
            vertical-alignment: center;
        }

        inc-ta := TouchArea {
            enabled: root.enabled;
            clicked => {
                root.value = Math.min(root.max-value, root.value + root.step);
                root.changed(root.value);
            }
        }
    }

    ta := TouchArea { }
}
```

#### `PinInput` ([slint/inputs/PinInput.slint](file:///home/lion/Documents/GitHub/ltk/slint/inputs/PinInput.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component PinInput inherits Rectangle {
    in property <int> pin-length: 6;
    in property <bool> enabled: true;

    callback completed(string);

    height: Theme.input-height-lg;
    opacity: enabled ? 1.0 : Theme.opacity-disabled;

    HorizontalLayout {
        spacing: Theme.sp-2;

        for digit[i] in root.pin-length: Rectangle {
            horizontal-stretch: 1;
            border-radius: Theme.radius-input;
            border-width: 1px;
            border-color: i == 0 ? Theme.border-focus : Theme.border-base;
            background: enabled ? Theme.bg-surface : Theme.btn-disabled-bg;

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: "•";
                color: Theme.text-primary;
                font-size: Theme.text-xl;
                vertical-alignment: center;
            }
        }
    }

    ta := TouchArea { }
}
```

#### `RadioButton` ([slint/inputs/RadioButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/inputs/RadioButton.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component RadioButton inherits Rectangle {
    in-out property <string> text: "";
    in-out property <string> label: text;
    in-out property <bool> checked: false;
    in-out property <bool> selected: checked;
    in property <bool> enabled: true;

    callback clicked();

    height: Theme.button-height-md;
    opacity: enabled ? 1.0 : Theme.opacity-disabled;

    HorizontalLayout {
        spacing: Theme.sp-2;
        alignment: start;

        // Radio circle container for vertical centering
        Rectangle {
            width: 18px;
            height: 18px;
            y: (parent.height - self.height) / 2;
            horizontal-stretch: 0;

            border-radius: Theme.radius-full;
            border-width: 2px;
            border-color: !enabled ? Theme.text-disabled : ((checked || selected) ? Theme.accent : (ta.has-hover ? Theme.border-strong : Theme.border-base));
            background: ((checked || selected) && enabled) ? Theme.accent-subtle : #00000000;

            if (checked || selected): Rectangle {
                width: 8px;
                height: 8px;
                border-radius: Theme.radius-full;
                background: enabled ? Theme.accent : Theme.text-disabled;
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
            }
        }

        // Label
        if (text != "" || label != ""): Text {
            text: text != "" ? text : label;
            color: enabled ? Theme.text-primary : Theme.text-disabled;
            font-family: Theme.font-ui;
            font-size: Theme.text-base;
            font-weight: Theme.weight-regular;
            vertical-alignment: center;
            horizontal-stretch: 1;
        }
    }

    ta := TouchArea {
        clicked => { root.clicked(); }
    }
}
```

#### `SearchInput` ([slint/inputs/SearchInput.slint](file:///home/lion/Documents/GitHub/ltk/slint/inputs/SearchInput.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component SearchInput inherits Rectangle {
    in property <string> placeholder: "Search...";
    in property <bool> enabled: true;

    in-out property <string> text <=> input.text;

    callback accepted(string);
    callback cleared();

    min-width: 200px;
    height: Theme.input-height-md;
    background: transparent;

    bg := Rectangle {
        border-radius: Theme.radius-button;
        border-width: 1px;
        border-color: !enabled ? Theme.border-subtle : (input.has-focus ? Theme.border-focus : Theme.border-base);
        background: !enabled ? Theme.btn-disabled-bg : Theme.bg-raised;
        animate border-color { duration: Theme.dur-fast; easing: Theme.ease-standard; }
    }

    // Search icon
    search-icon := Text {
        x: Theme.sp-3;
        y: (parent.height - self.height) / 2;
        text: "⌕";
        color: Theme.text-tertiary;
        font-size: Theme.text-lg;
        vertical-alignment: center;
        horizontal-alignment: center;
    }

    input := TextInput {
        x: Theme.sp-3 + Theme.icon-md + Theme.sp-2;
        y: (parent.height - self.height) / 2;
        width: parent.width - self.x - (clear-btn.visible ? Theme.button-height-md : Theme.sp-3);
        height: self.preferred-height;
        color: enabled ? Theme.text-primary : Theme.text-disabled;
        font-family: Theme.font-ui;
        font-size: Theme.text-base;
        vertical-alignment: center;
        enabled: root.enabled;
        accepted => { root.accepted(self.text); }
    }

    placeholder-text := Text {
        x: input.x;
        y: (parent.height - self.height) / 2;
        text: root.placeholder;
        color: Theme.text-tertiary;
        font-family: Theme.font-ui;
        font-size: Theme.text-base;
        visible: input.text == "" && !input.has-focus;
        vertical-alignment: center;
    }

    // Clear button
    clear-btn := Rectangle {
        x: parent.width - Theme.button-height-md;
        width: Theme.button-height-md;
        height: Theme.button-height-md;
        visible: input.text != "";
        background: transparent;

        Text {
            x: (parent.width - self.width) / 2;
            y: (parent.height - self.height) / 2;
            text: "✕";
            color: Theme.text-tertiary;
            font-size: Theme.text-sm;
            vertical-alignment: center;
        }

        TouchArea {
            clicked => {
                input.text = "";
                root.cleared();
            }
        }
    }
}
```

#### `SegmentedControl` ([slint/inputs/SegmentedControl.slint](file:///home/lion/Documents/GitHub/ltk/slint/inputs/SegmentedControl.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component SegmentedControl inherits Rectangle {
    in property <[string]> segments: [];
    in-out property <int> current-index: 0;

    callback segment-changed(int);

    height: 32px;

    bg := Rectangle {
        border-radius: Theme.radius-button;
        background: Theme.bg-overlay;
        border-width: 1px;
        border-color: Theme.border-subtle;
    }

    // Indicator
    indicator := Rectangle {
        x: ta-touch.x;
        width: ta-touch.width;
        height: parent.height - 4px;
        y: 2px;
        border-radius: Theme.radius-sm;
        background: Theme.bg-surface;
        drop-shadow-offset-y: 1px;
        drop-shadow-blur: 4px;
        drop-shadow-color: Theme.border-default;
    }

    ta-touch := TouchArea { }

    HorizontalLayout {
        padding: 2px;
        spacing: 2px;

        for seg[i] in root.segments: TouchArea {
            horizontal-stretch: 1;
            clicked => {
                root.current-index = i;
                root.segment-changed(i);
            }

            bg2 := Rectangle {
                border-radius: Theme.radius-sm;
                background: transparent;
            }

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: seg;
                color: root.current-index == i ? Theme.text-primary : Theme.text-tertiary;
                font-family: Theme.font-ui;
                font-size: Theme.text-sm;
                font-weight: root.current-index == i ? Theme.weight-semibold : Theme.weight-regular;
                horizontal-alignment: center;
                vertical-alignment: center;
            }
        }
    }
}
```

#### `TextArea` ([slint/inputs/TextArea.slint](file:///home/lion/Documents/GitHub/ltk/slint/inputs/TextArea.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component TextArea inherits Rectangle {
    in property <string> placeholder: "";
    in property <bool> enabled: true;
    in property <int> max-height-lines: 8;

    in-out property <string> text <=> input.text;

    callback accepted(string);
    callback edited(string);

    height: Theme.input-height-md * 3;
    background: transparent;

    bg := Rectangle {
        border-radius: Theme.radius-input;
        border-width: 1px;
        border-color: !enabled ? Theme.border-subtle : (input.has-focus ? Theme.border-focus : Theme.border-base);
        background: !enabled ? Theme.btn-disabled-bg : Theme.bg-raised;
        animate border-color { duration: Theme.dur-fast; easing: Theme.ease-standard; }
    }

    placeholder-text := Text {
        x: Theme.input-padding-x;
        y: Theme.sp-3;
        text: root.placeholder;
        color: Theme.text-tertiary;
        font-family: Theme.font-ui;
        font-size: Theme.text-base;
        visible: input.text == "";
    }

    input := TextInput {
        x: Theme.input-padding-x;
        y: Theme.sp-3;
        width: parent.width - Theme.input-padding-x * 2;
        height: parent.height - Theme.sp-3 * 2;
        color: enabled ? Theme.text-primary : Theme.text-disabled;
        font-family: Theme.font-ui;
        font-size: Theme.text-base;
        wrap: word-wrap;
        single-line: false;
        enabled: root.enabled;
        accepted => { root.accepted(self.text); }
        edited => { root.edited(self.text); }
    }
}
```


### 5.3 Data Display Surfaces & Cards Code Manual

#### `ActivityFeed` ([slint/data-display/ActivityFeed.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/ActivityFeed.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component ActivityFeed inherits Rectangle {
    in property <int> item-count: 4;

    background: Theme.bg-surface;
    border-radius: Theme.radius-card;
    border-width: 1px;
    border-color: Theme.border-base;
    clip: true;

    in property <[string]> demo-names: ["Alice", "Bob", "Carol", "Dave"];
    in property <[string]> demo-actions: ["uploaded file.pdf", "commented on PR #42", "merged branch main", "deployed to staging"];
    in property <[string]> demo-times: ["2h ago", "1h ago", "45m ago", "10m ago"];
    in property <[color]> demo-colors: [Theme.accent, Theme.green-500, #CD7F32, Theme.state-warning];

    VerticalLayout {
        for item[idx] in root.item-count: VerticalLayout {
            Rectangle {
                height: 56px;

                HorizontalLayout {
                    padding-left: Theme.sp-3;
                    padding-right: Theme.sp-3;
                    spacing: Theme.sp-3;

                    Rectangle {
                        width: 32px;
                        height: 32px;
                        horizontal-stretch: 0;

                        Rectangle {
                            width: 32px;
                            height: 32px;
                            border-radius: Theme.radius-full;
                            background: idx < root.demo_colors.length ? root.demo_colors[idx] : Theme.accent;

                            Text {
                                x: (parent.width - self.width) / 2;
                                y: (parent.height - self.height) / 2;
                                text: idx < root.demo_names.length ? root.demo_names[idx] : "";
                                color: #ffffff;
                                font-family: Theme.font-ui;
                                font-size: Theme.text-xs;
                                font-weight: Theme.weight-bold;
                                vertical-alignment: center;
                            }
                        }
                    }

                    VerticalLayout {
                        horizontal-stretch: 1;
                        spacing: Theme.sp-0-5;

                        Text {
                            text: idx < root.demo_names.length ? "\{root.demo_names[idx]} \{root.demo_actions[idx]}" : "";
                            color: Theme.text-primary;
                            font-family: Theme.font-ui;
                            font-size: Theme.text-sm;
                            wrap: word-wrap;
                        }
                    }

                    Text {
                        text: idx < root.demo_times.length ? root.demo_times[idx] : "";
                        color: Theme.text-tertiary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-xs;
                        vertical-alignment: top;
                        horizontal-stretch: 0;
                    }
                }
            }

            if idx < root.item_count - 1: Rectangle {
                height: 1px;
                x: 56px;
                width: 320px;
                background: Theme.border-subtle;
            }
        }
    }
}
```

#### `Avatar` ([slint/data-display/Avatar.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/Avatar.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component Avatar inherits Rectangle {
    in property <image> source: @image-url("");
    in property <string> fallback: "";
    in property <length> avatar-size: 36px;
    in property <color> status-color: #00000000;

    width: avatar-size;
    height: avatar-size;
    background: transparent;
    clip: true;

    bg-circle := Rectangle {
        border-radius: Theme.radius-full;
        background: Theme.bg-overlay;
        border-width: 1px;
        border-color: Theme.border-subtle;

        if source == @image-url("") && fallback != "": Text {
            x: (parent.width - self.width) / 2;
            y: (parent.height - self.height) / 2;
            text: fallback;
            color: Theme.text-secondary;
            font-family: Theme.font-ui;
            font-size: avatar-size * 0.38;
            font-weight: Theme.weight-semibold;
            horizontal-alignment: center;
            vertical-alignment: center;
        }

        if source != @image-url(""): Image {
            source: root.source;
            width: parent.width;
            height: parent.height;
            image-fit: cover;
        }
    }

    // Status indicator
    if status-color != #00000000: Rectangle {
        x: parent.width - 10px;
        y: parent.height - 10px;
        width: 10px;
        height: 10px;
        border-radius: Theme.radius-full;
        background: status-color;
        border-width: 2px;
        border-color: Theme.bg-surface;
    }
}
```

#### `AvatarGroup` ([slint/data-display/AvatarGroup.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/AvatarGroup.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { Avatar } from "Avatar.slint";

export component AvatarGroup inherits Rectangle {
    in property <length> avatar-size: 36px;
    in property <int> max-visible: 4;

    in property <image> img-1: @image-url("");
    in property <string> fb-1: "";
    in property <image> img-2: @image-url("");
    in property <string> fb-2: "";
    in property <image> img-3: @image-url("");
    in property <string> fb-3: "";
    in property <image> img-4: @image-url("");
    in property <string> fb-4: "";
    in property <image> img-5: @image-url("");
    in property <string> fb-5: "";
    in property <int> overflow-count: 0;

    width: avatar-size * 3 + Theme.sp-1 * 2;
    height: avatar-size;
    background: transparent;

    if max-visible >= 1: Avatar {
        x: 0;
        y: (parent.height - self.height) / 2;
        source: root.img-1;
        fallback: root.fb-1;
        avatar-size: root.avatar-size;
    }

    if max-visible >= 2: Avatar {
        x: avatar-size - 8px;
        y: (parent.height - self.height) / 2;
        source: root.img-2;
        fallback: root.fb-2;
        avatar-size: root.avatar-size;
    }

    if max-visible >= 3: Avatar {
        x: (avatar-size - 8px) * 2;
        y: (parent.height - self.height) / 2;
        source: root.img-3;
        fallback: root.fb-3;
        avatar-size: root.avatar-size;
    }

    if overflow-count > 0: Rectangle {
        x: (avatar-size - 8px) * max-visible;
        y: (parent.height - self.height) / 2;
        width: avatar-size;
        height: avatar-size;
        border-radius: Theme.radius-full;
        background: Theme.bg-overlay;
        border-width: 1px;
        border-color: Theme.border-subtle;

        Text {
            x: (parent.width - self.width) / 2;
            y: (parent.height - self.height) / 2;
            text: "+" + overflow-count;
            color: Theme.text-secondary;
            font-family: Theme.font-ui;
            font-size: avatar-size * 0.35;
            font-weight: Theme.weight-semibold;
            horizontal-alignment: center;
            vertical-alignment: center;
        }
    }
}
```

#### `AvatarList` ([slint/data-display/AvatarList.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/AvatarList.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component AvatarList inherits Rectangle {
    in property <[string]> titles: [];
    in property <[string]> subtitles: [];
    in property <[string]> avatar-labels: [];

    callback clicked(int);

    background: Theme.bg-surface;
    border-radius: Theme.radius-sm;
    clip: true;

    VerticalLayout {
        for title[idx] in root.titles: Rectangle {
            height: 52px;
            background: row-ta.pressed ? Theme.surface-pressed
                : row-ta.has-hover ? Theme.surface-hover
                : #00000000;

            animate background { duration: Theme.dur-fast; }

            HorizontalLayout {
                padding-left: Theme.sp-3;
                padding-right: Theme.sp-3;
                spacing: Theme.sp-3;
                alignment: stretch;

                Rectangle {
                    width: 36px;
                    height: 36px;
                    horizontal-stretch: 0;
                    border-radius: Theme.radius-full;
                    background: Math.mod(idx, 3) == 0 ? Theme.accent
                        : (Math.mod(idx, 3) == 1 ? Theme.green-500 : Theme.amber-500);

                    Text {
                        x: (parent.width - self.width) / 2;
                        y: (parent.height - self.height) / 2;
                        text: idx < root.avatar-labels.length ? root.avatar-labels[idx] : "";
                        color: #ffffff;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-sm;
                        font-weight: Theme.weight-semibold;
                        vertical-alignment: center;
                    }
                }

                VerticalLayout {
                    horizontal-stretch: 1;
                    spacing: Theme.sp-0-5;
                    alignment: center;

                    Text {
                        text: root.titles[idx];
                        color: Theme.text-primary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-sm;
                        font-weight: Theme.weight-medium;
                        vertical-alignment: center;
                    }

                    Text {
                        text: root.subtitles[idx];
                        color: Theme.text-secondary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-xs;
                        vertical-alignment: center;
                    }
                }
            }

            row-ta := TouchArea {
                clicked => { root.clicked(idx); }
            }
        }
    }
}
```

#### `Badge` ([slint/data-display/Badge.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/Badge.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component Badge inherits Rectangle {
    in property <string> text: "";
    in property <int> count: -1;
    in property <bool> dot: false;
    in property <color> badge-color: Theme.accent;
    in property <bool> error: false;

    visible: dot || text != "" || count >= 0;
    background: transparent;
    horizontal-stretch: 0;
    vertical-stretch: 0;

    // Dot badge
    if dot: Rectangle {
        width: 8px;
        height: 8px;
        border-radius: Theme.radius-full;
        background: error ? Theme.state-error : badge-color;
    }

    // Text/count badge
    if !dot && (text != "" || count >= 0): Rectangle {
        min-width: max(label.preferred-width + Theme.sp-2 * 2, 20px);
        height: 18px;
        border-radius: Theme.radius-full;
        background: error ? Theme.state-error : badge-color;
        horizontal-stretch: 0;

        label := Text {
            x: (parent.width - self.width) / 2;
            y: (parent.height - self.height) / 2;
            text: root.count >= 0 ? (root.count > 99 ? "99+" : root.count) : root.text;
            color: Theme.text-on-accent;
            font-family: Theme.font-ui;
            font-size: Theme.text-xs;
            font-weight: Theme.weight-semibold;
            horizontal-alignment: center;
            vertical-alignment: center;
        }
    }
}
```

#### `BulkActionBar` ([slint/data-display/BulkActionBar.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/BulkActionBar.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component BulkActionBar inherits Rectangle {
    in property <int> selected-count: 0;
    in property <bool> show-bar: false;

    callback deselect-all();

    height: show-bar ? 48px : 0px;
    background: Theme.accent;
    visible: show-bar;

    animate height { duration: Theme.dur-normal; }

    HorizontalLayout {
        padding-left: Theme.sp-4;
        padding-right: Theme.sp-4;
        spacing: Theme.sp-3;

        Text {
            text: "\{root.selected_count} selected";
            color: #ffffff;
            font-family: Theme.font-ui;
            font-size: Theme.text-sm;
            font-weight: Theme.weight-semibold;
            vertical-alignment: center;
        }

        Rectangle { horizontal-stretch: 1; }

        Rectangle {
            width: 80px;
            height: 28px;
            border-radius: Theme.radius-sm;
            border-width: 1px;
            border-color: #ffffff80;

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: "Deselect";
                color: #ffffff;
                font-family: Theme.font-ui;
                font-size: Theme.text-xs;
                vertical-alignment: center;
            }

            deselect-ta := TouchArea {
                clicked => { root.deselect-all(); }
            }
        }
    }
}
```

#### `Card` ([slint/data-display/Card.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/Card.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component Card inherits Rectangle {
    in property <bool> elevated: false;
    in property <bool> outlined: false;
    in property <bool> padded: true;
    in property <bool> pressable: false;

    callback clicked();

    min-width: 120px;
    min-height: 60px;
    background: Theme.bg-surface;
    border-radius: Theme.radius-card;
    border-width: 1px;
    border-color: outlined ? Theme.border-base : transparent;

    drop-shadow-offset-x: 0px;
    drop-shadow-offset-y: elevated ? Theme.elevation-2-y : 0px;
    drop-shadow-blur: elevated ? Theme.elevation-2-blur : 0px;
    drop-shadow-color: elevated ? Theme.elevation-2-color : transparent;

    if pressable: TouchArea {
        clicked => { root.clicked(); }
    }
}
```

#### `ChatMessageThread` ([slint/data-display/ChatMessageThread.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/ChatMessageThread.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component ChatMessageThread inherits Rectangle {
    in property <[bool]> is-own: [false, true, false, true];

    background: Theme.bg-surface;
    border-radius: Theme.radius-card;
    border-width: 1px;
    border-color: Theme.border-base;
    clip: true;

    in property <[string]> demo-senders: ["Alice", "You", "Bob", "You"];
    in property <[string]> demo-messages: ["Hey, have you seen the new designs?", "Yes, they look great!", "Can we schedule a review?", "Sure, let's do it tomorrow."];
    in property <[string]> demo-times: ["10:30 AM", "10:32 AM", "10:35 AM", "10:36 AM"];
    in property <[color]> demo-avatar-colors: [Theme.accent, Theme.green-500, #CD7F32, Theme.green-500];

    VerticalLayout {
        padding: Theme.sp-3;
        spacing: Theme.sp-3;

        for msg[idx] in root.is-own: Rectangle {
            height: 64px;

            HorizontalLayout {
                spacing: Theme.sp-2;
                alignment: root.is-own[idx] ? end : start;

                if !root.is-own[idx]: Rectangle {
                    width: 28px;
                    height: 28px;
                    horizontal-stretch: 0;

                    Rectangle {
                        width: 28px;
                        height: 28px;
                        border-radius: Theme.radius-full;
                        background: idx < root.demo_avatar_colors.length ? root.demo_avatar_colors[idx] : Theme.accent;

                        Text {
                            x: (parent.width - self.width) / 2;
                            y: (parent.height - self.height) / 2;
                            text: idx < root.demo_senders.length ? root.demo_senders[idx] : "";
                            color: #ffffff;
                            font-family: Theme.font-ui;
                            font-size: 9px;
                            font-weight: Theme.weight-bold;
                            vertical-alignment: center;
                        }
                    }
                }

                Rectangle {
                    width: 200px;
                    horizontal-stretch: 0;

                    if !root.is-own[idx]: Text {
                        text: idx < root.demo_senders.length ? root.demo_senders[idx] : "";
                        color: Theme.text-secondary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-xs;
                        font-weight: Theme.weight-semibold;
                    }

                    Rectangle {
                        y: root.is-own[idx] ? 0px : 16px;
                        height: 40px;
                        border-radius: Theme.radius-md;
                        background: root.is-own[idx] ? Theme.accent : Theme.bg-overlay;

                        Text {
                            x: Theme.sp-3;
                            y: (parent.height - self.height) / 2;
                            text: idx < root.demo_messages.length ? root.demo_messages[idx] : "";
                            color: root.is-own[idx] ? #ffffff : Theme.text-primary;
                            font-family: Theme.font-ui;
                            font-size: Theme.text-sm;
                            wrap: word-wrap;
                            vertical-alignment: center;
                        }
                    }

                    if root.is-own[idx]: Text {
                        y: 42px;
                        text: idx < root.demo_times.length ? root.demo_times[idx] : "";
                        color: Theme.text-tertiary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-xs;
                        horizontal-alignment: right;
                    }

                    if !root.is-own[idx]: Text {
                        y: 42px;
                        text: idx < root.demo_times.length ? root.demo_times[idx] : "";
                        color: Theme.text-tertiary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-xs;
                    }
                }
            }
        }
    }
}
```

#### `Checklist` ([slint/data-display/Checklist.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/Checklist.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component Checklist inherits Rectangle {
    in property <[string]> items: [];
    in property <[bool]> checked: [];

    callback check-toggle(int);

    background: Theme.bg-surface;
    border-radius: Theme.radius-sm;
    clip: true;

    VerticalLayout {
        for item_text[idx] in root.items: Rectangle {
            height: 44px;
            background: row-ta.pressed ? Theme.surface-pressed
                : row-ta.has-hover ? Theme.surface-hover
                : #00000000;

            animate background { duration: Theme.dur-fast; }

            HorizontalLayout {
                padding-left: Theme.sp-3;
                padding-right: Theme.sp-4;
                spacing: Theme.sp-3;
                alignment: stretch;

                Rectangle {
                    width: 18px;
                    height: 18px;
                    horizontal-stretch: 0;
                    vertical-stretch: 0;

                    y: (parent.height - self.height) / 2;

                    Rectangle {
                        width: 18px;
                        height: 18px;
                        border-radius: Theme.radius-xs;
                        background: idx < root.checked.length && root.checked[idx]
                            ? Theme.accent
                            : #00000000;
                        border-width: 2px;
                        border-color: idx < root.checked.length && root.checked[idx]
                            ? Theme.accent
                            : Theme.border-base;

                        if idx < root.checked.length && root.checked[idx]: Text {
                            x: (parent.width - self.width) / 2;
                            y: (parent.height - self.height) / 2;
                            text: "✓";
                            color: #ffffff;
                            font-size: 10px;
                            font-weight: Theme.weight-bold;
                            vertical-alignment: center;
                        }
                    }
                }

                Rectangle {
                    horizontal-stretch: 1;
                    vertical-stretch: 0;

                    Text {
                        y: (parent.height - self.height) / 2;
                        text: root.items[idx];
                        color: idx < root.checked.length && root.checked[idx]
                            ? Theme.text-tertiary
                            : Theme.text-primary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-sm;
                        vertical-alignment: center;
                    }

                    if idx < root.checked.length && root.checked[idx]: Rectangle {
                        y: (parent.height - 1px) / 2;
                        height: 1px;
                        width: parent.width;
                        background: Theme.text-tertiary;
                    }
                }
            }

            row-ta := TouchArea {
                clicked => { root.check-toggle(idx); }
            }
        }
    }
}
```

#### `Chip` ([slint/data-display/Chip.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/Chip.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component Chip inherits Rectangle {
    in property <string> text: "";
    in property <bool> checked: false;
    in property <bool> enabled: true;
    in property <color> chip-accent: Theme.accent;

    callback clicked();

    min-width: max(label.preferred-width + Theme.sp-4 * 2, 48px);
    height: 28px;
    opacity: enabled ? 1.0 : Theme.opacity-disabled;

    bg := Rectangle {
        border-radius: Theme.radius-full;
        border-width: 1px;
        border-color: !enabled ? Theme.border-subtle : (checked ? chip-accent : Theme.border-base);
        background: !enabled ? Theme.btn-disabled-bg : (checked ? chip-accent.with-alpha(0.15) : (ta.pressed ? Theme.surface-pressed : (ta.has-hover ? Theme.surface-hover : Theme.bg-raised)));
        animate background { duration: Theme.dur-fast; easing: Theme.ease-standard; }
    }

    label := Text {
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
        text: root.text;
        color: !enabled ? Theme.text-disabled : (checked ? chip-accent : Theme.text-secondary);
        font-family: Theme.font-ui;
        font-size: Theme.text-sm;
        font-weight: Theme.weight-medium;
        horizontal-alignment: center;
        vertical-alignment: center;
    }

    ta := TouchArea {
        clicked => { root.clicked(); }
    }
}
```

#### `CodeBlock` ([slint/data-display/CodeBlock.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/CodeBlock.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component CodeBlock inherits Rectangle {
    in property <string> code: "";
    in property <string> language: "";

    background: Theme.dark-mode ? #0D1117 : #f6f8fa;
    border-radius: Theme.radius-md;
    border-width: 1px;
    border-color: Theme.border-subtle;
    clip: true;

    VerticalLayout {
        // Header
        if language != "": Rectangle {
            height: 32px;
            background: Theme.dark-mode ? #161b22 : #eaeef2;

            HorizontalLayout {
                padding-left: Theme.sp-3;
                padding-right: Theme.sp-3;
                alignment: stretch;

                Text {
                    text: root.language;
                    color: Theme.text-tertiary;
                    font-family: Theme.font-mono;
                    font-size: Theme.text-xs;
                    vertical-alignment: center;
                }
            }
        }

        // Code
        Rectangle {
            vertical-stretch: 1;
            clip: true;

            Text {
                x: Theme.sp-4;
                y: Theme.sp-3;
                text: root.code;
                color: Theme.text-primary;
                font-family: Theme.font-mono;
                font-size: Theme.text-sm;
                wrap: no-wrap;
            }
        }
    }
}
```

#### `ColumnHeader` ([slint/data-display/ColumnHeader.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/ColumnHeader.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component ColumnHeader inherits Rectangle {
    in property <string> label: "";
    in property <int> sort-state: 0;
    in property <length> col-width: 120px;

    callback clicked();

    width: col-width;
    height: 40px;
    background: ta.pressed ? Theme.surface-pressed
        : ta.has-hover ? Theme.surface-hover
        : Theme.bg-overlay;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        padding-left: Theme.sp-3;
        spacing: Theme.sp-1;

        Text {
            text: root.label;
            color: root.sort-state != 0 ? Theme.accent : Theme.text-secondary;
            font-family: Theme.font-ui;
            font-size: Theme.text-xs;
            font-weight: Theme.weight-semibold;
            vertical-alignment: center;
        }

        if root.sort-state == 1: Text {
            text: "▲";
            color: Theme.accent;
            font-family: Theme.font-ui;
            font-size: Theme.text-xs;
            vertical-alignment: center;
        }

        if root.sort-state == 2: Text {
            text: "▼";
            color: Theme.accent;
            font-family: Theme.font-ui;
            font-size: Theme.text-xs;
            vertical-alignment: center;
        }
    }

    ta := TouchArea {
        clicked => { root.clicked(); }
    }
}
```

#### `DayAgendaCalendar` ([slint/data-display/DayAgendaCalendar.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/DayAgendaCalendar.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component DayAgendaCalendar inherits Rectangle {
    in property <string> selected-day: "Wednesday, July 23";
    in property <int> item-count: 3;

    in property <[string]> event-times: ["9:00 AM", "10:30 AM", "2:00 PM"];
    in property <[string]> event-titles: ["Team Standup", "Code Review", "Planning"];
    in property <[string]> event-subtitles: ["Daily sync with the team", "Review pull request #142", "Sprint planning session"];
    in property <[color]> event-colors: [Theme.accent, Theme.green-500, Theme.amber-500];

    background: Theme.bg-surface;
    border-radius: Theme.radius-card;
    border-width: 1px;
    border-color: Theme.border-base;
    clip: true;

    VerticalLayout {
        padding: Theme.card-padding;
        spacing: Theme.sp-4;

        // Header
        VerticalLayout {
            spacing: Theme.sp-1;
            vertical-stretch: 0;

            Text {
                text: root.selected-day;
                font-family: Theme.font-ui;
                font-size: Theme.text-xl;
                font-weight: Theme.weight-semibold;
                color: Theme.text-primary;
            }

            Rectangle {
                width: 40px;
                height: 3px;
                border-radius: 1px;
                background: Theme.accent;
            }
        }

        // Event list
        VerticalLayout {
            spacing: Theme.sp-4;
            vertical-stretch: 1;

            for evt_idx in root.item-count: Rectangle {
                height: 64px;
                border-radius: Theme.radius-sm;
                background: Theme.surface-hover;

                HorizontalLayout {
                    padding: Theme.sp-3;
                    spacing: Theme.sp-3;

                    // Time label
                    Rectangle {
                        width: 80px;

                        Text {
                            text: root.event-times[evt_idx];
                            font-family: Theme.font-mono;
                            font-size: Theme.text-xs;
                            color: Theme.text-tertiary;
                            vertical-alignment: center;
                        }
                    }

                    // Colored left border
                    Rectangle {
                        width: 3px;
                        border-radius: 1px;
                        background: root.event-colors[evt_idx];
                    }

                    // Event content
                    VerticalLayout {
                        spacing: Theme.sp-0-5;

                        Text {
                            text: root.event-titles[evt_idx];
                            font-family: Theme.font-ui;
                            font-size: Theme.text-base;
                            font-weight: Theme.weight-semibold;
                            color: Theme.text-primary;
                        }

                        Text {
                            text: root.event-subtitles[evt_idx];
                            font-family: Theme.font-ui;
                            font-size: Theme.text-sm;
                            color: Theme.text-secondary;
                        }
                    }
                }
            }
        }
    }
}
```

#### `DefinitionList` ([slint/data-display/DefinitionList.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/DefinitionList.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component DefinitionList inherits Rectangle {
    in property <[string]> keys: [];
    in property <[string]> values: [];

    background: Theme.bg-surface;
    border-radius: Theme.radius-sm;
    clip: true;

    VerticalLayout {
        for key[idx] in root.keys: Rectangle {
            height: 44px;
            background: Math.mod(idx, 2) == 0 ? Theme.bg-overlay : #00000000;

            HorizontalLayout {
                padding-left: Theme.sp-4;
                padding-right: Theme.sp-4;
                spacing: Theme.sp-6;
                alignment: stretch;

                Text {
                    text: root.keys[idx];
                    color: Theme.text-primary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-sm;
                    font-weight: Theme.weight-semibold;
                    vertical-alignment: center;
                    horizontal-stretch: 1;
                }

                Text {
                    text: root.values[idx];
                    color: Theme.text-secondary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-sm;
                    vertical-alignment: center;
                    horizontal-stretch: 2;
                }
            }
        }
    }
}
```

#### `DenseList` ([slint/data-display/DenseList.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/DenseList.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component DenseList inherits Rectangle {
    in property <int> item-count: 0;
    in property <[string]> items: [];

    callback clicked(int);

    background: Theme.bg-surface;
    border-radius: Theme.radius-sm;
    clip: true;

    VerticalLayout {
        for idx in root.item-count: Rectangle {
            height: 32px;
            background: row-ta.pressed ? Theme.surface-pressed
                : row-ta.has-hover ? Theme.surface-hover
                : #00000000;

            animate background { duration: Theme.dur-fast; }

            HorizontalLayout {
                padding-left: Theme.sp-3;
                padding-right: Theme.sp-3;
                alignment: start;

                Text {
                    text: idx < root.items.length ? root.items[idx] : "";
                    color: Theme.text-primary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-xs;
                    vertical-alignment: center;
                }
            }

            row-ta := TouchArea {
                clicked => { root.clicked(idx); }
            }
        }
    }
}
```

#### `DiffViewer` ([slint/data-display/DiffViewer.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/DiffViewer.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component DiffViewer inherits Rectangle {
    in property <string> left-title: "Before";
    in property <string> right-title: "After";

    background: Theme.bg-surface;
    border-radius: Theme.radius-card;
    border-width: 1px;
    border-color: Theme.border-subtle;
    clip: true;

    HorizontalLayout {
        padding: 0;
        spacing: 1px;

        // Left panel — removed lines
        Rectangle {
            background: Theme.bg-base;
            clip: true;
            horizontal-stretch: 1;

            VerticalLayout {
                padding: 0;

                // Header
                Rectangle {
                    background: Theme.bg-overlay;
                    height: 32px;

                    HorizontalLayout {
                        padding-left: Theme.sp-3;
                        padding-right: Theme.sp-3;

                        Text {
                            text: root.left-title;
                            font-family: Theme.font-ui;
                            font-size: Theme.text-sm;
                            font-weight: Theme.weight-semibold;
                            color: Theme.text-secondary;
                            vertical-alignment: center;
                        }
                    }
                }

                // Lines
                Rectangle {
                    clip: true;

                    VerticalLayout {
                        padding: 0;
                        spacing: 0;

                        // Line 1: unchanged
                        Rectangle {
                            height: 26px;
                            background: Theme.bg-base;

                            HorizontalLayout {
                                padding-left: Theme.sp-3;
                                padding-right: Theme.sp-3;
                                spacing: Theme.sp-3;

                                Text {
                                    text: "1";
                                    font-family: Theme.font-mono;
                                    font-size: Theme.text-sm;
                                    color: Theme.text-tertiary;
                                    vertical-alignment: center;
                                    width: 30px;
                                }

                                Text {
                                    text: "  const config = load();";
                                    font-family: Theme.font-mono;
                                    font-size: Theme.text-sm;
                                    color: Theme.text-primary;
                                    vertical-alignment: center;
                                }
                            }
                        }

                        // Line 2: unchanged
                        Rectangle {
                            height: 26px;
                            background: Theme.bg-base;

                            HorizontalLayout {
                                padding-left: Theme.sp-3;
                                padding-right: Theme.sp-3;
                                spacing: Theme.sp-3;

                                Text {
                                    text: "2";
                                    font-family: Theme.font-mono;
                                    font-size: Theme.text-sm;
                                    color: Theme.text-tertiary;
                                    vertical-alignment: center;
                                    width: 30px;
                                }

                                Text {
                                    text: "  if (config.debug) {";
                                    font-family: Theme.font-mono;
                                    font-size: Theme.text-sm;
                                    color: Theme.text-primary;
                                    vertical-alignment: center;
                                }
                            }
                        }

                        // Line 3: removed (red tint)
                        Rectangle {
                            height: 26px;
                            background: Theme.red-100.darker(0.1);

                            HorizontalLayout {
                                padding-left: Theme.sp-3;
                                padding-right: Theme.sp-3;
                                spacing: Theme.sp-3;

                                Text {
                                    text: "3";
                                    font-family: Theme.font-mono;
                                    font-size: Theme.text-sm;
                                    color: Theme.text-tertiary;
                                    vertical-alignment: center;
                                    width: 30px;
                                }

                                Text {
                                    text: "-   console.log(config);";
                                    font-family: Theme.font-mono;
                                    font-size: Theme.text-sm;
                                    color: Theme.red-600;
                                    vertical-alignment: center;
                                }
                            }
                        }

                        // Line 4: unchanged
                        Rectangle {
                            height: 26px;
                            background: Theme.bg-base;

                            HorizontalLayout {
                                padding-left: Theme.sp-3;
                                padding-right: Theme.sp-3;
                                spacing: Theme.sp-3;

                                Text {
                                    text: "4";
                                    font-family: Theme.font-mono;
                                    font-size: Theme.text-sm;
                                    color: Theme.text-tertiary;
                                    vertical-alignment: center;
                                    width: 30px;
                                }

                                Text {
                                    text: "  }";
                                    font-family: Theme.font-mono;
                                    font-size: Theme.text-sm;
                                    color: Theme.text-primary;
                                    vertical-alignment: center;
                                }
                            }
                        }
                    }
                }
            }
        }

        // Right panel — added lines
        Rectangle {
            background: Theme.bg-base;
            clip: true;
            horizontal-stretch: 1;

            VerticalLayout {
                padding: 0;

                // Header
                Rectangle {
                    background: Theme.bg-overlay;
                    height: 32px;

                    HorizontalLayout {
                        padding-left: Theme.sp-3;
                        padding-right: Theme.sp-3;

                        Text {
                            text: root.right-title;
                            font-family: Theme.font-ui;
                            font-size: Theme.text-sm;
                            font-weight: Theme.weight-semibold;
                            color: Theme.text-secondary;
                            vertical-alignment: center;
                        }
                    }
                }

                // Lines
                Rectangle {
                    clip: true;

                    VerticalLayout {
                        padding: 0;
                        spacing: 0;

                        // Line 1: unchanged
                        Rectangle {
                            height: 26px;
                            background: Theme.bg-base;

                            HorizontalLayout {
                                padding-left: Theme.sp-3;
                                padding-right: Theme.sp-3;
                                spacing: Theme.sp-3;

                                Text {
                                    text: "1";
                                    font-family: Theme.font-mono;
                                    font-size: Theme.text-sm;
                                    color: Theme.text-tertiary;
                                    vertical-alignment: center;
                                    width: 30px;
                                }

                                Text {
                                    text: "  const config = load();";
                                    font-family: Theme.font-mono;
                                    font-size: Theme.text-sm;
                                    color: Theme.text-primary;
                                    vertical-alignment: center;
                                }
                            }
                        }

                        // Line 2: unchanged
                        Rectangle {
                            height: 26px;
                            background: Theme.bg-base;

                            HorizontalLayout {
                                padding-left: Theme.sp-3;
                                padding-right: Theme.sp-3;
                                spacing: Theme.sp-3;

                                Text {
                                    text: "2";
                                    font-family: Theme.font-mono;
                                    font-size: Theme.text-sm;
                                    color: Theme.text-tertiary;
                                    vertical-alignment: center;
                                    width: 30px;
                                }

                                Text {
                                    text: "  if (config.debug) {";
                                    font-family: Theme.font-mono;
                                    font-size: Theme.text-sm;
                                    color: Theme.text-primary;
                                    vertical-alignment: center;
                                }
                            }
                        }

                        // Line 3: added (green tint)
                        Rectangle {
                            height: 26px;
                            background: Theme.green-100.darker(0.1);

                            HorizontalLayout {
                                padding-left: Theme.sp-3;
                                padding-right: Theme.sp-3;
                                spacing: Theme.sp-3;

                                Text {
                                    text: "3";
                                    font-family: Theme.font-mono;
                                    font-size: Theme.text-sm;
                                    color: Theme.text-tertiary;
                                    vertical-alignment: center;
                                    width: 30px;
                                }

                                Text {
                                    text: "+   logger.info(config);";
                                    font-family: Theme.font-mono;
                                    font-size: Theme.text-sm;
                                    color: Theme.green-600;
                                    vertical-alignment: center;
                                }
                            }
                        }

                        // Line 4: added (green tint)
                        Rectangle {
                            height: 26px;
                            background: Theme.green-100.darker(0.1);

                            HorizontalLayout {
                                padding-left: Theme.sp-3;
                                padding-right: Theme.sp-3;
                                spacing: Theme.sp-3;

                                Text {
                                    text: "4";
                                    font-family: Theme.font-mono;
                                    font-size: Theme.text-sm;
                                    color: Theme.text-tertiary;
                                    vertical-alignment: center;
                                    width: 30px;
                                }

                                Text {
                                    text: "+   metrics.record();";
                                    font-family: Theme.font-mono;
                                    font-size: Theme.text-sm;
                                    color: Theme.green-600;
                                    vertical-alignment: center;
                                }
                            }
                        }

                        // Line 5: unchanged
                        Rectangle {
                            height: 26px;
                            background: Theme.bg-base;

                            HorizontalLayout {
                                padding-left: Theme.sp-3;
                                padding-right: Theme.sp-3;
                                spacing: Theme.sp-3;

                                Text {
                                    text: "5";
                                    font-family: Theme.font-mono;
                                    font-size: Theme.text-sm;
                                    color: Theme.text-tertiary;
                                    vertical-alignment: center;
                                    width: 30px;
                                }

                                Text {
                                    text: "  }";
                                    font-family: Theme.font-mono;
                                    font-size: Theme.text-sm;
                                    color: Theme.text-primary;
                                    vertical-alignment: center;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
```

#### `Divider` ([slint/data-display/Divider.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/Divider.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component Divider inherits Rectangle {
    in property <string> label: "";
    in property <bool> vertical: false;

    background: transparent;
    horizontal-stretch: 1;

    if vertical: Rectangle {
        width: 1px;
        height: parent.height;
        x: (parent.width - self.width) / 2;
        background: Theme.divider;
    }

    if !vertical && label == "": Rectangle {
        height: 1px;
        width: parent.width;
        y: (parent.height - self.height) / 2;
        background: Theme.divider;
    }

    if !vertical && label != "": HorizontalLayout {
        height: parent.height;
        alignment: center;
        spacing: Theme.sp-3;

        Rectangle { vertical-stretch: 1; height: 1px; y: (parent.height - self.height) / 2; background: Theme.divider; }

        if label != "": Text {
            text: root.label;
            color: Theme.text-tertiary;
            font-family: Theme.font-ui;
            font-size: Theme.text-xs;
            font-weight: Theme.weight-medium;
            vertical-alignment: center;
            horizontal-stretch: 0;
        }

        Rectangle { vertical-stretch: 1; height: 1px; y: (parent.height - self.height) / 2; background: Theme.divider; }
    }
}
```

#### `EditableInlineTable` ([slint/data-display/EditableInlineTable.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/EditableInlineTable.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component EditableInlineTable inherits Rectangle {
    in property <[string]> columns: [];
    in property <int> row-count: 0;

    background: Theme.bg-surface;
    border-radius: Theme.radius-card;
    border-width: 1px;
    border-color: Theme.border-base;
    clip: true;

    VerticalLayout {
        if columns.length > 0: Rectangle {
            height: 40px;
            background: Theme.bg-overlay;

            HorizontalLayout {
                for col[i] in root.columns: Rectangle {
                    horizontal-stretch: 1;

                    Text {
                        x: Theme.sp-3;
                        y: (parent.height - self.height) / 2;
                        text: col;
                        color: Theme.text-secondary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-xs;
                        font-weight: Theme.weight-semibold;
                        vertical-alignment: center;
                        horizontal-stretch: 1;
                    }
                }
            }
        }

        Rectangle { height: 1px; background: Theme.border-subtle; }

        VerticalLayout {
            for row[rowidx] in root.row-count: Rectangle {
                height: 40px;
                background: row-ta.pressed ? Theme.surface-pressed
                    : row-ta.has-hover ? Theme.surface-hover
                    : transparent;

                animate background { duration: Theme.dur-fast; }

                HorizontalLayout {
                    for col[i] in root.columns: Rectangle {
                        horizontal-stretch: 1;

                        Rectangle {
                            x: Theme.sp-2;
                            y: 6px;
                            width: parent.width - Theme.sp-4;
                            height: 28px;
                            border-radius: Theme.radius-sm;
                            border-width: 1px;
                            border-color: cell-ta.has-hover ? Theme.accent : transparent;

                            animate border-color { duration: Theme.dur-fast; }

                            Text {
                                x: Theme.sp-2;
                                y: (parent.height - self.height) / 2;
                                text: "Row \{rowidx + 1} — \{col}";
                                color: Theme.text-primary;
                                font-family: Theme.font-ui;
                                font-size: Theme.text-sm;
                                vertical-alignment: center;
                                horizontal-stretch: 1;
                            }

                            cell-ta := TouchArea {}
                        }
                    }
                }

                row-ta := TouchArea {}
            }
        }
    }
}
```

#### `EmptyState` ([slint/data-display/EmptyState.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/EmptyState.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component EmptyState inherits Rectangle {
    in property <string> title: "No data";
    in property <string> description: "";
    in property <string> icon: "";

    background: transparent;
    horizontal-stretch: 1;

    VerticalLayout {
        spacing: Theme.sp-3;
        alignment: center;

        if icon != "": Text {
            text: icon;
            color: Theme.text-tertiary;
            font-size: 36px;
            horizontal-alignment: center;
        }

        Text {
            text: root.title;
            color: Theme.text-secondary;
            font-family: Theme.font-ui;
            font-size: Theme.text-lg;
            font-weight: Theme.weight-medium;
            horizontal-alignment: center;
        }

        if description != "": Text {
            text: root.description;
            color: Theme.text-tertiary;
            font-family: Theme.font-ui;
            font-size: Theme.text-sm;
            horizontal-alignment: center;
            wrap: word-wrap;
        }
    }
}
```

#### `ExpandableRowTable` ([slint/data-display/ExpandableRowTable.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/ExpandableRowTable.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component ExpandableRowTable inherits Rectangle {
    in property <[string]> columns: [];
    in property <int> row-count: 0;
    in property <int> expanded-row: -1;

    callback expand-toggle(int);

    background: Theme.bg-surface;
    border-radius: Theme.radius-card;
    border-width: 1px;
    border-color: Theme.border-base;
    clip: true;

    VerticalLayout {
        if columns.length > 0: Rectangle {
            height: 40px;
            background: Theme.bg-overlay;

            HorizontalLayout {
                Rectangle { width: 32px; horizontal-stretch: 0; }

                for col[i] in root.columns: Rectangle {
                    horizontal-stretch: 1;

                    Text {
                        x: Theme.sp-3;
                        y: (parent.height - self.height) / 2;
                        text: col;
                        color: Theme.text-secondary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-xs;
                        font-weight: Theme.weight-semibold;
                        vertical-alignment: center;
                        horizontal-stretch: 1;
                    }
                }
            }
        }

        Rectangle { height: 1px; background: Theme.border-subtle; }

        VerticalLayout {
            for row[rowidx] in root.row-count: VerticalLayout {
                Rectangle {
                    height: 40px;
                    background: row-ta.pressed ? Theme.surface-pressed
                        : row-ta.has-hover ? Theme.surface-hover
                        : transparent;

                    animate background { duration: Theme.dur-fast; }

                    HorizontalLayout {
                        Rectangle {
                            width: 32px;
                            horizontal-stretch: 0;

                            Text {
                                x: (parent.width - self.width) / 2;
                                y: (parent.height - self.height) / 2;
                                text: root.expanded-row == rowidx ? "▼" : "▶";
                                color: Theme.text-tertiary;
                                font-family: Theme.font-ui;
                                font-size: Theme.text-xs;
                                vertical-alignment: center;
                            }
                        }

                        for col[i] in root.columns: Rectangle {
                            horizontal-stretch: 1;

                            Text {
                                x: Theme.sp-3;
                                y: (parent.height - self.height) / 2;
                                text: "Row \{rowidx + 1} — \{col}";
                                color: Theme.text-primary;
                                font-family: Theme.font-ui;
                                font-size: Theme.text-sm;
                                vertical-alignment: center;
                                horizontal-stretch: 1;
                            }
                        }
                    }

                    row-ta := TouchArea {
                        clicked => { root.expand-toggle(rowidx); }
                    }
                }

                if root.expanded-row == rowidx: Rectangle {
                    height: 48px;
                    background: Theme.bg-overlay;

                    Text {
                        x: 48px;
                        y: (parent.height - self.height) / 2;
                        text: "Detail content for row \{rowidx + 1}";
                        color: Theme.text-secondary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-sm;
                        vertical-alignment: center;
                    }
                }
            }
        }
    }
}
```

#### `FeatureComparisonMatrix` ([slint/data-display/FeatureComparisonMatrix.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/FeatureComparisonMatrix.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component FeatureComparisonMatrix inherits Rectangle {
    in property <[string]> features: [];
    in property <[string]> plans: [];
    in property <[bool]> matrix: [];

    background: Theme.bg-surface;
    border-radius: Theme.radius-card;
    border-width: 1px;
    border-color: Theme.border-base;
    clip: true;

    VerticalLayout {
        Rectangle {
            height: 40px;
            background: Theme.bg-overlay;

            HorizontalLayout {
                Rectangle { width: 160px; horizontal-stretch: 0; }

                for plan[i] in root.plans: Rectangle {
                    horizontal-stretch: 1;

                    Text {
                        x: (parent.width - self.width) / 2;
                        y: (parent.height - self.height) / 2;
                        text: plan;
                        color: Theme.text-primary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-xs;
                        font-weight: Theme.weight-semibold;
                        vertical-alignment: center;
                        horizontal-alignment: center;
                        horizontal-stretch: 1;
                    }
                }
            }
        }

        Rectangle { height: 1px; background: Theme.border-subtle; }

        for feat[fi] in root.features: Rectangle {
            height: 40px;
            background: mod(fi, 2) == 1 ? Theme.bg-overlay : transparent;

            HorizontalLayout {
                Rectangle {
                    width: 160px;
                    horizontal-stretch: 0;

                    Text {
                        x: Theme.sp-3;
                        y: (parent.height - self.height) / 2;
                        text: feat;
                        color: Theme.text-primary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-sm;
                        vertical-alignment: center;
                    }
                }

                for plan[pi] in root.plans: Rectangle {
                    horizontal-stretch: 1;

                    Text {
                        x: (parent.width - self.width) / 2;
                        y: (parent.height - self.height) / 2;
                        text: (fi * root.plans.length + pi) < root.matrix.length && root.matrix[fi * root.plans.length + pi] ? "✓" : "✗";
                        color: (fi * root.plans.length + pi) < root.matrix.length && root.matrix[fi * root.plans.length + pi] ? Theme.state-success : Theme.text-tertiary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-sm;
                        font-weight: Theme.weight-bold;
                        vertical-alignment: center;
                        horizontal-alignment: center;
                        horizontal-stretch: 1;
                    }
                }
            }
        }
    }
}
```

#### `FrozenColumnTable` ([slint/data-display/FrozenColumnTable.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/FrozenColumnTable.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component FrozenColumnTable inherits Rectangle {
    in property <[string]> columns: [];
    in property <int> row-count: 0;

    background: Theme.bg-surface;
    border-radius: Theme.radius-card;
    border-width: 1px;
    border-color: Theme.border-base;
    clip: true;

    VerticalLayout {
        if columns.length > 0: Rectangle {
            height: 40px;
            background: Theme.bg-overlay;

            HorizontalLayout {
                for col[i] in root.columns: Rectangle {
                    horizontal-stretch: i == 0 ? 0 : 1;
                    width: i == 0 ? 120px : 0px;

                    if i > 0: Rectangle {
                        width: 1px;
                        height: 24px;
                        y: (parent.height - self.height) / 2;
                        background: Theme.border-subtle;
                        horizontal-stretch: 0;
                    }

                    Text {
                        x: Theme.sp-3;
                        y: (parent.height - self.height) / 2;
                        text: col;
                        color: Theme.text-secondary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-xs;
                        font-weight: Theme.weight-semibold;
                        vertical-alignment: center;
                        horizontal-stretch: i == 0 ? 0 : 1;
                    }
                }
            }
        }

        Rectangle { height: 1px; background: Theme.border-subtle; }

        VerticalLayout {
            for row[rowidx] in root.row-count: Rectangle {
                height: 40px;
                background: row-ta.pressed ? Theme.surface-pressed
                    : row-ta.has-hover ? Theme.surface-hover
                    : transparent;

                animate background { duration: Theme.dur-fast; }

                HorizontalLayout {
                    for col[i] in root.columns: Rectangle {
                        horizontal-stretch: i == 0 ? 0 : 1;
                        width: i == 0 ? 120px : 0px;

                        if i > 0: Rectangle {
                            width: 1px;
                            height: 24px;
                            y: (parent.height - self.height) / 2;
                            background: Theme.border-subtle;
                            horizontal-stretch: 0;
                        }

                        Text {
                            x: Theme.sp-3;
                            y: (parent.height - self.height) / 2;
                            text: "Row \{rowidx + 1} — \{col}";
                            color: i == 0 ? Theme.text-primary : Theme.text-secondary;
                            font-family: Theme.font-ui;
                            font-size: Theme.text-sm;
                            font-weight: i == 0 ? Theme.weight-semibold : Theme.weight-medium;
                            vertical-alignment: center;
                            horizontal-stretch: i == 0 ? 0 : 1;
                        }
                    }
                }

                row-ta := TouchArea {}
            }
        }
    }
}
```

#### `GanttTimeline` ([slint/data-display/GanttTimeline.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/GanttTimeline.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component GanttTimeline inherits Rectangle {
    in property <int> item-count: 4;

    background: Theme.bg-surface;
    border-radius: Theme.radius-card;
    border-width: 1px;
    border-color: Theme.border-base;
    clip: true;

    in property <[string]> demo-tasks: ["Design", "Develop", "Test", "Deploy"];
    in property <[int]> demo-starts: [0, 20, 60, 85];
    in property <[int]> demo-ends: [30, 70, 90, 100];
    in property <[color]> demo-colors: [Theme.accent, Theme.green-500, #D97706, Theme.red-500];

    VerticalLayout {
        Rectangle {
            height: 32px;
            background: Theme.bg-overlay;

            HorizontalLayout {
                Rectangle { width: 100px; horizontal-stretch: 0; }
                Rectangle { horizontal-stretch: 1;
                    HorizontalLayout {
                        for g[i] in [0, 1, 2, 3, 4]: Rectangle {
                            horizontal-stretch: 1;

                            Text {
                                x: (parent.width - self.width) / 2;
                                y: (parent.height - self.height) / 2;
                                text: "\{i * 25}%";
                                color: Theme.text-tertiary;
                                font-family: Theme.font-mono;
                                font-size: Theme.text-xs;
                                vertical-alignment: center;
                            }
                        }
                    }
                }
            }
        }

        Rectangle { height: 1px; background: Theme.border-subtle; }

        for row[rowidx] in root.item-count: Rectangle {
            height: 40px;

            HorizontalLayout {
                Rectangle {
                    width: 100px;
                    horizontal-stretch: 0;

                    Text {
                        x: Theme.sp-3;
                        y: (parent.height - self.height) / 2;
                        text: rowidx < root.demo_tasks.length ? root.demo_tasks[rowidx] : "";
                        color: Theme.text-primary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-sm;
                        font-weight: Theme.weight-medium;
                        vertical-alignment: center;
                    }
                }

                Rectangle {
                    horizontal-stretch: 1;

                    // Grid lines
                    for g[gidx] in [0, 1, 2, 3, 4]: Rectangle {
                        x: gidx * (parent.width / 4);
                        width: 1px;
                        height: parent.height;
                        background: Theme.border-subtle;
                    }

                    // Task bar
                    Rectangle {
                        x: rowidx < root.demo_starts.length ? root.demo_starts[rowidx] * (parent.width - 0px) / 100 : 0px;
                        width: rowidx < root.demo_ends.length && rowidx < root.demo_starts.length ? (root.demo_ends[rowidx] - root.demo_starts[rowidx]) * parent.width / 100 : 0px;
                        height: 24px;
                        y: (parent.height - self.height) / 2;
                        border-radius: Theme.radius-sm;
                        background: rowidx < root.demo_colors.length ? root.demo_colors[rowidx] : Theme.accent;
                    }
                }
            }
        }
    }
}
```

#### `GroupedList` ([slint/data-display/GroupedList.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/GroupedList.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component GroupedList inherits Rectangle {
    in property <[string]> sections: [];
    in property <[int]> items-per-section: [];

    callback item-clicked(int, int);

    background: Theme.bg-surface;
    border-radius: Theme.radius-sm;
    clip: true;

    VerticalLayout {
        for section[si] in root.sections: Rectangle {
            height: items-per-section[si] > 0
                ? 32px + items-per-section[si] * 40px
                : 32px;

            VerticalLayout {
                Rectangle {
                    height: 32px;
                    background: Theme.bg-overlay;

                    HorizontalLayout {
                        padding-left: Theme.sp-4;
                        padding-right: Theme.sp-4;
                        alignment: start;

                        Text {
                            text: section;
                            color: Theme.text-secondary;
                            font-family: Theme.font-ui;
                            font-size: Theme.text-xs;
                            font-weight: Theme.weight-bold;
                            letter-spacing: Theme.tracking-wide;
                            vertical-alignment: center;
                        }
                    }
                }

                Rectangle {
                    height: items-per-section[si] * 40px;
                    background: item-ta.pressed ? Theme.surface-pressed
                        : item-ta.has-hover ? Theme.surface-hover
                        : #00000000;

                    animate background { duration: Theme.dur-fast; }

                    HorizontalLayout {
                        padding-left: Theme.sp-8;
                        padding-right: Theme.sp-4;
                        alignment: start;

                        Text {
                            text: "Item " + (si + 1);
                            color: Theme.text-primary;
                            font-family: Theme.font-ui;
                            font-size: Theme.text-sm;
                            vertical-alignment: center;
                        }
                    }

                    item-ta := TouchArea {
                        clicked => { root.item-clicked(si, 0); }
                    }
                }
            }
        }
    }
}
```

#### `HorizontalTimeline` ([slint/data-display/HorizontalTimeline.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/HorizontalTimeline.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component HorizontalTimeline inherits Rectangle {
    in property <[string]> items: [];
    in property <int> active-index: 0;

    height: 80px;
    background: transparent;

    Rectangle {
        y: 19px;
        height: 2px;
        x: 24px;
        width: parent.width - 48px;
        background: Theme.border-subtle;
    }

    for item[idx] in root.items: Rectangle {
        x: 24px + idx * ((parent.width - 48px) / (root.items.length > 1 ? root.items.length - 1 : 1)) - 10px;
        width: 20px;

        Rectangle {
            width: 20px;
            height: 20px;
            border-radius: Theme.radius-full;
            background: idx <= root.active_index ? Theme.accent : Theme.bg-surface;
            border-width: 2px;
            border-color: idx <= root.active_index ? Theme.accent : Theme.border-base;
        }

        Text {
            y: 28px;
            width: 80px;
            x: -30px;
            text: item;
            color: idx <= root.active_index ? Theme.text-primary : Theme.text-tertiary;
            font-family: Theme.font-ui;
            font-size: Theme.text-xs;
            horizontal-alignment: center;
        }
    }
}
```

#### `JsonTreeViewer` ([slint/data-display/JsonTreeViewer.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/JsonTreeViewer.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component JsonTreeViewer inherits Rectangle {
    in property <int> item-count: 4;

    background: Theme.bg-surface;
    border-radius: Theme.radius-card;
    border-width: 1px;
    border-color: Theme.border-subtle;
    clip: true;

    VerticalLayout {
        padding: 0;

        // Header
        Rectangle {
            background: Theme.bg-overlay;
            height: 32px;

            HorizontalLayout {
                padding-left: Theme.sp-3;
                padding-right: Theme.sp-3;
                alignment: start;

                Text {
                    text: "JSON Tree";
                    font-family: Theme.font-ui;
                    font-size: Theme.text-sm;
                    font-weight: Theme.weight-semibold;
                    color: Theme.text-secondary;
                    vertical-alignment: center;
                }
            }
        }

        // Tree content
        Rectangle {
            clip: true;

            VerticalLayout {
                padding: Theme.sp-3;
                spacing: 0;

                // Root opening brace
                HorizontalLayout {
                    spacing: 0;

                    Text {
                        text: "▼ ";
                        font-family: Theme.font-mono;
                        font-size: Theme.text-sm;
                        color: Theme.text-tertiary;
                        vertical-alignment: center;
                    }

                    Text {
                        text: "{";
                        font-family: Theme.font-mono;
                        font-size: Theme.text-sm;
                        font-weight: Theme.weight-semibold;
                        color: Theme.text-primary;
                        vertical-alignment: center;
                    }
                }

                // "name" key — string value
                HorizontalLayout {
                    padding-left: Theme.sp-5;
                    spacing: 0;

                    Text {
                        text: "\"name\"";
                        font-family: Theme.font-mono;
                        font-size: Theme.text-sm;
                        color: Theme.accent;
                        vertical-alignment: center;
                    }

                    Text {
                        text: ": ";
                        font-family: Theme.font-mono;
                        font-size: Theme.text-sm;
                        color: Theme.text-primary;
                        vertical-alignment: center;
                    }

                    Text {
                        text: "\"LTK\"";
                        font-family: Theme.font-mono;
                        font-size: Theme.text-sm;
                        color: Theme.green-600;
                        vertical-alignment: center;
                    }
                }

                // "version" key — number value
                HorizontalLayout {
                    padding-left: Theme.sp-5;
                    spacing: 0;

                    Text {
                        text: "\"version\"";
                        font-family: Theme.font-mono;
                        font-size: Theme.text-sm;
                        color: Theme.accent;
                        vertical-alignment: center;
                    }

                    Text {
                        text: ": ";
                        font-family: Theme.font-mono;
                        font-size: Theme.text-sm;
                        color: Theme.text-primary;
                        vertical-alignment: center;
                    }

                    Text {
                        text: "1.0";
                        font-family: Theme.font-mono;
                        font-size: Theme.text-sm;
                        color: Theme.amber-600;
                        vertical-alignment: center;
                    }
                }

                // "debug" key — boolean value
                HorizontalLayout {
                    padding-left: Theme.sp-5;
                    spacing: 0;

                    Text {
                        text: "\"debug\"";
                        font-family: Theme.font-mono;
                        font-size: Theme.text-sm;
                        color: Theme.accent;
                        vertical-alignment: center;
                    }

                    Text {
                        text: ": ";
                        font-family: Theme.font-mono;
                        font-size: Theme.text-sm;
                        color: Theme.text-primary;
                        vertical-alignment: center;
                    }

                    Text {
                        text: "false";
                        font-family: Theme.font-mono;
                        font-size: Theme.text-sm;
                        color: Theme.red-600;
                        vertical-alignment: center;
                    }
                }

                // "tags" key — array (collapsed indicator)
                HorizontalLayout {
                    padding-left: Theme.sp-5;
                    spacing: 0;

                    Text {
                        text: "▶ ";
                        font-family: Theme.font-mono;
                        font-size: Theme.text-sm;
                        color: Theme.text-tertiary;
                        vertical-alignment: center;
                    }

                    Text {
                        text: "\"tags\"";
                        font-family: Theme.font-mono;
                        font-size: Theme.text-sm;
                        color: Theme.accent;
                        vertical-alignment: center;
                    }

                    Text {
                        text: ": ";
                        font-family: Theme.font-mono;
                        font-size: Theme.text-sm;
                        color: Theme.text-primary;
                        vertical-alignment: center;
                    }

                    Text {
                        text: "[…]";
                        font-family: Theme.font-mono;
                        font-size: Theme.text-sm;
                        color: Theme.text-tertiary;
                        vertical-alignment: center;
                    }
                }

                // Root closing brace
                HorizontalLayout {
                    spacing: 0;

                    Text {
                        text: "}";
                        font-family: Theme.font-mono;
                        font-size: Theme.text-sm;
                        font-weight: Theme.weight-semibold;
                        color: Theme.text-primary;
                        vertical-alignment: center;
                    }
                }
            }
        }
    }
}
```

#### `KPITile` ([slint/data-display/KPITile.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/KPITile.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component KPITile inherits Rectangle {
    in property <string> label: "";
    in property <string> value: "";
    in property <string> change: "";
    in-out property <int> trend: 0;

    min-width: 140px;
    min-height: 80px;
    background: Theme.bg-surface;
    border-radius: Theme.radius-card;
    border-width: 1px;
    border-color: Theme.border-subtle;

    VerticalLayout {
        padding: Theme.card-padding;
        spacing: Theme.sp-1;

        Text {
            text: root.label;
            color: Theme.text-tertiary;
            font-family: Theme.font-ui;
            font-size: Theme.text-xs;
            font-weight: Theme.weight-semibold;
            letter-spacing: Theme.tracking-wide;
        }

        Text {
            text: root.value;
            color: Theme.text-primary;
            font-family: Theme.font-display;
            font-size: Theme.text-2xl;
            font-weight: Theme.weight-bold;
        }

        if root.change != "": HorizontalLayout {
            spacing: Theme.sp-1;

            Text {
                text: root.trend == 1 ? "▲" : (root.trend == 2 ? "▼" : "");
                color: root.trend == 1 ? Theme.green-700 : (root.trend == 2 ? Theme.red-600 : Theme.text-tertiary);
                font-family: Theme.font-ui;
                font-size: Theme.text-xs;
                vertical-alignment: center;
            }

            Text {
                text: root.change;
                color: root.trend == 1 ? Theme.green-700 : (root.trend == 2 ? Theme.red-600 : Theme.text-tertiary);
                font-family: Theme.font-ui;
                font-size: Theme.text-sm;
                font-weight: Theme.weight-medium;
                vertical-alignment: center;
            }
        }
    }
}
```

#### `Kbd` ([slint/data-display/Kbd.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/Kbd.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component Kbd inherits Rectangle {
    in property <string> text: "";

    horizontal-stretch: 0;

    bg := Rectangle {
        border-radius: Theme.radius-xs;
        border-width: 1px;
        border-color: Theme.border-base;
        background: Theme.bg-overlay;
    }

    label := Text {
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
        text: root.text;
        color: Theme.text-secondary;
        font-family: Theme.font-mono;
        font-size: Theme.text-xs;
        font-weight: Theme.weight-medium;
        vertical-alignment: center;
    }

    width: max(label.preferred-width + Theme.sp-3, 24px);
    height: 22px;
}
```

#### `KeyValueRow` ([slint/data-display/KeyValueRow.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/KeyValueRow.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component KeyValueRow inherits Rectangle {
    in property <string> key: "";
    in property <string> value: "";
    in property <brush> value-color: Theme.text-primary;

    height: 36px;
    background: transparent;

    HorizontalLayout {
        padding-left: Theme.sp-3;
        padding-right: Theme.sp-3;

        Text {
            text: root.key;
            color: Theme.text-secondary;
            font-family: Theme.font-ui;
            font-size: Theme.text-sm;
            font-weight: Theme.weight-medium;
            vertical-alignment: center;
            horizontal-stretch: 1;
        }

        Text {
            text: root.value;
            color: root.value-color;
            font-family: Theme.font-ui;
            font-size: Theme.text-sm;
            vertical-alignment: center;
            horizontal-stretch: 1;
        }
    }

    Rectangle {
        y: parent.height - 1px;
        height: 1px;
        background: Theme.border-subtle;
    }
}
```

#### `LeaderboardTable` ([slint/data-display/LeaderboardTable.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/LeaderboardTable.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component LeaderboardTable inherits Rectangle {
    in property <int> row-count: 8;
    in property <int> highlight-rank: 3;

    background: Theme.bg-surface;
    border-radius: Theme.radius-card;
    border-width: 1px;
    border-color: Theme.border-base;
    clip: true;

    in property <[string]> demo-names: ["Alice", "Bob", "Carol", "Dave", "Eve", "Frank", "Grace", "Heidi"];
    in property <[string]> demo-scores: ["12,450", "11,200", "10,800", "9,300", "8,100", "7,600", "6,900", "5,400"];

    VerticalLayout {
        Rectangle {
            height: 40px;
            background: Theme.bg-overlay;

            HorizontalLayout {
                Rectangle { width: 48px; horizontal-stretch: 0; }
                Rectangle { horizontal-stretch: 1; Text { x: Theme.sp-3; y: (parent.height - self.height) / 2; text: "Player"; color: Theme.text-secondary; font-family: Theme.font-ui; font-size: Theme.text-xs; font-weight: Theme.weight-semibold; vertical-alignment: center; horizontal-stretch: 1; } }
                Rectangle { width: 80px; horizontal-stretch: 0; Text { x: Theme.sp-3; y: (parent.height - self.height) / 2; text: "Score"; color: Theme.text-secondary; font-family: Theme.font-ui; font-size: Theme.text-xs; font-weight: Theme.weight-semibold; vertical-alignment: center; horizontal-stretch: 1; } }
            }
        }

        Rectangle { height: 1px; background: Theme.border-subtle; }

        for row[rowidx] in root.row-count: Rectangle {
            height: 44px;
            background: row-ta.pressed ? Theme.surface-pressed
                : row-ta.has-hover ? Theme.surface-hover
                : transparent;

            animate background { duration: Theme.dur-fast; }

            HorizontalLayout {
                Rectangle {
                    width: 48px;
                    horizontal-stretch: 0;

                    Rectangle {
                        width: 28px;
                        height: 28px;
                        x: 10px;
                        y: (parent.height - self.height) / 2;
                        border-radius: Theme.radius-full;
                        background: rowidx == 0 ? #FFD700
                            : rowidx == 1 ? #C0C0C0
                            : rowidx == 2 ? #CD7F32
                            : Theme.bg-overlay;

                        Text {
                            x: (parent.width - self.width) / 2;
                            y: (parent.height - self.height) / 2;
                            text: "\{rowidx + 1}";
                            color: rowidx < 3 ? #ffffff : Theme.text-secondary;
                            font-family: Theme.font-ui;
                            font-size: Theme.text-xs;
                            font-weight: Theme.weight-bold;
                            vertical-alignment: center;
                        }
                    }
                }

                Rectangle {
                    horizontal-stretch: 1;

                    Text {
                        x: Theme.sp-3;
                        y: (parent.height - self.height) / 2;
                        text: rowidx < root.demo-names.length ? root.demo_names[rowidx] : "";
                        color: Theme.text-primary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-sm;
                        font-weight: rowidx < root.highlight-rank ? Theme.weight-semibold : Theme.weight-medium;
                        vertical-alignment: center;
                        horizontal-stretch: 1;
                    }
                }

                Rectangle {
                    width: 80px;
                    horizontal-stretch: 0;

                    Text {
                        x: Theme.sp-3;
                        y: (parent.height - self.height) / 2;
                        text: rowidx < root.demo-scores.length ? root.demo_scores[rowidx] : "";
                        color: rowidx == 0 ? #FFD700 : Theme.text-secondary;
                        font-family: Theme.font-mono;
                        font-size: Theme.text-sm;
                        font-weight: Theme.weight-semibold;
                        vertical-alignment: center;
                        horizontal-stretch: 1;
                    }
                }
            }

            row-ta := TouchArea {}
        }
    }
}
```

#### `LeadingIconList` ([slint/data-display/LeadingIconList.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/LeadingIconList.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component LeadingIconList inherits Rectangle {
    in property <[string]> titles: [];
    in property <[string]> icons: [];

    callback clicked(int);

    background: Theme.bg-surface;
    border-radius: Theme.radius-sm;
    clip: true;

    VerticalLayout {
        for title[idx] in root.titles: Rectangle {
            height: 44px;
            background: row-ta.pressed ? Theme.surface-pressed
                : row-ta.has-hover ? Theme.surface-hover
                : #00000000;

            animate background { duration: Theme.dur-fast; }

            HorizontalLayout {
                padding-left: Theme.sp-3;
                padding-right: Theme.sp-3;
                spacing: Theme.sp-3;
                alignment: stretch;

                Rectangle {
                    width: 32px;
                    height: 32px;
                    horizontal-stretch: 0;
                    border-radius: Theme.radius-sm;
                    background: Theme.bg-overlay;

                    Text {
                        x: (parent.width - self.width) / 2;
                        y: (parent.height - self.height) / 2;
                        text: root.icons[idx];
                        color: Theme.text-secondary;
                        font-size: Theme.text-lg;
                        vertical-alignment: center;
                    }
                }

                Text {
                    text: root.titles[idx];
                    color: Theme.text-primary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-sm;
                    vertical-alignment: center;
                    horizontal-stretch: 1;
                }
            }

            row-ta := TouchArea {
                clicked => { root.clicked(idx); }
            }
        }
    }
}
```

#### `ListItem` ([slint/data-display/ListItem.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/ListItem.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component ListItem inherits Rectangle {
    in property <string> title: "";
    in property <string> subtitle: "";
    in property <string> icon: "";
    in property <string> trailing: "";
    in property <bool> pressable: true;

    callback clicked();

    height: 52px;
    background: ta.pressed ? Theme.surface-pressed
        : ta.has-hover ? Theme.surface-hover
        : transparent;
    border-radius: Theme.radius-sm;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        padding-left: Theme.sp-3;
        padding-right: Theme.sp-3;
        spacing: Theme.sp-3;
        alignment: stretch;

        if icon != "": Rectangle {
            width: 36px;
            height: 36px;
            y: (parent.height - self.height) / 2;
            horizontal-stretch: 0;

            icon-bg := Rectangle {
                width: 36px;
                height: 36px;
                border-radius: Theme.radius-sm;
                background: Theme.bg-overlay;

                Text {
                    x: (parent.width - self.width) / 2;
                    y: (parent.height - self.height) / 2;
                    text: root.icon;
                    color: Theme.text-secondary;
                    font-size: Theme.text-lg;
                    vertical-alignment: center;
                    horizontal-alignment: center;
                }
            }
        }

        VerticalLayout {
            horizontal-stretch: 1;
            alignment: center;
            spacing: Theme.sp-0-5;

            if title != "": Text {
                text: root.title;
                color: Theme.text-primary;
                font-family: Theme.font-ui;
                font-size: Theme.text-sm;
                font-weight: Theme.weight-medium;
                vertical-alignment: center;
            }

            if subtitle != "": Text {
                text: root.subtitle;
                color: Theme.text-tertiary;
                font-family: Theme.font-ui;
                font-size: Theme.text-xs;
                vertical-alignment: center;
            }
        }

        if trailing != "": Text {
            text: root.trailing;
            color: Theme.text-tertiary;
            font-family: Theme.font-ui;
            font-size: Theme.text-sm;
            vertical-alignment: center;
            horizontal-stretch: 0;
        }
    }

    ta := TouchArea {
        enabled: root.pressable;
        clicked => { root.clicked(); }
    }
}
```

#### `LogViewer` ([slint/data-display/LogViewer.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/LogViewer.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component LogViewer inherits Rectangle {
    in property <int> item-count: 6;

    background: Theme.bg-surface;
    border-radius: Theme.radius-card;
    border-width: 1px;
    border-color: Theme.border-subtle;
    clip: true;

    in property <[string]> levels: ["INFO", "WARN", "ERROR", "INFO", "DEBUG", "ERROR"];
    in property <[brush]> colors: [Theme.text-secondary, Theme.amber-600, Theme.red-600, Theme.text-secondary, Theme.text-tertiary, Theme.red-600];
    in property <[string]> timestamps: ["10:23:01", "10:23:04", "10:23:07", "10:23:10", "10:23:12", "10:23:15"];
    in property <[string]> messages: ["Application started successfully", "Cache miss for key: user_session", "Connection refused: db-primary:5432", "Retrying connection (attempt 2)", "WebSocket frame received: 128 bytes", "Unhandled exception in worker thread"];

    VerticalLayout {
        Rectangle {
            background: Theme.bg-overlay;
            height: 32px;

            HorizontalLayout {
                padding-left: Theme.sp-3;
                padding-right: Theme.sp-3;

                Text {
                    text: "Log Viewer";
                    font-family: Theme.font-ui;
                    font-size: Theme.text-sm;
                    font-weight: Theme.weight-semibold;
                    color: Theme.text-secondary;
                    vertical-alignment: center;
                }
            }
        }

        Rectangle {
            clip: true;

            VerticalLayout {
                for entry[idx] in root.item-count: Rectangle {
                    height: 30px;
                    background: idx == root.item-count - 1 ? Theme.surface-hover : Theme.bg-surface;

                    HorizontalLayout {
                        padding-left: Theme.sp-3;
                        padding-right: Theme.sp-3;
                        spacing: Theme.sp-3;

                        Text {
                            text: "\{idx + 1}";
                            font-family: Theme.font-mono;
                            font-size: Theme.text-sm;
                            color: Theme.text-tertiary;
                            vertical-alignment: center;
                            width: 24px;
                            horizontal-alignment: right;
                        }

                        Text {
                            text: idx < root.levels.length ? root.levels[idx] : "";
                            font-family: Theme.font-mono;
                            font-size: Theme.text-xs;
                            font-weight: Theme.weight-medium;
                            color: idx < root.colors.length ? root.colors[idx] : Theme.text-primary;
                            vertical-alignment: center;
                            width: 48px;
                        }

                        Text {
                            text: idx < root.timestamps.length ? root.timestamps[idx] : "";
                            font-family: Theme.font-mono;
                            font-size: Theme.text-sm;
                            color: Theme.text-tertiary;
                            vertical-alignment: center;
                            width: 60px;
                        }

                        Text {
                            text: idx < root.messages.length ? root.messages[idx] : "";
                            font-family: Theme.font-mono;
                            font-size: Theme.text-sm;
                            color: Theme.text-primary;
                            vertical-alignment: center;
                            horizontal-stretch: 1;
                        }
                    }
                }
            }
        }
    }
}
```

#### `MonthCalendar` ([slint/data-display/MonthCalendar.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/MonthCalendar.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component MonthCalendar inherits Rectangle {
    in property <string> month: "July";
    in property <string> year: "2026";
    in-out property <int> selected-day: 23;
    in property <int> current-day: 23;

    in property <[string]> weekdays: ["S", "M", "T", "W", "T", "F", "S"];
    in property <[int]> demo-days: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 0, 0, 0];

    background: Theme.bg-surface;
    border-radius: Theme.radius-card;
    border-width: 1px;
    border-color: Theme.border-base;
    clip: true;

    VerticalLayout {
        padding: Theme.card-padding;
        spacing: Theme.sp-3;

        // Header: month + year + nav arrows
        HorizontalLayout {
            alignment: space-between;
            vertical-stretch: 0;

            Text {
                text: root.month + " " + root.year;
                font-family: Theme.font-ui;
                font-size: Theme.text-xl;
                font-weight: Theme.weight-semibold;
                color: Theme.text-primary;
            }

            HorizontalLayout {
                spacing: Theme.sp-2;
                alignment: center;

                Rectangle {
                    width: 28px;
                    height: 28px;
                    border-radius: Theme.radius-sm;
                    background: touch-left.has-hover ? Theme.surface-hover : #00000000;

                    touch-left := TouchArea {}

                    Text {
                        text: "◀";
                        font-size: Theme.text-sm;
                        color: Theme.text-secondary;
                        horizontal-alignment: center;
                        vertical-alignment: center;
                    }
                }

                Rectangle {
                    width: 28px;
                    height: 28px;
                    border-radius: Theme.radius-sm;
                    background: touch-right.has-hover ? Theme.surface-hover : #00000000;

                    touch-right := TouchArea {}

                    Text {
                        text: "▶";
                        font-size: Theme.text-sm;
                        color: Theme.text-secondary;
                        horizontal-alignment: center;
                        vertical-alignment: center;
                    }
                }
            }
        }

        // Weekday header row
        HorizontalLayout {
            vertical-stretch: 0;

            for day in root.weekdays: Rectangle {
                horizontal-stretch: 1;
                height: 28px;

                Text {
                    text: day;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-xs;
                    font-weight: Theme.weight-medium;
                    color: Theme.text-tertiary;
                    horizontal-alignment: center;
                    vertical-alignment: center;
                    letter-spacing: Theme.tracking-wide;
                }
            }
        }

        // 5 rows x 7 columns grid
        for row_idx in 5: HorizontalLayout {
            vertical-stretch: 1;

            for col_idx in 7: Rectangle {
                horizontal-stretch: 1;

                property <int> cell-index: row_idx * 7 + col_idx;
                property <int> day-val: root.demo-days[cell-index];
                property <bool> is-selected: day-val == root.selected-day && day-val > 0;
                property <bool> is-today: day-val == root.current-day && day-val > 0;

                Rectangle {
                    x: (parent.width - 30px) / 2;
                    y: (parent.height - 30px) / 2;
                    width: 30px;
                    height: 30px;
                    border-radius: Theme.radius-full;
                    background: is-selected ? Theme.accent : #00000000;
                    border-width: is-today && !is-selected ? 1.5px : 0px;
                    border-color: is-today && !is-selected ? Theme.accent : #00000000;

                    Text {
                        text: day-val > 0 ? day-val : "";
                        font-family: Theme.font-ui;
                        font-size: Theme.text-sm;
                        font-weight: is-selected ? Theme.weight-semibold : Theme.weight-medium;
                        color: is-selected ? Theme.on-accent : Theme.text-primary;
                        horizontal-alignment: center;
                        vertical-alignment: center;
                    }
                }
            }
        }
    }
}
```

#### `NestedCommentSection` ([slint/data-display/NestedCommentSection.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/NestedCommentSection.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component NestedCommentSection inherits Rectangle {
    in property <int> item-count: 3;

    background: Theme.bg-surface;
    border-radius: Theme.radius-card;
    border-width: 1px;
    border-color: Theme.border-base;
    clip: true;

    in property <[string]> demo-users: ["Alice", "Bob", "Carol"];
    in property <[string]> demo-texts: ["Great work on this feature!", "Thanks! I'll address the feedback.", "Looks good to me."];
    in property <[string]> demo-times: ["2h ago", "1h ago", "30m ago"];
    in property <[int]> demo-depths: [0, 1, 2];
    in property <[color]> demo-colors: [Theme.accent, Theme.green-500, #CD7F32];

    VerticalLayout {
        padding: Theme.sp-3;
        spacing: Theme.sp-2;

        for cmt[idx] in root.item-count: Rectangle {
            height: 64px;
            x: idx < root.demo_depths.length ? root.demo_depths[idx] * 24px : 0px;

            HorizontalLayout {
                spacing: Theme.sp-2;

                Rectangle {
                    width: 28px;
                    height: 28px;
                    horizontal-stretch: 0;

                    Rectangle {
                        width: 28px;
                        height: 28px;
                        border-radius: Theme.radius-full;
                        background: idx < root.demo_colors.length ? root.demo_colors[idx] : Theme.accent;

                        Text {
                            x: (parent.width - self.width) / 2;
                            y: (parent.height - self.height) / 2;
                            text: idx < root.demo_users.length ? root.demo_users[idx] : "";
                            color: #ffffff;
                            font-family: Theme.font-ui;
                            font-size: 9px;
                            font-weight: Theme.weight-bold;
                            vertical-alignment: center;
                        }
                    }
                }

                VerticalLayout {
                    horizontal-stretch: 1;
                    spacing: Theme.sp-0-5;

                    HorizontalLayout {
                        spacing: Theme.sp-2;

                        Text {
                            text: idx < root.demo_users.length ? root.demo_users[idx] : "";
                            color: Theme.text-primary;
                            font-family: Theme.font-ui;
                            font-size: Theme.text-sm;
                            font-weight: Theme.weight-semibold;
                            vertical-alignment: center;
                        }

                        Text {
                            text: idx < root.demo_times.length ? root.demo_times[idx] : "";
                            color: Theme.text-tertiary;
                            font-family: Theme.font-ui;
                            font-size: Theme.text-xs;
                            vertical-alignment: center;
                        }
                    }

                    Text {
                        text: idx < root.demo_texts.length ? root.demo_texts[idx] : "";
                        color: Theme.text-secondary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-sm;
                        wrap: word-wrap;
                    }

                    Text {
                        text: "Reply";
                        color: Theme.accent;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-xs;
                        font-weight: Theme.weight-medium;
                    }
                }
            }
        }
    }
}
```

#### `NetworkGraph` ([slint/data-display/NetworkGraph.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/NetworkGraph.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component NetworkGraph inherits Rectangle {
    in property <int> node-count: 4;

    background: Theme.bg-surface;
    border-radius: Theme.radius-card;
    border-width: 1px;
    border-color: Theme.border-base;

    in property <[string]> demo-labels: ["A", "B", "C", "D"];

    Rectangle {
        // Horizontal connection line A-B
        x: 80px;
        y: 100px;
        width: 140px;
        height: 2px;
        background: Theme.border-subtle;
    }

    Rectangle {
        // Vertical connection line A-C
        x: 148px;
        y: 50px;
        width: 2px;
        height: 100px;
        background: Theme.border-subtle;
    }

    Rectangle {
        // Horizontal connection line C-D
        x: 148px;
        y: 148px;
        width: 140px;
        height: 2px;
        background: Theme.border-subtle;
    }

    Rectangle {
        // Vertical connection line B-D
        x: 218px;
        y: 100px;
        width: 2px;
        height: 100px;
        background: Theme.border-subtle;
    }

    // Node A
    Rectangle {
        x: 124px;
        y: 74px;
        width: 48px;
        height: 48px;
        border-radius: Theme.radius-full;
        background: Theme.accent;

        Text {
            x: (parent.width - self.width) / 2;
            y: (parent.height - self.height) / 2;
            text: "A";
            color: #ffffff;
            font-family: Theme.font-ui;
            font-size: Theme.text-base;
            font-weight: Theme.weight-bold;
            vertical-alignment: center;
        }
    }

    // Node B
    Rectangle {
        x: 194px;
        y: 74px;
        width: 48px;
        height: 48px;
        border-radius: Theme.radius-full;
        background: Theme.bg-surface;
        border-width: 2px;
        border-color: Theme.border-base;

        Text {
            x: (parent.width - self.width) / 2;
            y: (parent.height - self.height) / 2;
            text: "B";
            color: Theme.text-secondary;
            font-family: Theme.font-ui;
            font-size: Theme.text-base;
            font-weight: Theme.weight-bold;
            vertical-alignment: center;
        }
    }

    // Node C
    Rectangle {
        x: 124px;
        y: 174px;
        width: 48px;
        height: 48px;
        border-radius: Theme.radius-full;
        background: Theme.bg-surface;
        border-width: 2px;
        border-color: Theme.border-base;

        Text {
            x: (parent.width - self.width) / 2;
            y: (parent.height - self.height) / 2;
            text: "C";
            color: Theme.text-secondary;
            font-family: Theme.font-ui;
            font-size: Theme.text-base;
            font-weight: Theme.weight-bold;
            vertical-alignment: center;
        }
    }

    // Node D
    Rectangle {
        x: 194px;
        y: 174px;
        width: 48px;
        height: 48px;
        border-radius: Theme.radius-full;
        background: Theme.bg-surface;
        border-width: 2px;
        border-color: Theme.border-base;

        Text {
            x: (parent.width - self.width) / 2;
            y: (parent.height - self.height) / 2;
            text: "D";
            color: Theme.text-secondary;
            font-family: Theme.font-ui;
            font-size: Theme.text-base;
            font-weight: Theme.weight-bold;
            vertical-alignment: center;
        }
    }
}
```

#### `OrgChart` ([slint/data-display/OrgChart.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/OrgChart.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component OrgChart inherits Rectangle {
    in property <int> item-count: 3;

    background: Theme.bg-surface;
    border-radius: Theme.radius-card;
    border-width: 1px;
    border-color: Theme.border-base;

    in property <[string]> demo-titles: ["CEO", "VP Eng", "VP Design"];
    in property <[int]> demo-levels: [0, 1, 1];
    in property <[color]> demo-colors: [Theme.accent, Theme.green-500, #CD7F32];

    // CEO Node
    Rectangle {
        x: (parent.width - 100px) / 2;
        y: 20px;
        width: 100px;
        height: 40px;
        border-radius: Theme.radius-md;
        background: Theme.accent;
        border-width: 1px;
        border-color: Theme.accent;

        Text {
            x: (parent.width - self.width) / 2;
            y: (parent.height - self.height) / 2;
            text: "CEO";
            color: #ffffff;
            font-family: Theme.font-ui;
            font-size: Theme.text-sm;
            font-weight: Theme.weight-bold;
            vertical-alignment: center;
        }
    }

    // Vertical line from CEO
    Rectangle {
        x: (parent.width) / 2 - 1px;
        y: 60px;
        width: 2px;
        height: 20px;
        background: Theme.border-subtle;
    }

    // Horizontal connector line
    Rectangle {
        x: (parent.width - 140px) / 2;
        y: 78px;
        width: 140px;
        height: 2px;
        background: Theme.border-subtle;
    }

    // VP Eng Node
    Rectangle {
        x: (parent.width - 140px) / 2;
        y: 80px;
        width: 100px;
        height: 40px;
        border-radius: Theme.radius-md;
        background: Theme.bg-surface;
        border-width: 1px;
        border-color: Theme.green-500;

        Text {
            x: (parent.width - self.width) / 2;
            y: (parent.height - self.height) / 2;
            text: "VP Eng";
            color: Theme.green-500;
            font-family: Theme.font-ui;
            font-size: Theme.text-sm;
            font-weight: Theme.weight-semibold;
            vertical-alignment: center;
        }
    }

    // Vertical line from connector to VP Eng
    Rectangle {
        x: (parent.width - 140px) / 2 + 50px - 1px;
        y: 80px;
        width: 2px;
        height: 0px;
        background: Theme.border-subtle;
    }

    // VP Design Node
    Rectangle {
        x: (parent.width + 40px) / 2;
        y: 80px;
        width: 100px;
        height: 40px;
        border-radius: Theme.radius-md;
        background: Theme.bg-surface;
        border-width: 1px;
        border-color: #CD7F32;

        Text {
            x: (parent.width - self.width) / 2;
            y: (parent.height - self.height) / 2;
            text: "VP Design";
            color: #CD7F32;
            font-family: Theme.font-ui;
            font-size: Theme.text-sm;
            font-weight: Theme.weight-semibold;
            vertical-alignment: center;
        }
    }
}
```

#### `PriceComparisonTable` ([slint/data-display/PriceComparisonTable.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/PriceComparisonTable.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component PriceComparisonTable inherits Rectangle {
    in property <[string]> features: [];
    in property <[string]> plans: [];
    in property <[string]> prices: [];
    in property <[bool]> matrix: [];
    in property <int> featured-plan: -1;

    background: Theme.bg-surface;
    border-radius: Theme.radius-card;
    border-width: 1px;
    border-color: Theme.border-base;
    clip: true;

    VerticalLayout {
        Rectangle {
            height: 80px;
            background: Theme.bg-overlay;

            HorizontalLayout {
                Rectangle { width: 160px; horizontal-stretch: 0; }

                for plan[i] in root.plans: Rectangle {
                    horizontal-stretch: 1;
                    background: root.featured-plan == i ? Theme.accent-subtle : transparent;

                    VerticalLayout {
                        alignment: center;

                        Text {
                            x: (parent.width - self.width) / 2;
                            text: plan;
                            color: Theme.text-primary;
                            font-family: Theme.font-ui;
                            font-size: Theme.text-sm;
                            font-weight: Theme.weight-bold;
                            horizontal-alignment: center;
                        }

                        if i < root.prices.length: Text {
                            x: (parent.width - self.width) / 2;
                            text: root.prices[i];
                            color: Theme.accent;
                            font-family: Theme.font-display;
                            font-size: Theme.text-lg;
                            font-weight: Theme.weight-bold;
                            horizontal-alignment: center;
                        }
                    }
                }
            }
        }

        Rectangle { height: 1px; background: Theme.border-subtle; }

        for feat[fi] in root.features: Rectangle {
            height: 40px;
            background: mod(fi, 2) == 1 ? Theme.bg-overlay : transparent;

            HorizontalLayout {
                Rectangle {
                    width: 160px;
                    horizontal-stretch: 0;

                    Text {
                        x: Theme.sp-3;
                        y: (parent.height - self.height) / 2;
                        text: feat;
                        color: Theme.text-primary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-sm;
                        vertical-alignment: center;
                    }
                }

                for plan[pi] in root.plans: Rectangle {
                    horizontal-stretch: 1;
                    background: root.featured-plan == pi ? Theme.accent-subtle : transparent;

                    Text {
                        x: (parent.width - self.width) / 2;
                        y: (parent.height - self.height) / 2;
                        text: (fi * root.plans.length + pi) < root.matrix.length && root.matrix[fi * root.plans.length + pi] ? "✓" : "✗";
                        color: (fi * root.plans.length + pi) < root.matrix.length && root.matrix[fi * root.plans.length + pi] ? Theme.state-success : Theme.text-tertiary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-sm;
                        font-weight: Theme.weight-bold;
                        vertical-alignment: center;
                        horizontal-alignment: center;
                        horizontal-stretch: 1;
                    }
                }
            }
        }
    }
}
```

#### `PriceDisplay` ([slint/data-display/PriceDisplay.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/PriceDisplay.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component PriceDisplay inherits Rectangle {
    in property <string> amount: "";
    in property <string> currency: "$";
    in property <string> period: "";

    background: transparent;

    HorizontalLayout {
        alignment: center;
        spacing: 0px;

        Text {
            text: root.currency;
            color: Theme.text-secondary;
            font-family: Theme.font-ui;
            font-size: Theme.text-lg;
            font-weight: Theme.weight-medium;
            vertical-alignment: top;
        }

        Text {
            text: root.amount;
            color: Theme.text-primary;
            font-family: Theme.font-display;
            font-size: Theme.text-3xl;
            font-weight: Theme.weight-bold;
            vertical-alignment: bottom;
        }

        if root.period != "": Text {
            text: root.period;
            color: Theme.text-tertiary;
            font-family: Theme.font-ui;
            font-size: Theme.text-sm;
            vertical-alignment: bottom;
        }
    }
}
```

#### `PropertyInspectorPanel` ([slint/data-display/PropertyInspectorPanel.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/PropertyInspectorPanel.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component PropertyInspectorPanel inherits Rectangle {
    in property <[string]> keys: [];
    in property <[string]> values: [];

    background: Theme.bg-surface;
    border-radius: Theme.radius-card;
    border-width: 1px;
    border-color: Theme.border-subtle;
    clip: true;

    VerticalLayout {
        padding: Theme.sp-3;

        for item[idx] in root.keys: Rectangle {
            height: 36px;

            HorizontalLayout {
                padding-left: Theme.sp-1;
                padding-right: Theme.sp-1;

                Text {
                    text: item;
                    color: Theme.text-secondary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-sm;
                    font-weight: Theme.weight-medium;
                    vertical-alignment: center;
                    horizontal-stretch: 1;
                }

                Text {
                    text: idx < root.values.length ? root.values[idx] : "";
                    color: Theme.text-primary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-sm;
                    vertical-alignment: center;
                    horizontal-stretch: 1;
                }
            }

            if idx < root.keys.length - 1: Rectangle {
                height: 1px;
                background: Theme.border-subtle;
            }
        }
    }
}
```

#### `ReceiptInvoiceView` ([slint/data-display/ReceiptInvoiceView.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/ReceiptInvoiceView.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component ReceiptInvoiceView inherits Rectangle {
    in property <string> title: "Invoice #1234";
    in property <[string]> item-labels: [];
    in property <[string]> item-amounts: [];
    in property <string> total-label: "Total";
    in property <string> total-amount: "";

    background: Theme.bg-surface;
    border-radius: Theme.radius-card;
    border-width: 1px;
    border-color: Theme.border-subtle;
    clip: true;

    VerticalLayout {
        padding: Theme.sp-5;
        spacing: Theme.sp-3;

        Text {
            text: root.title;
            color: Theme.text-primary;
            font-family: Theme.font-ui;
            font-size: Theme.text-xl;
            font-weight: Theme.weight-bold;
        }

        Rectangle { height: 1px; background: Theme.border-subtle; }

        for item[idx] in root.item-labels: HorizontalLayout {
            height: 32px;

            Text {
                text: item;
                color: Theme.text-primary;
                font-family: Theme.font-ui;
                font-size: Theme.text-sm;
                vertical-alignment: center;
                horizontal-stretch: 1;
            }

            Text {
                text: idx < root.item_amounts.length ? root.item_amounts[idx] : "";
                color: Theme.text-primary;
                font-family: Theme.font-mono;
                font-size: Theme.text-sm;
                vertical-alignment: center;
                horizontal-stretch: 0;
            }
        }

        Rectangle { height: 1px; background: Theme.border-base; }

        HorizontalLayout {
            height: 36px;

            Text {
                text: root.total_label;
                color: Theme.text-primary;
                font-family: Theme.font-ui;
                font-size: Theme.text-base;
                font-weight: Theme.weight-bold;
                vertical-alignment: center;
                horizontal-stretch: 1;
            }

            Text {
                text: root.total_amount;
                color: Theme.accent;
                font-family: Theme.font-mono;
                font-size: Theme.text-base;
                font-weight: Theme.weight-bold;
                vertical-alignment: center;
                horizontal-stretch: 0;
            }
        }
    }
}
```

#### `SelectionList` ([slint/data-display/SelectionList.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/SelectionList.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component SelectionList inherits Rectangle {
    in property <[string]> items: [];
    in property <[bool]> selected: [];

    callback toggle(int);

    background: Theme.bg-surface;
    border-radius: Theme.radius-sm;
    clip: true;

    VerticalLayout {
        for item[idx] in root.items: Rectangle {
            height: 44px;
            background: row-ta.pressed ? Theme.surface-pressed
                : row-ta.has-hover ? Theme.surface-hover
                : #00000000;

            animate background { duration: Theme.dur-fast; }

            HorizontalLayout {
                padding-left: Theme.sp-3;
                padding-right: Theme.sp-4;
                spacing: Theme.sp-3;
                alignment: stretch;

                Rectangle {
                    width: 20px;
                    height: 20px;
                    horizontal-stretch: 0;
                    vertical-stretch: 0;

                    y: (parent.height - self.height) / 2;

                    Rectangle {
                        width: 20px;
                        height: 20px;
                        border-radius: Theme.radius-full;
                        background: idx < root.selected.length && root.selected[idx]
                            ? Theme.accent
                            : #00000000;
                        border-width: 2px;
                        border-color: idx < root.selected.length && root.selected[idx]
                            ? Theme.accent
                            : Theme.border-base;

                        if idx < root.selected.length && root.selected[idx]: Text {
                            x: (parent.width - self.width) / 2;
                            y: (parent.height - self.height) / 2;
                            text: "✓";
                            color: #ffffff;
                            font-size: Theme.text-xs;
                            font-weight: Theme.weight-bold;
                            vertical-alignment: center;
                        }
                    }
                }

                Text {
                    text: root.items[idx];
                    color: Theme.text-primary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-sm;
                    vertical-alignment: center;
                    horizontal-stretch: 1;
                }
            }

            row-ta := TouchArea {
                clicked => { root.toggle(idx); }
            }
        }
    }
}
```

#### `SortableDataTable` ([slint/data-display/SortableDataTable.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/SortableDataTable.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component SortableDataTable inherits Rectangle {
    in property <[string]> columns: [];
    in property <int> row-count: 0;
    in property <int> sort-column: -1;
    in property <bool> sort-ascending: true;

    callback column-clicked(int);

    background: Theme.bg-surface;
    border-radius: Theme.radius-card;
    border-width: 1px;
    border-color: Theme.border-base;
    clip: true;

    VerticalLayout {
        if columns.length > 0: Rectangle {
            height: 40px;
            background: Theme.bg-overlay;

            HorizontalLayout {
                for col[i] in root.columns: Rectangle {
                    horizontal-stretch: 1;
                    height: 40px;

                    HorizontalLayout {
                        padding-left: Theme.sp-3;
                        spacing: Theme.sp-1;
                        alignment: start;

                        Text {
                            text: col;
                            color: root.sort-column == i ? Theme.accent : Theme.text-secondary;
                            font-family: Theme.font-ui;
                            font-size: Theme.text-xs;
                            font-weight: Theme.weight-semibold;
                            vertical-alignment: center;
                        }

                        if root.sort-column == i: Text {
                            text: root.sort-ascending ? "▲" : "▼";
                            color: Theme.accent;
                            font-family: Theme.font-ui;
                            font-size: Theme.text-xs;
                            vertical-alignment: center;
                        }
                    }

                    col-ta := TouchArea {
                        clicked => { root.column-clicked(i); }
                    }
                }
            }
        }

        if columns.length > 0: Rectangle {
            height: 1px;
            background: Theme.border-subtle;
        }

        VerticalLayout {
            for row[rowidx] in root.row-count: Rectangle {
                height: 40px;
                background: row-ta.pressed ? Theme.surface-pressed
                    : row-ta.has-hover ? Theme.surface-hover
                    : transparent;

                animate background { duration: Theme.dur-fast; }

                HorizontalLayout {
                    for col[i] in root.columns: Rectangle {
                        horizontal-stretch: 1;

                        Text {
                            x: Theme.sp-3;
                            y: (parent.height - self.height) / 2;
                            text: "Row \{rowidx + 1} — \{col}";
                            color: Theme.text-primary;
                            font-family: Theme.font-ui;
                            font-size: Theme.text-sm;
                            vertical-alignment: center;
                            horizontal-stretch: 1;
                        }
                    }
                }

                row-ta := TouchArea {}
            }
        }
    }
}
```

#### `SortableList` ([slint/data-display/SortableList.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/SortableList.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component SortableList inherits Rectangle {
    in property <[string]> items: [];

    callback reorder-request(int, int);

    background: Theme.bg-surface;
    border-radius: Theme.radius-sm;
    clip: true;

    VerticalLayout {
        for item[idx] in root.items: Rectangle {
            height: 44px;
            background: row-ta.pressed ? Theme.surface-pressed
                : row-ta.has-hover ? Theme.surface-hover
                : #00000000;

            animate background { duration: Theme.dur-fast; }

            HorizontalLayout {
                padding-left: Theme.sp-3;
                padding-right: Theme.sp-4;
                spacing: Theme.sp-3;
                alignment: stretch;

                Rectangle {
                    width: 28px;
                    height: 28px;
                    horizontal-stretch: 0;

                    handle-ta := TouchArea {
                        clicked => { root.reorder-request(idx, idx + 1); }
                    }

                    Text {
                        x: (parent.width - self.width) / 2;
                        y: (parent.height - self.height) / 2;
                        text: "⠿";
                        color: Theme.text-tertiary;
                        font-size: Theme.text-lg;
                        vertical-alignment: center;
                    }
                }

                Text {
                    text: root.items[idx];
                    color: Theme.text-primary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-sm;
                    vertical-alignment: center;
                    horizontal-stretch: 1;
                }
            }

            row-ta := TouchArea {
                clicked => { root.reorder-request(idx, idx); }
            }
        }
    }
}
```

#### `StatCard` ([slint/data-display/StatCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/StatCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component StatCard inherits Rectangle {
    in property <string> label: "";
    in property <string> value: "";
    in property <string> subtitle: "";
    in property <color> stat-color: Theme.accent;

    min-width: 140px;
    min-height: 80px;
    background: Theme.bg-surface;
    border-radius: Theme.radius-card;
    border-width: 1px;
    border-color: Theme.border-subtle;

    VerticalLayout {
        padding: Theme.card-padding;
        spacing: Theme.sp-2;

        Text {
            text: root.label;
            color: Theme.text-tertiary;
            font-family: Theme.font-ui;
            font-size: Theme.text-xs;
            font-weight: Theme.weight-semibold;
            letter-spacing: Theme.tracking-wide;
        }

        Text {
            text: root.value;
            color: Theme.text-primary;
            font-family: Theme.font-display;
            font-size: Theme.text-3xl;
            font-weight: Theme.weight-bold;
        }

        if root.subtitle != "": Text {
            text: root.subtitle;
            color: stat-color;
            font-family: Theme.font-ui;
            font-size: Theme.text-sm;
            font-weight: Theme.weight-medium;
        }
    }
}
```

#### `SummaryFooterRow` ([slint/data-display/SummaryFooterRow.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/SummaryFooterRow.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component SummaryFooterRow inherits Rectangle {
    in property <[string]> cells: [];
    in property <bool> bold: true;

    height: 40px;
    background: Theme.bg-overlay;
    border-radius: Theme.radius-card;

    HorizontalLayout {
        for cell[i] in root.cells: Rectangle {
            horizontal-stretch: 1;

            Text {
                x: Theme.sp-3;
                y: (parent.height - self.height) / 2;
                text: cell;
                color: Theme.text-primary;
                font-family: Theme.font-ui;
                font-size: Theme.text-sm;
                font-weight: root.bold ? Theme.weight-bold : Theme.weight-medium;
                vertical-alignment: center;
                horizontal-stretch: 1;
            }
        }
    }
}
```

#### `Table` ([slint/data-display/Table.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/Table.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component Table inherits Rectangle {
    in property <[string]> columns: [];
    in property <int> row-count: 0;
    in property <bool> striped: false;
    in property <bool> bordered: false;
    in property <bool> header-visible: true;

    callback row-clicked(int);

    background: Theme.bg-surface;
    border-radius: Theme.radius-card;
    border-width: 1px;
    border-color: Theme.border-base;
    clip: true;

    VerticalLayout {
        // Header
        if header-visible && columns.length > 0: Rectangle {
            height: 36px;
            background: Theme.bg-overlay;

            HorizontalLayout {
                for col[i] in root.columns: Rectangle {
                    horizontal-stretch: 1;

                    Text {
                        x: Theme.sp-3;
                        y: (parent.height - self.height) / 2;
                        text: col;
                        color: Theme.text-secondary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-xs;
                        font-weight: Theme.weight-semibold;
                        vertical-alignment: center;
                        horizontal-stretch: 1;
                    }
                }
            }
        }

        // Divider
        if header-visible && columns.length > 0: Rectangle {
            height: 1px;
            background: Theme.border-subtle;
        }

        // Rows
        VerticalLayout {
            for row[rowidx] in root.row-count: Rectangle {
                height: 40px;
                background: row-ta.pressed ? Theme.surface-pressed
                    : row-ta.has-hover ? Theme.surface-hover
                    : root.striped && mod(rowidx, 2) == 1 ? Theme.bg-overlay
                    : transparent;

                animate background { duration: Theme.dur-fast; }

                HorizontalLayout {
                    for col[i] in root.columns: Rectangle {
                        horizontal-stretch: 1;

                        Rectangle {
                            height: 1px;
                            y: parent.height - 1px;
                            background: root.bordered ? Theme.border-subtle : transparent;
                        }

                        Text {
                            x: Theme.sp-3;
                            y: (parent.height - self.height) / 2;
                            text: "Row \{rowidx + 1} — \{col}";
                            color: Theme.text-primary;
                            font-family: Theme.font-ui;
                            font-size: Theme.text-sm;
                            vertical-alignment: center;
                            horizontal-stretch: 1;
                        }
                    }
                }

                row-ta := TouchArea {
                    clicked => { root.row-clicked(rowidx); }
                }
            }
        }
    }
}
```

#### `TableToolbar` ([slint/data-display/TableToolbar.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/TableToolbar.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component TableToolbar inherits Rectangle {
    in property <string> search-placeholder: "Search...";
    in property <int> result-count: 0;

    height: 48px;
    background: Theme.bg-surface;

    HorizontalLayout {
        padding-left: Theme.sp-3;
        padding-right: Theme.sp-3;
        spacing: Theme.sp-3;

        Rectangle {
            width: 200px;
            height: 32px;
            border-radius: Theme.radius-sm;
            border-width: 1px;
            border-color: Theme.border-base;

            Text {
                x: Theme.sp-2;
                y: (parent.height - self.height) / 2;
                text: root.search_placeholder;
                color: Theme.text-placeholder;
                font-family: Theme.font-ui;
                font-size: Theme.text-sm;
                vertical-alignment: center;
            }
        }

        Rectangle {
            width: 1px;
            horizontal-stretch: 0;
        }

        Text {
            text: "\{root.result_count} results";
            color: Theme.text-tertiary;
            font-family: Theme.font-ui;
            font-size: Theme.text-sm;
            vertical-alignment: center;
            horizontal-stretch: 0;
        }

        Rectangle {
            horizontal-stretch: 1;
        }
    }
}
```

#### `TerminalOutput` ([slint/data-display/TerminalOutput.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/TerminalOutput.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component TerminalOutput inherits Rectangle {
    in property <string> prompt-text: "$";

    background: Theme.dark-mode ? #0D1117 : #1a1a2e;
    border-radius: Theme.radius-card;
    clip: true;

    VerticalLayout {
        padding: Theme.sp-3;
        spacing: Theme.sp-1;

        // Title bar
        Rectangle {
            height: 24px;

            HorizontalLayout {
                spacing: Theme.sp-1-5;
                alignment: start;

                Rectangle {
                    width: 10px;
                    height: 10px;
                    border-radius: Theme.radius-full;
                    background: Theme.red-500;
                }

                Rectangle {
                    width: 10px;
                    height: 10px;
                    border-radius: Theme.radius-full;
                    background: Theme.amber-500;
                }

                Rectangle {
                    width: 10px;
                    height: 10px;
                    border-radius: Theme.radius-full;
                    background: Theme.green-500;
                }
            }
        }

        // Terminal lines
        Rectangle {
            clip: true;

            VerticalLayout {
                padding: 0;
                spacing: Theme.sp-0-5;

                // Line 1: command
                HorizontalLayout {
                    spacing: Theme.sp-2;

                    Text {
                        text: root.prompt-text;
                        font-family: Theme.font-mono;
                        font-size: Theme.text-sm;
                        color: Theme.green-500;
                        vertical-alignment: center;
                    }

                    Text {
                        text: "ls -la";
                        font-family: Theme.font-mono;
                        font-size: Theme.text-sm;
                        color: #F1F2F5;
                        vertical-alignment: center;
                    }
                }

                // Line 2: output
                HorizontalLayout {
                    spacing: Theme.sp-2;

                    Text {
                        text: "      ";
                        font-family: Theme.font-mono;
                        font-size: Theme.text-sm;
                        vertical-alignment: center;
                    }

                    Text {
                        text: "total 48";
                        font-family: Theme.font-mono;
                        font-size: Theme.text-sm;
                        color: #9CA3AF;
                        vertical-alignment: center;
                    }
                }

                // Line 3: command
                HorizontalLayout {
                    spacing: Theme.sp-2;

                    Text {
                        text: root.prompt-text;
                        font-family: Theme.font-mono;
                        font-size: Theme.text-sm;
                        color: Theme.green-500;
                        vertical-alignment: center;
                    }

                    Text {
                        text: "cargo build";
                        font-family: Theme.font-mono;
                        font-size: Theme.text-sm;
                        color: #F1F2F5;
                        vertical-alignment: center;
                    }
                }

                // Line 4: output
                HorizontalLayout {
                    spacing: Theme.sp-2;

                    Text {
                        text: "      ";
                        font-family: Theme.font-mono;
                        font-size: Theme.text-sm;
                        vertical-alignment: center;
                    }

                    Text {
                        text: "Finished dev profile [unoptimized + debuginfo]";
                        font-family: Theme.font-mono;
                        font-size: Theme.text-sm;
                        color: #9CA3AF;
                        vertical-alignment: center;
                    }
                }
            }
        }
    }
}
```

#### `Timeline` ([slint/data-display/Timeline.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/Timeline.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component Timeline inherits Rectangle {
    in property <bool> vertical: true;

    background: transparent;
    horizontal-stretch: 1;
}

export component TimelineItem inherits Rectangle {
    in property <string> title: "";
    in property <string> description: "";
    in property <string> time: "";
    in property <bool> active: false;
    in property <bool> completed: false;

    height: 64px;
    background: transparent;

    HorizontalLayout {
        spacing: Theme.sp-3;

        // Dot + line
        VerticalLayout {
            horizontal-stretch: 0;
            width: 16px;
            spacing: 0px;
            alignment: start;

            Rectangle {
                width: 12px;
                height: 12px;
                border-radius: Theme.radius-full;
                background: completed ? Theme.accent
                    : active ? Theme.accent
                    : Theme.bg-overlay;
                border-width: 2px;
                border-color: completed ? Theme.accent
                    : active ? Theme.accent
                    : Theme.border-base;
                horizontal-stretch: 0;
            }

            Rectangle {
                width: 2px;
                vertical-stretch: 1;
                background: Theme.border-subtle;
                horizontal-stretch: 0;
            }
        }

        // Content
        VerticalLayout {
            horizontal-stretch: 1;
            spacing: Theme.sp-0-5;

            Text {
                text: root.title;
                color: Theme.text-primary;
                font-family: Theme.font-ui;
                font-size: Theme.text-sm;
                font-weight: Theme.weight-medium;
                wrap: word-wrap;
            }

            if description != "": Text {
                text: root.description;
                color: Theme.text-secondary;
                font-family: Theme.font-ui;
                font-size: Theme.text-xs;
                wrap: word-wrap;
            }

            if time != "": Text {
                text: root.time;
                color: Theme.text-tertiary;
                font-family: Theme.font-ui;
                font-size: Theme.text-xs;
            }
        }
    }
}
```

#### `Tooltip` ([slint/data-display/Tooltip.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/Tooltip.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component Tooltip inherits Rectangle {
    visible: false;
    in property <string> text: "";

    width: max(label.preferred-width + Theme.sp-4 * 2, 40px);
    height: 28px;
    visible: root.visible;
    background: transparent;
    z-layer: 999;

    tip := Rectangle {
        border-radius: Theme.radius-tooltip;
        background: Theme.bg-elevated;
        border-width: 1px;
        border-color: Theme.border-subtle;
        drop-shadow-offset-y: 2px;
        drop-shadow-blur: 8px;
        drop-shadow-color: Theme.border-default;

        Text {
            x: (parent.width - self.width) / 2;
            y: (parent.height - self.height) / 2;
            text: root.text;
            color: Theme.text-primary;
            font-family: Theme.font-ui;
            font-size: Theme.text-sm;
            font-weight: Theme.weight-regular;
            horizontal-alignment: center;
            vertical-alignment: center;
        }
    }
}
```

#### `TrailingActionList` ([slint/data-display/TrailingActionList.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/TrailingActionList.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component TrailingActionList inherits Rectangle {
    in property <[string]> titles: [];
    in property <[string]> trailing-icons: [];

    callback action(int);

    background: Theme.bg-surface;
    border-radius: Theme.radius-sm;
    clip: true;

    VerticalLayout {
        for title[idx] in root.titles: Rectangle {
            height: 48px;
            background: row-ta.pressed ? Theme.surface-pressed
                : row-ta.has-hover ? Theme.surface-hover
                : #00000000;

            animate background { duration: Theme.dur-fast; }

            HorizontalLayout {
                padding-left: Theme.sp-4;
                padding-right: Theme.sp-4;
                spacing: Theme.sp-3;
                alignment: stretch;

                Text {
                    text: root.titles[idx];
                    color: Theme.text-primary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-base;
                    vertical-alignment: center;
                    horizontal-stretch: 1;
                }

                Rectangle {
                    width: 32px;
                    height: 32px;
                    horizontal-stretch: 0;

                    icon-ta := TouchArea {
                        clicked => { root.action(idx); }
                    }

                    Rectangle {
                        width: 32px;
                        height: 32px;
                        border-radius: Theme.radius-sm;
                        background: icon-ta.pressed ? Theme.surface-pressed
                            : icon-ta.has-hover ? Theme.surface-hover
                            : #00000000;

                        Text {
                            x: (parent.width - self.width) / 2;
                            y: (parent.height - self.height) / 2;
                            text: root.trailing-icons[idx];
                            color: Theme.text-tertiary;
                            font-size: Theme.text-lg;
                            vertical-alignment: center;
                        }
                    }
                }
            }

            row-ta := TouchArea {
                clicked => { root.action(idx); }
            }
        }
    }
}
```

#### `TreeTable` ([slint/data-display/TreeTable.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/TreeTable.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component TreeTable inherits Rectangle {
    in property <[string]> columns: [];
    in property <int> row-count: 0;
    in property <[int]> depths: [];

    background: Theme.bg-surface;
    border-radius: Theme.radius-card;
    border-width: 1px;
    border-color: Theme.border-base;
    clip: true;

    VerticalLayout {
        if columns.length > 0: Rectangle {
            height: 40px;
            background: Theme.bg-overlay;

            HorizontalLayout {
                Rectangle { width: 24px; horizontal-stretch: 0; }

                for col[i] in root.columns: Rectangle {
                    horizontal-stretch: 1;

                    Text {
                        x: Theme.sp-3;
                        y: (parent.height - self.height) / 2;
                        text: col;
                        color: Theme.text-secondary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-xs;
                        font-weight: Theme.weight-semibold;
                        vertical-alignment: center;
                        horizontal-stretch: 1;
                    }
                }
            }
        }

        Rectangle { height: 1px; background: Theme.border-subtle; }

        VerticalLayout {
            for row[rowidx] in root.row-count: Rectangle {
                height: 36px;
                background: row-ta.pressed ? Theme.surface-pressed
                    : row-ta.has-hover ? Theme.surface-hover
                    : transparent;

                animate background { duration: Theme.dur-fast; }

                HorizontalLayout {
                    Rectangle {
                        width: 24px;
                        horizontal-stretch: 0;

                        Text {
                            x: (parent.width - self.width) / 2;
                            y: (parent.height - self.height) / 2;
                            text: rowidx < root.depths.length && root.depths[rowidx] > 0 ? "├" : " ";
                            color: Theme.text-tertiary;
                            font-family: Theme.font-mono;
                            font-size: Theme.text-xs;
                            vertical-alignment: center;
                        }
                    }

                    Rectangle {
                        width: rowidx < root.depths.length ? root.depths[rowidx] * 16px : 0px;
                        horizontal-stretch: 0;
                    }

                    for col[i] in root.columns: Rectangle {
                        horizontal-stretch: 1;

                        Text {
                            x: Theme.sp-3;
                            y: (parent.height - self.height) / 2;
                            text: "Row \{rowidx + 1} — \{col}";
                            color: Theme.text-primary;
                            font-family: Theme.font-ui;
                            font-size: Theme.text-sm;
                            vertical-alignment: center;
                            horizontal-stretch: 1;
                        }
                    }
                }

                row-ta := TouchArea {}
            }
        }
    }
}
```

#### `TreeView` ([slint/data-display/TreeView.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/TreeView.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component TreeView inherits Rectangle {
    in property <int> item-count: 4;

    callback clicked(int);

    background: Theme.bg-surface;
    border-radius: Theme.radius-card;
    border-width: 1px;
    border-color: Theme.border-base;
    clip: true;

    in property <[string]> demo-names: ["src/", "  main.rs", "  lib.rs", "Cargo.toml"];
    in property <[bool]> demo-is-folder: [true, false, false, false];
    in property <[string]> demo-icons: ["📁", "📄", "📄", "📄"];

    VerticalLayout {
        for item[idx] in root.item-count: Rectangle {
            height: 36px;
            background: item-ta.pressed ? Theme.surface-pressed
                : item-ta.has-hover ? Theme.surface-hover
                : transparent;

            animate background { duration: Theme.dur-fast; }

            HorizontalLayout {
                padding-left: Theme.sp-3;
                padding-right: Theme.sp-3;
                spacing: Theme.sp-2;

                Rectangle {
                    width: 16px;
                    horizontal-stretch: 0;

                    Text {
                        x: (parent.width - self.width) / 2;
                        y: (parent.height - self.height) / 2;
                        text: idx < root.demo_is_folder.length && root.demo_is_folder[idx] ? "▼" : " ";
                        color: Theme.text-tertiary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-xs;
                        vertical-alignment: center;
                    }
                }

                Text {
                    text: idx < root.demo_names.length ? root.demo_names[idx] : "";
                    color: Theme.text-primary;
                    font-family: Theme.font-mono;
                    font-size: Theme.text-sm;
                    vertical-alignment: center;
                    horizontal-stretch: 1;
                }

                Text {
                    text: idx < root.demo_icons.length ? root.demo_icons[idx] : "";
                    font-size: Theme.text-sm;
                    vertical-alignment: center;
                    horizontal-stretch: 0;
                }
            }

            item-ta := TouchArea {
                clicked => { root.clicked(idx); }
            }
        }
    }
}
```

#### `TwoLineList` ([slint/data-display/TwoLineList.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/TwoLineList.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component TwoLineList inherits Rectangle {
    in property <[string]> titles: [];
    in property <[string]> subtitles: [];
    in property <[string]> metas: [];

    callback clicked(int);

    background: Theme.bg-surface;
    border-radius: Theme.radius-sm;
    clip: true;

    VerticalLayout {
        for title[idx] in root.titles: Rectangle {
            height: 56px;
            background: row-ta.pressed ? Theme.surface-pressed
                : row-ta.has-hover ? Theme.surface-hover
                : #00000000;

            animate background { duration: Theme.dur-fast; }

            HorizontalLayout {
                padding-left: Theme.sp-4;
                padding-right: Theme.sp-4;
                spacing: Theme.sp-3;
                alignment: stretch;

                VerticalLayout {
                    horizontal-stretch: 1;
                    spacing: Theme.sp-0-5;
                    alignment: center;

                    Text {
                        text: root.titles[idx];
                        color: Theme.text-primary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-base;
                        font-weight: Theme.weight-semibold;
                        vertical-alignment: center;
                    }

                    Text {
                        text: root.subtitles[idx];
                        color: Theme.text-secondary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-sm;
                        vertical-alignment: center;
                    }
                }

                Text {
                    text: root.metas[idx];
                    color: Theme.text-tertiary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-sm;
                    vertical-alignment: center;
                    horizontal-stretch: 0;
                }
            }

            row-ta := TouchArea {
                clicked => { root.clicked(idx); }
            }
        }
    }
}
```

#### `VirtualizedList` ([slint/data-display/VirtualizedList.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/VirtualizedList.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component VirtualizedList inherits Rectangle {
    in property <int> total-items: 0;
    in property <int> visible-start: 0;
    in property <int> visible-count: 20;

    background: Theme.bg-surface;
    border-radius: Theme.radius-md;
    clip: true;

    in property <int> visible-end: Math.min(visible-start + visible-count, total-items);
    in property <bool> has-scroll-up: visible-start > 0;
    in property <bool> has-scroll-down: visible-end < total-items;

    VerticalLayout {
        Rectangle {
            height: 36px;
            background: Theme.bg-overlay;

            HorizontalLayout {
                padding-left: Theme.sp-4;
                padding-right: Theme.sp-4;
                alignment: start;

                Text {
                    text: "Showing " + (root.visible-start + 1) + "–" + root.visible-end + " of " + root.total-items;
                    color: Theme.text-secondary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-xs;
                    font-weight: Theme.weight-medium;
                    vertical-alignment: center;
                }
            }
        }

        Rectangle {
            height: 1px;
            background: Theme.border-subtle;
        }

        if root.has-scroll-up: Rectangle {
            height: 24px;
            background: Theme.bg-overlay;

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: "▲ Scroll up";
                color: Theme.text-tertiary;
                font-family: Theme.font-ui;
                font-size: Theme.text-xs;
                vertical-alignment: center;
            }
        }

        for item[idx] in root.visible-end - root.visible-start: Rectangle {
            height: 40px;
            background: row-ta.pressed ? Theme.surface-pressed
                : row-ta.has-hover ? Theme.surface-hover
                : #00000000;

            animate background { duration: Theme.dur-fast; }

            HorizontalLayout {
                padding-left: Theme.sp-4;
                padding-right: Theme.sp-4;
                alignment: start;

                Text {
                    text: "Item " + (root.visible-start + idx + 1);
                    color: Theme.text-primary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-sm;
                    vertical-alignment: center;
                }
            }

            row-ta := TouchArea { }
        }

        if root.has-scroll-down: Rectangle {
            height: 24px;
            background: Theme.bg-overlay;

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: "▼ Scroll down";
                color: Theme.text-tertiary;
                font-family: Theme.font-ui;
                font-size: Theme.text-xs;
                vertical-alignment: center;
            }
        }
    }
}
```

#### `VirtualizedTable` ([slint/data-display/VirtualizedTable.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/VirtualizedTable.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component VirtualizedTable inherits Rectangle {
    in property <[string]> columns: [];
    in property <int> total-rows: 100;
    in property <int> visible-start: 0;
    in property <int> visible-count: 20;

    background: Theme.bg-surface;
    border-radius: Theme.radius-card;
    border-width: 1px;
    border-color: Theme.border-base;
    clip: true;

    VerticalLayout {
        Rectangle {
            height: 32px;
            background: Theme.bg-overlay;

            HorizontalLayout {
                padding-left: Theme.sp-3;
                padding-right: Theme.sp-3;

                Text {
                    text: "Showing \{root.visible_start + 1}–\{root.visible_start + root.visible_count} of \{root.total_rows} rows";
                    color: Theme.text-tertiary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-xs;
                    vertical-alignment: center;
                }
            }
        }

        if columns.length > 0: Rectangle {
            height: 36px;
            background: Theme.bg-overlay;

            HorizontalLayout {
                for col[i] in root.columns: Rectangle {
                    horizontal-stretch: 1;

                    Text {
                        x: Theme.sp-3;
                        y: (parent.height - self.height) / 2;
                        text: col;
                        color: Theme.text-secondary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-xs;
                        font-weight: Theme.weight-semibold;
                        vertical-alignment: center;
                        horizontal-stretch: 1;
                    }
                }
            }
        }

        Rectangle { height: 1px; background: Theme.border-subtle; }

        VerticalLayout {
            for row[rowidx] in root.visible-count: Rectangle {
                height: 36px;
                background: row-ta.pressed ? Theme.surface-pressed
                    : row-ta.has-hover ? Theme.surface-hover
                    : transparent;

                animate background { duration: Theme.dur-fast; }

                HorizontalLayout {
                    for col[i] in root.columns: Rectangle {
                        horizontal-stretch: 1;

                        Text {
                            x: Theme.sp-3;
                            y: (parent.height - self.height) / 2;
                            text: "Row \{root.visible_start + rowidx + 1} — \{col}";
                            color: Theme.text-primary;
                            font-family: Theme.font-ui;
                            font-size: Theme.text-sm;
                            vertical-alignment: center;
                            horizontal-stretch: 1;
                        }
                    }
                }

                row-ta := TouchArea {}
            }
        }
    }
}
```

#### `WeekCalendar` ([slint/data-display/WeekCalendar.slint](file:///home/lion/Documents/GitHub/ltk/slint/data-display/WeekCalendar.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component WeekCalendar inherits Rectangle {
    in-out property <int> selected-day: 2;

    in property <[string]> day-headers: ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    in property <[string]> time-slots: ["9 AM", "10 AM", "11 AM"];

    background: Theme.bg-surface;
    border-radius: Theme.radius-card;
    border-width: 1px;
    border-color: Theme.border-base;
    clip: true;

    VerticalLayout {
        padding: Theme.card-padding;
        spacing: Theme.sp-3;

        // Day column headers
        HorizontalLayout {
            spacing: Theme.sp-1;
            vertical-stretch: 0;

            // Time gutter placeholder
            Rectangle {
                width: 60px;
            }

            for hdr_idx in 7: Rectangle {
                horizontal-stretch: 1;
                height: 32px;
                border-radius: Theme.radius-sm;
                background: hdr_idx == root.selected-day ? Theme.accent : #00000000;

                Text {
                    text: root.day-headers[hdr_idx];
                    font-family: Theme.font-ui;
                    font-size: Theme.text-xs;
                    font-weight: Theme.weight-semibold;
                    color: hdr_idx == root.selected-day ? Theme.on-accent : Theme.text-secondary;
                    horizontal-alignment: center;
                    vertical-alignment: center;
                    letter-spacing: Theme.tracking-wide;
                }
            }
        }

        // Time slot rows
        for time_idx in 3: HorizontalLayout {
            spacing: Theme.sp-1;
            vertical-stretch: 1;

            // Time label
            Rectangle {
                width: 60px;

                Text {
                    text: root.time-slots[time_idx];
                    font-family: Theme.font-ui;
                    font-size: Theme.text-xs;
                    color: Theme.text-tertiary;
                    horizontal-alignment: right;
                    vertical-alignment: top;
                }
            }

            // 7 day columns per row
            for day_col in 7: Rectangle {
                horizontal-stretch: 1;
                border-width: 1px;
                border-color: Theme.border-subtle;
                border-radius: Theme.radius-xs;

                // Event on Wed (index 2) at 10 AM (index 1)
                visible: day_col == 2 && time_idx == 1;

                Rectangle {
                    x: 4px;
                    y: 4px;
                    width: parent.width - 8px;
                    height: parent.height - 8px;
                    border-radius: Theme.radius-xs;
                    background: Theme.accent-subtle;

                    Rectangle {
                        x: 0px;
                        y: 0px;
                        width: 3px;
                        height: parent.height;
                        border-radius: 1px;
                        background: Theme.accent;
                    }

                    Text {
                        x: 10px;
                        y: 4px;
                        text: "Meeting";
                        font-family: Theme.font-ui;
                        font-size: Theme.text-xs;
                        font-weight: Theme.weight-medium;
                        color: Theme.text-primary;
                    }
                }
            }
        }
    }
}
```


### 5.4 Feedback, Overlays, and Status Indicators Code Manual

#### `AboutDialog` ([slint/feedback/AboutDialog.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/AboutDialog.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component AboutDialog inherits Rectangle {
    in property <image> icon-source: @image-url("");
    in property <string> name: "";
    in property <string> description: "";
    in property <string> link: "https://github.com/lionxlover/ltk";
    in property <bool> active: false;

    callback link-clicked(string);

    width: 340px;
    visible: active;
    background: Theme.bg-surface;
    border-radius: Theme.radius-dialog;
    border-width: 1px;
    border-color: Theme.border-subtle;

    drop-shadow-offset-x: 0px;
    drop-shadow-offset-y: Theme.elevation-2-y;
    drop-shadow-blur: Theme.elevation-2-blur;
    drop-shadow-color: Theme.elevation-2-color;

    VerticalLayout {
        padding: Theme.sp-8;
        spacing: 0px;
        alignment: center;

        Rectangle { vertical-stretch: 1; }

        Image {
            source: root.icon-source;
            width: 72px;
            height: 72px;
            image-fit: contain;
            horizontal-stretch: 0;
        }

        Rectangle { height: Theme.sp-4; vertical-stretch: 0; }

        Text {
            text: root.name;
            color: Theme.text-primary;
            font-family: Theme.font-display;
            font-size: Theme.text-lg;
            font-weight: Theme.weight-bold;
            horizontal-alignment: center;
            horizontal-stretch: 1;
        }

        Rectangle { height: Theme.sp-2; vertical-stretch: 0; }

        Text {
            text: root.description;
            color: Theme.text-secondary;
            font-family: Theme.font-ui;
            font-size: Theme.text-base;
            horizontal-alignment: center;
            horizontal-stretch: 1;
            wrap: word-wrap;
        }

        Rectangle { height: Theme.sp-4; vertical-stretch: 0; }

        TouchArea {
            horizontal-stretch: 1;
            clicked => { root.link-clicked(root.link); }

            Text {
                text: root.link;
                color: Theme.text-link;
                font-family: Theme.font-ui;
                font-size: Theme.text-base;
                horizontal-alignment: center;
            }
        }

        Rectangle { vertical-stretch: 1; }
    }

    ta := TouchArea { }
}
```

#### `ActionToast` ([slint/feedback/ActionToast.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/ActionToast.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component ActionToast inherits Rectangle {
    in property <string> message: "";
    in property <string> action-text: "";
    in property <int> kind: 0;

    callback action-clicked();
    callback dismiss();

    min-height: 48px;
    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: kind == 3 ? Theme.state-error.with-alpha(0.25) : (kind == 2 ? Theme.state-warning.with-alpha(0.25) : (kind == 1 ? Theme.state-success.with-alpha(0.25) : Theme.state-info.with-alpha(0.25)));

    drop-shadow-offset-x: 0px;
    drop-shadow-offset-y: Theme.elevation-2-y;
    drop-shadow-blur: Theme.elevation-2-blur;
    drop-shadow-color: Theme.elevation-2-color;

    HorizontalLayout {
        padding: Theme.sp-3;
        spacing: Theme.sp-3;
        alignment: stretch;

        Rectangle {
            width: 3px;
            horizontal-stretch: 0;
            border-radius: 2px;
            background: kind == 3 ? Theme.state-error : (kind == 2 ? Theme.state-warning : (kind == 1 ? Theme.state-success : Theme.state-info));
        }

        Text {
            text: root.message;
            color: Theme.text-primary;
            font-family: Theme.font-ui;
            font-size: Theme.text-sm;
            vertical-alignment: center;
            horizontal-stretch: 1;
            wrap: word-wrap;
        }

        if action-text != "": Rectangle {
            horizontal-stretch: 0;
            height: 28px;
            border-radius: Theme.radius-sm;
            background: action-ta.pressed ? Theme.btn-primary-bg-pressed : (action-ta.has-hover ? Theme.btn-primary-bg-hover : Theme.btn-primary-bg);

            animate background { duration: Theme.dur-fast; easing: Theme.ease-standard; }

            action-ta := TouchArea {
                clicked => { root.action-clicked(); }
            }

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: root.action-text;
                color: Theme.on-accent;
                font-family: Theme.font-ui;
                font-size: Theme.text-xs;
                font-weight: Theme.weight-semibold;
                vertical-alignment: center;
            }
        }

        Rectangle {
            width: 24px;
            height: 24px;
            horizontal-stretch: 0;

            dismiss-bg := Rectangle {
                border-radius: Theme.radius-xs;
                background: dismiss-ta.pressed ? Theme.surface-pressed : (dismiss-ta.has-hover ? Theme.surface-hover : #00000000);
            }

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: "✕";
                color: Theme.text-tertiary;
                font-size: Theme.text-xs;
                vertical-alignment: center;
            }

            dismiss-ta := TouchArea {
                clicked => { root.dismiss(); }
            }
        }
    }
}
```

#### `Alert` ([slint/feedback/Alert.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/Alert.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component Alert inherits Rectangle {
    in property <string> title: "";
    in property <string> text: "";
    in property <string> kind: "info"; // info, success, warning, error
    in property <bool> show-icon: true;

    callback close();

    min-height: 44px;
    background: kind == "error" ? Theme.state-error.with-alpha(0.12) : (kind == "warning" ? Theme.state-warning.with-alpha(0.12) : (kind == "success" ? Theme.state-success.with-alpha(0.12) : Theme.state-info.with-alpha(0.12)));
    border-radius: Theme.radius-md;
    border-width: 1px;
    border-color: kind == "error" ? Theme.state-error.with-alpha(0.25) : (kind == "warning" ? Theme.state-warning.with-alpha(0.25) : (kind == "success" ? Theme.state-success.with-alpha(0.25) : Theme.state-info.with-alpha(0.25)));

    HorizontalLayout {
        padding: Theme.sp-3;
        spacing: Theme.sp-3;
        alignment: stretch;

        // Icon
        if show-icon: Text {
            y: (parent.height - self.height) / 2;
            text: kind == "error" ? "✕" : (kind == "warning" ? "⚠" : (kind == "success" ? "✓" : "ℹ"));
            color: kind == "error" ? Theme.state-error : (kind == "warning" ? Theme.state-warning : (kind == "success" ? Theme.state-success : Theme.state-info));
            font-size: Theme.text-lg;
            vertical-alignment: center;
            horizontal-alignment: center;
            horizontal-stretch: 0;
        }

        // Content
        VerticalLayout {
            horizontal-stretch: 1;
            alignment: center;
            spacing: Theme.sp-1;

            if title != "": Text {
                text: root.title;
                color: Theme.text-primary;
                font-family: Theme.font-ui;
                font-size: Theme.text-base;
                font-weight: Theme.weight-semibold;
                wrap: word-wrap;
            }

            if text != "": Text {
                text: root.text;
                color: Theme.text-secondary;
                font-family: Theme.font-ui;
                font-size: Theme.text-sm;
                wrap: word-wrap;
            }
        }

        // Close button
        Rectangle {
            width: 24px;
            height: 24px;
            y: (parent.height - self.height) / 2;
            horizontal-stretch: 0;
            vertical-stretch: 0;
            background: transparent;

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: "✕";
                color: Theme.text-tertiary;
                font-size: Theme.text-sm;
                vertical-alignment: center;
                horizontal-alignment: center;
            }

            TouchArea {
                clicked => { root.close(); }
            }
        }
    }
}
```

#### `BusyIndicator` ([slint/feedback/BusyIndicator.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/BusyIndicator.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component BusyIndicator inherits Rectangle {
    in property <bool> running: true;

    visible: running;

    Image {
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
        width: parent.width;
        height: parent.height;
        source: @image-url("../../src/images/refresh.svg");
        image-fit: contain;
        colorize: Theme.text-tertiary;
        opacity: running ? 1.0 : 0.3;
        animate opacity { duration: Theme.dur-normal; easing: Theme.ease-standard; }
    }
}
```

#### `Callout` ([slint/feedback/Callout.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/Callout.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component Callout inherits Rectangle {
    in property <string> title: "";
    in property <string> text: "";
    in property <int> kind: 0;

    min-height: 56px;
    background: kind == 3 ? Theme.state-error.with-alpha(0.08) : (kind == 2 ? Theme.state-warning.with-alpha(0.08) : (kind == 1 ? Theme.state-success.with-alpha(0.08) : Theme.state-info.with-alpha(0.08)));
    border-radius: Theme.radius-md;

    HorizontalLayout {
        padding: Theme.sp-4;
        spacing: Theme.sp-3;
        alignment: stretch;

        Rectangle {
            width: 4px;
            horizontal-stretch: 0;
            border-radius: 2px;
            background: kind == 3 ? Theme.state-error : (kind == 2 ? Theme.state-warning : (kind == 1 ? Theme.state-success : Theme.state-info));
        }

        VerticalLayout {
            horizontal-stretch: 1;
            spacing: Theme.sp-1;

            if title != "": Text {
                text: root.title;
                color: Theme.text-primary;
                font-family: Theme.font-ui;
                font-size: Theme.text-base;
                font-weight: Theme.weight-semibold;
                wrap: word-wrap;
            }

            if text != "": Text {
                text: root.text;
                color: Theme.text-secondary;
                font-family: Theme.font-ui;
                font-size: Theme.text-sm;
                wrap: word-wrap;
            }
        }
    }
}
```

#### `CircularProgress` ([slint/feedback/CircularProgress.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/CircularProgress.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component CircularProgress inherits Rectangle {
    in property <float> value: 0;
    in property <float> max: 100;
    in property <length> size: 48px;

    in property <color> progress-color: Theme.accent;
    in property <color> track-color: Theme.bg-overlay;

    width: size * 1px;
    height: size * 1px;
    background: #00000000;

    Rectangle {
        width: parent.width;
        height: parent.height;
        border-radius: Theme.radius-full;
        border-width: 4px;
        border-color: track-color;
        background: #00000000;
    }

    Rectangle {
        width: parent.width;
        height: parent.height;
        border-radius: Theme.radius-full;
        border-width: 4px;
        border-color: progress-color;
        background: #00000000;
        visible: value > 0;
    }

    center-text := Text {
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
        text: Math.round(value) + "%";
        color: Theme.text-primary;
        font-family: Theme.font-ui;
        font-size: Theme.text-xs;
        font-weight: Theme.weight-semibold;
        horizontal-alignment: center;
        vertical-alignment: center;
    }
}
```

#### `ConfettiBurst` ([slint/feedback/ConfettiBurst.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/ConfettiBurst.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component ConfettiBurst inherits Rectangle {
    background: #00000000;

    for i in 20: Rectangle {
        x: (mod(i * 37, 100)) * 1%;
        y: (mod(i * 53, 100)) * 1%;
        width: 6px;
        height: 6px;
        background: i == 0 ? Theme.accent : (i == 1 ? Theme.error : (i == 2 ? Theme.success : (i == 3 ? Theme.warning : Theme.accent)));
        border-radius: 3px;
        opacity: 0.8;
        rotate-angle: i * 18;
    }
}
```

#### `Dialog` ([slint/feedback/Dialog.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/Dialog.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component Dialog inherits Rectangle {
    in property <string> title: "";
    in property <string> text: "";
    in property <string> left-button-text: "";
    in property <string> right-button-text: "";
    in property <bool> active: false;

    callback left-button-clicked();
    callback right-button-clicked();

    width: 480px;
    min-height: 200px;
    visible: active;
    background: Theme.bg-surface;
    border-radius: Theme.radius-dialog;
    border-width: 1px;
    border-color: Theme.border-subtle;

    drop-shadow-offset-x: 0px;
    drop-shadow-offset-y: Theme.elevation-2-y;
    drop-shadow-blur: Theme.elevation-2-blur;
    drop-shadow-color: Theme.elevation-2-color;

    VerticalLayout {
        padding: Theme.sp-6;
        spacing: 0px;

        Text {
            text: root.title;
            color: Theme.text-primary;
            font-family: Theme.font-ui;
            font-size: Theme.text-xl;
            font-weight: Theme.weight-bold;
            wrap: no-wrap;
        }

        Rectangle { height: Theme.sp-2; vertical-stretch: 0; }

        Text {
            text: root.text;
            color: Theme.text-secondary;
            font-family: Theme.font-ui;
            font-size: Theme.text-base;
            wrap: word-wrap;
            vertical-stretch: 1;
        }

        Rectangle { vertical-stretch: 1; }
        Rectangle { height: Theme.sp-4; vertical-stretch: 0; }

        HorizontalLayout {
            spacing: Theme.sp-3;
            alignment: end;
            height: Theme.button-height-md;

            if left-button-text != "": Rectangle {
                horizontal-stretch: 1;
                min-width: 100px;
                height: Theme.button-height-md;
                border-radius: Theme.radius-button;
                border-width: 1px;
                border-color: Theme.border-base;
                background: left-ta.has-hover ? Theme.surface-hover : Theme.bg-raised;

                animate background { duration: Theme.dur-fast; easing: Theme.ease-standard; }

                left-ta := TouchArea {
                    clicked => { root.left-button-clicked(); }
                }

                Text {
                    text: root.left-button-text;
                    color: Theme.text-primary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-base;
                    font-weight: Theme.weight-medium;
                    horizontal-alignment: center;
                    vertical-alignment: center;
                }
            }

            if right-button-text != "": Rectangle {
                horizontal-stretch: 1;
                min-width: 100px;
                height: Theme.button-height-md;
                border-radius: Theme.radius-button;
                background: right-ta.has-hover ? Theme.btn-primary-bg-hover : Theme.btn-primary-bg;

                animate background { duration: Theme.dur-fast; easing: Theme.ease-standard; }

                right-ta := TouchArea {
                    clicked => { root.right-button-clicked(); }
                }

                Text {
                    text: root.right-button-text;
                    color: Theme.on-accent;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-base;
                    font-weight: Theme.weight-medium;
                    horizontal-alignment: center;
                    vertical-alignment: center;
                }
            }
        }
    }

    ta := TouchArea { }
}
```

#### `DotsLoader` ([slint/feedback/DotsLoader.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/DotsLoader.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component DotsLoader inherits Rectangle {
    in property <length> size: 16px;
    in property <color> dot-color: Theme.accent;

    width: size * 3px;
    height: size * 1px;
    background: #00000000;

    dot1 := Rectangle {
        x: 0px;
        y: (parent.height - self.height) / 2;
        width: size * 1px;
        height: size * 1px;
        border-radius: Theme.radius-full;
        background: dot-color;
        opacity: Math.mod(animation-tick() + 0ms, 1200ms) < 600ms ? 1.0 : 0.3;
        animate opacity { duration: 600ms; easing: Theme.ease-in-out; }
    }

    dot2 := Rectangle {
        x: parent.width * 0.33;
        y: (parent.height - self.height) / 2;
        width: size * 1px;
        height: size * 1px;
        border-radius: Theme.radius-full;
        background: dot-color;
        opacity: Math.mod(animation-tick() + 400ms, 1200ms) < 600ms ? 1.0 : 0.3;
        animate opacity { duration: 600ms; easing: Theme.ease-in-out; }
    }

    dot3 := Rectangle {
        x: parent.width * 0.66;
        y: (parent.height - self.height) / 2;
        width: size * 1px;
        height: size * 1px;
        border-radius: Theme.radius-full;
        background: dot-color;
        opacity: Math.mod(animation-tick() + 800ms, 1200ms) < 600ms ? 1.0 : 0.3;
        animate opacity { duration: 600ms; easing: Theme.ease-in-out; }
    }
}
```

#### `EmptyState` ([slint/feedback/EmptyState.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/EmptyState.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component EmptyState inherits Rectangle {
    in property <string> title: "Nothing here";
    in property <string> description: "No data to display yet.";

    callback action();

    background: Theme.bg-base;
    horizontal-stretch: 1;

    VerticalLayout {
        alignment: center;
        spacing: 12px;

        Rectangle {
            width: 64px;
            height: 64px;
            background: Theme.bg-overlay;
            border-radius: 32px;

            FaIcon {
                source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/inbox.svg");
                fa-size: 28px;
                fa-color: Theme.text-tertiary;
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
            }
        }

        Text {
            text: root.title;
            color: Theme.text-primary;
            font-size: 16px;
            font-weight: Theme.weight-semibold;
            horizontal-alignment: center;
        }

        Text {
            text: root.description;
            color: Theme.text-secondary;
            font-size: 13px;
            horizontal-alignment: center;
            wrap: word-wrap;
        }
    }
}
```

#### `ErrorState` ([slint/feedback/ErrorState.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/ErrorState.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component ErrorState inherits Rectangle {
    in property <string> message: "Something went wrong";

    callback retry();

    background: Theme.bg-base;
    horizontal-stretch: 1;

    VerticalLayout {
        alignment: center;
        spacing: 12px;

        Rectangle {
            width: 64px;
            height: 64px;
            background: Theme.error;
            border-radius: 32px;

            FaIcon {
                source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/triangle-exclamation.svg");
                fa-size: 28px;
                fa-color: Theme.error;
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
            }
        }

        Text {
            text: "Error";
            color: Theme.text-primary;
            font-size: 16px;
            font-weight: Theme.weight-semibold;
            horizontal-alignment: center;
        }

        Text {
            text: root.message;
            color: Theme.text-secondary;
            font-size: 13px;
            horizontal-alignment: center;
            wrap: word-wrap;
        }

        Rectangle {
            height: 36px;
            background: Theme.accent;
            border-radius: Theme.radius-md;
            horizontal-stretch: 0;

            Text {
                x: (parent.width - self.width) / 2;
                text: "Retry";
                color: #ffffff;
                font-size: 13px;
                font-weight: Theme.weight-semibold;
                vertical-alignment: center;
            }

            TouchArea { clicked => { root.retry(); } }
        }
    }
}
```

#### `IndeterminateProgress` ([slint/feedback/IndeterminateProgress.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/IndeterminateProgress.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component IndeterminateProgress inherits Rectangle {
    in property <color> bar-color: Theme.accent;
    in property <color> track-color: Theme.bg-overlay;

    height: 4px;
    background: #00000000;

    Rectangle {
        border-radius: Theme.radius-full;
        background: track-color;
    }

    pulse := Rectangle {
        x: -parent.width * 0.4;
        y: 0;
        width: parent.width * 0.4;
        height: parent.height;
        border-radius: Theme.radius-full;
        background: bar-color;
        animate x { duration: 1400ms; easing: Theme.ease-in-out; loop-count: -1; }
    }
}
```

#### `IndeterminateSpinner` ([slint/feedback/IndeterminateSpinner.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/IndeterminateSpinner.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component IndeterminateSpinner inherits Rectangle {
    in property <length> size: 24px;
    in property <color> spinner-color: Theme.accent;
    in property <length> stroke-width: 3px;

    width: size * 1px;
    height: size * 1px;
    background: #00000000;

    Rectangle {
        width: parent.width;
        height: parent.height;
        border-radius: Theme.radius-full;
        border-width: stroke-width;
        border-color: spinner-color.with-alpha(0.15);
        background: #00000000;
    }

    Rectangle {
        width: parent.width;
        height: parent.height;
        border-radius: Theme.radius-full;
        border-width: stroke-width;
        border-color: spinner-color;
        background: #00000000;
        transform-rotation: Math.mod(animation-tick(), 1s) / 1s * 360 * 1deg;
    }
}
```

#### `MultilineToast` ([slint/feedback/MultilineToast.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/MultilineToast.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component MultilineToast inherits Rectangle {
    in property <string> title: "";
    in property <string> message: "";
    in property <int> kind: 0;

    callback dismiss();

    min-height: 72px;
    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: kind == 3 ? Theme.state-error.with-alpha(0.25) : (kind == 2 ? Theme.state-warning.with-alpha(0.25) : (kind == 1 ? Theme.state-success.with-alpha(0.25) : Theme.state-info.with-alpha(0.25)));

    drop-shadow-offset-x: 0px;
    drop-shadow-offset-y: Theme.elevation-2-y;
    drop-shadow-blur: Theme.elevation-2-blur;
    drop-shadow-color: Theme.elevation-2-color;

    HorizontalLayout {
        padding: Theme.sp-4;
        spacing: Theme.sp-3;
        alignment: stretch;

        Rectangle {
            width: 3px;
            horizontal-stretch: 0;
            border-radius: 2px;
            background: kind == 3 ? Theme.state-error : (kind == 2 ? Theme.state-warning : (kind == 1 ? Theme.state-success : Theme.state-info));
        }

        VerticalLayout {
            horizontal-stretch: 1;
            spacing: Theme.sp-1;

            if title != "": Text {
                text: root.title;
                color: Theme.text-primary;
                font-family: Theme.font-ui;
                font-size: Theme.text-base;
                font-weight: Theme.weight-semibold;
                wrap: word-wrap;
            }

            if message != "": Text {
                text: root.message;
                color: Theme.text-secondary;
                font-family: Theme.font-ui;
                font-size: Theme.text-sm;
                wrap: word-wrap;
            }
        }

        Rectangle {
            width: 24px;
            height: 24px;
            horizontal-stretch: 0;

            dismiss-bg := Rectangle {
                border-radius: Theme.radius-xs;
                background: dismiss-ta.pressed ? Theme.surface-pressed : (dismiss-ta.has-hover ? Theme.surface-hover : #00000000);
            }

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: "✕";
                color: Theme.text-tertiary;
                font-size: Theme.text-xs;
                vertical-alignment: center;
            }

            dismiss-ta := TouchArea {
                clicked => { root.dismiss(); }
            }
        }
    }
}
```

#### `NoResultsState` ([slint/feedback/NoResultsState.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/NoResultsState.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component NoResultsState inherits Rectangle {
    in property <string> query: "";

    background: Theme.bg-base;
    horizontal-stretch: 1;

    VerticalLayout {
        alignment: center;
        spacing: 12px;

        Rectangle {
            width: 64px;
            height: 64px;
            background: Theme.bg-overlay;
            border-radius: 32px;

            FaIcon {
                source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/magnifying-glass.svg");
                fa-size: 28px;
                fa-color: Theme.text-tertiary;
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
            }
        }

        Text {
            text: "No results found";
            color: Theme.text-primary;
            font-size: 16px;
            font-weight: Theme.weight-semibold;
            horizontal-alignment: center;
        }

        Text {
            text: root.query != "" ? "No results for \"{root.query}\"" : "Try a different search term";
            color: Theme.text-secondary;
            font-size: 13px;
            horizontal-alignment: center;
            wrap: word-wrap;
        }
    }
}
```

#### `Notification` ([slint/feedback/Notification.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/Notification.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component Notification inherits Rectangle {
    in property <string> title: "";
    in property <string> text: "";
    in property <string> kind: "info"; // info, success, warning, error
    in property <bool> show-icon: true;

    callback close();

    min-height: 48px;
    background: Theme.bg-elevated;
    border-radius: Theme.radius-md;
    border-width: 1px;
    border-color: Theme.border-subtle;

    drop-shadow-offset-y: 4px;
    drop-shadow-blur: 12px;
    drop-shadow-color: Theme.border-default;

    HorizontalLayout {
        padding: Theme.sp-3;
        spacing: Theme.sp-3;
        alignment: stretch;

        // Left accent bar
        Rectangle {
            width: 3px;
            horizontal-stretch: 0;
            border-radius: 2px;
            background: kind == "error" ? Theme.state-error : (kind == "warning" ? Theme.state-warning : (kind == "success" ? Theme.state-success : Theme.state-info));
        }

        // Icon
        if show-icon: Text {
            text: kind == "error" ? "✕" : (kind == "warning" ? "⚠" : (kind == "success" ? "✓" : "ℹ"));
            color: kind == "error" ? Theme.state-error : (kind == "warning" ? Theme.state-warning : (kind == "success" ? Theme.state-success : Theme.state-info));
            font-size: Theme.text-lg;
            vertical-alignment: center;
            horizontal-stretch: 0;
        }

        // Content
        VerticalLayout {
            horizontal-stretch: 1;
            spacing: Theme.sp-0-5;

            if title != "": Text {
                text: root.title;
                color: Theme.text-primary;
                font-family: Theme.font-ui;
                font-size: Theme.text-sm;
                font-weight: Theme.weight-semibold;
                wrap: word-wrap;
            }

            if text != "": Text {
                text: root.text;
                color: Theme.text-secondary;
                font-family: Theme.font-ui;
                font-size: Theme.text-xs;
                wrap: word-wrap;
            }
        }

        // Close
        Rectangle {
            width: 24px;
            height: 24px;
            horizontal-stretch: 0;

            close-bg := Rectangle {
                border-radius: Theme.radius-xs;
                background: close-ta.pressed ? Theme.surface-pressed
                    : close-ta.has-hover ? Theme.surface-hover
                    : transparent;
            }

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: "✕";
                color: Theme.text-tertiary;
                font-size: Theme.text-xs;
                vertical-alignment: center;
            }

            close-ta := TouchArea {
                clicked => { root.close(); }
            }
        }
    }
}
```

#### `OfflineState` ([slint/feedback/OfflineState.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/OfflineState.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component OfflineState inherits Rectangle {
    in property <string> message: "No internet connection";

    background: Theme.bg-base;
    horizontal-stretch: 1;

    VerticalLayout {
        alignment: center;
        spacing: 12px;

        Rectangle {
            width: 64px;
            height: 64px;
            background: Theme.bg-overlay;
            border-radius: 32px;

            FaIcon {
                source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/wifi-slash.svg");
                fa-size: 28px;
                fa-color: Theme.text-tertiary;
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
            }
        }

        Text {
            text: "Offline";
            color: Theme.text-primary;
            font-size: 16px;
            font-weight: Theme.weight-semibold;
            horizontal-alignment: center;
        }

        Text {
            text: root.message;
            color: Theme.text-secondary;
            font-size: 13px;
            horizontal-alignment: center;
            wrap: word-wrap;
        }
    }
}
```

#### `PermissionDeniedState` ([slint/feedback/PermissionDeniedState.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/PermissionDeniedState.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component PermissionDeniedState inherits Rectangle {
    in property <string> message: "You don't have permission to access this";

    background: Theme.bg-base;
    horizontal-stretch: 1;

    VerticalLayout {
        alignment: center;
        spacing: 12px;

        Rectangle {
            width: 64px;
            height: 64px;
            background: Theme.bg-overlay;
            border-radius: 32px;

            FaIcon {
                source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/lock.svg");
                fa-size: 28px;
                fa-color: Theme.text-tertiary;
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
            }
        }

        Text {
            text: "Access Denied";
            color: Theme.text-primary;
            font-size: 16px;
            font-weight: Theme.weight-semibold;
            horizontal-alignment: center;
        }

        Text {
            text: root.message;
            color: Theme.text-secondary;
            font-size: 13px;
            horizontal-alignment: center;
            wrap: word-wrap;
        }
    }
}
```

#### `PersistentBanner` ([slint/feedback/PersistentBanner.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/PersistentBanner.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component PersistentBanner inherits Rectangle {
    in property <string> text: "";
    in property <int> position: 0;
    in property <int> kind: 0;

    callback close();

    height: 40px;
    background: kind == 3 ? Theme.state-error.with-alpha(0.12) : (kind == 2 ? Theme.state-warning.with-alpha(0.12) : (kind == 1 ? Theme.state-success.with-alpha(0.12) : Theme.state-info.with-alpha(0.12)));

    border-line := Rectangle {
        x: 0;
        y: position == 0 ? parent.height - 1px : 0px;
        width: parent.width;
        height: 1px;
        background: kind == 3 ? Theme.state-error.with-alpha(0.25) : (kind == 2 ? Theme.state-warning.with-alpha(0.25) : (kind == 1 ? Theme.state-success.with-alpha(0.25) : Theme.state-info.with-alpha(0.25)));
    }

    HorizontalLayout {
        padding: Theme.sp-3;
        spacing: Theme.sp-3;
        alignment: stretch;

        Text {
            text: root.text;
            color: kind == 3 ? Theme.state-error : (kind == 2 ? Theme.state-warning : (kind == 1 ? Theme.state-success : Theme.state-info));
            font-family: Theme.font-ui;
            font-size: Theme.text-sm;
            font-weight: Theme.weight-medium;
            vertical-alignment: center;
            horizontal-stretch: 1;
            wrap: word-wrap;
        }

        Rectangle {
            width: 24px;
            height: 24px;
            horizontal-stretch: 0;

            close-bg := Rectangle {
                border-radius: Theme.radius-xs;
                background: close-ta.pressed ? Theme.surface-pressed : (close-ta.has-hover ? Theme.surface-hover : #00000000);
            }

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: "✕";
                color: Theme.text-tertiary;
                font-size: Theme.text-xs;
                vertical-alignment: center;
            }

            close-ta := TouchArea {
                clicked => { root.close(); }
            }
        }
    }
}
```

#### `ProgressBar` ([slint/feedback/ProgressBar.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/ProgressBar.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component ProgressBar inherits Rectangle {
    in property <float> value: 0;
    in property <float> maximum: 100;
    in property <bool> indeterminate: false;
    in property <color> progress-color: Theme.accent;
    in property <color> track-color: Theme.bg-overlay;

    height: 6px;
    background: transparent;

    track := Rectangle {
        border-radius: Theme.radius-full;
        background: track-color;
    }

    fill := Rectangle {
        x: 0;
        y: 0;
        width: indeterminate ? parent.width * 0.4 : (parent.width * Math.min(value, maximum) / maximum);
        height: parent.height;
        border-radius: Theme.radius-full;
        background: progress-color;
        visible: !indeterminate;
    }

    indeterminate-rect := Rectangle {
        x: indeterminate ? parent.width * 0.3 : 0px;
        y: 0;
        width: parent.width * 0.4;
        height: parent.height;
        border-radius: Theme.radius-full;
        background: progress-color;
        visible: indeterminate;
        opacity: indeterminate ? 0.6 : 0;
        animate x { duration: 1200ms; easing: Theme.ease-in-out; }
        animate opacity { duration: 300ms; }
    }
}
```

#### `PulseLoader` ([slint/feedback/PulseLoader.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/PulseLoader.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component PulseLoader inherits Rectangle {
    in property <length> size: 12px;

    width: size;
    height: size;
    background: Theme.accent;
    border-radius: size / 2;
    opacity: 0.4;

    animate opacity { duration: 800ms; easing: ease-in-out; }
}
```

#### `SegmentedProgress` ([slint/feedback/SegmentedProgress.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/SegmentedProgress.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component SegmentedProgress inherits Rectangle {
    in property <int> steps: 5;
    in property <int> current: 0;

    in property <color> completed-color: Theme.accent;
    in property <color> upcoming-color: Theme.bg-overlay;

    height: 8px;
    background: #00000000;

    HorizontalLayout {
        spacing: 3px;
        alignment: stretch;

        for idx in steps: Rectangle {
            border-radius: Theme.radius-xs;
            background: idx < current ? completed-color : upcoming-color;
            horizontal-stretch: 1;
        }
    }
}
```

#### `Skeleton` ([slint/feedback/Skeleton.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/Skeleton.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component Skeleton inherits Rectangle {
    in property <length> skeleton-height: 16px;
    in property <bool> circle: false;

    horizontal-stretch: 1;
    height: skeleton-height;
    background: Theme.bg-overlay;
    border-radius: circle ? Theme.radius-full : Theme.radius-sm;
    clip: true;
}
```

#### `SkeletonAvatar` ([slint/feedback/SkeletonAvatar.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/SkeletonAvatar.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component SkeletonAvatar inherits Rectangle {
    in property <length> size: 40px;

    width: size;
    height: size;
    background: Theme.bg-overlay;
    border-radius: size / 2;
}
```

#### `SkeletonCard` ([slint/feedback/SkeletonCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/SkeletonCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component SkeletonCard inherits Rectangle {
    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;

    VerticalLayout {
        Rectangle {
            height: 120px;
            background: Theme.bg-overlay;
            border-radius: Theme.radius-lg;
        }

        VerticalLayout {
            padding: 12px;
            spacing: 8px;

            SkeletonHeading { skeleton-width: parent.width * 0.6; }

            SkeletonText { skeleton-width: parent.width; }
            SkeletonText { skeleton-width: parent.width * 0.8; }
        }
    }
}
```

#### `SkeletonHeading` ([slint/feedback/SkeletonHeading.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/SkeletonHeading.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component SkeletonHeading inherits Rectangle {
    in property <length> skeleton-width: 180px;

    width: skeleton-width;
    height: 24px;
    background: Theme.bg-overlay;
    border-radius: 4px;
}
```

#### `SkeletonImage` ([slint/feedback/SkeletonImage.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/SkeletonImage.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component SkeletonImage inherits Rectangle {
    in property <length> image-height: 120px;

    height: image-height;
    background: Theme.bg-overlay;
    border-radius: Theme.radius-md;
}
```

#### `SkeletonListItem` ([slint/feedback/SkeletonListItem.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/SkeletonListItem.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component SkeletonListItem inherits Rectangle {
    height: 56px;

    HorizontalLayout {
        spacing: 12px;
        alignment: start;

        SkeletonAvatar { size: 40; }

        VerticalLayout {
            spacing: 8px;
            y: (parent.height - self.preferred-height) / 2;

            SkeletonText { skeleton-width: 140px; }
            SkeletonText { skeleton-width: 200px; }
        }
    }
}
```

#### `SkeletonParagraph` ([slint/feedback/SkeletonParagraph.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/SkeletonParagraph.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component SkeletonParagraph inherits Rectangle {
    VerticalLayout {
        spacing: 8px;

        SkeletonText { skeleton-width: parent.width; }
        SkeletonText { skeleton-width: parent.width * 0.9; }
        SkeletonText { skeleton-width: parent.width * 0.75; }
    }
}
```

#### `SkeletonTableRow` ([slint/feedback/SkeletonTableRow.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/SkeletonTableRow.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component SkeletonTableRow inherits Rectangle {
    in property <[int]> columns: [0, 1, 2, 3];

    height: 40px;

    HorizontalLayout {
        spacing: 12px;

        for idx in root.columns: Rectangle {
            horizontal-stretch: 1;
            height: 14px;
            background: Theme.bg-overlay;
            border-radius: 4px;
            y: (parent.height - self.height) / 2;
        }
    }
}
```

#### `SkeletonText` ([slint/feedback/SkeletonText.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/SkeletonText.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component SkeletonText inherits Rectangle {
    in property <length> skeleton-width: 200px;

    width: skeleton-width;
    height: 14px;
    background: Theme.bg-overlay;
    border-radius: 4px;
}
```

#### `Snackbar` ([slint/feedback/Snackbar.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/Snackbar.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component Snackbar inherits Rectangle {
    in property <string> text: "";
    in property <string> action-text: "";

    callback action-clicked();
    callback dismissed();

    height: 48px;
    background: Theme.dark-mode ? Theme.slate_800 : Theme.slate_900;
    border-radius: Theme.radius-md;

    drop-shadow-offset-y: 4px;
    drop-shadow-blur: 8px;
    drop-shadow-color: Theme.border-default;

    HorizontalLayout {
        padding-left: Theme.sp-4;
        padding-right: Theme.sp-2;
        spacing: Theme.sp-3;
        alignment: stretch;

        Text {
            text: root.text;
            color: #ffffff;
            font-family: Theme.font-ui;
            font-size: Theme.text-sm;
            vertical-alignment: center;
            horizontal-stretch: 1;
        }

        if action-text != "": Rectangle {
            horizontal-stretch: 0;

            action-bg := Rectangle {
                border-radius: Theme.radius-sm;
                background: action-ta.pressed ? Theme.accent-dim
                    : action-ta.has-hover ? Theme.accent
                    : transparent;
            }

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: root.action-text;
                color: Theme.accent;
                font-family: Theme.font-ui;
                font-size: Theme.text-sm;
                font-weight: Theme.weight-semibold;
                vertical-alignment: center;
            }

            action-ta := TouchArea {
                clicked => { root.action-clicked(); }
            }
        }

        Rectangle {
            width: 24px;
            height: 24px;
            horizontal-stretch: 0;

            dismiss-bg := Rectangle {
                border-radius: Theme.radius-xs;
                background: dismiss-ta.pressed ? rgba(255, 255, 255, 0.15)
                    : dismiss-ta.has-hover ? rgba(255, 255, 255, 0.08)
                    : transparent;
            }

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: "✕";
                color: rgba(255, 255, 255, 0.6);
                font-size: Theme.text-xs;
                vertical-alignment: center;
            }

            dismiss-ta := TouchArea {
                clicked => { root.dismissed(); }
            }
        }
    }
}
```

#### `StripedProgress` ([slint/feedback/StripedProgress.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/StripedProgress.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component StripedProgress inherits Rectangle {
    in property <float> value: 0;
    in property <float> max: 100;

    in property <color> bar-color: Theme.accent;
    in property <color> track-color: Theme.bg-overlay;

    height: 10px;
    background: #00000000;

    track-bg := Rectangle {
        border-radius: Theme.radius-full;
        background: track-color;
    }

    fill-rect := Rectangle {
        x: 0;
        y: 0;
        width: parent.width * Math.min(value, max) / max;
        height: parent.height;
        border-radius: Theme.radius-full;
        background: bar-color;
        clip: true;

        stripe1 := Rectangle {
            x: -16px;
            y: -4px;
            width: 6px;
            height: parent.height + 8px;
            background: rgba(255, 255, 255, 0.15);
            rotation-angle: -45deg;
            animate x { duration: 800ms; easing: Theme.ease-linear; loop-count: -1; }
        }

        stripe2 := Rectangle {
            x: 4px;
            y: -4px;
            width: 6px;
            height: parent.height + 8px;
            background: rgba(255, 255, 255, 0.15);
            rotation-angle: -45deg;
            animate x { duration: 800ms; easing: Theme.ease-linear; loop-count: -1; }
        }

        stripe3 := Rectangle {
            x: 24px;
            y: -4px;
            width: 6px;
            height: parent.height + 8px;
            background: rgba(255, 255, 255, 0.15);
            rotation-angle: -45deg;
            animate x { duration: 800ms; easing: Theme.ease-linear; loop-count: -1; }
        }

        stripe4 := Rectangle {
            x: 44px;
            y: -4px;
            width: 6px;
            height: parent.height + 8px;
            background: rgba(255, 255, 255, 0.15);
            rotation-angle: -45deg;
            animate x { duration: 800ms; easing: Theme.ease-linear; loop-count: -1; }
        }

        stripe5 := Rectangle {
            x: 64px;
            y: -4px;
            width: 6px;
            height: parent.height + 8px;
            background: rgba(255, 255, 255, 0.15);
            rotation-angle: -45deg;
            animate x { duration: 800ms; easing: Theme.ease-linear; loop-count: -1; }
        }
    }
}
```

#### `SuccessState` ([slint/feedback/SuccessState.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/SuccessState.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component SuccessState inherits Rectangle {
    in property <string> title: "Success!";
    in property <string> message: "Your action was completed.";

    background: Theme.bg-base;
    horizontal-stretch: 1;

    VerticalLayout {
        alignment: center;
        spacing: 12px;

        Rectangle {
            width: 64px;
            height: 64px;
            background: Theme.success;
            border-radius: 32px;

            FaIcon {
                source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/circle-check.svg");
                fa-size: 28px;
                fa-color: Theme.success;
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
            }
        }

        Text {
            text: root.title;
            color: Theme.text-primary;
            font-size: 16px;
            font-weight: Theme.weight-semibold;
            horizontal-alignment: center;
        }

        Text {
            text: root.message;
            color: Theme.text-secondary;
            font-size: 13px;
            horizontal-alignment: center;
            wrap: word-wrap;
        }
    }
}
```

#### `Toast` ([slint/feedback/Toast.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/Toast.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component Toast inherits Rectangle {
    in property <string> popup-text: "";
    in-out property <bool> active: false;
    in property <duration> auto-hide-duration: 7000ms;

    callback dismissed();

    visible: active;
    min-height: 48px;
    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-subtle;

    drop-shadow-offset-x: 0px;
    drop-shadow-offset-y: Theme.elevation-2-y;
    drop-shadow-blur: Theme.elevation-2-blur;
    drop-shadow-color: Theme.elevation-2-color;

    opacity: active ? 1 : 0;
    animate opacity { duration: Theme.dur-slow; easing: Theme.ease-soft; }

    label := Text {
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
        text: root.popup-text;
        color: Theme.text-primary;
        font-family: Theme.font-ui;
        font-size: Theme.text-base;
        vertical-alignment: center;
        horizontal-alignment: center;
        overflow: elide;
    }

    ta := TouchArea {
        clicked => {
            root.active = false;
            root.dismissed();
        }
    }

    timer := Timer {
        interval: root.auto-hide-duration;
        running: root.active;
        triggered => {
            root.active = false;
            root.dismissed();
        }
    }
}
```

#### `WaveLoader` ([slint/feedback/WaveLoader.slint](file:///home/lion/Documents/GitHub/ltk/slint/feedback/WaveLoader.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component WaveLoader inherits Rectangle {
    in property <length> size: 32px;

    width: size * 1.5;
    height: size;
    background: #00000000;

    Rectangle {
        width: 4px;
        height: parent.height * 0.6;
        x: parent.width * 0.2;
        y: (parent.height - self.height) / 2;
        background: Theme.accent;
        border-radius: 2px;
    }

    Rectangle {
        width: 4px;
        height: parent.height * 0.8;
        x: parent.width * 0.45;
        y: (parent.height - self.height) / 2;
        background: Theme.accent;
        border-radius: 2px;
    }

    Rectangle {
        width: 4px;
        height: parent.height;
        x: parent.width * 0.7;
        y: (parent.height - self.height) / 2;
        background: Theme.accent;
        border-radius: 2px;
    }
}
```


### 5.5 Navigation Components & Layout Structures Code Manual

#### `AnchorNavBar` ([slint/navigation/AnchorNavBar.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/AnchorNavBar.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component AnchorNavBar inherits Rectangle {
    in property <int> active: 0;
    in property <[string]> items: [];

    callback anchor-clicked(int);

    height: 40px;
    background: #00000000;

    HorizontalLayout {
        padding-left: Theme.sp-4;
        padding-right: Theme.sp-4;
        spacing: Theme.sp-1;

        for link[idx] in root.items: Rectangle {
            height: 32px;
            background: link-ta.pressed ? Theme.surface-pressed
                : link-ta.has-hover ? Theme.surface-hover
                : idx == root.active ? Theme.accent-subtle
                : #00000000;
            border-radius: Theme.radius-sm;

            animate background { duration: Theme.dur-fast; }

            HorizontalLayout {
                padding-left: Theme.sp-3;
                padding-right: Theme.sp-3;

                Text {
                    text: link;
                    color: idx == root.active ? Theme.accent : Theme.text-secondary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-sm;
                    font-weight: idx == root.active ? Theme.weight-medium : Theme.weight-regular;
                    vertical-alignment: center;
                }
            }

            if idx == root.active: Rectangle {
                height: 2px;
                y: parent.height - 2px;
                x: Theme.sp-3;
                width: parent.width - Theme.sp-6;
                background: Theme.accent;
                border-radius: 1px;
            }

            link-ta := TouchArea {
                clicked => { root.anchor-clicked(idx); }
            }
        }
    }
}
```

#### `BottomNavBar` ([slint/navigation/BottomNavBar.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/BottomNavBar.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component BottomNavBar inherits Rectangle {
    in-out property <int> active: 0;

    height: 64px;
    background: Theme.bg-elevated;

    HorizontalLayout {
        Rectangle {
            horizontal-stretch: 1;
            background: root.active == 0 ? Theme.accent-subtle : #00000000;

            VerticalLayout {
                alignment: center;
                spacing: 2px;

                Image {
                    source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/house.svg");
                    width: 20px;
                    height: 20px;
                    colorize: root.active == 0 ? Theme.accent : Theme.text-tertiary;
                    horizontal-alignment: center;
                }

                Text {
                    text: "Home";
                    color: root.active == 0 ? Theme.accent : Theme.text-tertiary;
                    font-size: 10px;
                    font-weight: Theme.weight-medium;
                    horizontal-alignment: center;
                }
            }

            TouchArea { clicked => { root.active = 0; } }
        }

        Rectangle {
            horizontal-stretch: 1;
            background: root.active == 1 ? Theme.accent-subtle : #00000000;

            VerticalLayout {
                alignment: center;
                spacing: 2px;

                Image {
                    source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/magnifying-glass.svg");
                    width: 20px;
                    height: 20px;
                    colorize: root.active == 1 ? Theme.accent : Theme.text-tertiary;
                    horizontal-alignment: center;
                }

                Text {
                    text: "Search";
                    color: root.active == 1 ? Theme.accent : Theme.text-tertiary;
                    font-size: 10px;
                    font-weight: Theme.weight-medium;
                    horizontal-alignment: center;
                }
            }

            TouchArea { clicked => { root.active = 1; } }
        }

        Rectangle {
            horizontal-stretch: 1;
            background: root.active == 2 ? Theme.accent-subtle : #00000000;

            VerticalLayout {
                alignment: center;
                spacing: 2px;

                Image {
                    source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/bell.svg");
                    width: 20px;
                    height: 20px;
                    colorize: root.active == 2 ? Theme.accent : Theme.text-tertiary;
                    horizontal-alignment: center;
                }

                Text {
                    text: "Alerts";
                    color: root.active == 2 ? Theme.accent : Theme.text-tertiary;
                    font-size: 10px;
                    font-weight: Theme.weight-medium;
                    horizontal-alignment: center;
                }
            }

            TouchArea { clicked => { root.active = 2; } }
        }

        Rectangle {
            horizontal-stretch: 1;
            background: root.active == 3 ? Theme.accent-subtle : #00000000;

            VerticalLayout {
                alignment: center;
                spacing: 2px;

                Image {
                    source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/user.svg");
                    width: 20px;
                    height: 20px;
                    colorize: root.active == 3 ? Theme.accent : Theme.text-tertiary;
                    horizontal-alignment: center;
                }

                Text {
                    text: "Profile";
                    color: root.active == 3 ? Theme.accent : Theme.text-tertiary;
                    font-size: 10px;
                    font-weight: Theme.weight-medium;
                    horizontal-alignment: center;
                }
            }

            TouchArea { clicked => { root.active = 3; } }
        }
    }
}
```

#### `BottomNavRail` ([slint/navigation/BottomNavRail.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/BottomNavRail.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component BottomNavRail inherits Rectangle {
    in-out property <int> active: 0;

    height: 80px;
    background: Theme.bg-elevated;

    HorizontalLayout {
        Rectangle {
            horizontal-stretch: 1;
            background: root.active == 0 ? Theme.accent-subtle : #00000000;

            VerticalLayout {
                alignment: center;
                spacing: 4px;

                Image {
                    source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/house.svg");
                    width: 24px;
                    height: 24px;
                    colorize: root.active == 0 ? Theme.accent : Theme.text-tertiary;
                    horizontal-alignment: center;
                }

                Text {
                    text: "Home";
                    color: root.active == 0 ? Theme.accent : Theme.text-tertiary;
                    font-size: 12px;
                    font-weight: Theme.weight-medium;
                    horizontal-alignment: center;
                }
            }

            TouchArea { clicked => { root.active = 0; } }
        }

        Rectangle {
            horizontal-stretch: 1;
            background: root.active == 1 ? Theme.accent-subtle : #00000000;

            VerticalLayout {
                alignment: center;
                spacing: 4px;

                Image {
                    source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/magnifying-glass.svg");
                    width: 24px;
                    height: 24px;
                    colorize: root.active == 1 ? Theme.accent : Theme.text-tertiary;
                    horizontal-alignment: center;
                }

                Text {
                    text: "Search";
                    color: root.active == 1 ? Theme.accent : Theme.text-tertiary;
                    font-size: 12px;
                    font-weight: Theme.weight-medium;
                    horizontal-alignment: center;
                }
            }

            TouchArea { clicked => { root.active = 1; } }
        }

        Rectangle {
            horizontal-stretch: 1;
            background: root.active == 2 ? Theme.accent-subtle : #00000000;

            VerticalLayout {
                alignment: center;
                spacing: 4px;

                Image {
                    source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/bell.svg");
                    width: 24px;
                    height: 24px;
                    colorize: root.active == 2 ? Theme.accent : Theme.text-tertiary;
                    horizontal-alignment: center;
                }

                Text {
                    text: "Alerts";
                    color: root.active == 2 ? Theme.accent : Theme.text-tertiary;
                    font-size: 12px;
                    font-weight: Theme.weight-medium;
                    horizontal-alignment: center;
                }
            }

            TouchArea { clicked => { root.active = 2; } }
        }
    }
}
```

#### `Breadcrumb` ([slint/navigation/Breadcrumb.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/Breadcrumb.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component Breadcrumb inherits Rectangle {
    in property <[string]> items: [];
    in property <int> active-index: -1;

    callback item-clicked(int);

    height: 32px;
    background: transparent;

    horizontal-stretch: 1;

    HorizontalLayout {
        spacing: Theme.sp-2;
        alignment: start;

        for item[i] in root.items: HorizontalLayout {
            spacing: Theme.sp-2;
            horizontal-stretch: 0;

            if i > 0: Text {
                text: "›";
                color: Theme.text-tertiary;
                font-family: Theme.font-ui;
                font-size: Theme.text-md;
                vertical-alignment: center;
                horizontal-stretch: 0;
            }

            ta := TouchArea {
                width: label.preferred-width + Theme.sp-2 * 2;
                height: 28px;
                y: (parent.height - self.height) / 2;
                clicked => { root.item-clicked(i); }

                bg := Rectangle {
                    width: parent.width;
                    height: parent.height;
                    border-radius: Theme.radius-sm;
                    background: ta.pressed ? Theme.surface-pressed
                        : ta.has-hover ? Theme.surface-hover
                        : transparent;

                    animate background { duration: Theme.dur-fast; }

                    label := Text {
                        x: (parent.width - self.width) / 2;
                        y: (parent.height - self.height) / 2;
                        text: item;
                        color: (root.active-index == i || (root.active-index == -1 && i == root.items.length - 1))
                            ? Theme.text-primary : Theme.text-secondary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-sm;
                        font-weight: (root.active-index == i || (root.active-index == -1 && i == root.items.length - 1))
                            ? Theme.weight-semibold : Theme.weight-regular;
                        vertical-alignment: center;
                    }
                }
            }
        }
    }
}
```

#### `CollapsingHeader` ([slint/navigation/CollapsingHeader.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/CollapsingHeader.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component CollapsingHeader inherits Rectangle {
    in property <string> title: "";
    in property <string> subtitle: "";
    in property <bool> expanded: true;

    background: Theme.bg-elevated;

    if root.expanded: Rectangle {
        height: 100px;

        VerticalLayout {
            padding: 16px;
            spacing: 4px;

            Text {
                text: root.title;
                color: Theme.text-primary;
                font-size: 28px;
                font-weight: Theme.weight-bold;
            }

            Text {
                text: root.subtitle;
                color: Theme.text-secondary;
                font-size: 14px;
            }
        }
    }

    if !root.expanded: Rectangle {
        height: 48px;

        Text {
            x: 16px;
            text: root.title;
            color: Theme.text-primary;
            font-size: 18px;
            font-weight: Theme.weight-semibold;
            vertical-alignment: center;
        }
    }
}
```

#### `CommandPalette` ([slint/navigation/CommandPalette.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/CommandPalette.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component CommandPalette inherits Rectangle {
    visible: false;
    in property <string> query: "";
    in property <[string]> results: [];

    callback closed();
    callback result-selected(int);

    x: 0;
    y: 0;
    width: parent.width;
    height: parent.height;
    background: Theme.backdrop;
    visible: root.visible;

    animate opacity { duration: Theme.dur-fast; }
    opacity: visible ? 1 : 0;

    Rectangle {
        width: 560px;
        height: palette-content.preferred-height + Theme.sp-4;
        y: 80px;
        x: (parent.width - self.width) / 2;
        background: Theme.bg-elevated;
        border-radius: Theme.radius-lg;
        border-width: 1px;
        border-color: Theme.border-subtle;
    }

    palette-content := VerticalLayout {
        width: 560px;
        y: 80px;
        x: (parent.width - self.width) / 2;
        spacing: 0;

        HorizontalLayout {
            padding-left: Theme.sp-4;
            padding-right: Theme.sp-4;
            height: Theme.input-height-lg;
            spacing: Theme.sp-2;

            Text {
                text: ">";
                color: Theme.accent;
                font-family: Theme.font-mono;
                font-size: Theme.text-lg;
                vertical-alignment: center;
                horizontal-stretch: 0;
            }

            TextInput {
                text: root.query;
                color: Theme.text-primary;
                font-family: Theme.font-ui;
                font-size: Theme.text-md;
                vertical-alignment: center;
                horizontal-stretch: 1;
                placeholder-text: "Type a command...";
            }
        }

        Rectangle {
            height: 1px;
            background: Theme.border-subtle;
        }

        VerticalLayout {
            padding: Theme.sp-2;
            spacing: Theme.sp-1;

            if root.results.length == 0: Rectangle {
                height: 60px;

                HorizontalLayout {
                    alignment: center;

                    Text {
                        text: "No results found";
                        color: Theme.text-tertiary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-sm;
                        vertical-alignment: center;
                    }
                }
            }

            for res[idx] in root.results: Rectangle {
                height: 36px;
                background: res-ta.pressed ? Theme.surface-pressed
                    : res-ta.has-hover ? Theme.surface-hover
                    : #00000000;
                border-radius: Theme.radius-sm;

                HorizontalLayout {
                    padding-left: Theme.sp-3;
                    padding-right: Theme.sp-3;

                    Text {
                        text: res;
                        color: Theme.text-primary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-sm;
                        vertical-alignment: center;
                        horizontal-stretch: 1;
                    }

                    Text {
                        text: "Enter";
                        color: Theme.text-tertiary;
                        font-family: Theme.font-mono;
                        font-size: Theme.text-xs;
                        vertical-alignment: center;
                    }
                }

                res-ta := TouchArea {
                    clicked => { root.result-selected(idx); }
                }
            }
        }
    }

    ta := TouchArea {
        clicked => { root.closed(); }
    }
}
```

#### `Dock` ([slint/navigation/Dock.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/Dock.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component Dock inherits Rectangle {
    background: Theme.bg-raised;
    border-radius: 16px;
    border-width: 1px;
    border-color: Theme.border-default;
    height: 64px;
    horizontal-stretch: 0;
    clip: true;

    in property <bool> floating: true;
    in-out property <int> active-index: -1;

    in property <[string]> icons: ["🏠", "📁", "🌐", "✉️", "🎵"];
    in property <[string]> labels: ["Home", "Files", "Web", "Mail", "Music"];

    HorizontalLayout {
        padding: Theme.sp-2;
        spacing: Theme.sp-1;
        alignment: center;

        for item[idx] in root.icons: Rectangle {
            width: 48px;
            height: 48px;
            border-radius: 12px;
            background: active-index == idx ? Theme.accent-subtle : transparent;

            VerticalLayout {
                alignment: center;
                spacing: 2px;

                Text {
                    text: item;
                    font-size: 18px;
                    horizontal-alignment: center;
                }

                if idx < 5 && idx < root.labels.length: Text {
                    text: root.labels[idx];
                    font-size: 9px;
                    color: active-index == idx ? Theme.accent : Theme.text-tertiary;
                    horizontal-alignment: center;
                    overflow: elide;
                }
            }

            TouchArea {
                clicked => { root.active-index = idx; }
            }
        }
    }
}
```

#### `DotStepper` ([slint/navigation/DotStepper.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/DotStepper.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component DotStepper inherits Rectangle {
    in property <[int]> total: [0];
    in property <int> current: 0;

    height: 24px;
    background: #00000000;

    HorizontalLayout {
        alignment: center;
        spacing: Theme.sp-2;

        for dot[idx] in root.total: Rectangle {
            width: 8px;
            height: 8px;
            horizontal-stretch: 0;
            border-radius: Theme.radius-full;
            background: idx == root.current ? Theme.accent
                : idx < root.current ? Theme.accent-dim
                : Theme.bg-overlay;
            opacity: idx == root.current ? 1.0
                : idx < root.current ? 0.6
                : 0.4;
        }
    }
}
```

#### `Drawer` ([slint/navigation/Drawer.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/Drawer.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component Drawer inherits Rectangle {
    in property <bool> open: false;
    in property <string> side: "start"; // start, end
    in property <length> drawer-width: 280px;
    in property <bool> modal: true;

    callback closed();

    background: transparent;
    clip: true;

    // Backdrop
    if modal && open: Rectangle {
        background: Theme.backdrop;

        ta-backdrop := TouchArea {
            clicked => { root.closed(); }
        }

        animate background { duration: Theme.dur-medium; }
    }

    // Drawer panel
    panel := Rectangle {
        x: {
            if !open { side == "start" ? -drawer-width : parent.width }
            else { side == "start" ? 0px : parent.width - drawer-width }
        }
        width: drawer-width;
        height: parent.height;
        background: Theme.bg-surface;
        border-radius: side == "start" ? 0px : Theme.radius-lg;
        border-width: 1px;
        border-color: Theme.border-subtle;
        clip: true;

        drop-shadow-offset-x: open ? (side == "start" ? 4px : -4px) : 0px;
        drop-shadow-blur: open ? 16px : 0px;
        drop-shadow-color: open ? Theme.border-default : transparent;

        animate x { duration: Theme.dur-medium; easing: Theme.ease-decelerate; }
        animate drop-shadow-offset-x { duration: Theme.dur-medium; }
        animate drop-shadow-blur { duration: Theme.dur-medium; }
        animate drop-shadow-color { duration: Theme.dur-medium; }

        // Header
        Rectangle {
            height: 56px;
            background: transparent;

            HorizontalLayout {
                padding-left: Theme.sp-4;
                padding-right: Theme.sp-4;
                alignment: stretch;

                Text {
                    text: root.open ? "Menu" : "";
                    color: Theme.text-primary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-lg;
                    font-weight: Theme.weight-semibold;
                    vertical-alignment: center;
                    horizontal-stretch: 1;
                }

                Rectangle {
                    width: 32px;
                    height: 32px;
                    horizontal-stretch: 0;
                    vertical-stretch: 0;

                    close-bg := Rectangle {
                        border-radius: Theme.radius-sm;
                        background: ta-close.pressed ? Theme.surface-pressed
                            : ta-close.has-hover ? Theme.surface-hover
                            : transparent;
                        animate background { duration: Theme.dur-fast; }
                    }

                    Text {
                        x: (parent.width - self.width) / 2;
                        y: (parent.height - self.height) / 2;
                        text: "✕";
                        color: Theme.text-secondary;
                        font-size: Theme.text-sm;
                        vertical-alignment: center;
                    }

                    ta-close := TouchArea {
                        clicked => { root.closed(); }
                    }
                }
            }
        }

        // Divider
        Rectangle {
            height: 1px;
            background: Theme.border-subtle;
        }

        // Content slot
        content-layout := VerticalLayout {
            padding: Theme.sp-2;
        }
    }
}
```

#### `EllipsisBreadcrumb` ([slint/navigation/EllipsisBreadcrumb.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/EllipsisBreadcrumb.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component EllipsisBreadcrumb inherits Rectangle {
    background: transparent;
    height: 32px;
    horizontal-stretch: 1;
    clip: true;

    in property <[string]> items: ["Home", "Documents", "Projects", "LTK", "src"];
    in-out property <int> active-index: 4;
    in property <int> max-visible: 3;
    in property <string> separator: "›";

    HorizontalLayout {
        spacing: Theme.sp-1;
        alignment: start;

        for item[idx] in root.items: HorizontalLayout {
            spacing: Theme.sp-1;
            visible: idx < max-visible || idx == root.items.length - 1 || (idx > root.items.length - max-visible - 1);

            if idx > 0 && idx < root.items.length: Text {
                text: root.separator;
                font-size: Theme.text-sm;
                color: Theme.text-tertiary;
                y: 4px;
            }

            if idx == max-visible - 1 && root.items.length > root.max-visible + 1: Rectangle {
                width: 24px;
                height: 24px;
                border-radius: 4px;
                y: 4px;
                background: transparent;

                Text {
                    text: "•••";
                    font-size: Theme.text-sm;
                    color: Theme.text-tertiary;
                    x: (parent.width - self.width) / 2;
                    y: (parent.height - self.height) / 2;
                }

                TouchArea {
                    clicked => { /* expand all */ }
                }
            }

            if idx != max-visible - 1 || root.items.length <= root.max-visible + 1: Rectangle {
                height: 24px;
                border-radius: 4px;
                background: active-index == idx ? Theme.accent-subtle : transparent;
                y: 4px;

                HorizontalLayout {
                    padding-left: Theme.sp-2;
                    padding-right: Theme.sp-2;
                    spacing: Theme.sp-1;

                    Text {
                        text: item;
                        font-size: Theme.text-sm;
                        font-weight: active-index == idx ? Theme.weight-medium : Theme.weight-regular;
                        color: active-index == idx ? Theme.accent : Theme.text-secondary;
                        y: 1px;
                        overflow: elide;
                        max-width: 120px;
                    }
                }

                TouchArea {
                    clicked => { root.active-index = idx; }
                }
            }
        }
    }
}
```

#### `FloatingNavPanel` ([slint/navigation/FloatingNavPanel.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/FloatingNavPanel.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component FloatingNavPanel inherits Rectangle {
    width: 220px;
    background: Theme.bg-elevated;
    border-radius: Theme.radius-lg;
    visible: root.visible;

    drop-shadow-offset-y: 4px;
    drop-shadow-blur: 16px;
    drop-shadow-color: Theme.border-default;

    VerticalLayout {
        padding: 8px;
        spacing: 2px;

        Rectangle {
            height: 36px;
            background: Theme.accent-subtle;
            border-radius: Theme.radius-sm;

            Text {
                x: 12px;
                text: "Dashboard";
                color: Theme.accent;
                font-size: 13px;
                font-weight: Theme.weight-semibold;
                vertical-alignment: center;
            }

            TouchArea { clicked => { } }
        }

        Rectangle {
            height: 36px;
            border-radius: Theme.radius-sm;

            Text {
                x: 12px;
                text: "Projects";
                color: Theme.text-primary;
                font-size: 13px;
                vertical-alignment: center;
            }

            TouchArea { clicked => { } }
        }

        Rectangle {
            height: 36px;
            border-radius: Theme.radius-sm;

            Text {
                x: 12px;
                text: "Settings";
                color: Theme.text-primary;
                font-size: 13px;
                vertical-alignment: center;
            }

            TouchArea { clicked => { } }
        }
    }
}
```

#### `FlyoutMenu` ([slint/navigation/FlyoutMenu.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/FlyoutMenu.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component FlyoutMenuItem inherits Rectangle {
    in property <string> icon: "";
    in property <string> label: "";
    in property <string> shortcut: "";
    in property <bool> show-separator: false;
    in property <bool> danger: false;

    height: show-separator ? 20px : 32px;
    border-radius: 6px;

    if show-separator: HorizontalLayout {
        padding-left: Theme.sp-2;
        padding-right: Theme.sp-2;
        Rectangle {
            y: 9px;
            height: 1px;
            background: Theme.border-default;
        }
    }

    if !show-separator: HorizontalLayout {
        padding-left: Theme.sp-3;
        padding-right: Theme.sp-3;
        spacing: Theme.sp-3;
        alignment: stretch;

        if icon != "": Text {
            text: icon;
            font-size: 14px;
            y: (parent.height - self.height) / 2;
            width: 20px;
            horizontal-alignment: center;
        }

        Text {
            text: label;
            font-size: Theme.text-sm;
            color: danger ? Theme.error : Theme.text-primary;
            y: (parent.height - self.height) / 2;
            vertical-stretch: 1;
            overflow: elide;
        }

        if shortcut != "": Text {
            text: shortcut;
            font-size: Theme.text-xs;
            color: Theme.text-tertiary;
            y: (parent.height - self.height) / 2;
        }
    }

    if !show-separator: TouchArea { }
}

export component FlyoutMenu inherits Rectangle {
    background: Theme.bg-elevated;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-default;
    padding: Theme.sp-1;
    width: 220px;
    clip: true;

    in property <bool> show-icons: true;
    in property <bool> show-shortcuts: true;
}
```

#### `HamburgerButton` ([slint/navigation/HamburgerButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/HamburgerButton.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component HamburgerButton inherits Rectangle {
    in property <bool> open: false;

    callback clicked();

    width: 36px;
    height: 36px;
    background: ta.pressed ? Theme.surface-pressed
        : ta.has-hover ? Theme.surface-hover
        : #00000000;
    border-radius: Theme.radius-sm;

    animate background { duration: Theme.dur-fast; }

    Rectangle {
        width: 18px;
        height: 14px;
        y: (parent.height - self.height) / 2;
        x: (parent.width - self.width) / 2;

        Rectangle {
            width: 18px;
            height: 2px;
            y: open ? 6px : 0px;
            background: Theme.text-primary;
            border-radius: 1px;

            rotate-angle: open ? 45deg : 0deg;
            animate y { duration: Theme.dur-normal; easing: Theme.ease-standard; }
            animate rotate-angle { duration: Theme.dur-normal; easing: Theme.ease-standard; }
        }

        Rectangle {
            width: 18px;
            height: 2px;
            y: 6px;
            background: Theme.text-primary;
            border-radius: 1px;
            opacity: open ? 0 : 1;

            animate opacity { duration: Theme.dur-fast; }
        }

        Rectangle {
            width: 18px;
            height: 2px;
            y: open ? 6px : 12px;
            background: Theme.text-primary;
            border-radius: 1px;

            rotate-angle: open ? -45deg : 0deg;
            animate y { duration: Theme.dur-normal; easing: Theme.ease-standard; }
            animate rotate-angle { duration: Theme.dur-normal; easing: Theme.ease-standard; }
        }
    }

    ta := TouchArea {
        clicked => { root.clicked(); }
    }
}
```

#### `HorizontalStepper` ([slint/navigation/HorizontalStepper.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/HorizontalStepper.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component HorizontalStepper inherits Rectangle {
    in property <[string]> steps: [];
    in property <int> current: 0;

    height: 80px;
    background: #00000000;

    HorizontalLayout {
        alignment: center;
        spacing: 0;

        for step[idx] in root.steps: HorizontalLayout {
            horizontal-stretch: 1;
            spacing: 0;

            // Step circle + label column
            VerticalLayout {
                horizontal-stretch: 0;
                spacing: Theme.sp-2;
                alignment: center;

                Rectangle {
                    width: 32px;
                    height: 32px;
                    horizontal-stretch: 0;
                    border-radius: Theme.radius-full;
                    background: idx < root.current ? Theme.accent
                        : idx == root.current ? Theme.accent
                        : Theme.bg-overlay;
                    border-width: idx <= root.current ? 0px : 2px;
                    border-color: Theme.border-base;

                    Text {
                        x: (parent.width - self.width) / 2;
                        y: (parent.height - self.height) / 2;
                        text: idx < root.current ? "✓" : (idx + 1);
                        color: idx <= root.current ? Theme.on-accent : Theme.text-tertiary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-sm;
                        font-weight: Theme.weight-semibold;
                        vertical-alignment: center;
                        horizontal-alignment: center;
                    }
                }

                Text {
                    text: step;
                    color: idx <= root.current ? Theme.text-primary : Theme.text-tertiary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-xs;
                    font-weight: idx == root.current ? Theme.weight-semibold : Theme.weight-regular;
                    vertical-alignment: center;
                    horizontal-alignment: center;
                    horizontal-stretch: 1;
                }
            }

            // Connector
            if idx < root.steps.length - 1: Rectangle {
                height: 2px;
                horizontal-stretch: 1;
                y: 16px;
                background: idx < root.current ? Theme.accent : Theme.border-subtle;
            }
        }
    }
}
```

#### `IconStepper` ([slint/navigation/IconStepper.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/IconStepper.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component IconStepper inherits Rectangle {
    in property <int> current: 0;

    in property <[image]> icons: [];
    in property <[string]> labels: [];

    height: 80px;
    background: #00000000;

    HorizontalLayout {
        alignment: center;
        spacing: 0;

        for icon[idx] in root.icons: HorizontalLayout {
            horizontal-stretch: 1;
            spacing: 0;

            // Icon + label column
            VerticalLayout {
                horizontal-stretch: 0;
                spacing: Theme.sp-2;
                alignment: center;

                Rectangle {
                    width: 36px;
                    height: 36px;
                    horizontal-stretch: 0;
                    border-radius: Theme.radius-full;
                    background: idx < root.current ? Theme.accent
                        : idx == root.current ? Theme.accent
                        : Theme.bg-overlay;
                    border-width: idx <= root.current ? 0px : 2px;
                    border-color: Theme.border-base;

                    FaIcon {
                        x: (parent.width - self.width) / 2;
                        y: (parent.height - self.height) / 2;
                        source: icon;
                        fa-size: 14px;
                        fa-color: idx <= root.current ? Theme.on-accent : Theme.text-tertiary;
                    }
                }

                if idx < root.labels.length: Text {
                    text: root.labels[idx];
                    color: idx <= root.current ? Theme.text-primary : Theme.text-tertiary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-xs;
                    font-weight: idx == root.current ? Theme.weight-semibold : Theme.weight-regular;
                    vertical-alignment: center;
                    horizontal-alignment: center;
                    horizontal-stretch: 1;
                }
            }

            // Connector
            if idx < root.icons.length - 1: Rectangle {
                height: 2px;
                horizontal-stretch: 1;
                y: 18px;
                background: idx < root.current ? Theme.accent : Theme.border-subtle;
            }
        }
    }
}
```

#### `InfiniteScrollTrigger` ([slint/navigation/InfiniteScrollTrigger.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/InfiniteScrollTrigger.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component InfiniteScrollTrigger inherits Rectangle {
    in property <bool> loading: false;

    height: 60px;
    background: #00000000;

    if loading: Rectangle {
        width: 24px;
        height: 24px;
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
        border-radius: Theme.radius-full;
        border-width: 3px;
        border-color: Theme.accent;
        border-color: #00000000;
    }
}
```

#### `LaunchpadGrid` ([slint/navigation/LaunchpadGrid.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/LaunchpadGrid.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component LaunchpadGrid inherits Rectangle {
    in property <int> count: 0;

    callback app-clicked(int);

    background: #00000000;

    in property <int> cols: 4;
    in property <int> row-count: Math.ceil(count / cols);

    VerticalLayout {
        padding: Theme.sp-6;
        spacing: Theme.sp-8;

        for row-idx in root.row-count: HorizontalLayout {
            spacing: Theme.sp-6;
            alignment: center;

            for col-idx in root.cols: Rectangle {
                width: 80px;
                height: 96px;
                background: app-ta.pressed ? Theme.surface-pressed
                    : app-ta.has-hover ? Theme.surface-hover
                    : #00000000;
                border-radius: Theme.radius-md;
                visible: (row-idx * root.cols + col-idx) < root.count;

                VerticalLayout {
                    alignment: center;
                    spacing: Theme.sp-2;

                    Rectangle {
                        width: 52px;
                        height: 52px;
                        x: (parent.width - self.width) / 2;
                        background: Theme.bg-overlay;
                        border-radius: Theme.radius-lg;
                    }

                    Text {
                        text: "App";
                        color: Theme.text-primary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-xs;
                        horizontal-alignment: center;
                    }
                }

                app-ta := TouchArea {
                    clicked => { root.app-clicked(row-idx * root.cols + col-idx); }
                }
            }
        }
    }
}
```

#### `LoadMoreButton` ([slint/navigation/LoadMoreButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/LoadMoreButton.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component LoadMoreButton inherits Rectangle {
    in property <bool> loading: false;

    callback clicked();

    height: 44px;
    background: #00000000;

    Rectangle {
        width: 180px;
        height: 36px;
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
        border-radius: Theme.radius-sm;
        border-width: 1px;
        border-color: loading ? Theme.border-subtle : Theme.border-base;
        background: load-more-ta.pressed ? Theme.surface-pressed
            : load-more-ta.has-hover ? Theme.surface-hover
            : #00000000;
        opacity: loading ? 0.6 : 1.0;

        HorizontalLayout {
            alignment: center;
            spacing: Theme.sp-2;

            if loading: Rectangle {
                width: 14px;
                height: 14px;
                horizontal-stretch: 0;
                border-radius: Theme.radius-full;
                border-width: 2px;
                border-color: Theme.accent;
                border-color: #00000000;
            }

            Text {
                text: loading ? "Loading..." : "Load more";
                color: loading ? Theme.text-tertiary : Theme.text-secondary;
                font-family: Theme.font-ui;
                font-size: Theme.text-sm;
                font-weight: Theme.weight-medium;
                vertical-alignment: center;
            }
        }

        load-more-ta := TouchArea {
            clicked => {
                root.clicked();
            }
        }
    }
}
```

#### `MegaMenu` ([slint/navigation/MegaMenu.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/MegaMenu.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component MegaMenu inherits Rectangle {
    visible: false;

    callback link-clicked(string);

    width: 560px;
    height: visible ? menu-content.preferred-height + Theme.sp-4 : 0px;
    background: Theme.bg-elevated;
    border-radius: Theme.radius-md;
    border-width: 1px;
    border-color: Theme.border-subtle;

    animate height { duration: Theme.dur-normal; easing: Theme.ease-standard; }
    animate opacity { duration: Theme.dur-fast; }
    opacity: visible ? 1 : 0;

    menu-content := VerticalLayout {
        padding: Theme.sp-4;
        spacing: Theme.sp-4;

        HorizontalLayout {
            spacing: Theme.sp-6;

            VerticalLayout {
                spacing: Theme.sp-2;
                horizontal-stretch: 1;

                Text {
                    text: "Navigation";
                    color: Theme.text-tertiary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-xs;
                    font-weight: Theme.weight-semibold;
                    letter-spacing: Theme.tracking-wide;
                }

                for item in ["Dashboard", "Analytics", "Reports", "Settings"]: Rectangle {
                    height: 30px;
                    background: item-ta.pressed ? Theme.surface-pressed
                        : item-ta.has-hover ? Theme.surface-hover
                        : #00000000;
                    border-radius: Theme.radius-xs;

                    HorizontalLayout {
                        padding-left: Theme.sp-2;
                        padding-right: Theme.sp-2;

                        Text {
                            text: item;
                            color: Theme.text-primary;
                            font-family: Theme.font-ui;
                            font-size: Theme.text-sm;
                            vertical-alignment: center;
                        }
                    }

                    item-ta := TouchArea {
                        clicked => { root.link-clicked(item); }
                    }
                }
            }

            Rectangle {
                width: 1px;
                background: Theme.border-subtle;
                vertical-stretch: 1;
            }

            VerticalLayout {
                spacing: Theme.sp-2;
                horizontal-stretch: 1;

                Text {
                    text: "Actions";
                    color: Theme.text-tertiary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-xs;
                    font-weight: Theme.weight-semibold;
                    letter-spacing: Theme.tracking-wide;
                }

                for action in ["Create New", "Import Data", "Export Report", "Team Settings"]: Rectangle {
                    height: 30px;
                    background: act-ta.pressed ? Theme.surface-pressed
                        : act-ta.has-hover ? Theme.surface-hover
                        : #00000000;
                    border-radius: Theme.radius-xs;

                    HorizontalLayout {
                        padding-left: Theme.sp-2;
                        padding-right: Theme.sp-2;

                        Text {
                            text: action;
                            color: Theme.text-primary;
                            font-family: Theme.font-ui;
                            font-size: Theme.text-sm;
                            vertical-alignment: center;
                        }
                    }

                    act-ta := TouchArea {
                        clicked => { root.link-clicked(action); }
                    }
                }
            }
        }

        Rectangle {
            height: 1px;
            background: Theme.border-subtle;
        }

        HorizontalLayout {
            padding-left: Theme.sp-2;
            padding-right: Theme.sp-2;

            Text {
                text: "Press Enter to select, Esc to close";
                color: Theme.text-tertiary;
                font-family: Theme.font-ui;
                font-size: Theme.text-xs;
                vertical-alignment: center;
            }
        }
    }
}
```

#### `MiniSidebar` ([slint/navigation/MiniSidebar.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/MiniSidebar.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component MiniSidebar inherits Rectangle {
    in-out property <int> active: 0;

    width: 56px;
    background: Theme.bg-elevated;

    VerticalLayout {
        padding-top: 8px;
        spacing: 4px;
        alignment: start;

        Rectangle {
            width: 40px;
            height: 40px;
            x: 8px;
            background: root.active == 0 ? Theme.accent-subtle : #00000000;
            border-radius: Theme.radius-md;

            Image {
                source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/house.svg");
                width: 18px;
                height: 18px;
                colorize: root.active == 0 ? Theme.accent : Theme.text-tertiary;
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
            }

            TouchArea { clicked => { root.active = 0; } }
        }

        Rectangle {
            width: 40px;
            height: 40px;
            x: 8px;
            background: root.active == 1 ? Theme.accent-subtle : #00000000;
            border-radius: Theme.radius-md;

            Image {
                source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/folder.svg");
                width: 18px;
                height: 18px;
                colorize: root.active == 1 ? Theme.accent : Theme.text-tertiary;
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
            }

            TouchArea { clicked => { root.active = 1; } }
        }

        Rectangle {
            width: 40px;
            height: 40px;
            x: 8px;
            background: root.active == 2 ? Theme.accent-subtle : #00000000;
            border-radius: Theme.radius-md;

            Image {
                source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/gear.svg");
                width: 18px;
                height: 18px;
                colorize: root.active == 2 ? Theme.accent : Theme.text-tertiary;
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
            }

            TouchArea { clicked => { root.active = 2; } }
        }
    }
}
```

#### `NavBadge` ([slint/navigation/NavBadge.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/NavBadge.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component NavBadge inherits Rectangle {
    in property <string> label: "";
    in property <int> count: 0;
    in property <bool> active: false;

    callback clicked();

    height: 36px;
    background: active ? Theme.accent-subtle
        : ta.pressed ? Theme.surface-pressed
        : ta.has-hover ? Theme.surface-hover
        : #00000000;
    border-radius: Theme.radius-sm;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        padding-left: Theme.sp-3;
        padding-right: Theme.sp-3;
        spacing: Theme.sp-2;
        alignment: stretch;

        FaIcon {
            source: @image-url("");
            fa-size: Theme.icon-md;
            fa-color: active ? Theme.accent : Theme.text-tertiary;
            vertical-alignment: center;
            horizontal-stretch: 0;
        }

        Text {
            text: root.label;
            color: active ? Theme.accent : Theme.text-primary;
            font-family: Theme.font-ui;
            font-size: Theme.text-sm;
            font-weight: active ? Theme.weight-medium : Theme.weight-regular;
            vertical-alignment: center;
            horizontal-stretch: 1;
        }

        if count > 0: Rectangle {
            height: 20px;
            min-width: 20px;
            background: Theme.accent;
            border-radius: Theme.radius-full;

            HorizontalLayout {
                alignment: center;

                Text {
                    text: root.count > 99 ? "99+" : root.count;
                    color: Theme.on-accent;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-xs;
                    font-weight: Theme.weight-semibold;
                    vertical-alignment: center;
                }
            }
        }
    }

    ta := TouchArea {
        clicked => { root.clicked(); }
    }
}
```

#### `NavCollapse` ([slint/navigation/NavCollapse.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/NavCollapse.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component NavCollapse inherits Rectangle {
    in property <string> label: "";
    in property <bool> expanded: false;

    callback toggled();

    height: expanded ? header-row.height + content.preferred-height : header-row.height;
    background: #00000000;

    header-row := Rectangle {
        height: 36px;
        background: ta.pressed ? Theme.surface-pressed
            : ta.has-hover ? Theme.surface-hover
            : #00000000;
        border-radius: Theme.radius-sm;

        animate background { duration: Theme.dur-fast; }

        HorizontalLayout {
            padding-left: Theme.sp-3;
            padding-right: Theme.sp-3;
            spacing: Theme.sp-2;

            Text {
                text: root.label;
                color: Theme.text-primary;
                font-family: Theme.font-ui;
                font-size: Theme.text-sm;
                font-weight: Theme.weight-medium;
                vertical-alignment: center;
                horizontal-stretch: 1;
            }

            Text {
                text: "\u{25B6}";
                color: Theme.text-tertiary;
                font-size: 10px;
                vertical-alignment: center;

                rotate-angle: root.expanded ? 90deg : 0deg;
                animate rotate-angle { duration: Theme.dur-normal; easing: Theme.ease-standard; }
            }
        }

        ta := TouchArea {
            clicked => { root.toggled(); }
        }
    }

    if expanded: content := VerticalLayout {
        padding-left: Theme.sp-4;
        padding-top: Theme.sp-1;
        padding-bottom: Theme.sp-1;
        spacing: Theme.sp-1;
    }
}
```

#### `NavGroupHeader` ([slint/navigation/NavGroupHeader.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/NavGroupHeader.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component NavGroupHeader inherits Rectangle {
    in property <string> label: "";

    height: 28px;

    HorizontalLayout {
        padding-left: Theme.sp-3;
        padding-right: Theme.sp-3;
        alignment: start;

        Text {
            text: root.label;
            color: Theme.text-tertiary;
            font-family: Theme.font-ui;
            font-size: Theme.text-xs;
            font-weight: Theme.weight-semibold;
            letter-spacing: Theme.tracking-wide;
            vertical-alignment: center;
        }
    }
}
```

#### `NavItem` ([slint/navigation/NavItem.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/NavItem.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component NavItem inherits Rectangle {
    in property <string> label: "";
    in property <bool> active: false;
    in property <string> icon-name: "";

    callback clicked();

    height: 36px;
    background: active ? Theme.accent-subtle
        : ta.pressed ? Theme.surface-pressed
        : ta.has-hover ? Theme.surface-hover
        : #00000000;
    border-radius: Theme.radius-sm;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        padding-left: Theme.sp-3;
        padding-right: Theme.sp-3;
        spacing: Theme.sp-2;
        alignment: stretch;

        if icon-name != "": FaIcon {
            source: @image-url("");
            fa-size: Theme.icon-md;
            fa-color: active ? Theme.accent : Theme.text-tertiary;
            vertical-alignment: center;
            horizontal-stretch: 0;
        }

        Text {
            text: root.label;
            color: active ? Theme.accent : Theme.text-primary;
            font-family: Theme.font-ui;
            font-size: Theme.text-sm;
            font-weight: active ? Theme.weight-medium : Theme.weight-regular;
            vertical-alignment: center;
            horizontal-stretch: 1;
        }
    }

    ta := TouchArea {
        clicked => { root.clicked(); }
    }
}
```

#### `NestedTabs` ([slint/navigation/NestedTabs.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/NestedTabs.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component NestedTabs inherits Rectangle {
    in-out property <int> active-primary: 0;
    in-out property <int> active-secondary: 0;

    VerticalLayout {
        Rectangle {
            height: 40px;
            background: Theme.bg-surface;

            HorizontalLayout {
                padding: 4px;
                spacing: 4px;

                Rectangle {
                    height: 32px;
                    background: root.active-primary == 0 ? Theme.accent-subtle : #00000000;
                    border-radius: Theme.radius-sm;
                    horizontal-stretch: 0;

                    Text {
                        x: 12px;
                        text: "General";
                        color: root.active-primary == 0 ? Theme.accent : Theme.text-secondary;
                        font-size: 13px;
                        font-weight: root.active-primary == 0 ? Theme.weight-semibold : Theme.weight-regular;
                        vertical-alignment: center;
                    }

                    TouchArea { clicked => { root.active-primary = 0; } }
                }

                Rectangle {
                    height: 32px;
                    background: root.active-primary == 1 ? Theme.accent-subtle : #00000000;
                    border-radius: Theme.radius-sm;
                    horizontal-stretch: 0;

                    Text {
                        x: 12px;
                        text: "Advanced";
                        color: root.active-primary == 1 ? Theme.accent : Theme.text-secondary;
                        font-size: 13px;
                        font-weight: root.active-primary == 1 ? Theme.weight-semibold : Theme.weight-regular;
                        vertical-alignment: center;
                    }

                    TouchArea { clicked => { root.active-primary = 1; } }
                }
            }
        }

        Rectangle {
            height: 36px;
            background: Theme.bg-overlay;

            HorizontalLayout {
                padding: 4px;
                spacing: 4px;

                Rectangle {
                    height: 28px;
                    background: root.active-secondary == 0 ? Theme.bg-surface : #00000000;
                    border-radius: Theme.radius-sm;
                    horizontal-stretch: 0;

                    Text {
                        x: 10px;
                        text: "Sub A";
                        color: Theme.text-secondary;
                        font-size: 12px;
                        vertical-alignment: center;
                    }

                    TouchArea { clicked => { root.active-secondary = 0; } }
                }

                Rectangle {
                    height: 28px;
                    background: root.active-secondary == 1 ? Theme.bg-surface : #00000000;
                    border-radius: Theme.radius-sm;
                    horizontal-stretch: 0;

                    Text {
                        x: 10px;
                        text: "Sub B";
                        color: Theme.text-secondary;
                        font-size: 12px;
                        vertical-alignment: center;
                    }

                    TouchArea { clicked => { root.active-secondary = 1; } }
                }
            }
        }

        Rectangle {
            vertical-stretch: 1;
            background: Theme.bg-base;

            Text {
                x: 16px;
                y: 16px;
                text: "Content area";
                color: Theme.text-tertiary;
                font-size: 14px;
            }
        }
    }
}
```

#### `Pagination` ([slint/navigation/Pagination.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/Pagination.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component Pagination inherits Rectangle {
    in-out property <int> current-page: 1;
    in property <int> total-pages: 10;

    callback page-changed(int);

    height: 32px;
    background: transparent;

    HorizontalLayout {
        spacing: Theme.sp-1;

        // Previous
        Rectangle {
            width: 32px;
            height: 32px;
            horizontal-stretch: 0;
            border-radius: Theme.radius-sm;
            background: prev-ta.pressed ? Theme.surface-pressed
                : prev-ta.has-hover ? Theme.surface-hover : transparent;
            visible: current-page > 1;

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: "‹";
                color: Theme.text-secondary;
                font-size: Theme.text-lg;
                vertical-alignment: center;
            }

            prev-ta := TouchArea {
                clicked => {
                    if root.current-page > 1 {
                        root.current-page = root.current-page - 1;
                        root.page-changed(root.current-page);
                    }
                }
            }
        }

        // Pages
        for page[i] in root.total-pages: Rectangle {
            width: 32px;
            height: 32px;
            horizontal-stretch: 0;
            border-radius: Theme.radius-sm;
            background: page == root.current-page ? Theme.accent
                : page-ta.pressed ? Theme.surface-pressed
                : page-ta.has-hover ? Theme.surface-hover
                : transparent;

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: "\{page + 1}";
                color: page == root.current-page ? Theme.on-accent : Theme.text-secondary;
                font-family: Theme.font-ui;
                font-size: Theme.text-sm;
                font-weight: page == root.current-page ? Theme.weight-semibold : Theme.weight-regular;
                vertical-alignment: center;
            }

            page-ta := TouchArea {
                clicked => {
                    root.current-page = page + 1;
                    root.page-changed(root.current-page);
                }
            }
        }

        // Next
        Rectangle {
            width: 32px;
            height: 32px;
            horizontal-stretch: 0;
            border-radius: Theme.radius-sm;
            background: next-ta.pressed ? Theme.surface-pressed
                : next-ta.has-hover ? Theme.surface-hover : transparent;
            visible: current-page < total-pages;

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: "›";
                color: Theme.text-secondary;
                font-size: Theme.text-lg;
                vertical-alignment: center;
            }

            next-ta := TouchArea {
                clicked => {
                    if root.current-page < root.total-pages {
                        root.current-page = root.current-page + 1;
                        root.page-changed(root.current-page);
                    }
                }
            }
        }
    }
}
```

#### `PrevNextPagination` ([slint/navigation/PrevNextPagination.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/PrevNextPagination.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component PrevNextPagination inherits Rectangle {
    in property <int> current: 1;
    in property <int> total: 1;

    callback prev-clicked();
    callback next-clicked();

    height: 40px;
    background: #00000000;

    HorizontalLayout {
        alignment: center;
        spacing: Theme.sp-4;

        // Prev button
        Rectangle {
            width: 80px;
            height: 36px;
            horizontal-stretch: 0;
            border-radius: Theme.radius-sm;
            background: prev-ta.pressed ? Theme.surface-pressed
                : prev-ta.has-hover ? Theme.surface-hover
                : #00000000;
            visible: root.current > 1;

            HorizontalLayout {
                alignment: center;
                spacing: Theme.sp-2;

                FaIcon {
                    source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/chevron-left.svg");
                    fa-size: 12px;
                    fa-color: Theme.text-secondary;
                }

                Text {
                    text: "Prev";
                    color: Theme.text-secondary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-sm;
                    font-weight: Theme.weight-medium;
                    vertical-alignment: center;
                }
            }

            prev-ta := TouchArea {
                clicked => {
                    root.prev-clicked();
                }
            }
        }

        // Current / Total
        Rectangle {
            height: 36px;
            horizontal-stretch: 0;
            background: #00000000;

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: root.current + " / " + root.total;
                color: Theme.text-primary;
                font-family: Theme.font-ui;
                font-size: Theme.text-sm;
                font-weight: Theme.weight-semibold;
                vertical-alignment: center;
            }
        }

        // Next button
        Rectangle {
            width: 80px;
            height: 36px;
            horizontal-stretch: 0;
            border-radius: Theme.radius-sm;
            background: next-ta.pressed ? Theme.surface-pressed
                : next-ta.has-hover ? Theme.surface-hover
                : #00000000;
            visible: root.current < root.total;

            HorizontalLayout {
                alignment: center;
                spacing: Theme.sp-2;

                Text {
                    text: "Next";
                    color: Theme.text-secondary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-sm;
                    font-weight: Theme.weight-medium;
                    vertical-alignment: center;
                }

                FaIcon {
                    source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/chevron-right.svg");
                    fa-size: 12px;
                    fa-color: Theme.text-secondary;
                }
            }

            next-ta := TouchArea {
                clicked => {
                    root.next-clicked();
                }
            }
        }
    }
}
```

#### `ScrollableTabs` ([slint/navigation/ScrollableTabs.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/ScrollableTabs.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component ScrollableTabs inherits Rectangle {
    in-out property <int> active: 0;

    height: 40px;
    background: Theme.bg-surface;

    HorizontalLayout {
        padding: 4px;
        spacing: 4px;

        Rectangle {
            height: 32px;
            background: root.active == 0 ? Theme.accent-subtle : #00000000;
            border-radius: Theme.radius-sm;
            horizontal-stretch: 0;
            min-width: 80px;

            Text {
                x: (parent.width - self.width) / 2;
                text: "Tab 1";
                color: root.active == 0 ? Theme.accent : Theme.text-secondary;
                font-size: 13px;
                font-weight: root.active == 0 ? Theme.weight-semibold : Theme.weight-regular;
                vertical-alignment: center;
            }

            TouchArea { clicked => { root.active = 0; } }
        }

        Rectangle {
            height: 32px;
            background: root.active == 1 ? Theme.accent-subtle : #00000000;
            border-radius: Theme.radius-sm;
            horizontal-stretch: 0;
            min-width: 80px;

            Text {
                x: (parent.width - self.width) / 2;
                text: "Tab 2";
                color: root.active == 1 ? Theme.accent : Theme.text-secondary;
                font-size: 13px;
                font-weight: root.active == 1 ? Theme.weight-semibold : Theme.weight-regular;
                vertical-alignment: center;
            }

            TouchArea { clicked => { root.active = 1; } }
        }

        Rectangle {
            height: 32px;
            background: root.active == 2 ? Theme.accent-subtle : #00000000;
            border-radius: Theme.radius-sm;
            horizontal-stretch: 0;
            min-width: 80px;

            Text {
                x: (parent.width - self.width) / 2;
                text: "Tab 3";
                color: root.active == 2 ? Theme.accent : Theme.text-secondary;
                font-size: 13px;
                font-weight: root.active == 2 ? Theme.weight-semibold : Theme.weight-regular;
                vertical-alignment: center;
            }

            TouchArea { clicked => { root.active = 2; } }
        }
    }
}
```

#### `SidebarItem` ([slint/navigation/SidebarItem.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/SidebarItem.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component SidebarItem inherits Rectangle {
    in property <string> text: "";
    in property <bool> active: false;
    in property <string> icon: "";

    callback clicked();

    height: 36px;
    background: active ? Theme.accent-subtle
        : ta.pressed ? Theme.surface-pressed
        : ta.has-hover ? Theme.surface-hover
        : transparent;
    border-radius: Theme.radius-sm;

    animate background { duration: Theme.dur-fast; }

    HorizontalLayout {
        padding-left: Theme.sp-3;
        padding-right: Theme.sp-3;
        spacing: Theme.sp-2;
        alignment: stretch;

        if icon != "": Text {
            text: icon;
            color: active ? Theme.accent : Theme.text-tertiary;
            font-size: Theme.text-base;
            vertical-alignment: center;
            horizontal-stretch: 0;
        }

        Text {
            text: root.text;
            color: active ? Theme.accent : Theme.text-primary;
            font-family: Theme.font-ui;
            font-size: Theme.text-sm;
            font-weight: active ? Theme.weight-medium : Theme.weight-regular;
            vertical-alignment: center;
            horizontal-stretch: 1;
        }
    }

    ta := TouchArea {
        clicked => { root.clicked(); }
    }
}
```

#### `Steps` ([slint/navigation/Steps.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/Steps.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component Steps inherits Rectangle {
    in property <int> current-step: 0;
    in property <[string]> steps: [];

    background: transparent;
    horizontal-stretch: 1;

    HorizontalLayout {
        spacing: Theme.sp-2;

        for step[i] in root.steps: HorizontalLayout {
            horizontal-stretch: 1;
            spacing: Theme.sp-2;

            // Dot
            Rectangle {
                width: 24px;
                height: 24px;
                horizontal-stretch: 0;

                dot := Rectangle {
                    width: 24px;
                    height: 24px;
                    border-radius: Theme.radius-full;
                    background: i <= root.current-step ? Theme.accent : Theme.bg-overlay;
                    border-width: 1px;
                    border-color: i <= root.current-step ? Theme.accent : Theme.border-base;

                    Text {
                        x: (parent.width - self.width) / 2;
                        y: (parent.height - self.height) / 2;
                        text: i < root.current-step ? "✓" : (i + 1);
                        color: i <= root.current-step ? Theme.on-accent : Theme.text-tertiary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-xs;
                        font-weight: Theme.weight-semibold;
                        vertical-alignment: center;
                        horizontal-alignment: center;
                    }
                }
            }

            // Label
            Text {
                text: step;
                color: i <= root.current-step ? Theme.text-primary : Theme.text-tertiary;
                font-family: Theme.font-ui;
                font-size: Theme.text-xs;
                font-weight: i == root.current-step ? Theme.weight-semibold : Theme.weight-regular;
                vertical-alignment: center;
                horizontal-stretch: 1;
            }

            // Connector line
            if i < root.steps.length - 1: Rectangle {
                width: 20px;
                height: 2px;
                horizontal-stretch: 0;
                y: 11px;
                background: i < root.current-step ? Theme.accent : Theme.border-subtle;
            }
        }
    }
}
```

#### `StickyHeader` ([slint/navigation/StickyHeader.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/StickyHeader.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component StickyHeader inherits Rectangle {
    in property <string> title: "";

    height: 48px;
    background: Theme.bg-elevated;

    Text {
        x: 16px;
        text: root.title;
        color: Theme.text-primary;
        font-size: 16px;
        font-weight: Theme.weight-semibold;
        vertical-alignment: center;
    }
}
```

#### `TabBar` ([slint/navigation/TabBar.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/TabBar.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { RoundImageButton } from "RoundImageButton.slint";

export component TabBar inherits Rectangle {
    in property <[string]> tabs: [];
    in property <int> current-index: 0;
    in property <bool> new-tab-visible: true;

    callback new-tab-clicked();
    callback tab-clicked(int);
    callback tab-close-clicked(int);

    height: 40px;
    background: transparent;

    HorizontalLayout {
        x: 0;
        y: 0;
        width: parent.width;
        height: parent.height;
        spacing: Theme.spacing.small / 2;

        for tab[idx] in root.tabs: TabButton {
            horizontal-stretch: 1;
            text: tab;
            checked: idx == root.current-index;
            clicked => { root.tab-clicked(idx); }
            close-clicked => { root.tab-close-clicked(idx); }
        }

        if new-tab-visible: RoundImageButton {
            horizontal-stretch: 0;
            width: 31px;
            height: 31px;
            icon-margins: 2px;
            source: @image-url("../src/images/light/add.svg");
            clicked => { root.new-tab-clicked(); }
        }

        Rectangle { horizontal-stretch: 1; }
    }
}
```

#### `TabButton` ([slint/navigation/TabButton.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/TabButton.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component TabButton inherits Rectangle {
    in property <bool> checked: false;
    in property <string> text: "";
    in property <bool> close-visible: true;

    callback clicked();
    callback close-clicked();

    height: Theme.button-height-md;

    Rectangle {
        x: Theme.sp-0-5;
        y: Theme.sp-0-5;
        width: parent.width - Theme.sp-1;
        height: parent.height - Theme.sp-1;
        border-radius: Theme.radius-sm;
        background: ta.pressed ? Theme.surface-pressed
            : ta.has-hover ? Theme.surface-hover
            : transparent;
    }

    if checked: Rectangle {
        x: Theme.sp-0-5;
        y: Theme.sp-0-5;
        width: parent.width - Theme.sp-1;
        height: parent.height - Theme.sp-1;
        border-radius: Theme.radius-sm;
        background: Theme.accent;
    }

    ta := TouchArea {
        width: close-visible && checked ? parent.width - 24px : parent.width;
        height: parent.height;
        clicked => { root.clicked(); }
    }

    HorizontalLayout {
        x: Theme.sp-1;
        y: Theme.sp-0-5;
        width: parent.width - Theme.sp-2;
        height: parent.height - Theme.sp-1;
        spacing: 0px;
        alignment: center;

        Text {
            horizontal-stretch: 1;
            text: root.text;
            color: checked ? Theme.on-accent : Theme.text-primary;
            font-family: Theme.font-ui;
            font-size: Theme.text-base;
            font-weight: Theme.weight-medium;
            vertical-alignment: center;
            horizontal-alignment: center;
        }

        if close-visible && checked: Rectangle {
            width: 20px;
            height: 20px;
            border-radius: 10px;
            horizontal-stretch: 0;

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: "\u{00D7}";
                color: Theme.on-accent;
                font-size: 12px;
                vertical-alignment: center;
                horizontal-alignment: center;
            }

            TouchArea {
                clicked => { root.close-clicked(); }
            }
        }
    }
}
```

#### `Taskbar` ([slint/navigation/Taskbar.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/Taskbar.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component Taskbar inherits Rectangle {
    background: Theme.bg-base;
    border-width: 1px;
    border-color: Theme.border-default;
    height: 40px;
    horizontal-stretch: 0;

    in property <string> clock-text: "12:00 PM";
    in-out property <int> active-app-index: -1;
    in property <[string]> pinned-apps: ["🌐", "📁", "✉️", "🎵", "⚙️"];
    in property <string> system-tray-icons: "🔊 📶 🔋";

    HorizontalLayout {
        padding-left: Theme.sp-2;
        padding-right: Theme.sp-2;

        Rectangle {
            width: 32px;
            height: 32px;
            y: 4px;

            Text {
                text: "🦞";
                font-size: 16px;
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
            }
        }

        HorizontalLayout {
            width: parent.width - 64px - 120px;
            spacing: Theme.sp-1;
            alignment: center;

            for app[idx] in root.pinned-apps: Rectangle {
                width: 36px;
                height: 32px;
                border-radius: 8px;
                background: active-app-index == idx ? Theme.bg-surface : transparent;

                Text {
                    text: app;
                    font-size: 16px;
                    x: (parent.width - self.width) / 2;
                    y: (parent.height - self.height) / 2;
                }

                TouchArea {
                    clicked => { root.active-app-index = idx; }
                }
            }
        }

        Rectangle {
            width: 120px;
            height: 32px;
            y: 4px;
            border-radius: 8px;
            background: Theme.bg-surface;

            Text {
                text: root.clock-text;
                font-size: 12px;
                font-weight: Theme.weight-medium;
                color: Theme.text-primary;
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
            }
        }
    }
}
```

#### `TokenPagination` ([slint/navigation/TokenPagination.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/TokenPagination.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component TokenPagination inherits Rectangle {
    in property <bool> has-more: false;

    callback load-more();

    height: 44px;
    background: #00000000;

    Rectangle {
        width: 160px;
        height: 36px;
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
        border-radius: Theme.radius-sm;
        border-width: 1px;
        border-color: Theme.border-base;
        background: btn-ta.pressed ? Theme.surface-pressed
            : btn-ta.has-hover ? Theme.surface-hover
            : #00000000;
        visible: root.has-more;

        HorizontalLayout {
            alignment: center;
            spacing: Theme.sp-2;

            FaIcon {
                source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/angles-down.svg");
                fa-size: 12px;
                fa-color: Theme.text-secondary;
            }

            Text {
                text: "Load more";
                color: Theme.text-secondary;
                font-family: Theme.font-ui;
                font-size: Theme.text-sm;
                font-weight: Theme.weight-medium;
                vertical-alignment: center;
            }
        }

        btn-ta := TouchArea {
            clicked => {
                root.load-more();
            }
        }
    }
}
```

#### `TopAppBar` ([slint/navigation/TopAppBar.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/TopAppBar.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component TopAppBar inherits Rectangle {
    in property <string> title: "";
    in property <bool> has-back: false;

    height: 48px;
    background: Theme.bg-elevated;

    HorizontalLayout {
        padding-left: 8px;
        padding-right: 16px;

        if root.has-back: Rectangle {
            width: 40px;
            height: 40px;

            Image {
                source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/arrow-left.svg");
                width: 16px;
                height: 16px;
                colorize: Theme.text-primary;
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
            }

            TouchArea { clicked => { } }
        }

        Text {
            text: root.title;
            color: Theme.text-primary;
            font-size: 18px;
            font-weight: Theme.weight-semibold;
            vertical-alignment: center;
            horizontal-stretch: 1;
        }
    }
}
```

#### `VerticalStepper` ([slint/navigation/VerticalStepper.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/VerticalStepper.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component VerticalStepper inherits Rectangle {
    in property <[string]> steps: [];
    in property <int> current: 0;

    background: #00000000;
    horizontal-stretch: 1;

    VerticalLayout {
        spacing: 0;

        for step[idx] in root.steps: VerticalLayout {
            spacing: 0;
            horizontal-stretch: 1;

            HorizontalLayout {
                spacing: Theme.sp-3;

                // Circle column
                VerticalLayout {
                    horizontal-stretch: 0;
                    spacing: 0;

                    Rectangle {
                        width: 28px;
                        height: 28px;
                        horizontal-stretch: 0;
                        border-radius: Theme.radius-full;
                        background: idx < root.current ? Theme.accent
                            : idx == root.current ? Theme.accent
                            : Theme.bg-overlay;
                        border-width: idx <= root.current ? 0px : 2px;
                        border-color: Theme.border-base;

                        Text {
                            x: (parent.width - self.width) / 2;
                            y: (parent.height - self.height) / 2;
                            text: idx < root.current ? "✓" : (idx + 1);
                            color: idx <= root.current ? Theme.on-accent : Theme.text-tertiary;
                            font-family: Theme.font-ui;
                            font-size: Theme.text-xs;
                            font-weight: Theme.weight-semibold;
                            vertical-alignment: center;
                            horizontal-alignment: center;
                        }
                    }

                    // Connector line below circle
                    if idx < root.steps.length - 1: Rectangle {
                        width: 2px;
                        height: 28px;
                        horizontal-stretch: 0;
                        x: 13px;
                        background: idx < root.current ? Theme.accent : Theme.border-subtle;
                    }
                }

                // Label column
                Text {
                    text: step;
                    color: idx <= root.current ? Theme.text-primary : Theme.text-tertiary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-sm;
                    font-weight: idx == root.current ? Theme.weight-semibold : Theme.weight-regular;
                    vertical-alignment: center;
                    horizontal-stretch: 1;
                }
            }
        }
    }
}
```

#### `VerticalTabBar` ([slint/navigation/VerticalTabBar.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/VerticalTabBar.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component VerticalTabBar inherits Rectangle {
    in-out property <int> active: 0;

    width: 160px;
    background: Theme.bg-surface;

    VerticalLayout {
        padding: 8px;
        spacing: 2px;

        Rectangle {
            height: 36px;
            background: root.active == 0 ? Theme.accent-subtle : #00000000;
            border-radius: Theme.radius-sm;

            Text {
                x: 12px;
                text: "Overview";
                color: root.active == 0 ? Theme.accent : Theme.text-primary;
                font-size: 13px;
                font-weight: root.active == 0 ? Theme.weight-semibold : Theme.weight-regular;
                vertical-alignment: center;
            }

            TouchArea { clicked => { root.active = 0; } }
        }

        Rectangle {
            height: 36px;
            background: root.active == 1 ? Theme.accent-subtle : #00000000;
            border-radius: Theme.radius-sm;

            Text {
                x: 12px;
                text: "Analytics";
                color: root.active == 1 ? Theme.accent : Theme.text-primary;
                font-size: 13px;
                font-weight: root.active == 1 ? Theme.weight-semibold : Theme.weight-regular;
                vertical-alignment: center;
            }

            TouchArea { clicked => { root.active = 1; } }
        }

        Rectangle {
            height: 36px;
            background: root.active == 2 ? Theme.accent-subtle : #00000000;
            border-radius: Theme.radius-sm;

            Text {
                x: 12px;
                text: "Reports";
                color: root.active == 2 ? Theme.accent : Theme.text-primary;
                font-size: 13px;
                font-weight: root.active == 2 ? Theme.weight-semibold : Theme.weight-regular;
                vertical-alignment: center;
            }

            TouchArea { clicked => { root.active = 2; } }
        }
    }
}
```

#### `WorkspaceSwitcher` ([slint/navigation/WorkspaceSwitcher.slint](file:///home/lion/Documents/GitHub/ltk/slint/navigation/WorkspaceSwitcher.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component WorkspaceSwitcher inherits Rectangle {
    background: Theme.bg-raised;
    border-radius: 12px;
    border-width: 1px;
    border-color: Theme.border-default;
    horizontal-stretch: 0;
    width: 220px;
    clip: true;

    in property <int> workspaces: 4;
    in-out property <int> active-workspace: 0;
    in property <[string]> workspace-names: ["Desktop 1", "Desktop 2", "Desktop 3", "Desktop 4"];
    in property <[bool]> workspace-visible: [true, true, true, true];

    VerticalLayout {
        padding: Theme.sp-2;

        Text {
            text: "Workspaces";
            font-size: Theme.text-xs;
            color: Theme.text-tertiary;
            font-weight: Theme.weight-semibold;
        }

        VerticalLayout {
            spacing: Theme.sp-1;

            for ws[idx] in root.workspace-names: Rectangle {
                height: 36px;
                border-radius: 8px;
                background: active-workspace == idx ? Theme.accent-subtle : transparent;
                border-width: 1px;
                border-color: active-workspace == idx ? Theme.accent-subtle : transparent;

                HorizontalLayout {
                    padding-left: Theme.sp-3;
                    padding-right: Theme.sp-3;
                    spacing: Theme.sp-2;

                    Rectangle {
                        width: 24px;
                        height: 24px;
                        border-radius: 6px;
                        background: active-workspace == idx ? Theme.accent : Theme.bg-base;
                        y: 6px;

                        Text {
                            text: "\{Math.mod(idx + 1, 10)}";
                            font-size: 11px;
                            font-weight: Theme.weight-bold;
                            color: active-workspace == idx ? #ffffff : Theme.text-secondary;
                            x: (parent.width - self.width) / 2;
                            y: (parent.height - self.height) / 2;
                        }
                    }

                    Text {
                        text: ws;
                        font-size: Theme.text-sm;
                        color: active-workspace == idx ? Theme.accent : Theme.text-primary;
                        font-weight: active-workspace == idx ? Theme.weight-medium : Theme.weight-regular;
                        y: 6px;
                        overflow: elide;
                    }
                }

                TouchArea {
                    clicked => { root.active-workspace = idx; }
                }
            }
        }
    }
}
```


### 5.6 Cards & Modular Panels Code Manual

#### `AchievementCard` ([slint/cards/AchievementCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/AchievementCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component AchievementCard inherits Rectangle {
    in-out property <string> title: "Achievement Unlocked";
    in-out property <string> description: "Completed 100 tasks";
    in-out property <string> date: "Jul 23, 2026";

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: #FBBF24;
    horizontal-stretch: 1;

    VerticalLayout {
        padding: 20px;
        spacing: 8px;
        alignment: center;

        Rectangle {
            width: 56px;
            height: 56px;
            border-radius: Theme.radius-full;
            background: #FBBF2420;

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: "🏆";
                font-size: 28px;
                vertical-alignment: center;
            }
        }

        Text {
            text: root.title;
            color: #FBBF24;
            font-size: 14px;
            font-weight: Theme.weight-bold;
            horizontal-alignment: center;
        }

        Text {
            text: root.description;
            color: Theme.text-secondary;
            font-size: 12px;
            horizontal-alignment: center;
        }

        Text {
            text: root.date;
            color: Theme.text-tertiary;
            font-size: 11px;
            horizontal-alignment: center;
        }
    }
}
```

#### `AlertCard` ([slint/cards/AlertCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/AlertCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component AlertCard inherits Rectangle {
    in-out property <string> title: "Alert";
    in-out property <string> message: "Something needs your attention.";
    in-out property <string> kind: "info";

    background: Theme.state-info;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    horizontal-stretch: 1;

    HorizontalLayout {
        padding: 16px;
        spacing: 12px;

        Rectangle {
            width: 20px;
            height: 20px;
            horizontal-stretch: 0;

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: root.kind == "error" ? "!" : (root.kind == "warning" ? "⚠" : (root.kind == "success" ? "✓" : "i"));
                color: Theme.text-primary;
                font-size: 14px;
                font-weight: Theme.weight-bold;
                vertical-alignment: center;
            }
        }

        VerticalLayout {
            horizontal-stretch: 1;
            spacing: 2px;

            Text {
                text: root.title;
                color: Theme.text-primary;
                font-size: 13px;
                font-weight: Theme.weight-semibold;
            }

            Text {
                text: root.message;
                color: Theme.text-primary;
                font-size: 12px;
                wrap: word-wrap;
            }
        }
    }
}
```

#### `AppListingCard` ([slint/cards/AppListingCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/AppListingCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component AppListingCard inherits Rectangle {
    in-out property <string> name: "App Name";
    in-out property <string> developer: "Developer";
    in-out property <string> category: "Productivity";
    in-out property <string> rating: "4.8";

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;

    HorizontalLayout {
        padding: 12px;
        spacing: 12px;

        Rectangle {
            width: 56px;
            height: 56px;
            border-radius: Theme.radius-md;
            background: Theme.accent-subtle;
            horizontal-stretch: 0;

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: "📱";
                font-size: 24px;
                vertical-alignment: center;
            }
        }

        VerticalLayout {
            horizontal-stretch: 1;
            spacing: 2px;

            Text {
                text: root.name;
                color: Theme.text-primary;
                font-size: 14px;
                font-weight: Theme.weight-semibold;
            }

            Text {
                text: root.developer;
                color: Theme.text-secondary;
                font-size: 11px;
            }

            HorizontalLayout {
                spacing: 8px;

                Text {
                    text: root.category;
                    color: Theme.text-tertiary;
                    font-size: 11px;
                    horizontal-stretch: 1;
                }

                Text {
                    text: "★ " + root.rating;
                    color: #FBBF24;
                    font-size: 11px;
                    font-weight: Theme.weight-medium;
                }
            }
        }
    }
}
```

#### `ArticlePreviewCard` ([slint/cards/ArticlePreviewCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/ArticlePreviewCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component ArticlePreviewCard inherits Rectangle {
    in-out property <string> title: "Article Title";
    in-out property <string> excerpt: "A brief excerpt from the article...";
    in-out property <string> author: "Author Name";
    in-out property <string> date: "Jul 23, 2026";

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;

    VerticalLayout {
        Rectangle {
            height: 120px;
            background: Theme.bg-overlay;
            clip: true;

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: "Article Image";
                color: Theme.text-tertiary;
                font-size: 13px;
            }
        }

        VerticalLayout {
            padding: 16px;
            spacing: 8px;

            Text {
                text: root.title;
                color: Theme.text-primary;
                font-size: 16px;
                font-weight: Theme.weight-semibold;
                wrap: word-wrap;
            }

            Text {
                text: root.excerpt;
                color: Theme.text-secondary;
                font-size: 13px;
                wrap: word-wrap;
            }

            HorizontalLayout {
                spacing: 8px;

                Text {
                    text: root.author;
                    color: Theme.text-tertiary;
                    font-size: 11px;
                    horizontal-stretch: 1;
                }

                Text {
                    text: root.date;
                    color: Theme.text-tertiary;
                    font-size: 11px;
                }
            }
        }
    }
}
```

#### `BasicCard` ([slint/cards/BasicCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/BasicCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component BasicCard inherits Rectangle {
    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;
}
```

#### `BlogPostCard` ([slint/cards/BlogPostCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/BlogPostCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component BlogPostCard inherits Rectangle {
    in-out property <string> title: "Blog Post Title";
    in-out property <string> excerpt: "A brief excerpt from the blog post content...";
    in-out property <string> date: "Jan 1, 2026";

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;

    VerticalLayout {
        Rectangle {
            height: 120px;
            background: Theme.bg-overlay;
        }

        VerticalLayout {
            padding: 16px;
            spacing: 8px;

            Text {
                text: root.title;
                color: Theme.text-primary;
                font-size: 16px;
                font-weight: Theme.weight-semibold;
            }

            Text {
                text: root.excerpt;
                color: Theme.text-secondary;
                font-size: 13px;
                wrap: word-wrap;
            }

            Text {
                text: root.date;
                color: Theme.text-tertiary;
                font-size: 12px;
            }
        }
    }
}
```

#### `ContactCard` ([slint/cards/ContactCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/ContactCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component ContactCard inherits Rectangle {
    in-out property <string> name: "Contact Name";
    in-out property <string> phone: "+1 (555) 000-0000";
    in-out property <string> email: "email@example.com";

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;

    HorizontalLayout {
        padding: 16px;
        spacing: 12px;

        Rectangle {
            width: 48px;
            height: 48px;
            border-radius: Theme.radius-full;
            background: Theme.accent-subtle;

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: "?";
                color: Theme.accent;
                font-size: 18px;
                font-weight: Theme.weight-bold;
                vertical-alignment: center;
            }
        }

        VerticalLayout {
            horizontal-stretch: 1;
            spacing: 2px;

            Text {
                text: root.name;
                color: Theme.text-primary;
                font-size: 14px;
                font-weight: Theme.weight-semibold;
            }

            Text {
                text: root.phone;
                color: Theme.text-secondary;
                font-size: 12px;
            }

            Text {
                text: root.email;
                color: Theme.text-tertiary;
                font-size: 12px;
            }
        }
    }
}
```

#### `CourseCard` ([slint/cards/CourseCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/CourseCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component CourseCard inherits Rectangle {
    in-out property <string> title: "Course Title";
    in-out property <string> instructor: "Instructor";
    in-out property <string> lessons: "24 lessons";
    in-out property <string> duration: "8h 30m";

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;

    VerticalLayout {
        Rectangle {
            height: 100px;
            background: Theme.accent-subtle;
            clip: true;

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: "📚";
                font-size: 32px;
            }
        }

        VerticalLayout {
            padding: 16px;
            spacing: 4px;

            Text {
                text: root.title;
                color: Theme.text-primary;
                font-size: 14px;
                font-weight: Theme.weight-semibold;
                wrap: word-wrap;
            }

            Text {
                text: root.instructor;
                color: Theme.text-secondary;
                font-size: 12px;
            }

            HorizontalLayout {
                spacing: 12px;

                Text {
                    text: root.lessons;
                    color: Theme.text-tertiary;
                    font-size: 11px;
                }

                Text {
                    text: root.duration;
                    color: Theme.text-tertiary;
                    font-size: 11px;
                }
            }
        }
    }
}
```

#### `ElevatedCard` ([slint/cards/ElevatedCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/ElevatedCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component ElevatedCard inherits Rectangle {
    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    horizontal-stretch: 1;

    drop-shadow-color: Theme.dark-mode ? #40000000 : #20000000;
    drop-shadow-blur: 12px;
    drop-shadow-offset-y: 4px;
}
```

#### `EmptyCard` ([slint/cards/EmptyCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/EmptyCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component EmptyCard inherits Rectangle {
    in-out property <string> label: "Empty";

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;

    Rectangle {
        x: 1px; y: 1px;
        width: parent.width - 2px;
        height: parent.height - 2px;
        border-radius: Theme.radius-lg;
        border-width: 1px;
        border-color: Theme.text-tertiary;
    }

    Text {
        text: root.label;
        color: Theme.text-tertiary;
        font-size: 14px;
        horizontal-alignment: center;
        vertical-alignment: center;
    }
}
```

#### `EventCard` ([slint/cards/EventCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/EventCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component EventCard inherits Rectangle {
    in-out property <string> title: "Event Name";
    in-out property <string> date: "Jul 25, 2026";
    in-out property <string> time: "6:00 PM";
    in-out property <string> location: "San Francisco, CA";

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;

    HorizontalLayout {
        Rectangle {
            width: 64px;
            background: Theme.accent-subtle;
            horizontal-stretch: 0;

            VerticalLayout {
                alignment: center;

                Text {
                    x: (parent.width - self.width) / 2;
                    text: root.date;
                    color: Theme.accent;
                    font-size: 10px;
                    font-weight: Theme.weight-bold;
                    horizontal-alignment: center;
                }
            }
        }

        VerticalLayout {
            padding: 16px;
            spacing: 4px;
            horizontal-stretch: 1;

            Text {
                text: root.title;
                color: Theme.text-primary;
                font-size: 14px;
                font-weight: Theme.weight-semibold;
            }

            HorizontalLayout {
                spacing: 12px;

                Text {
                    text: root.time;
                    color: Theme.text-secondary;
                    font-size: 12px;
                }

                Text {
                    text: "📍 " + root.location;
                    color: Theme.text-tertiary;
                    font-size: 12px;
                }
            }
        }
    }
}
```

#### `ExpandableCard` ([slint/cards/ExpandableCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/ExpandableCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component ExpandableCard inherits Rectangle {
    in-out property <string> title: "Expandable Section";
    in-out property <string> content: "Hidden content revealed on expand";
    in-out property <bool> expanded: false;

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;

    VerticalLayout {
        Rectangle {
            height: 48px;
            background: Theme.bg-raised;
            border-radius: Theme.radius-lg;

            HorizontalLayout {
                padding-left: 16px;
                padding-right: 16px;

                Text {
                    text: root.title;
                    color: Theme.text-primary;
                    font-size: 14px;
                    font-weight: Theme.weight-medium;
                    vertical-alignment: center;
                }

                Rectangle { horizontal-stretch: 1; }

                Rectangle {
                    width: 14px;
                    height: 14px;
                    y: (parent.height - self.height) / 2;

                    Image {
                        source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/chevron-down.svg");
                        width: 14px;
                        height: 14px;
                        visible: !root.expanded;
                        colorize: Theme.text-tertiary;
                    }

                    Image {
                        source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/chevron-up.svg");
                        width: 14px;
                        height: 14px;
                        visible: root.expanded;
                        colorize: Theme.text-tertiary;
                    }
                }
            }

            TouchArea { clicked => { root.expanded = !root.expanded; } }
        }

        if root.expanded: Rectangle {
            background: Theme.bg-surface;
            border-radius: Theme.radius-lg;

            VerticalLayout {
                padding: 16px;
                Text {
                    text: root.content;
                    color: Theme.text-secondary;
                    font-size: 13px;
                    wrap: word-wrap;
                }
            }
        }
    }
}
```

#### `FeatureCard` ([slint/cards/FeatureCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/FeatureCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component FeatureCard inherits Rectangle {
    in-out property <string> title: "Feature";
    in-out property <string> description: "Feature description goes here";

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;

    VerticalLayout {
        padding: 20px;
        spacing: 12px;

        Rectangle {
            width: 44px;
            height: 44px;
            background: Theme.accent-subtle;
            border-radius: Theme.radius-md;

            Image {
                source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/star.svg");
                width: 20px;
                height: 20px;
                colorize: Theme.accent;
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
            }
        }

        Text {
            text: root.title;
            color: Theme.text-primary;
            font-size: 16px;
            font-weight: Theme.weight-semibold;
        }

        Text {
            text: root.description;
            color: Theme.text-secondary;
            font-size: 13px;
            wrap: word-wrap;
        }
    }
}
```

#### `FileCard` ([slint/cards/FileCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/FileCard.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component FileCard inherits Rectangle {
    in-out property <string> filename: "document.pdf";
    in-out property <string> size: "2.4 MB";

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;

    HorizontalLayout {
        padding: 12px;
        spacing: 12px;

        Rectangle {
            width: 44px;
            height: 44px;
            background: Theme.accent-subtle;
            border-radius: Theme.radius-md;

            FaIcon {
                source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/file.svg");
                fa-size: 20px;
                fa-color: Theme.accent;
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
            }
        }

        VerticalLayout {
            spacing: 2px;

            Text {
                text: root.filename;
                color: Theme.text-primary;
                font-size: 14px;
                font-weight: Theme.weight-medium;
            }

            Text {
                text: root.size;
                color: Theme.text-tertiary;
                font-size: 12px;
            }
        }
    }
}
```

#### `FlipCard` ([slint/cards/FlipCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/FlipCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component FlipCard inherits Rectangle {
    in-out property <string> front-title: "Front";
    in-out property <string> back-title: "Back";
    in-out property <bool> flipped: false;

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    height: 200px;
    horizontal-stretch: 1;

    front := Rectangle {
        background: Theme.bg-surface;
        border-radius: Theme.radius-lg;
        visible: !root.flipped;

        Text {
            text: root.front-title;
            color: Theme.text-primary;
            font-size: 18px;
            font-weight: Theme.weight-semibold;
            horizontal-alignment: center;
            vertical-alignment: center;
        }
    }

    back := Rectangle {
        background: Theme.accent-subtle;
        border-radius: Theme.radius-lg;
        visible: root.flipped;

        Text {
            text: root.back-title;
            color: Theme.accent;
            font-size: 18px;
            font-weight: Theme.weight-semibold;
            horizontal-alignment: center;
            vertical-alignment: center;
        }
    }

    TouchArea { clicked => { root.flipped = !root.flipped; } }
}
```

#### `GlassCard` ([slint/cards/GlassCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/GlassCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component GlassCard inherits Rectangle {
    background: Theme.dark-mode ? #20ffffff : #40ffffff;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: #20ffffff;
    horizontal-stretch: 1;
}
```

#### `HorizontalCard` ([slint/cards/HorizontalCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/HorizontalCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component HorizontalCard inherits Rectangle {
    in-out property <string> title: "Horizontal Card";
    in-out property <string> description: "Card with side image layout";

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    height: 100px;
    horizontal-stretch: 1;

    HorizontalLayout {
        Rectangle {
            width: 100px;
            background: Theme.bg-overlay;
            clip: true;

            Text {
                text: "Img";
                color: Theme.text-tertiary;
                font-size: 12px;
                horizontal-alignment: center;
                vertical-alignment: center;
            }
        }

        VerticalLayout {
            padding: 12px;
            spacing: 4px;

            Text {
                text: root.title;
                color: Theme.text-primary;
                font-size: 15px;
                font-weight: Theme.weight-semibold;
            }

            Text {
                text: root.description;
                color: Theme.text-secondary;
                font-size: 13px;
            }
        }
    }
}
```

#### `InteractiveCard` ([slint/cards/InteractiveCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/InteractiveCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component InteractiveCard inherits Rectangle {
    in-out property <string> title: "Card Title";
    in-out property <string> subtitle: "Subtitle text";

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;

    VerticalLayout {
        padding: 16px;
        spacing: 8px;

        Text {
            text: root.title;
            color: Theme.text-primary;
            font-size: 16px;
            font-weight: Theme.weight-semibold;
        }

        Text {
            text: root.subtitle;
            color: Theme.text-secondary;
            font-size: 13px;
        }
    }

    TouchArea { clicked => { } }
}
```

#### `JobListingCard` ([slint/cards/JobListingCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/JobListingCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component JobListingCard inherits Rectangle {
    in-out property <string> title: "Job Title";
    in-out property <string> company: "Company";
    in-out property <string> location: "Remote";
    in-out property <string> salary: "$80K – $120K";

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;

    VerticalLayout {
        padding: 16px;
        spacing: 8px;

        HorizontalLayout {
            spacing: 8px;

            Text {
                text: root.title;
                color: Theme.text-primary;
                font-size: 15px;
                font-weight: Theme.weight-semibold;
                horizontal-stretch: 1;
            }

            Rectangle {
                height: 22px;
                background: Theme.accent-subtle;
                border-radius: Theme.radius-sm;
                horizontal-stretch: 0;

                Text {
                    x: (parent.width - self.width) / 2;
                    y: (parent.height - self.height) / 2;
                    text: "New";
                    color: Theme.accent;
                    font-size: 10px;
                    font-weight: Theme.weight-bold;
                    vertical-alignment: center;
                }
            }
        }

        Text {
            text: root.company;
            color: Theme.text-secondary;
            font-size: 13px;
        }

        HorizontalLayout {
            spacing: 16px;

            Text {
                text: root.location;
                color: Theme.text-tertiary;
                font-size: 12px;
            }

            Text {
                text: root.salary;
                color: Theme.state-success;
                font-size: 12px;
                font-weight: Theme.weight-medium;
            }
        }
    }
}
```

#### `LinkPreviewCard` ([slint/cards/LinkPreviewCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/LinkPreviewCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component LinkPreviewCard inherits Rectangle {
    in-out property <string> url: "https://example.com";
    in-out property <string> title: "Link Preview";
    in-out property <string> description: "A preview of the linked content.";
    in-out property <string> domain: "example.com";

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;

    VerticalLayout {
        Rectangle {
            height: 80px;
            background: Theme.bg-overlay;
            clip: true;

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: "🔗";
                font-size: 24px;
            }
        }

        VerticalLayout {
            padding: 12px;
            spacing: 4px;

            Text {
                text: root.title;
                color: Theme.text-primary;
                font-size: 14px;
                font-weight: Theme.weight-semibold;
                wrap: word-wrap;
            }

            Text {
                text: root.description;
                color: Theme.text-secondary;
                font-size: 12px;
                wrap: word-wrap;
            }

            Text {
                text: root.domain;
                color: Theme.text-tertiary;
                font-size: 11px;
            }
        }
    }
}
```

#### `LocationCard` ([slint/cards/LocationCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/LocationCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component LocationCard inherits Rectangle {
    in-out property <string> name: "Place Name";
    in-out property <string> address: "123 Main St, City";
    in-out property <string> category: "Restaurant";

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;

    VerticalLayout {
        Rectangle {
            height: 100px;
            background: Theme.bg-overlay;
            clip: true;

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: "📍 Map";
                color: Theme.text-tertiary;
                font-size: 14px;
            }
        }

        VerticalLayout {
            padding: 16px;
            spacing: 4px;

            HorizontalLayout {
                spacing: 8px;

                Text {
                    text: root.name;
                    color: Theme.text-primary;
                    font-size: 14px;
                    font-weight: Theme.weight-semibold;
                    horizontal-stretch: 1;
                }

                Rectangle {
                    height: 20px;
                    background: Theme.accent-subtle;
                    border-radius: Theme.radius-sm;
                    horizontal-stretch: 0;

                    Text {
                        x: (parent.width - self.width) / 2;
                        y: (parent.height - self.height) / 2;
                        text: root.category;
                        color: Theme.accent;
                        font-size: 10px;
                        font-weight: Theme.weight-semibold;
                        vertical-alignment: center;
                    }
                }
            }

            Text {
                text: root.address;
                color: Theme.text-secondary;
                font-size: 12px;
            }
        }
    }
}
```

#### `MapEmbedCard` ([slint/cards/MapEmbedCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/MapEmbedCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component MapEmbedCard inherits Rectangle {
    in-out property <string> label: "Map";
    in-out property <string> sublabel: "Interactive map preview";

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;

    VerticalLayout {
        Rectangle {
            height: 180px;
            background: Theme.dark-mode ? #1a2332 : #e8f0fe;
            clip: true;

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: "🗺";
                font-size: 32px;
            }
        }

        HorizontalLayout {
            padding-left: 16px;
            padding-right: 16px;
            padding-top: 12px;
            padding-bottom: 2px;
            Text {
                text: root.label;
                color: Theme.text-primary;
                font-size: 13px;
                font-weight: Theme.weight-semibold;
            }
        }

        HorizontalLayout {
            padding-left: 16px;
            padding-right: 16px;
            padding-bottom: 12px;
            Text {
                text: root.sublabel;
                color: Theme.text-tertiary;
                font-size: 11px;
            }
        }
    }
}
```

#### `MediaCard` ([slint/cards/MediaCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/MediaCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component MediaCard inherits Rectangle {
    in-out property <string> title: "Media Card";

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;

    VerticalLayout {
        Rectangle {
            height: 140px;
            background: Theme.bg-overlay;
            clip: true;

            Text {
                text: "Image";
                color: Theme.text-tertiary;
                font-size: 14px;
                horizontal-alignment: center;
                vertical-alignment: center;
            }
        }

        VerticalLayout {
            padding-left: 16px;
            padding-right: 16px;
            padding-top: 12px;
            padding-bottom: 12px;
            Text {
                text: root.title;
                color: Theme.text-primary;
                font-size: 15px;
                font-weight: Theme.weight-semibold;
            }
        }
    }
}
```

#### `MetricSparklineCard` ([slint/cards/MetricSparklineCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/MetricSparklineCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component MetricSparklineCard inherits Rectangle {
    in-out property <string> label: "Revenue";
    in-out property <string> value: "$12,340";
    in-out property <string> change: "+12.5%";

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;

    VerticalLayout {
        padding: 16px;
        spacing: 8px;

        Text {
            text: root.label;
            color: Theme.text-tertiary;
            font-size: 11px;
            font-weight: Theme.weight-semibold;
        }

        HorizontalLayout {
            spacing: 8px;
            alignment: start;

            Text {
                text: root.value;
                color: Theme.text-primary;
                font-family: Theme.font-display;
                font-size: 24px;
                font-weight: Theme.weight-bold;
                vertical-alignment: bottom;
            }

            Text {
                text: root.change;
                color: Theme.state-success;
                font-size: 12px;
                font-weight: Theme.weight-medium;
                vertical-alignment: bottom;
            }
        }

        Rectangle {
            height: 32px;
            background: transparent;

            Rectangle {
                y: parent.height - 8px;
                height: 2px;
                width: parent.width;
                background: Theme.accent-subtle;
            }

            Rectangle {
                x: parent.width * 0.1;
                y: parent.height * 0.3;
                width: 3px;
                height: 3px;
                border-radius: Theme.radius-full;
                background: Theme.accent;
            }

            Rectangle {
                x: parent.width * 0.3;
                y: parent.height * 0.5;
                width: 3px;
                height: 3px;
                border-radius: Theme.radius-full;
                background: Theme.accent;
            }

            Rectangle {
                x: parent.width * 0.5;
                y: parent.height * 0.2;
                width: 3px;
                height: 3px;
                border-radius: Theme.radius-full;
                background: Theme.accent;
            }

            Rectangle {
                x: parent.width * 0.7;
                y: parent.height * 0.4;
                width: 3px;
                height: 3px;
                border-radius: Theme.radius-full;
                background: Theme.accent;
            }

            Rectangle {
                x: parent.width * 0.9;
                y: parent.height * 0.1;
                width: 3px;
                height: 3px;
                border-radius: Theme.radius-full;
                background: Theme.accent;
            }
        }
    }
}
```

#### `MusicTrackCard` ([slint/cards/MusicTrackCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/MusicTrackCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component MusicTrackCard inherits Rectangle {
    in-out property <string> title: "Track Title";
    in-out property <string> artist: "Artist Name";
    in-out property <string> duration: "3:42";

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;

    HorizontalLayout {
        padding: 12px;
        spacing: 12px;

        Rectangle {
            width: 48px;
            height: 48px;
            border-radius: Theme.radius-md;
            background: Theme.accent-subtle;

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: "♫";
                color: Theme.accent;
                font-size: 20px;
                vertical-alignment: center;
            }
        }

        VerticalLayout {
            horizontal-stretch: 1;
            spacing: 2px;

            Text {
                text: root.title;
                color: Theme.text-primary;
                font-size: 14px;
                font-weight: Theme.weight-semibold;
            }

            Text {
                text: root.artist;
                color: Theme.text-secondary;
                font-size: 12px;
            }
        }

        Text {
            text: root.duration;
            color: Theme.text-tertiary;
            font-family: Theme.font-mono;
            font-size: 12px;
            vertical-alignment: center;
            horizontal-stretch: 0;
        }
    }
}
```

#### `NotificationCard` ([slint/cards/NotificationCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/NotificationCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component NotificationCard inherits Rectangle {
    in-out property <string> title: "New notification";
    in-out property <string> message: "You have a new message.";
    in-out property <string> time: "2m ago";

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;

    HorizontalLayout {
        padding: 16px;
        spacing: 12px;

        Rectangle {
            width: 8px;
            height: 8px;
            border-radius: Theme.radius-full;
            background: Theme.accent;
            horizontal-stretch: 0;
            vertical-stretch: 0;
        }

        VerticalLayout {
            horizontal-stretch: 1;
            spacing: 4px;

            Text {
                text: root.title;
                color: Theme.text-primary;
                font-size: 13px;
                font-weight: Theme.weight-semibold;
            }

            Text {
                text: root.message;
                color: Theme.text-secondary;
                font-size: 12px;
                wrap: word-wrap;
            }
        }

        Text {
            text: root.time;
            color: Theme.text-tertiary;
            font-size: 11px;
            vertical-alignment: top;
            horizontal-stretch: 0;
        }
    }
}
```

#### `OutlinedCard` ([slint/cards/OutlinedCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/OutlinedCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component OutlinedCard inherits Rectangle {
    background: transparent;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;
}
```

#### `PodcastEpisodeCard` ([slint/cards/PodcastEpisodeCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/PodcastEpisodeCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component PodcastEpisodeCard inherits Rectangle {
    in-out property <string> title: "Episode Title";
    in-out property <string> podcast: "Podcast Name";
    in-out property <string> duration: "45 min";
    in-out property <string> date: "Jul 20, 2026";

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;

    HorizontalLayout {
        padding: 16px;
        spacing: 12px;

        Rectangle {
            width: 56px;
            height: 56px;
            border-radius: Theme.radius-md;
            background: Theme.accent-subtle;

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: "🎙";
                font-size: 24px;
                vertical-alignment: center;
            }
        }

        VerticalLayout {
            horizontal-stretch: 1;
            spacing: 4px;

            Text {
                text: root.title;
                color: Theme.text-primary;
                font-size: 14px;
                font-weight: Theme.weight-semibold;
                wrap: word-wrap;
            }

            Text {
                text: root.podcast;
                color: Theme.accent;
                font-size: 12px;
                font-weight: Theme.weight-medium;
            }

            HorizontalLayout {
                spacing: 12px;

                Text {
                    text: root.duration;
                    color: Theme.text-tertiary;
                    font-size: 11px;
                }

                Text {
                    text: root.date;
                    color: Theme.text-tertiary;
                    font-size: 11px;
                }
            }
        }
    }
}
```

#### `PricingCard` ([slint/cards/PricingCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/PricingCard.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component PricingCard inherits Rectangle {
    in-out property <string> tier: "Pro";
    in-out property <string> price: "$19/mo";
    in-out property <bool> featured: false;

    background: root.featured ? Theme.accent : Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: root.featured ? Theme.accent : Theme.border-base;
    horizontal-stretch: 1;

    VerticalLayout {
        padding: 24px;
        spacing: 12px;

        Text {
            text: root.tier;
            color: root.featured ? #ffffff : Theme.text-primary;
            font-size: 18px;
            font-weight: Theme.weight-bold;
            horizontal-alignment: center;
        }

        Text {
            text: root.price;
            color: root.featured ? #ffffff : Theme.accent;
            font-size: 32px;
            font-weight: Theme.weight-bold;
            horizontal-alignment: center;
        }

        Rectangle {
            height: 1px;
            horizontal-stretch: 1;
            background: root.featured ? rgba(255,255,255,0.2) : Theme.border-base;
        }

        Text {
            text: "All features included";
            color: root.featured ? rgba(255,255,255,0.8) : Theme.text-secondary;
            font-size: 13px;
            horizontal-alignment: center;
        }
    }
}
```

#### `ProductCard` ([slint/cards/ProductCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/ProductCard.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component ProductCard inherits Rectangle {
    in-out property <string> name: "Product Name";
    in-out property <string> price: "$29.99";

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;

    VerticalLayout {
        Rectangle {
            height: 160px;
            background: Theme.bg-overlay;
            clip: true;

            FaIcon {
                source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/image.svg");
                fa-size: 32px;
                fa-color: Theme.text-tertiary;
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
            }
        }

        VerticalLayout {
            padding: 12px;
            spacing: 4px;

            Text {
                text: root.name;
                color: Theme.text-primary;
                font-size: 14px;
                font-weight: Theme.weight-medium;
            }

            Text {
                text: root.price;
                color: Theme.accent;
                font-size: 16px;
                font-weight: Theme.weight-bold;
            }
        }
    }
}
```

#### `ProductComparisonCard` ([slint/cards/ProductComparisonCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/ProductComparisonCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component ProductComparisonCard inherits Rectangle {
    in-out property <string> plan-name: "Pro";
    in-out property <string> price: "$29";
    in-out property <string> period: "/mo";
    in-out property <bool> featured: false;
    in-out property <[string]> features: [];

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 2px;
    border-color: root.featured ? Theme.accent : Theme.border-base;
    horizontal-stretch: 1;

    VerticalLayout {
        padding: 20px;
        spacing: 12px;

        if root.featured: Rectangle {
            height: 20px;
            background: Theme.accent;
            border-radius: Theme.radius-sm;

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: "Most Popular";
                color: #ffffff;
                font-size: 10px;
                font-weight: Theme.weight-bold;
                vertical-alignment: center;
            }
        }

        Text {
            text: root.plan-name;
            color: Theme.text-primary;
            font-size: 18px;
            font-weight: Theme.weight-bold;
            horizontal-alignment: center;
        }

        HorizontalLayout {
            alignment: center;

            Text {
                text: root.price;
                color: Theme.text-primary;
                font-family: Theme.font-display;
                font-size: 32px;
                font-weight: Theme.weight-bold;
                vertical-alignment: bottom;
            }

            Text {
                text: root.period;
                color: Theme.text-tertiary;
                font-size: 13px;
                vertical-alignment: bottom;
            }
        }

        Rectangle {
            height: 1px;
            background: Theme.border-subtle;
        }

        for feat[i] in root.features: HorizontalLayout {
            height: 28px;
            spacing: 8px;

            Text {
                text: "✓";
                color: Theme.state-success;
                font-size: 12px;
                vertical-alignment: center;
                horizontal-stretch: 0;
            }

            Text {
                text: feat;
                color: Theme.text-secondary;
                font-size: 12px;
                vertical-alignment: center;
                horizontal-stretch: 1;
            }
        }
    }
}
```

#### `ProfileCard` ([slint/cards/ProfileCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/ProfileCard.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component ProfileCard inherits Rectangle {
    in-out property <string> name: "John Doe";
    in-out property <string> role: "Software Engineer";

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;

    VerticalLayout {
        alignment: center;
        padding: 24px;
        spacing: 12px;

        Rectangle {
            width: 64px;
            height: 64px;
            background: Theme.accent-subtle;
            border-radius: 32px;
            x: (parent.width - self.width) / 2;

            FaIcon {
                source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/user.svg");
                fa-size: 28px;
                fa-color: Theme.accent;
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
            }
        }

        Text {
            text: root.name;
            color: Theme.text-primary;
            font-size: 16px;
            font-weight: Theme.weight-semibold;
            horizontal-alignment: center;
        }

        Text {
            text: root.role;
            color: Theme.text-secondary;
            font-size: 13px;
            horizontal-alignment: center;
        }
    }
}
```

#### `ReviewCard` ([slint/cards/ReviewCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/ReviewCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component ReviewCard inherits Rectangle {
    in-out property <string> reviewer: "User";
    in-out property <int> stars: 4;
    in-out property <string> text: "Great product, works as expected.";

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;

    VerticalLayout {
        padding: 16px;
        spacing: 8px;

        HorizontalLayout {
            spacing: 8px;

            Text {
                text: root.reviewer;
                color: Theme.text-primary;
                font-size: 13px;
                font-weight: Theme.weight-semibold;
                horizontal-stretch: 1;
                vertical-alignment: center;
            }

            HorizontalLayout {
                spacing: 2px;

                for s[i] in 5: Text {
                    text: i < root.stars ? "★" : "☆";
                    color: i < root.stars ? #FBBF24 : Theme.text-tertiary;
                    font-size: 14px;
                }
            }
        }

        Text {
            text: root.text;
            color: Theme.text-secondary;
            font-size: 13px;
            wrap: word-wrap;
        }
    }
}
```

#### `StatTileCard` ([slint/cards/StatTileCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/StatTileCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component StatTileCard inherits Rectangle {
    in-out property <string> label: "Metric";
    in-out property <string> value: "0";
    in-out property <brush> accent-color: Theme.accent;

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;

    VerticalLayout {
        padding: 16px;
        spacing: 4px;

        Text {
            text: root.label;
            color: Theme.text-tertiary;
            font-size: 11px;
            font-weight: Theme.weight-semibold;
        }

        Text {
            text: root.value;
            color: Theme.text-primary;
            font-family: Theme.font-display;
            font-size: 28px;
            font-weight: Theme.weight-bold;
        }
    }
}
```

#### `SwipeableCard` ([slint/cards/SwipeableCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/SwipeableCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component SwipeableCard inherits Rectangle {
    in-out property <string> title: "Swipeable Card";
    in-out property <string> description: "Swipe left to reveal actions";

    callback dismissed();

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;
    clip: true;

    Rectangle {
        width: parent.width;
        height: parent.height;
        background: Theme.state-error;

        HorizontalLayout {
            padding-left: 16px;
            alignment: end;

            Text {
                text: "Delete";
                color: #ffffff;
                font-size: 14px;
                font-weight: Theme.weight-semibold;
                vertical-alignment: center;
            }
        }
    }

    cardsurface := Rectangle {
        width: parent.width;
        height: parent.height;
        background: Theme.bg-surface;
        border-radius: Theme.radius-lg;
        border-width: 1px;
        border-color: Theme.border-base;

        HorizontalLayout {
            padding: 16px;
            spacing: 12px;

            VerticalLayout {
                horizontal-stretch: 1;
                spacing: 4px;

                Text {
                    text: root.title;
                    color: Theme.text-primary;
                    font-size: 14px;
                    font-weight: Theme.weight-semibold;
                }

                Text {
                    text: root.description;
                    color: Theme.text-secondary;
                    font-size: 12px;
                    wrap: word-wrap;
                }
            }

            Text {
                text: "⋮";
                color: Theme.text-tertiary;
                font-size: 18px;
                vertical-alignment: center;
                horizontal-stretch: 0;
            }
        }

        ta := TouchArea {
            clicked => { root.dismissed(); }
        }
    }
}
```

#### `TeamMemberCard` ([slint/cards/TeamMemberCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/TeamMemberCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component TeamMemberCard inherits Rectangle {
    in-out property <string> name: "Name";
    in-out property <string> role: "Role";
    in-out property <string> email: "email@example.com";

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;

    VerticalLayout {
        padding: 20px;
        spacing: 8px;
        alignment: center;

        Rectangle {
            width: 64px;
            height: 64px;
            border-radius: Theme.radius-full;
            background: Theme.accent-subtle;

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: "?";
                color: Theme.accent;
                font-size: 24px;
                font-weight: Theme.weight-bold;
                vertical-alignment: center;
            }
        }

        Text {
            text: root.name;
            color: Theme.text-primary;
            font-size: 14px;
            font-weight: Theme.weight-semibold;
            horizontal-alignment: center;
        }

        Text {
            text: root.role;
            color: Theme.text-secondary;
            font-size: 12px;
            horizontal-alignment: center;
        }

        Text {
            text: root.email;
            color: Theme.text-tertiary;
            font-size: 11px;
            horizontal-alignment: center;
        }
    }
}
```

#### `TestimonialCard` ([slint/cards/TestimonialCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/TestimonialCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component TestimonialCard inherits Rectangle {
    in-out property <string> quote: "This product changed how we work. Highly recommended.";
    in-out property <string> author: "Jane Doe";
    in-out property <string> role: "CTO, Acme Corp";

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;

    VerticalLayout {
        padding: 20px;
        spacing: 16px;

        Text {
            text: ">";
            color: Theme.accent;
            font-family: Theme.font-display;
            font-size: 48px;
            font-weight: Theme.weight-bold;
        }

        Text {
            text: root.quote;
            color: Theme.text-primary;
            font-size: 14px;
            wrap: word-wrap;
        }

        HorizontalLayout {
            spacing: 8px;
            alignment: start;

            Rectangle {
                width: 32px;
                height: 32px;
                border-radius: Theme.radius-full;
                background: Theme.accent-subtle;

                Text {
                    x: (parent.width - self.width) / 2;
                    y: (parent.height - self.height) / 2;
                    text: "?";
                    color: Theme.accent;
                    font-size: 14px;
                    font-weight: Theme.weight-bold;
                    vertical-alignment: center;
                }
            }

            VerticalLayout {
                spacing: 0px;

                Text {
                    text: root.author;
                    color: Theme.text-primary;
                    font-size: 12px;
                    font-weight: Theme.weight-semibold;
                }

                Text {
                    text: root.role;
                    color: Theme.text-tertiary;
                    font-size: 11px;
                }
            }
        }
    }
}
```

#### `VideoThumbnailCard` ([slint/cards/VideoThumbnailCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/VideoThumbnailCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component VideoThumbnailCard inherits Rectangle {
    in-out property <string> title: "Video Title";
    in-out property <string> channel: "Channel Name";
    in-out property <string> duration: "12:34";
    in-out property <string> views: "1.2K views";

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;

    VerticalLayout {
        Rectangle {
            height: 140px;
            background: Theme.dark-mode ? #1a1a2e : #e8e8f0;
            clip: true;

            Rectangle {
                x: (parent.width - 40px) / 2;
                y: (parent.height - 32px) / 2;
                width: 40px;
                height: 32px;
                border-radius: Theme.radius-sm;
                background: #000000aa;

                Text {
                    x: (parent.width - self.width) / 2;
                    y: (parent.height - self.height) / 2;
                    text: "▶";
                    color: #ffffff;
                    font-size: 14px;
                    vertical-alignment: center;
                }
            }

            Rectangle {
                x: parent.width - 56px;
                y: parent.height - 24px;
                height: 20px;
                border-radius: Theme.radius-sm;
                background: #000000cc;

                Text {
                    x: (parent.width - self.width) / 2;
                    y: (parent.height - self.height) / 2;
                    text: root.duration;
                    color: #ffffff;
                    font-size: 10px;
                    font-weight: Theme.weight-medium;
                    vertical-alignment: center;
                }
            }
        }

        VerticalLayout {
            padding: 12px;
            spacing: 4px;

            Text {
                text: root.title;
                color: Theme.text-primary;
                font-size: 13px;
                font-weight: Theme.weight-semibold;
                wrap: word-wrap;
            }

            HorizontalLayout {
                spacing: 8px;

                Text {
                    text: root.channel;
                    color: Theme.text-secondary;
                    font-size: 11px;
                    horizontal-stretch: 1;
                }

                Text {
                    text: root.views;
                    color: Theme.text-tertiary;
                    font-size: 11px;
                }
            }
        }
    }
}
```

#### `WeatherCard` ([slint/cards/WeatherCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/cards/WeatherCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component WeatherCard inherits Rectangle {
    in-out property <string> city: "San Francisco";
    in-out property <int> temp: 72;
    in-out property <string> condition: "Sunny";

    background: Theme.bg-surface;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-base;
    horizontal-stretch: 1;

    HorizontalLayout {
        padding: 20px;
        spacing: 16px;

        VerticalLayout {
            spacing: 4px;

            Text {
                text: root.city;
                color: Theme.text-secondary;
                font-size: 13px;
            }

            Text {
                text: root.temp;
                color: Theme.text-primary;
                font-size: 40px;
                font-weight: Theme.weight-bold;
            }

            Text {
                text: root.condition;
                color: Theme.text-tertiary;
                font-size: 14px;
            }
        }
    }
}
```


### 5.7 Forms & Selection Controls Code Manual

#### `AccordionItem` ([slint/forms/AccordionItem.slint](file:///home/lion/Documents/GitHub/ltk/slint/forms/AccordionItem.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component AccordionItem inherits Rectangle {
    in property <string> title: "";
    in property <bool> expanded: false;

    callback toggle();

    background: transparent;

    VerticalLayout {
        // Header
        Rectangle {
            height: 44px;
            background: header-ta.pressed ? Theme.surface-pressed
                : header-ta.has-hover ? Theme.surface-hover
                : transparent;
            border-radius: Theme.radius-sm;

            animate background { duration: Theme.dur-fast; }

            HorizontalLayout {
                padding-left: Theme.sp-3;
                padding-right: Theme.sp-3;
                spacing: Theme.sp-2;
                alignment: stretch;

                Text {
                    text: root.title;
                    color: Theme.text-primary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-sm;
                    font-weight: Theme.weight-medium;
                    vertical-alignment: center;
                    horizontal-stretch: 1;
                }

                Text {
                    text: root.expanded ? "▾" : "›";
                    color: Theme.text-tertiary;
                    font-size: Theme.text-sm;
                    vertical-alignment: center;
                    horizontal-stretch: 0;
                }
            }

            header-ta := TouchArea {
                clicked => { root.toggle(); }
            }
        }

        // Content
        if expanded: VerticalLayout {
            padding-left: Theme.sp-3;
            padding-right: Theme.sp-3;
            padding-bottom: Theme.sp-3;
            spacing: Theme.sp-2;

            content-layout := VerticalLayout { }
        }
    }
}
```

#### `CheckboxGroup` ([slint/forms/CheckboxGroup.slint](file:///home/lion/Documents/GitHub/ltk/slint/forms/CheckboxGroup.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component CheckboxGroup inherits Rectangle {
    in property <[string]> options: [];
    in-out property <[bool]> checked-items: [];

    callback changed(int, bool);

    background: transparent;

    VerticalLayout {
        spacing: Theme.sp-2;

        for opt[i] in root.options: Rectangle {
            height: 28px;
            background: transparent;

            HorizontalLayout {
                spacing: Theme.sp-2;
                alignment: start;

                Rectangle {
                    width: 16px;
                    height: 16px;
                    horizontal-stretch: 0;
                    border-radius: Theme.radius-xs;
                    border-width: 1.5px;
                    border-color: i < root.checked-items.length && root.checked-items[i]
                        ? Theme.accent : Theme.border-strong;
                    background: i < root.checked-items.length && root.checked-items[i]
                        ? Theme.accent : transparent;

                    if i < root.checked-items.length && root.checked-items[i]: Text {
                        x: (parent.width - self.width) / 2;
                        y: (parent.height - self.height) / 2;
                        text: "✓";
                        color: Theme.on-accent;
                        font-size: Theme.text-xs;
                        vertical-alignment: center;
                    }
                }

                Text {
                    text: opt;
                    color: Theme.text-primary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-sm;
                    vertical-alignment: center;
                }
            }

            TouchArea {
                clicked => {
                    if i < root.checked-items.length {
                        root.checked-items[i] = !root.checked-items[i];
                        root.changed(i, root.checked-items[i]);
                    }
                }
            }
        }
    }
}
```

#### `Select` ([slint/forms/Select.slint](file:///home/lion/Documents/GitHub/ltk/slint/forms/Select.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component Select inherits Rectangle {
    in property <string> placeholder: "Select...";
    in property <[string]> options: [];
    in-out property <int> current-index: -1;
    in property <bool> enabled: true;

    callback changed(int);

    height: Theme.input-height-md;
    opacity: enabled ? 1.0 : Theme.opacity-disabled;

    bg := Rectangle {
        border-radius: Theme.radius-input;
        border-width: 1px;
        border-color: {
            if (ta.has-hover || expanded) && enabled { Theme.border-strong }
            else { Theme.border-base }
        }
        background: enabled ? Theme.bg-surface : Theme.btn-disabled-bg;

        animate border-color { duration: Theme.dur-fast; }
    }

    selected-text := Text {
        x: Theme.input-padding-x;
        y: (parent.height - self.height) / 2;
        width: parent.width - Theme.input-padding-x * 3 - chevron.preferred-width;
        text: current-index >= 0 && current-index < options.length
            ? options[current-index] : placeholder;
        color: current-index >= 0 ? Theme.text-primary : Theme.text-tertiary;
        font-family: Theme.font-ui;
        font-size: Theme.text-sm;
        vertical-alignment: center;
        overflow: elide;
    }

    chevron := Text {
        x: parent.width - Theme.input-padding-x - self.width;
        y: (parent.height - self.height) / 2;
        text: expanded ? "▴" : "▾";
        color: Theme.text-tertiary;
        font-size: Theme.text-sm;
        vertical-alignment: center;
    }

    // Dropdown overlay
    in-out property <bool> expanded: false;

    dropdown := Rectangle {
        y: parent.height + Theme.sp-1;
        width: parent.width;
        height: min(options.length * 32px + Theme.sp-2 * 2, 200px);
        background: Theme.bg-elevated;
        border-radius: Theme.radius-input;
        border-width: 1px;
        border-color: Theme.border-base;
        clip: true;
        visible: root.expanded;

        drop-shadow-offset-y: 4px;
        drop-shadow-blur: 12px;
        drop-shadow-color: Theme.border-default;

        VerticalLayout {
            padding: Theme.sp-1;

            for opt[i] in root.options: Rectangle {
                height: 32px;
                background: option-ta.pressed ? Theme.surface-pressed
                    : option-ta.has-hover ? Theme.surface-hover
                    : i == root.current-index ? Theme.accent-subtle
                    : transparent;
                border-radius: Theme.radius-xs;

                Text {
                    x: Theme.sp-3;
                    y: (parent.height - self.height) / 2;
                    text: opt;
                    color: i == root.current-index ? Theme.accent : Theme.text-primary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-sm;
                    font-weight: i == root.current-index ? Theme.weight-medium : Theme.weight-regular;
                    vertical-alignment: center;
                    horizontal-stretch: 1;
                }

                option-ta := TouchArea {
                    clicked => {
                        root.current-index = i;
                        root.expanded = false;
                        root.changed(i);
                    }
                }
            }
        }
    }

    ta := TouchArea {
        clicked => {
            if enabled { root.expanded = !root.expanded; }
        }
    }
}
```

#### `ToggleGroup` ([slint/forms/ToggleGroup.slint](file:///home/lion/Documents/GitHub/ltk/slint/forms/ToggleGroup.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component ToggleGroup inherits Rectangle {
    in property <[string]> options: [];
    in-out property <int> current-index: 0;

    callback changed(int);

    height: 32px;

    bg := Rectangle {
        border-radius: Theme.radius-button;
        background: Theme.bg-overlay;
        border-width: 1px;
        border-color: Theme.border-subtle;
    }

    HorizontalLayout {
        padding: 2px;
        spacing: 2px;

        for opt[i] in root.options: TouchArea {
            horizontal-stretch: 1;
            clicked => {
                root.current-index = i;
                root.changed(i);
            }

            Rectangle {
                border-radius: Theme.radius-sm;
                background: root.current-index == i ? Theme.accent
                    : ta2.pressed ? Theme.surface-pressed
                    : ta2.has-hover ? Theme.surface-hover
                    : transparent;
                animate background { duration: Theme.dur-fast; }

                Text {
                    x: (parent.width - self.width) / 2;
                    y: (parent.height - self.height) / 2;
                    text: opt;
                    color: root.current-index == i ? Theme.on-accent : Theme.text-secondary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-sm;
                    font-weight: root.current-index == i ? Theme.weight-semibold : Theme.weight-regular;
                    vertical-alignment: center;
                    horizontal-alignment: center;
                }
            }

            ta2 := TouchArea { }
        }
    }
}
```


### 5.8 Overlays, Modals & Dialogs Code Manual

#### `ActionSheet` ([slint/overlays/ActionSheet.slint](file:///home/lion/Documents/GitHub/ltk/slint/overlays/ActionSheet.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component ActionSheetItem inherits Rectangle {
    in property <string> label: "";
    in property <bool> destructive: false;
    in property <bool> cancel: false;

    callback activated();

    height: Theme.button-height-lg;
    background: item-ta.has-hover
        ? (root.cancel ? Theme.surface-hover : Theme.surface-hover)
        : #00000000;

    animate background { duration: Theme.dur-fast; easing: Theme.ease-standard; }

    Text {
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
        text: root.label;
        color: root.destructive ? Theme.state-error : (root.cancel ? Theme.text-primary : Theme.text-primary);
        font-family: Theme.font-ui;
        font-size: root.cancel ? Theme.text-base : Theme.text-md;
        font-weight: root.cancel ? Theme.weight-semibold : Theme.weight-regular;
        vertical-alignment: center;
    }

    item-ta := TouchArea {
        clicked => { root.activated(); }
    }
}

export component ActionSheet inherits Rectangle {
    in-out property <bool> active: false;
    in property <[string]> options: [];
    in property <[string]> destructive-options: [];
    in property <string> cancel-text: "Cancel";

    callback option-clicked(int);
    callback destructive-clicked(int);
    callback cancelled();

    width: 100%;
    height: 100%;
    background: #00000000;

    // Scrim
    Rectangle {
        width: 100%;
        height: 100%;
        background: Theme.backdrop;
        opacity: root.active ? 1 : 0;
        visible: root.active;
        animate opacity { duration: Theme.dur-normal; easing: Theme.ease-standard; }

        TouchArea {
            clicked => { root.cancelled(); }
        }
    }

    // Sheet
    Rectangle {
        x: 0;
        y: root.active ? parent.height - self.height : parent.height;
        width: parent.width;
        min-height: 120px;
        background: Theme.bg-surface;
        border-radius: Theme.radius-xl;
        border-width: 1px;
        border-color: Theme.border-subtle;

        animate y { duration: Theme.dur-slow; easing: Theme.ease-soft; }

        VerticalLayout {
            padding: Theme.sp-2;
            spacing: Theme.sp-2;

            for option[optionidx] in root.options: ActionSheetItem {
                label: option;
                activated => { root.option-clicked(optionidx); }
            }

            for doption[didx] in root.destructive-options: ActionSheetItem {
                label: doption;
                destructive: true;
                activated => { root.destructive-clicked(didx); }
            }

            Rectangle {
                height: 1px;
                background: Theme.divider;
                x: Theme.sp-4;
                width: parent.width - Theme.sp-4 * 2;
            }

            if root.cancel-text != "": ActionSheetItem {
                label: root.cancel-text;
                cancel: true;
                activated => { root.cancelled(); }
            }
        }

        TouchArea { }
    }
}
```

#### `AlertDialog` ([slint/overlays/AlertDialog.slint](file:///home/lion/Documents/GitHub/ltk/slint/overlays/AlertDialog.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component AlertDialog inherits Rectangle {
    visible: false;
    in property <string> title: "";
    in property <string> message: "";
    in property <string> destructive-label: "Delete";

    callback confirmed();
    callback cancelled();

    width: 100%;
    height: 100%;
    background: #00000000;

    Rectangle {
        width: 100%;
        height: 100%;
        background: Theme.backdrop;
        opacity: root.visible ? 1 : 0;
        visible: root.visible;
        animate opacity { duration: Theme.dur-normal; easing: Theme.ease-standard; }

        TouchArea {
            clicked => { root.cancelled(); }
        }
    }

    Rectangle {
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
        width: 400px;
        min-height: 160px;
        background: Theme.bg-surface;
        border-radius: Theme.radius-dialog;
        border-width: 1px;
        border-color: Theme.border-subtle;
        visible: root.visible;
        opacity: root.visible ? 1 : 0;

        drop-shadow-offset-x: 0px;
        drop-shadow-offset-y: Theme.elevation-3-y;
        drop-shadow-blur: Theme.elevation-3-blur;
        drop-shadow-color: Theme.elevation-3-color;

        animate opacity { duration: Theme.dur-medium; easing: Theme.ease-soft; }

        VerticalLayout {
            padding: Theme.sp-6;
            spacing: Theme.sp-4;

            if root.title != "": Text {
                text: root.title;
                color: Theme.state-error;
                font-family: Theme.font-ui;
                font-size: Theme.text-xl;
                font-weight: Theme.weight-bold;
                wrap: word-wrap;
            }

            if root.message != "": Text {
                text: root.message;
                color: Theme.text-secondary;
                font-family: Theme.font-ui;
                font-size: Theme.text-base;
                wrap: word-wrap;
                vertical-stretch: 1;
            }

            Rectangle { vertical-stretch: 1; }

            HorizontalLayout {
                spacing: Theme.sp-3;
                alignment: end;
                height: Theme.button-height-md;
                vertical-stretch: 0;

                Rectangle {
                    horizontal-stretch: 1;
                    min-width: 90px;
                    height: Theme.button-height-md;
                    border-radius: Theme.radius-button;
                    border-width: 1px;
                    border-color: Theme.border-base;
                    background: cancel-ta.has-hover ? Theme.surface-hover : #00000000;

                    animate background { duration: Theme.dur-fast; easing: Theme.ease-standard; }

                    cancel-ta := TouchArea {
                        clicked => { root.cancelled(); }
                    }

                    Text {
                        x: (parent.width - self.width) / 2;
                        y: (parent.height - self.height) / 2;
                        text: "Cancel";
                        color: Theme.text-primary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-base;
                        font-weight: Theme.weight-medium;
                        vertical-alignment: center;
                    }
                }

                Rectangle {
                    horizontal-stretch: 1;
                    min-width: 90px;
                    height: Theme.button-height-md;
                    border-radius: Theme.radius-button;
                    background: confirm-ta.pressed
                        ? Theme.state-error.darker(0.15)
                        : confirm-ta.has-hover
                        ? Theme.state-error.darker(0.08)
                        : Theme.state-error;

                    animate background { duration: Theme.dur-fast; easing: Theme.ease-standard; }

                    confirm-ta := TouchArea {
                        clicked => { root.confirmed(); }
                    }

                    Text {
                        x: (parent.width - self.width) / 2;
                        y: (parent.height - self.height) / 2;
                        text: root.destructive-label;
                        color: Theme.on-accent;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-base;
                        font-weight: Theme.weight-semibold;
                        vertical-alignment: center;
                    }
                }
            }
        }

        TouchArea { }
    }
}
```

#### `BottomSheet` ([slint/overlays/BottomSheet.slint](file:///home/lion/Documents/GitHub/ltk/slint/overlays/BottomSheet.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component BottomSheet inherits Rectangle {
    in-out property <bool> active: false;

    callback dismissed();

    width: 100%;
    height: 100%;
    background: #00000000;

    // Scrim
    Rectangle {
        width: 100%;
        height: 100%;
        background: Theme.backdrop;
        opacity: root.active ? 1 : 0;
        visible: root.active;
        animate opacity { duration: Theme.dur-normal; easing: Theme.ease-standard; }

        TouchArea {
            clicked => { root.dismissed(); }
        }
    }

    // Sheet panel
    Rectangle {
        x: 0;
        y: root.active ? parent.height - self.height : parent.height;
        width: parent.width;
        height: 480px;
        background: Theme.bg-surface;
        border-radius: Theme.radius-xl;
        border-width: 1px;
        border-color: Theme.border-subtle;

        drop-shadow-offset-y: -4px;
        drop-shadow-blur: 20px;
        drop-shadow-color: Theme.border-default;

        animate y { duration: Theme.dur-slow; easing: Theme.ease-soft; }

        VerticalLayout {
            padding: Theme.sp-4;
            spacing: Theme.sp-4;

            // Handle
            Rectangle {
                width: 36px;
                height: 4px;
                border-radius: 2px;
                background: Theme.border-strong;
                horizontal-stretch: 0;
                vertical-stretch: 0;
                x: (parent.width - self.width) / 2;
            }

            // Content slot
            Rectangle {
                vertical-stretch: 1;
            }
        }

        TouchArea { }
    }
}
```

#### `ContextTooltip` ([slint/overlays/ContextTooltip.slint](file:///home/lion/Documents/GitHub/ltk/slint/overlays/ContextTooltip.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component ContextTooltip inherits Rectangle {
    visible: false;
    in property <string> text: "";

    min-width: 60px;
    min-height: 28px;
    visible: root.visible;
    background: Theme.bg-elevated;
    border-radius: Theme.radius-tooltip;
    border-width: 1px;
    border-color: Theme.border-subtle;
    opacity: root.visible ? 1 : 0;

    drop-shadow-offset-x: 0px;
    drop-shadow-offset-y: Theme.elevation-1-y;
    drop-shadow-blur: Theme.elevation-1-blur;
    drop-shadow-color: Theme.elevation-1-color;

    animate opacity { duration: Theme.dur-fast; easing: Theme.ease-standard; }

    HorizontalLayout {
        padding: Theme.sp-2;
        spacing: Theme.sp-1-5;
        alignment: center;

        FaIcon {
            fa-size: 12px;
            fa-color: Theme.text-tertiary;
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/circle-info.svg");
        }

        if root.text != "": Text {
            text: root.text;
            color: Theme.text-primary;
            font-family: Theme.font-ui;
            font-size: Theme.text-xs;
            font-weight: Theme.weight-medium;
            vertical-alignment: center;
        }
    }
}
```

#### `DropdownMenu` ([slint/overlays/DropdownMenu.slint](file:///home/lion/Documents/GitHub/ltk/slint/overlays/DropdownMenu.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component DropdownMenuItem inherits Rectangle {
    in property <string> label: "";
    in property <string> icon-text: "";
    in property <bool> destructive: false;
    in property <bool> disabled: false;

    callback activated();

    height: 32px;
    background: item-ta.has-hover ? Theme.surface-hover : #00000000;

    animate background { duration: Theme.dur-fast; easing: Theme.ease-standard; }

    HorizontalLayout {
        padding-left: Theme.sp-3;
        padding-right: Theme.sp-3;
        spacing: Theme.sp-2;

        if root.icon-text != "": Text {
            text: root.icon-text;
            color: root.destructive ? Theme.state-error : Theme.text-tertiary;
            font-size: Theme.text-sm;
            vertical-alignment: center;
            horizontal-stretch: 0;
        }

        Text {
            text: root.label;
            color: root.disabled ? Theme.text-disabled : (root.destructive ? Theme.state-error : Theme.text-primary);
            font-family: Theme.font-ui;
            font-size: Theme.text-sm;
            vertical-alignment: center;
            horizontal-stretch: 1;
        }
    }

    item-ta := TouchArea {
        enabled: !root.disabled;
        clicked => { root.activated(); }
    }
}

export component DropdownMenu inherits Rectangle {
    in-out property <bool> active: false;
    in property <[string]> items: [];
    in property <[string]> icons: [];

    callback item-clicked(int);

    width: 200px;
    min-height: 40px;
    visible: active;
    background: Theme.bg-surface;
    border-radius: Theme.radius-md;
    border-width: 1px;
    border-color: Theme.border-subtle;
    opacity: active ? 1 : 0;

    drop-shadow-offset-x: 0px;
    drop-shadow-offset-y: Theme.elevation-2-y;
    drop-shadow-blur: Theme.elevation-2-blur;
    drop-shadow-color: Theme.elevation-2-color;

    animate opacity { duration: Theme.dur-fast; easing: Theme.ease-standard; }

    VerticalLayout {
        padding: Theme.sp-1;
        spacing: 0px;

        for item[index] in root.items: DropdownMenuItem {
            label: item;
            icon-text: index < root.icons.length ? root.icons[index] : "";
            activated => { root.item-clicked(index); }
        }
    }

    ta := TouchArea { }
}
```

#### `FloatingActionPanel` ([slint/overlays/FloatingActionPanel.slint](file:///home/lion/Documents/GitHub/ltk/slint/overlays/FloatingActionPanel.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

component ActionButton inherits Rectangle {
    in property <string> icon-source: "";
    in property <string> label: "";
    in property <int> action-index: 0;

    callback clicked(int);

    width: 64px;
    height: 72px;
    background: btn-ta.pressed ? Theme.surface-pressed
        : btn-ta.has-hover ? Theme.surface-hover
        : #00000000;

    animate background { duration: Theme.dur-fast; easing: Theme.ease-standard; }

    VerticalLayout {
        alignment: center;
        spacing: Theme.sp-1;

        Rectangle {
            width: 40px;
            height: 40px;
            border-radius: Theme.radius-md;
            background: Theme.bg-overlay;

            FaIcon {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                fa-size: 18px;
                fa-color: Theme.text-primary;
                source: root.icon-source;
            }
        }

        Text {
            text: root.label;
            color: Theme.text-secondary;
            font-family: Theme.font-ui;
            font-size: Theme.text-xs;
            horizontal-alignment: center;
        }
    }

    btn-ta := TouchArea {
        clicked => { root.clicked(root.action-index); }
    }
}

export component FloatingActionPanel inherits Rectangle {
    visible: false;

    callback action-clicked(int);

    width: 100%;
    height: 100%;
    background: #00000000;

    // Scrim
    Rectangle {
        width: 100%;
        height: 100%;
        background: Theme.backdrop;
        opacity: root.visible ? 1 : 0;
        visible: root.visible;
        animate opacity { duration: Theme.dur-normal; easing: Theme.ease-standard; }

        TouchArea {
            clicked => { root.visible = false; }
        }
    }

    // Floating panel
    Rectangle {
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
        width: 280px;
        height: 180px;
        background: Theme.bg-surface;
        border-radius: Theme.radius-xl;
        border-width: 1px;
        border-color: Theme.border-subtle;
        visible: root.visible;
        opacity: root.visible ? 1 : 0;

        drop-shadow-offset-x: 0px;
        drop-shadow-offset-y: Theme.elevation-2-y;
        drop-shadow-blur: Theme.elevation-2-blur;
        drop-shadow-color: Theme.elevation-2-color;

        animate opacity { duration: Theme.dur-medium; easing: Theme.ease-soft; }

        Rectangle {
            padding: Theme.sp-4;

            VerticalLayout {
                spacing: Theme.sp-4;

                Text {
                    text: "Actions";
                    color: Theme.text-primary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-md;
                    font-weight: Theme.weight-semibold;
                    vertical-stretch: 0;
                }

                // Action grid row 1
                HorizontalLayout {
                    spacing: Theme.sp-4;
                    alignment: space-around;
                    vertical-stretch: 0;

                    ActionButton {
                        icon-source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/star.svg");
                        label: "Favorite";
                        action-index: 0;
                        clicked(idx) => { root.action-clicked(idx); }
                    }

                    ActionButton {
                        icon-source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/eye.svg");
                        label: "Preview";
                        action-index: 1;
                        clicked(idx) => { root.action-clicked(idx); }
                    }

                    ActionButton {
                        icon-source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/expand.svg");
                        label: "Expand";
                        action-index: 2;
                        clicked(idx) => { root.action-clicked(idx); }
                    }
                }

                // Action grid row 2
                HorizontalLayout {
                    spacing: Theme.sp-4;
                    alignment: space-around;
                    vertical-stretch: 0;

                    ActionButton {
                        icon-source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/circle-info.svg");
                        label: "Info";
                        action-index: 3;
                        clicked(idx) => { root.action-clicked(idx); }
                    }

                    ActionButton {
                        icon-source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/circle-question.svg");
                        label: "Help";
                        action-index: 4;
                        clicked(idx) => { root.action-clicked(idx); }
                    }

                    ActionButton {
                        icon-source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/lightbulb.svg");
                        label: "Tips";
                        action-index: 5;
                        clicked(idx) => { root.action-clicked(idx); }
                    }
                }
            }
        }

        TouchArea { }
    }
}
```

#### `FloatingLabel` ([slint/overlays/FloatingLabel.slint](file:///home/lion/Documents/GitHub/ltk/slint/overlays/FloatingLabel.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component FloatingLabel inherits Rectangle {
    in property <string> label: "";
    in property <bool> has-value: false;
    in property <bool> focused: false;

    height: root.focused || root.has-value ? 18px : 36px;
    vertical-stretch: 0;

    Text {
        x: Theme.sp-3;
        y: root.focused || root.has-value ? 2px : (parent.height - self.height) / 2;
        text: root.label;
        color: root.focused ? Theme.accent
            : root.has-value ? Theme.text-secondary
            : Theme.text-placeholder;
        font-family: Theme.font-ui;
        font-size: root.focused || root.has-value ? Theme.text-xs : Theme.text-base;
        font-weight: root.focused || root.has-value ? Theme.weight-medium : Theme.weight-regular;
        vertical-alignment: center;

        animate y { duration: Theme.dur-fast; easing: Theme.ease-standard; }
        animate font-size { duration: Theme.dur-fast; easing: Theme.ease-standard; }
        animate color { duration: Theme.dur-fast; easing: Theme.ease-standard; }
    }
}
```

#### `FloatingToolbar` ([slint/overlays/FloatingToolbar.slint](file:///home/lion/Documents/GitHub/ltk/slint/overlays/FloatingToolbar.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component FloatingToolbarButton inherits Rectangle {
    in property <string> icon: "";
    in property <bool> active: false;
    in property <bool> disabled: false;

    callback clicked();

    width: 32px;
    height: 32px;

    btn-bg := Rectangle {
        border-radius: Theme.radius-sm;
        background: btn-ta.pressed ? Theme.surface-pressed
            : btn-ta.has-hover ? Theme.surface-hover
            : root.active ? Theme.accent-subtle
            : #00000000;

        animate background { duration: Theme.dur-fast; easing: Theme.ease-standard; }
    }

    Image {
        source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/ellipsis.svg");
        width: 14px;
        height: 14px;
        colorize: root.active ? Theme.accent : (root.disabled ? Theme.text-disabled : Theme.text-secondary);
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
    }

    btn-ta := TouchArea {
        enabled: !root.disabled;
        clicked => { root.clicked(); }
    }
}

export component FloatingToolbar inherits Rectangle {
    in-out property <bool> active: false;
    in property <[string]> icons: [];

    callback button-clicked(int);

    min-width: 40px;
    height: 40px;
    visible: active;
    background: Theme.bg-raised;
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: Theme.border-subtle;
    opacity: active ? 1 : 0;

    drop-shadow-offset-x: 0px;
    drop-shadow-offset-y: Theme.elevation-2-y;
    drop-shadow-blur: Theme.elevation-2-blur;
    drop-shadow-color: Theme.elevation-2-color;

    animate opacity { duration: Theme.dur-fast; easing: Theme.ease-standard; }

    HorizontalLayout {
        padding: Theme.sp-2;
        spacing: Theme.sp-1;

        for iconentry[iconidx] in root.icons: FloatingToolbarButton {
            icon: iconentry;
            clicked => { root.button-clicked(iconidx); }
        }
    }

    ta := TouchArea { }
}
```

#### `FullScreenDialog` ([slint/overlays/FullScreenDialog.slint](file:///home/lion/Documents/GitHub/ltk/slint/overlays/FullScreenDialog.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component FullScreenDialog inherits Rectangle {
    visible: false;
    in property <string> title: "";

    callback close();

    width: 100%;
    height: 100%;
    background: #00000000;
    visible: root.visible;

    Rectangle {
        width: 100%;
        height: 100%;
        background: Theme.bg-surface;
        opacity: root.visible ? 1 : 0;
        animate opacity { duration: Theme.dur-normal; easing: Theme.ease-standard; }

        VerticalLayout {
            spacing: 0px;

            // Header bar
            Rectangle {
                height: 52px;
                background: Theme.bg-headerbar;
                border-width: 0px;
                border-radius: 0px;

                HorizontalLayout {
                    padding: Theme.sp-3;
                    spacing: Theme.sp-3;

                    // Back arrow
                    Rectangle {
                        width: 32px;
                        height: 32px;
                        horizontal-stretch: 0;

                        back-bg := Rectangle {
                            border-radius: Theme.radius-sm;
                            background: back-ta.pressed ? Theme.surface-pressed
                                : back-ta.has-hover ? Theme.surface-hover
                                : #00000000;
                        }

                        FaIcon {
                            x: (parent.width - self.width) / 2;
                            y: (parent.height - self.height) / 2;
                            fa-size: 14px;
                            fa-color: Theme.text-primary;
                            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/arrow-left.svg");
                        }

                        back-ta := TouchArea {
                            clicked => { root.close(); }
                        }
                    }

                    // Title
                    if root.title != "": Text {
                        text: root.title;
                        color: Theme.text-primary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-lg;
                        font-weight: Theme.weight-semibold;
                        vertical-alignment: center;
                        horizontal-stretch: 1;
                    }

                    Rectangle { horizontal-stretch: 1; }

                    // Close button
                    Rectangle {
                        width: 32px;
                        height: 32px;
                        horizontal-stretch: 0;

                        close-bg := Rectangle {
                            border-radius: Theme.radius-sm;
                            background: close-ta.pressed ? Theme.surface-pressed
                                : close-ta.has-hover ? Theme.surface-hover
                                : #00000000;
                        }

                        FaIcon {
                            x: (parent.width - self.width) / 2;
                            y: (parent.height - self.height) / 2;
                            fa-size: 12px;
                            fa-color: Theme.text-tertiary;
                            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/xmark.svg");
                        }

                        close-ta := TouchArea {
                            clicked => { root.close(); }
                        }
                    }
                }
            }

            // Content slot
            Rectangle {
                vertical-stretch: 1;
            }
        }
    }
}
```

#### `HoverCard` ([slint/overlays/HoverCard.slint](file:///home/lion/Documents/GitHub/ltk/slint/overlays/HoverCard.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component HoverCard inherits Rectangle {
    visible: false;
    in property <string> title: "";
    in property <string> description: "";

    min-width: 240px;
    max-width: 320px;
    visible: root.visible;
    background: Theme.bg-elevated;
    border-radius: Theme.radius-md;
    border-width: 1px;
    border-color: Theme.border-subtle;
    opacity: root.visible ? 1 : 0;

    drop-shadow-offset-x: 0px;
    drop-shadow-offset-y: Theme.elevation-2-y;
    drop-shadow-blur: Theme.elevation-2-blur;
    drop-shadow-color: Theme.elevation-2-color;

    animate opacity { duration: Theme.dur-fast; easing: Theme.ease-standard; }

    VerticalLayout {
        padding: Theme.sp-3;
        spacing: Theme.sp-2;

        if root.title != "": Text {
            text: root.title;
            color: Theme.text-primary;
            font-family: Theme.font-ui;
            font-size: Theme.text-sm;
            font-weight: Theme.weight-semibold;
            wrap: word-wrap;
        }

        if root.description != "": Text {
            text: root.description;
            color: Theme.text-secondary;
            font-family: Theme.font-ui;
            font-size: Theme.text-xs;
            wrap: word-wrap;
        }
    }
}
```

#### `ImageLightbox` ([slint/overlays/ImageLightbox.slint](file:///home/lion/Documents/GitHub/ltk/slint/overlays/ImageLightbox.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component ImageLightbox inherits Rectangle {
    visible: false;
    in property <string> title: "";

    callback close();

    width: 100%;
    height: 100%;
    background: #00000000;

    // Scrim
    Rectangle {
        width: 100%;
        height: 100%;
        background: Theme.backdrop;
        opacity: root.visible ? 1 : 0;
        visible: root.visible;
        animate opacity { duration: Theme.dur-normal; easing: Theme.ease-standard; }

        TouchArea {
            clicked => { root.close(); }
        }
    }

    // Close button
    if root.visible: Rectangle {
        x: parent.width - 48px;
        y: Theme.sp-4;
        width: 32px;
        height: 32px;

        close-bg := Rectangle {
            border-radius: Theme.radius-sm;
            background: close-ta.pressed ? Theme.surface-pressed
                : close-ta.has-hover ? Theme.surface-hover
                : Theme.glass-regular;
        }

        FaIcon {
            x: (parent.width - self.width) / 2;
            y: (parent.height - self.height) / 2;
            fa-size: 14px;
            fa-color: Theme.text-primary;
            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/xmark.svg");
        }

        close-ta := TouchArea {
            clicked => { root.close(); }
        }
    }

    // Image area
    if root.visible: Rectangle {
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
        width: parent.width - Theme.sp-10 * 2;
        height: parent.height - Theme.sp-10 * 2;
        background: Theme.bg-elevated;
        border-radius: Theme.radius-lg;
        border-width: 1px;
        border-color: Theme.border-subtle;

        drop-shadow-offset-x: 0px;
        drop-shadow-offset-y: Theme.elevation-3-y;
        drop-shadow-blur: Theme.elevation-3-blur;
        drop-shadow-color: Theme.elevation-3-color;

        VerticalLayout {
            padding: Theme.sp-4;
            spacing: Theme.sp-3;

            // Title
            if root.title != "": Text {
                text: root.title;
                color: Theme.text-primary;
                font-family: Theme.font-ui;
                font-size: Theme.text-lg;
                font-weight: Theme.weight-semibold;
                vertical-alignment: center;
                horizontal-alignment: center;
                vertical-stretch: 0;
            }

            // Image placeholder
            Rectangle {
                vertical-stretch: 1;
                border-radius: Theme.radius-md;
                background: Theme.bg-overlay;
            }
        }

        TouchArea { }
    }
}
```

#### `InlineConfirmation` ([slint/overlays/InlineConfirmation.slint](file:///home/lion/Documents/GitHub/ltk/slint/overlays/InlineConfirmation.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component InlineConfirmation inherits Rectangle {
    in-out property <bool> active: false;
    in property <string> message: "Are you sure?";

    callback confirmed();
    callback cancelled();

    min-width: 200px;
    height: 44px;
    visible: active;
    background: Theme.bg-elevated;
    border-radius: Theme.radius-md;
    border-width: 1px;
    border-color: Theme.border-subtle;
    opacity: active ? 1 : 0;

    drop-shadow-offset-x: 0px;
    drop-shadow-offset-y: Theme.elevation-1-y;
    drop-shadow-blur: Theme.elevation-1-blur;
    drop-shadow-color: Theme.elevation-1-color;

    animate opacity { duration: Theme.dur-fast; easing: Theme.ease-standard; }

    HorizontalLayout {
        padding-left: Theme.sp-3;
        padding-right: Theme.sp-2;
        spacing: Theme.sp-2;
        alignment: stretch;

        Text {
            text: root.message;
            color: Theme.text-secondary;
            font-family: Theme.font-ui;
            font-size: Theme.text-sm;
            vertical-alignment: center;
            horizontal-stretch: 1;
        }

        // No button
        Rectangle {
            width: 52px;
            height: 28px;
            horizontal-stretch: 0;

            no-bg := Rectangle {
                border-radius: Theme.radius-sm;
                border-width: 1px;
                border-color: Theme.border-base;
                background: no-ta.pressed ? Theme.surface-pressed
                    : no-ta.has-hover ? Theme.surface-hover
                    : #00000000;
            }

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: "No";
                color: Theme.text-primary;
                font-family: Theme.font-ui;
                font-size: Theme.text-xs;
                font-weight: Theme.weight-medium;
                vertical-alignment: center;
            }

            no-ta := TouchArea {
                clicked => { root.cancelled(); }
            }
        }

        // Yes button
        Rectangle {
            width: 52px;
            height: 28px;
            horizontal-stretch: 0;

            yes-bg := Rectangle {
                border-radius: Theme.radius-sm;
                background: yes-ta.pressed ? Theme.btn-primary-bg-pressed
                    : yes-ta.has-hover ? Theme.btn-primary-bg-hover
                    : Theme.btn-primary-bg;
            }

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: "Yes";
                color: Theme.on-accent;
                font-family: Theme.font-ui;
                font-size: Theme.text-xs;
                font-weight: Theme.weight-semibold;
                vertical-alignment: center;
            }

            yes-ta := TouchArea {
                clicked => { root.confirmed(); }
            }
        }
    }
}
```

#### `ModalDialog` ([slint/overlays/ModalDialog.slint](file:///home/lion/Documents/GitHub/ltk/slint/overlays/ModalDialog.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component ModalDialog inherits Rectangle {
    in-out property <bool> active: false;
    in property <string> title: "";
    in property <string> body: "";
    in property <string> cancel-text: "Cancel";
    in property <string> confirm-text: "Confirm";
    in property <bool> destructive: false;

    callback cancelled();
    callback confirmed();

    width: 100%;
    height: 100%;
    background: #00000000;

    // Scrim
    Rectangle {
        width: 100%;
        height: 100%;
        background: Theme.backdrop;
        opacity: root.active ? 1 : 0;
        visible: root.active;
        animate opacity { duration: Theme.dur-normal; easing: Theme.ease-standard; }

        TouchArea {
            clicked => { root.cancelled(); }
        }
    }

    // Dialog card
    Rectangle {
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
        width: 420px;
        min-height: 180px;
        background: Theme.bg-surface;
        border-radius: Theme.radius-dialog;
        border-width: 1px;
        border-color: Theme.border-subtle;
        visible: root.active;
        opacity: root.active ? 1 : 0;

        drop-shadow-offset-x: 0px;
        drop-shadow-offset-y: Theme.elevation-3-y;
        drop-shadow-blur: Theme.elevation-3-blur;
        drop-shadow-color: Theme.elevation-3-color;

        animate opacity { duration: Theme.dur-medium; easing: Theme.ease-soft; }

        VerticalLayout {
            padding: Theme.sp-6;
            spacing: Theme.sp-4;

            if root.title != "": Text {
                text: root.title;
                color: Theme.text-primary;
                font-family: Theme.font-ui;
                font-size: Theme.text-xl;
                font-weight: Theme.weight-bold;
                wrap: word-wrap;
            }

            if root.body != "": Text {
                text: root.body;
                color: Theme.text-secondary;
                font-family: Theme.font-ui;
                font-size: Theme.text-base;
                wrap: word-wrap;
                vertical-stretch: 1;
            }

            Rectangle { vertical-stretch: 1; }

            HorizontalLayout {
                spacing: Theme.sp-3;
                alignment: end;
                height: Theme.button-height-md;
                vertical-stretch: 0;

                Rectangle {
                    horizontal-stretch: 1;
                    min-width: 90px;
                    height: Theme.button-height-md;
                    border-radius: Theme.radius-button;
                    border-width: 1px;
                    border-color: Theme.border-base;
                    background: cancel-ta.has-hover ? Theme.surface-hover : #00000000;

                    animate background { duration: Theme.dur-fast; easing: Theme.ease-standard; }

                    cancel-ta := TouchArea {
                        clicked => { root.cancelled(); }
                    }

                    Text {
                        x: (parent.width - self.width) / 2;
                        y: (parent.height - self.height) / 2;
                        text: root.cancel-text;
                        color: Theme.text-primary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-base;
                        font-weight: Theme.weight-medium;
                        vertical-alignment: center;
                    }
                }

                Rectangle {
                    horizontal-stretch: 1;
                    min-width: 90px;
                    height: Theme.button-height-md;
                    border-radius: Theme.radius-button;
                    background: confirm-ta.has-hover
                        ? (root.destructive ? Theme.state-error.darker(0.1) : Theme.btn-primary-bg-hover)
                        : (root.destructive ? Theme.state-error : Theme.btn-primary-bg);

                    animate background { duration: Theme.dur-fast; easing: Theme.ease-standard; }

                    confirm-ta := TouchArea {
                        clicked => { root.confirmed(); }
                    }

                    Text {
                        x: (parent.width - self.width) / 2;
                        y: (parent.height - self.height) / 2;
                        text: root.confirm-text;
                        color: Theme.on-accent;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-base;
                        font-weight: Theme.weight-medium;
                        vertical-alignment: center;
                    }
                }
            }
        }

        TouchArea { }
    }
}
```

#### `NestedDropdownMenu` ([slint/overlays/NestedDropdownMenu.slint](file:///home/lion/Documents/GitHub/ltk/slint/overlays/NestedDropdownMenu.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component NestedMenuItem inherits Rectangle {
    in property <string> label: "";
    in property <bool> has-submenu: false;
    in property <bool> submenu-open: false;

    callback activated();
    callback submenu-hover();

    height: 32px;
    background: item-ta.has-hover ? Theme.surface-hover : #00000000;

    animate background { duration: Theme.dur-fast; easing: Theme.ease-standard; }

    HorizontalLayout {
        padding: Theme.sp-1;
        spacing: Theme.sp-2;

        Text {
            text: root.label;
            color: Theme.text-primary;
            font-family: Theme.font-ui;
            font-size: Theme.text-sm;
            vertical-alignment: center;
            horizontal-stretch: 1;
        }

        if root.has-submenu: Text {
            text: "›";
            color: Theme.text-tertiary;
            font-size: Theme.text-sm;
            vertical-alignment: center;
            horizontal-stretch: 0;
        }
    }

    item-ta := TouchArea {
        clicked => {
            if (root.has-submenu) {
                root.submenu-hover();
            } else {
                root.activated();
            }
        }
        entered => {
            if (root.has-submenu) {
                root.submenu-hover();
            }
        }
    }
}

export component NestedDropdownMenu inherits Rectangle {
    visible: false;
    in property <[string]> items: [];
    in property <[string]> submenu-items: [];
    in-out property <bool> submenu-open: false;

    callback item-clicked(int);
    callback submenu-item-clicked(int);

    width: 220px;
    min-height: 40px;
    visible: root.visible;
    background: Theme.bg-surface;
    border-radius: Theme.radius-md;
    border-width: 1px;
    border-color: Theme.border-subtle;
    opacity: root.visible ? 1 : 0;

    drop-shadow-offset-x: 0px;
    drop-shadow-offset-y: Theme.elevation-2-y;
    drop-shadow-blur: Theme.elevation-2-blur;
    drop-shadow-color: Theme.elevation-2-color;

    animate opacity { duration: Theme.dur-fast; easing: Theme.ease-standard; }

    VerticalLayout {
        padding: Theme.sp-1;
        spacing: 0px;

        for itementry in root.items: NestedMenuItem {
            label: itementry;
            has-submenu: index == root.items.length - 1 && root.submenu-items.length > 0;
            submenu-open: root.submenu-open;
            activated => { root.item-clicked(index); }
            submenu-hover => { root.submenu-open = true; }
        }
    }

    // Sub-menu panel
    if root.submenu-open && root.submenu-items.length > 0: Rectangle {
        x: parent.width + 4px;
        y: 0;
        width: 180px;
        min-height: 40px;
        background: Theme.bg-surface;
        border-radius: Theme.radius-md;
        border-width: 1px;
        border-color: Theme.border-subtle;

        drop-shadow-offset-x: 0px;
        drop-shadow-offset-y: Theme.elevation-2-y;
        drop-shadow-blur: Theme.elevation-2-blur;
        drop-shadow-color: Theme.elevation-2-color;

        VerticalLayout {
            padding: Theme.sp-1;
            spacing: 0px;

            for subentry in root.submenu-items: Rectangle {
                height: 32px;
                background: sub-ta.has-hover ? Theme.surface-hover : #00000000;

                animate background { duration: Theme.dur-fast; easing: Theme.ease-standard; }

                Text {
                    x: Theme.sp-3;
                    y: (parent.height - self.height) / 2;
                    text: subentry;
                    color: Theme.text-primary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-sm;
                    vertical-alignment: center;
                }

                sub-ta := TouchArea {
                    clicked => { root.submenu-item-clicked(index); }
                }
            }
        }

        TouchArea { }
    }

    ta := TouchArea { }
}
```

#### `NotificationDrawer` ([slint/overlays/NotificationDrawer.slint](file:///home/lion/Documents/GitHub/ltk/slint/overlays/NotificationDrawer.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component NotificationItem inherits Rectangle {
    in property <string> title: "";
    in property <string> body: "";

    callback activated();

    height: 64px;
    background: item-ta.has-hover ? Theme.surface-hover : #00000000;

    animate background { duration: Theme.dur-fast; easing: Theme.ease-standard; }

    HorizontalLayout {
        padding: Theme.sp-3;
        spacing: Theme.sp-3;

        Rectangle {
            width: 32px;
            height: 32px;
            horizontal-stretch: 0;

            Rectangle {
                border-radius: Theme.radius-sm;
                background: Theme.accent-subtle;
            }

            FaIcon {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                fa-size: 14px;
                fa-color: Theme.accent;
                source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/bell.svg");
            }
        }

        VerticalLayout {
            spacing: Theme.sp-0-5;
            horizontal-stretch: 1;

            if root.title != "": Text {
                text: root.title;
                color: Theme.text-primary;
                font-family: Theme.font-ui;
                font-size: Theme.text-sm;
                font-weight: Theme.weight-semibold;
                wrap: no-wrap;
                overflow: elide;
            }

            if root.body != "": Text {
                text: root.body;
                color: Theme.text-secondary;
                font-family: Theme.font-ui;
                font-size: Theme.text-xs;
                wrap: no-wrap;
                overflow: elide;
            }
        }
    }

    Rectangle {
        y: parent.height - 1px;
        width: parent.width;
        height: 1px;
        background: Theme.divider;
    }

    item-ta := TouchArea {
        clicked => { root.activated(); }
    }
}

export component NotificationDrawer inherits Rectangle {
    in-out property <bool> drawer-visible: false;

    callback close();

    width: 100%;
    height: 100%;
    background: #00000000;

    // Scrim
    Rectangle {
        width: 100%;
        height: 100%;
        background: Theme.backdrop;
        opacity: root.drawer-visible ? 1 : 0;
        visible: root.drawer-visible;
        animate opacity { duration: Theme.dur-normal; easing: Theme.ease-standard; }

        TouchArea {
            clicked => { root.close(); }
        }
    }

    // Panel
    Rectangle {
        x: root.drawer-visible ? parent.width - self.width : parent.width;
        y: 0;
        width: 340px;
        height: parent.height;
        background: Theme.bg-surface;
        border-radius: Theme.radius-lg;
        border-width: 1px;
        border-color: Theme.border-subtle;

        drop-shadow-offset-x: -4px;
        drop-shadow-blur: 16px;
        drop-shadow-color: Theme.border-default;

        animate x { duration: Theme.dur-slow; easing: Theme.ease-soft; }

        VerticalLayout {
            padding: Theme.sp-4;
            spacing: Theme.sp-3;

            // Header
            HorizontalLayout {
                spacing: Theme.sp-3;
                alignment: space-between;
                vertical-stretch: 0;

                Text {
                    text: "Notifications";
                    color: Theme.text-primary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-xl;
                    font-weight: Theme.weight-bold;
                    horizontal-stretch: 1;
                    vertical-alignment: center;
                }

                Rectangle {
                    width: 28px;
                    height: 28px;
                    horizontal-stretch: 0;

                    close-bg := Rectangle {
                        border-radius: Theme.radius-xs;
                        background: close-ta.pressed ? Theme.surface-pressed
                            : close-ta.has-hover ? Theme.surface-hover
                            : #00000000;
                    }

                    FaIcon {
                        x: (parent.width - self.width) / 2;
                        y: (parent.height - self.height) / 2;
                        fa-size: 12px;
                        fa-color: Theme.text-tertiary;
                        source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/xmark.svg");
                    }

                    close-ta := TouchArea {
                        clicked => { root.close(); }
                    }
                }
            }

            Rectangle {
                height: 1px;
                background: Theme.divider;
                vertical-stretch: 0;
            }

            // Notification list slot
            Rectangle {
                vertical-stretch: 1;
            }
        }

        TouchArea { }
    }
}
```

#### `OnboardingOverlay` ([slint/overlays/OnboardingOverlay.slint](file:///home/lion/Documents/GitHub/ltk/slint/overlays/OnboardingOverlay.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component OnboardingOverlay inherits Rectangle {
    visible: false;
    in property <int> step: 1;
    in property <[int]> total-steps: [0, 1, 2];
    in property <string> title: "";
    in property <string> description: "";

    callback next();
    callback skip();

    width: 100%;
    height: 100%;
    background: #00000000;

    // Scrim
    Rectangle {
        width: 100%;
        height: 100%;
        background: Theme.backdrop;
        opacity: root.visible ? 1 : 0;
        visible: root.visible;
        animate opacity { duration: Theme.dur-normal; easing: Theme.ease-standard; }

        TouchArea {
            clicked => { root.skip(); }
        }
    }

    // Spotlight highlight
    Rectangle {
        x: (parent.width - 320px) / 2;
        y: 80px;
        width: 320px;
        height: 320px;
        border-radius: Theme.radius-xl;
        border-width: 2px;
        border-color: Theme.accent;
        background: #00000000;
        visible: root.visible;
        opacity: root.visible ? 1 : 0;

        animate opacity { duration: Theme.dur-medium; easing: Theme.ease-soft; }

        // Glow ring
        Rectangle {
            x: -4px;
            y: -4px;
            width: parent.width + 8px;
            height: parent.height + 8px;
            border-radius: 20px;
            border-width: 1px;
            border-color: Theme.accent-glow;
            background: #00000000;
        }

        // Step icon
        Rectangle {
            width: 56px;
            height: 56px;
            border-radius: 28px;
            background: Theme.accent-subtle;
            x: (parent.width - self.width) / 2;
            y: (parent.height - self.height) / 2;

            FaIcon {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                fa-size: 24px;
                fa-color: Theme.accent;
                source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/rocket.svg");
            }
        }
    }

    // Bottom card
    Rectangle {
        x: (parent.width - self.width) / 2;
        y: 80px + 320px + Theme.sp-6;
        width: 360px;
        min-height: 200px;
        background: Theme.bg-surface;
        border-radius: Theme.radius-xl;
        border-width: 1px;
        border-color: Theme.border-subtle;
        visible: root.visible;
        opacity: root.visible ? 1 : 0;

        drop-shadow-offset-x: 0px;
        drop-shadow-offset-y: Theme.elevation-2-y;
        drop-shadow-blur: Theme.elevation-2-blur;
        drop-shadow-color: Theme.elevation-2-color;

        animate opacity { duration: Theme.dur-medium; easing: Theme.ease-soft; }

        VerticalLayout {
            padding: Theme.sp-6;
            spacing: Theme.sp-4;

            // Step indicator dots
            HorizontalLayout {
                alignment: center;
                spacing: Theme.sp-2;
                vertical-stretch: 0;

                for idx in root.total-steps: Rectangle {
                    width: idx == root.step - 1 ? 24px : 8px;
                    height: 8px;
                    border-radius: 4px;
                    background: idx == root.step - 1 ? Theme.accent : Theme.border-strong;
                    vertical-alignment: center;

                    animate width { duration: Theme.dur-normal; easing: Theme.ease-standard; }
                    animate background { duration: Theme.dur-normal; easing: Theme.ease-standard; }
                }
            }

            if root.title != "": Text {
                text: root.title;
                color: Theme.text-primary;
                font-family: Theme.font-ui;
                font-size: Theme.text-xl;
                font-weight: Theme.weight-bold;
                wrap: word-wrap;
                horizontal-alignment: center;
                vertical-stretch: 0;
            }

            if root.description != "": Text {
                text: root.description;
                color: Theme.text-secondary;
                font-family: Theme.font-ui;
                font-size: Theme.text-base;
                wrap: word-wrap;
                horizontal-alignment: center;
                vertical-stretch: 1;
            }

            Rectangle { vertical-stretch: 1; }

            Rectangle {
                height: 1px;
                background: Theme.divider;
                vertical-stretch: 0;
            }

            // Action buttons
            HorizontalLayout {
                spacing: Theme.sp-3;
                height: Theme.button-height-md;
                vertical-stretch: 0;

                Rectangle {
                    horizontal-stretch: 1;
                    min-width: 90px;
                    height: Theme.button-height-md;
                    border-radius: Theme.radius-button;
                    border-width: 1px;
                    border-color: Theme.border-base;
                    background: skip-ta.has-hover ? Theme.surface-hover : #00000000;

                    animate background { duration: Theme.dur-fast; easing: Theme.ease-standard; }

                    skip-ta := TouchArea {
                        clicked => { root.skip(); }
                    }

                    Text {
                        x: (parent.width - self.width) / 2;
                        y: (parent.height - self.height) / 2;
                        text: "Skip";
                        color: Theme.text-primary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-base;
                        font-weight: Theme.weight-medium;
                        vertical-alignment: center;
                    }
                }

                Rectangle {
                    horizontal-stretch: 1;
                    min-width: 90px;
                    height: Theme.button-height-md;
                    border-radius: Theme.radius-button;
                    background: next-ta.pressed ? Theme.btn-primary-bg-pressed
                        : next-ta.has-hover ? Theme.btn-primary-bg-hover
                        : Theme.btn-primary-bg;

                    animate background { duration: Theme.dur-fast; easing: Theme.ease-standard; }

                    next-ta := TouchArea {
                        clicked => { root.next(); }
                    }

                    Text {
                        x: (parent.width - self.width) / 2;
                        y: (parent.height - self.height) / 2;
                        text: root.step < root.total-steps ? "Next" : "Finish";
                        color: Theme.on-accent;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-base;
                        font-weight: Theme.weight-medium;
                        vertical-alignment: center;
                    }
                }
            }
        }

        TouchArea { }
    }
}
```

#### `PeekSheet` ([slint/overlays/PeekSheet.slint](file:///home/lion/Documents/GitHub/ltk/slint/overlays/PeekSheet.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component PeekSheet inherits Rectangle {
    in-out property <bool> sheet-visible: false;
    in property <length> peek-height: 300px;

    callback expand();

    width: 100%;
    height: 100%;
    background: #00000000;

    Rectangle {
        width: 100%;
        height: 100%;
        background: Theme.backdrop;
        opacity: root.sheet-visible ? 1 : 0;
        visible: root.sheet-visible;
        animate opacity { duration: Theme.dur-normal; easing: Theme.ease-standard; }
    }

    Rectangle {
        x: 0;
        y: root.sheet-visible ? parent.height - root.peek-height : parent.height;
        width: parent.width;
        height: root.peek-height;
        background: Theme.bg-surface;
        border-radius: Theme.radius-xl;
        border-width: 1px;
        border-color: Theme.border-subtle;

        drop-shadow-offset-y: -4px;
        drop-shadow-blur: 20px;
        drop-shadow-color: Theme.border-default;

        animate y { duration: Theme.dur-slow; easing: Theme.ease-soft; }

        VerticalLayout {
            padding: Theme.sp-4;
            spacing: Theme.sp-3;

            // Handle indicator
            Rectangle {
                width: 36px;
                height: 4px;
                border-radius: 2px;
                background: Theme.border-strong;
                horizontal-stretch: 0;
                vertical-stretch: 0;
                x: (parent.width - self.width) / 2;
            }

            // Content slot
            Rectangle {
                vertical-stretch: 1;
            }

            // Expand indicator
            Rectangle {
                height: 28px;
                vertical-stretch: 0;

                expand-bg := Rectangle {
                    border-radius: Theme.radius-sm;
                    background: expand-ta.pressed ? Theme.surface-pressed
                        : expand-ta.has-hover ? Theme.surface-hover
                        : #00000000;
                    x: (parent.width - self.width) / 2;
                    width: 120px;
                }

                Text {
                    x: (parent.width - self.width) / 2;
                    y: (parent.height - self.height) / 2;
                    text: "Expand";
                    color: Theme.accent;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-xs;
                    font-weight: Theme.weight-medium;
                    vertical-alignment: center;
                }

                expand-ta := TouchArea {
                    clicked => { root.expand(); }
                }
            }
        }

        TouchArea { }
    }
}
```

#### `Popover` ([slint/overlays/Popover.slint](file:///home/lion/Documents/GitHub/ltk/slint/overlays/Popover.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component Popover inherits Rectangle {
    in-out property <bool> active: false;
    in property <length> anchor-x: 0px;
    in property <length> anchor-y: 0px;
    in property <string> placement: "bottom";

    callback dismissed();

    width: 100%;
    height: 100%;
    background: #00000000;

    // Triangle arrow
    Rectangle {
        x: root.placement == "bottom" ? root.anchor-x - 8px
            : root.placement == "top" ? root.anchor-x - 8px
            : root.placement == "left" ? root.anchor-x - 8px
            : root.anchor-x - 8px;
        y: root.placement == "bottom" ? root.anchor_y - 18px
            : root.placement == "top" ? root.anchor_y + 8px
            : root.anchor_y - 8px;
        width: 16px;
        height: 8px;
        visible: root.active;
        background: Theme.bg-surface;
        border-radius: 2px;
        opacity: root.active ? 1 : 0;

        animate opacity { duration: Theme.dur-fast; easing: Theme.ease-standard; }
    }

    // Panel
    Rectangle {
        x: root.placement == "bottom" ? root.anchor_x - self.width / 2
            : root.placement == "top" ? root.anchor_x - self.width / 2
            : root.placement == "left" ? root.anchor_x - self.width - 12px
            : root.anchor_x + 12px;
        y: root.placement == "bottom" ? root.anchor_y + 12px
            : root.placement == "top" ? root.anchor_y - self.height - 12px
            : root.anchor_y - self.height / 2;
        min-width: 160px;
        min-height: 40px;
        background: Theme.bg-surface;
        border-radius: Theme.radius-md;
        border-width: 1px;
        border-color: Theme.border-subtle;
        visible: root.active;
        opacity: root.active ? 1 : 0;

        drop-shadow-offset-x: 0px;
        drop-shadow-offset-y: Theme.elevation-2-y;
        drop-shadow-blur: Theme.elevation-2-blur;
        drop-shadow-color: Theme.elevation-2-color;

        animate opacity { duration: Theme.dur-fast; easing: Theme.ease-standard; }

        // Content slot
        Rectangle {
            vertical-stretch: 1;
        }

        TouchArea { }
    }

    // Dismiss catcher
    if root.active: TouchArea {
        width: 100%;
        height: 100%;
        clicked => { root.dismissed(); }
    }
}
```

#### `RichTooltip` ([slint/overlays/RichTooltip.slint](file:///home/lion/Documents/GitHub/ltk/slint/overlays/RichTooltip.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component RichTooltip inherits Rectangle {
    in-out property <bool> active: false;
    in property <string> tip-title: "";
    in property <string> tip-body: "";
    in property <string> action-text: "";

    callback action-clicked();

    width: 260px;
    min-height: 60px;
    visible: active;
    background: Theme.bg-elevated;
    border-radius: Theme.radius-tooltip;
    border-width: 1px;
    border-color: Theme.border-subtle;
    opacity: active ? 1 : 0;

    drop-shadow-offset-x: 0px;
    drop-shadow-offset-y: Theme.elevation-1-y;
    drop-shadow-blur: Theme.elevation-1-blur;
    drop-shadow-color: Theme.elevation-1-color;

    animate opacity { duration: Theme.dur-fast; easing: Theme.ease-standard; }

    VerticalLayout {
        padding: Theme.sp-3;
        spacing: Theme.sp-2;

        if root.tip-title != "": Text {
            text: root.tip-title;
            color: Theme.text-primary;
            font-family: Theme.font-ui;
            font-size: Theme.text-sm;
            font-weight: Theme.weight-semibold;
            wrap: word-wrap;
        }

        if root.tip-body != "": Text {
            text: root.tip-body;
            color: Theme.text-secondary;
            font-family: Theme.font-ui;
            font-size: Theme.text-xs;
            wrap: word-wrap;
        }

        if root.action-text != "": Rectangle {
            height: 24px;
            vertical-stretch: 0;

            action-bg := Rectangle {
                border-radius: Theme.radius-xs;
                background: action-ta.pressed ? Theme.accent-subtle
                    : action-ta.has-hover ? Theme.accent-subtle
                    : #00000000;
            }

            Text {
                x: (parent.width - self.width) / 2;
                y: (parent.height - self.height) / 2;
                text: root.action-text;
                color: Theme.accent;
                font-family: Theme.font-ui;
                font-size: Theme.text-xs;
                font-weight: Theme.weight-semibold;
                vertical-alignment: center;
            }

            action-ta := TouchArea {
                clicked => { root.action-clicked(); }
            }
        }
    }

    ta := TouchArea { }
}
```

#### `ScrimOverlay` ([slint/overlays/ScrimOverlay.slint](file:///home/lion/Documents/GitHub/ltk/slint/overlays/ScrimOverlay.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component ScrimOverlay inherits Rectangle {
    in-out property <bool> active: false;
    in property <float> scrim-opacity: 0.6;

    callback clicked();

    width: 100%;
    height: 100%;
    background: Theme.backdrop;
    opacity: active ? scrim-opacity : 0;
    visible: active;

    animate opacity { duration: Theme.dur-normal; easing: Theme.ease-standard; }

    ta := TouchArea {
        clicked => { root.clicked(); }
    }
}
```

#### `SideSheet` ([slint/overlays/SideSheet.slint](file:///home/lion/Documents/GitHub/ltk/slint/overlays/SideSheet.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component SideSheet inherits Rectangle {
    in-out property <bool> active: false;
    in property <string> title: "";

    callback closed();

    width: 100%;
    height: 100%;
    background: #00000000;

    // Scrim
    Rectangle {
        width: 100%;
        height: 100%;
        background: Theme.backdrop;
        opacity: root.active ? 1 : 0;
        visible: root.active;
        animate opacity { duration: Theme.dur-normal; easing: Theme.ease-standard; }

        TouchArea {
            clicked => { root.closed(); }
        }
    }

    // Panel
    Rectangle {
        x: root.active ? parent.width - self.width : parent.width;
        y: 0;
        width: 320px;
        height: parent.height;
        background: Theme.bg-surface;
        border-radius: Theme.radius-lg;
        border-width: 1px;
        border-color: Theme.border-subtle;

        drop-shadow-offset-x: -4px;
        drop-shadow-blur: 16px;
        drop-shadow-color: Theme.border-default;

        animate x { duration: Theme.dur-slow; easing: Theme.ease-soft; }

        VerticalLayout {
            padding: Theme.sp-4;
            spacing: Theme.sp-4;

            // Header
            HorizontalLayout {
                spacing: Theme.sp-3;
                alignment: space-between;
                vertical-stretch: 0;

                if root.title != "": Text {
                    text: root.title;
                    color: Theme.text-primary;
                    font-family: Theme.font-ui;
                    font-size: Theme.text-xl;
                    font-weight: Theme.weight-bold;
                    horizontal-stretch: 1;
                    vertical-alignment: center;
                }

                Rectangle {
                    width: 28px;
                    height: 28px;
                    horizontal-stretch: 0;

                    close-bg := Rectangle {
                        border-radius: Theme.radius-xs;
                        background: close-ta.pressed ? Theme.surface-pressed
                            : close-ta.has-hover ? Theme.surface-hover
                            : #00000000;
                    }

                    FaIcon {
                        x: (parent.width - self.width) / 2;
                        y: (parent.height - self.height) / 2;
                        fa-size: 12px;
                        fa-color: Theme.text-tertiary;
                        source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/xmark.svg");
                    }

                    close-ta := TouchArea {
                        clicked => { root.closed(); }
                    }
                }
            }

            Rectangle {
                height: 1px;
                background: Theme.divider;
                vertical-stretch: 0;
            }

            // Content slot
            Rectangle {
                vertical-stretch: 1;
            }
        }

        TouchArea { }
    }
}
```

#### `SimpleTooltip` ([slint/overlays/SimpleTooltip.slint](file:///home/lion/Documents/GitHub/ltk/slint/overlays/SimpleTooltip.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component SimpleTooltip inherits Rectangle {
    visible: false;
    in property <string> text: "";

    min-width: 40px;
    min-height: 28px;
    visible: root.visible;
    background: Theme.bg-elevated;
    border-radius: Theme.radius-tooltip;
    border-width: 1px;
    border-color: Theme.border-subtle;
    opacity: root.visible ? 1 : 0;

    drop-shadow-offset-x: 0px;
    drop-shadow-offset-y: Theme.elevation-1-y;
    drop-shadow-blur: Theme.elevation-1-blur;
    drop-shadow-color: Theme.elevation-1-color;

    animate opacity { duration: Theme.dur-fast; easing: Theme.ease-standard; }

    Text {
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
        text: root.text;
        color: Theme.text-primary;
        font-family: Theme.font-ui;
        font-size: Theme.text-xs;
        font-weight: Theme.weight-medium;
        vertical-alignment: center;
    }
}
```

#### `SpotlightOverlay` ([slint/overlays/SpotlightOverlay.slint](file:///home/lion/Documents/GitHub/ltk/slint/overlays/SpotlightOverlay.slint))

```slint
import { Theme } from "../core/Theme.slint";

export component SpotlightOverlay inherits Rectangle {
    visible: false;
    in property <string> title: "";
    in property <string> description: "";

    callback dismiss();

    width: 100%;
    height: 100%;
    background: #00000000;

    // Dark overlay with spotlight cut-out
    Rectangle {
        width: 100%;
        height: 100%;
        background: Theme.backdrop;
        opacity: root.visible ? 1 : 0;
        visible: root.visible;
        animate opacity { duration: Theme.dur-normal; easing: Theme.ease-standard; }

        TouchArea {
            clicked => { root.dismiss(); }
        }
    }

    // Spotlight highlight border
    Rectangle {
        x: (parent.width - 320px) / 2;
        y: (parent.height - 320px) / 2;
        width: 320px;
        height: 320px;
        border-radius: Theme.radius-xl;
        border-width: 2px;
        border-color: Theme.accent;
        background: #00000000;
        visible: root.visible;
        opacity: root.visible ? 1 : 0;

        animate opacity { duration: Theme.dur-medium; easing: Theme.ease-soft; }

        // Glow ring
        Rectangle {
            x: -4px;
            y: -4px;
            width: parent.width + 8px;
            height: parent.height + 8px;
            border-radius: 20px;
            border-width: 1px;
            border-color: Theme.accent-glow;
            background: #00000000;
        }

        // Center icon
        Rectangle {
            width: 48px;
            height: 48px;
            border-radius: 24px;
            background: Theme.accent-subtle;
            x: (parent.width - self.width) / 2;
            y: (parent.height - self.height) / 2;
        }
    }

    // Description card
    Rectangle {
        x: (parent.width - 360px) / 2;
        y: (parent.height + 320px) / 2 + Theme.sp-4;
        width: 360px;
        min-height: 120px;
        background: Theme.bg-surface;
        border-radius: Theme.radius-lg;
        border-width: 1px;
        border-color: Theme.border-subtle;
        visible: root.visible;
        opacity: root.visible ? 1 : 0;

        drop-shadow-offset-x: 0px;
        drop-shadow-offset-y: Theme.elevation-2-y;
        drop-shadow-blur: Theme.elevation-2-blur;
        drop-shadow-color: Theme.elevation-2-color;

        animate opacity { duration: Theme.dur-medium; easing: Theme.ease-soft; }

        VerticalLayout {
            padding: Theme.sp-6;
            spacing: Theme.sp-3;

            if root.title != "": Text {
                text: root.title;
                color: Theme.text-primary;
                font-family: Theme.font-ui;
                font-size: Theme.text-lg;
                font-weight: Theme.weight-bold;
                wrap: word-wrap;
                vertical-stretch: 0;
            }

            if root.description != "": Text {
                text: root.description;
                color: Theme.text-secondary;
                font-family: Theme.font-ui;
                font-size: Theme.text-base;
                wrap: word-wrap;
                vertical-stretch: 1;
            }

            Rectangle {
                height: 1px;
                background: Theme.divider;
                vertical-stretch: 0;
            }

            Rectangle {
                height: Theme.button-height-md;
                vertical-stretch: 0;

                Rectangle {
                    width: 100px;
                    height: Theme.button-height-md;
                    border-radius: Theme.radius-button;
                    background: dismiss-ta.pressed ? Theme.btn-primary-bg-pressed
                        : dismiss-ta.has-hover ? Theme.btn-primary-bg-hover
                        : Theme.btn-primary-bg;
                    x: parent.width - self.width;

                    animate background { duration: Theme.dur-fast; easing: Theme.ease-standard; }

                    dismiss-ta := TouchArea {
                        clicked => { root.dismiss(); }
                    }

                    Text {
                        x: (parent.width - self.width) / 2;
                        y: (parent.height - self.height) / 2;
                        text: "Got it";
                        color: Theme.on-accent;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-base;
                        font-weight: Theme.weight-medium;
                        vertical-alignment: center;
                    }
                }
            }
        }

        TouchArea { }
    }
}
```

#### `VideoLightbox` ([slint/overlays/VideoLightbox.slint](file:///home/lion/Documents/GitHub/ltk/slint/overlays/VideoLightbox.slint))

```slint
import { Theme } from "../core/Theme.slint";
import { FaIcon } from "../core/FaIcon.slint";

export component VideoLightbox inherits Rectangle {
    visible: false;
    in property <string> title: "";

    callback close();

    width: 100%;
    height: 100%;
    background: #00000000;

    // Scrim
    Rectangle {
        width: 100%;
        height: 100%;
        background: Theme.backdrop;
        opacity: root.visible ? 1 : 0;
        visible: root.visible;
        animate opacity { duration: Theme.dur-normal; easing: Theme.ease-standard; }

        TouchArea {
            clicked => { root.close(); }
        }
    }

    // Lightbox card
    Rectangle {
        x: (parent.width - self.width) / 2;
        y: (parent.height - self.height) / 2;
        width: 720px;
        height: 480px;
        background: Theme.bg-surface;
        border-radius: Theme.radius-dialog;
        border-width: 1px;
        border-color: Theme.border-subtle;
        visible: root.visible;
        opacity: root.visible ? 1 : 0;

        drop-shadow-offset-x: 0px;
        drop-shadow-offset-y: Theme.elevation-3-y;
        drop-shadow-blur: Theme.elevation-3-blur;
        drop-shadow-color: Theme.elevation-3-color;

        animate opacity { duration: Theme.dur-medium; easing: Theme.ease-soft; }

        VerticalLayout {
            spacing: 0px;

            // Header bar
            Rectangle {
                height: 44px;
                background: Theme.bg-raised;
                border-radius: Theme.radius-dialog;
                vertical-stretch: 0;

                HorizontalLayout {
                    padding-left: Theme.sp-4;
                    padding-right: Theme.sp-2;
                    spacing: Theme.sp-2;

                    if root.title != "": Text {
                        text: root.title;
                        color: Theme.text-primary;
                        font-family: Theme.font-ui;
                        font-size: Theme.text-md;
                        font-weight: Theme.weight-semibold;
                        horizontal-stretch: 1;
                        vertical-alignment: center;
                    }

                    Rectangle { horizontal-stretch: 1; }

                    Rectangle {
                        width: 28px;
                        height: 28px;
                        horizontal-stretch: 0;

                        close-bg := Rectangle {
                            border-radius: Theme.radius-xs;
                            background: close-ta.pressed ? Theme.surface-pressed
                                : close-ta.has-hover ? Theme.surface-hover
                                : #00000000;
                        }

                        FaIcon {
                            x: (parent.width - self.width) / 2;
                            y: (parent.height - self.height) / 2;
                            fa-size: 12px;
                            fa-color: Theme.text-tertiary;
                            source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/xmark.svg");
                        }

                        close-ta := TouchArea {
                            clicked => { root.close(); }
                        }
                    }
                }
            }

            // Video placeholder area
            Rectangle {
                background: Theme.bg-base;
                vertical-stretch: 1;

                Rectangle {
                    width: 64px;
                    height: 64px;
                    border-radius: 32px;
                    background: Theme.bg-overlay;
                    x: (parent.width - self.width) / 2;
                    y: (parent.height - self.height) / 2;

                    FaIcon {
                        x: (parent.width - self.width) / 2;
                        y: (parent.height - self.height) / 2;
                        fa-size: 28px;
                        fa-color: Theme.text-tertiary;
                        source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/clapperboard.svg");
                    }
                }
            }
        }

        TouchArea { }
    }
}
```

# 6. Motion, Transitions & Micro-Interactions

Motion adds physical continuity and user confidence.

---

## 6.1 Micro-Interaction Code Pattern

```slint
export component AnimatedCard inherits Rectangle {
    in property <string> title: "Card Title";
    
    background: ta.pressed ? Theme.surface-pressed : (ta.has-hover ? Theme.surface-hover : Theme.bg-surface);
    border-radius: Theme.radius-lg;
    border-width: 1px;
    border-color: ta.has-hover ? Theme.accent : Theme.border-subtle;

    animate background { duration: Theme.dur-fast; easing: Theme.ease-soft; }
    animate border-color { duration: Theme.dur-fast; easing: Theme.ease-soft; }

    VerticalLayout {
        padding: Theme.sp-4;
        Text {
            text: root.title;
            color: Theme.text-primary;
            font-size: Theme.text-lg;
            font-weight: Theme.weight-bold;
        }
    }

    ta := TouchArea { }
}
```

---

# 7. Rust Backend Integration & Asynchronous State Synchronization

LTK applications couple Slint UI components with high-performance Rust backends.

---

## 7.1 Coupling Slint Views with Rust Handlers (`src/main.rs`)

```rust
// src/main.rs

use std::error::Error;
use slint::ComponentHandle;

mod thememanager;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let main_window = LtkWindow::new()?;

    // Connect Rust D-Bus theme manager
    let main_window_weak = main_window.as_weak();
    let _theme_manager = thememanager::ThemeManager::new(move |dark_mode| {
        let _ = main_window_weak.upgrade_in_event_loop(move |window| {
            println!("[LTK] Theme changed via D-Bus: dark_mode = {}", dark_mode);
        });
    });

    main_window.run()?;
    Ok(())
}
```

---

## 7.2 Asynchronous Tokio D-Bus Theme Polling (`src/thememanager.rs`)

```rust
// src/thememanager.rs

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use zbus::blocking::Connection;

pub struct ThemeManager {
    running: Arc<AtomicBool>,
}

impl ThemeManager {
    pub fn new<F>(callback: F) -> Self
    where
        F: Fn(bool) + Send + 'static,
    {
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();

        thread::spawn(move || {
            let rt = match tokio::runtime::Builder::new_current_thread()
                .enable_time()
                .build()
            {
                Ok(rt) => rt,
                Err(e) => {
                    eprintln!("[LTK ThemeManager] Failed to create Tokio runtime: {:?}", e);
                    return;
                }
            };

            let mut last_state: Option<bool> = None;

            while running_clone.load(Ordering::Relaxed) {
                let current_dark_mode = query_system_dark_mode();

                if last_state != Some(current_dark_mode) {
                    last_state = Some(current_dark_mode);
                    callback(current_dark_mode);
                }

                rt.block_on(tokio::time::sleep(Duration::from_secs(5)));
            }
        });

        Self { running }
    }
}

impl Drop for ThemeManager {
    fn drop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
    }
}

fn query_system_dark_mode() -> bool {
    if let Ok(conn) = Connection::session() {
        if let Ok(reply) = conn.call_method(
            Some("org.freedesktop.portal.Desktop"),
            "/org/freedesktop/portal/desktop",
            Some("org.freedesktop.portal.Settings"),
            "Read",
            &("org.freedesktop.appearance", "color-scheme"),
        ) {
            if let Ok(value) = reply.body().deserialize::<zbus::zvariant::Value>() {
                if let zbus::zvariant::Value::U32(scheme) = value {
                    return scheme == 1; // 1 = Prefer Dark
                }
            }
        }
    }
    true // Fallback to dark mode
}
```

---

# 8. Complete End-to-End Liquid Reference Applications

---

## 8.1 Application 1: LTK Liquid Studio & Control Center

```slint
import { Theme } from "slint/core/Theme.slint";
import { PrimaryButton } from "slint/buttons/PrimaryButton.slint";
import { SecondaryButton } from "slint/buttons/SecondaryButton.slint";
import { SearchInput } from "slint/inputs/SearchInput.slint";
import { StatCard } from "slint/data-display/StatCard.slint";
import { ListItem } from "slint/data-display/ListItem.slint";
import { Card } from "slint/cards/Card.slint";
import { FaIcon } from "slint/core/FaIcon.slint";

export component LiquidStudioApp inherits Window {
    title: "LTK Liquid Studio Control Center";
    min-width: 540px;
    min-height: 420px;
    preferred-width: 1120px;
    preferred-height: 720px;
    background: Theme.bg-base;

    in-out property <int> active-tab: 0;
    property <bool> is-compact: self.width < 720px;

    HorizontalLayout {
        padding: Theme.sp-3;
        spacing: Theme.sp-3;

        // --- SIDEBAR (Collapses on compact windows) ---
        if !is-compact: Rectangle {
            width: 220px;
            background: Theme.bg-surface;
            border-radius: Theme.radius-lg;
            clip: true;

            VerticalLayout {
                padding: Theme.sp-4;
                spacing: Theme.sp-2;

                // Header Logo
                HorizontalLayout {
                    spacing: Theme.sp-2;
                    alignment: start;

                    FaIcon {
                        source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/shapes.svg");
                        fa-size: 22px;
                        fa-color: Theme.accent;
                        y: (parent.height - self.height) / 2;
                    }

                    Text {
                        text: "LTK Studio";
                        color: Theme.text-primary;
                        font-family: Theme.font-display;
                        font-size: Theme.text-xl;
                        font-weight: Theme.weight-bold;
                        vertical-alignment: center;
                    }
                }

                Rectangle { height: 16px; }

                // Sidebar Items
                for tab-name[i] in ["Dashboard", "Analytics", "Components", "Settings"]: Rectangle {
                    height: 40px;
                    border-radius: Theme.radius-md;
                    background: active-tab == i ? Theme.accent : (item-ta.has-hover ? Theme.surface-hover : transparent);
                    animate background { duration: Theme.dur-fast; }

                    Text {
                        x: Theme.sp-3;
                        y: (parent.height - self.height) / 2;
                        text: tab-name;
                        color: active-tab == i ? #FFFFFF : Theme.text-secondary;
                        font-size: Theme.text-base;
                        font-weight: active-tab == i ? Theme.weight-semibold : Theme.weight-regular;
                    }

                    item-ta := TouchArea {
                        clicked => { root.active-tab = i; }
                    }
                }

                Rectangle { vertical-stretch: 1; }
            }
        }

        // --- MAIN WORKSPACE VIEWPORT ---
        Rectangle {
            horizontal-stretch: 1;
            vertical-stretch: 1;
            background: Theme.bg-surface;
            border-radius: Theme.radius-lg;
            clip: true;

            Flickable {
                width: 100%;
                height: 100%;
                viewport-height: main-stack.preferred-height;

                main-stack := VerticalLayout {
                    padding: Theme.sp-6;
                    spacing: Theme.sp-6;

                    // Liquid Title Header
                    HorizontalLayout {
                        alignment: space-between;

                        VerticalLayout {
                            spacing: Theme.sp-1;

                            Text {
                                text: "System Telemetry & Controls";
                                color: Theme.text-primary;
                                font-family: Theme.font-display;
                                font-size: Math.max(20px, Math.min(32px, parent.width / 22));
                                font-weight: Theme.weight-bold;
                            }

                            Text {
                                text: "Real-time metrics and desktop portal state.";
                                color: Theme.text-secondary;
                                font-size: Theme.text-sm;
                            }
                        }

                        if !is-compact: PrimaryButton {
                            text: "Export Log";
                            horizontal-stretch: 0;
                        }
                    }

                    // Form Search & Action Bar
                    HorizontalLayout {
                        spacing: Theme.sp-3;

                        SearchInput {
                            placeholder: "Search telemetry records...";
                            horizontal-stretch: 1;
                        }

                        SecondaryButton {
                            text: "Filters";
                            horizontal-stretch: 0;
                        }
                    }

                    // Stat Cards Grid
                    HorizontalLayout {
                        spacing: Theme.sp-4;

                        StatCard {
                            label: "ACTIVE PROCESSES";
                            value: "148";
                            subtitle: "+12 background threads";
                            stat-color: Theme.accent;
                            horizontal-stretch: 1;
                        }

                        StatCard {
                            label: "CPU LATENCY";
                            value: "0.8 ms";
                            subtitle: "GL Hardware Accelerated";
                            stat-color: Theme.success;
                            horizontal-stretch: 1;
                        }

                        if !is-compact: StatCard {
                            label: "MEMORY ALLOCATED";
                            value: "42.4 MB";
                            subtitle: "Zero leak footprint";
                            stat-color: Theme.warning;
                            horizontal-stretch: 1;
                        }
                    }

                    // Detailed Card Panel
                    Card {
                        title: "Active Component Subsystems";
                        subtitle: "Loaded Slint modules and backend status.";

                        VerticalLayout {
                            padding-top: Theme.sp-3;
                            spacing: Theme.sp-2;

                            ListItem { title: "D-Bus Theme Sync"; subtitle: "Listening on org.freedesktop.portal.Settings"; }
                            ListItem { title: "Liquid Renderer"; subtitle: "Slint 1.17 GL backend active"; }
                            ListItem { title: "Component Harness"; subtitle: "32 test modules verified"; }
                        }
                    }
                }
            }
        }
    }
}
```

---

# Summary & Verification Commands

### Preview `.slint` Layouts Live
```bash
/home/lion/.cargo/bin/slint-viewer -I slint slint/ltk.slint
```

### Run Full Test Suite
```bash
cargo test
```
