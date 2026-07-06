# Leonux Design Reference
## Spacing · Radius Hierarchy · Nested Box Radii

---

## 1 · Radius Hierarchy (5-tier named system)

Leonux uses five named radius levels — **Primary through Quinary** —
inspired by the macOS corner-radius progression and squircle geometry.

| Level | Name | Value | Applies to |
|-------|------|-------|-----------|
| **r1** | **Primary** | **24px** | App windows, large modals, fullscreen overlays, onboarding screens |
| **r2** | **Secondary** | **16px** | Cards, panels, popovers, sidebars, bottom sheets |
| **r3** | **Tertiary** | **10px** | Buttons, text fields, inputs, menus, small cards |
| **r4** | **Quaternary** | **6px** | Tags, chips, badges, inner containers, selection rings |
| **r5** | **Quinary** | **3px** | Innermost elements, pixel-art precision, dense list items |
| ∞ | **Full** | **9999px** | Pills, toggles, avatar circles, search fields |

---

## 2 · Nested Box Rule — The Apple Formula

> **Inner radius = Outer radius − Gap between the two boxes**

When an element is placed inside another with `gap` pixels of space between
their borders, the inner element's radius must shrink by exactly `gap` so
the arcs appear concentric (not misaligned).

### Derivation

```
outer_radius = r
gap (padding) = p

inner_radius = r - p
```

This is used throughout macOS, iOS, visionOS, and is the canonical rule
in the Apple HIG. GNOME adopted it in libadwaita for its preference rows.

---

## 3 · Your Four Nested Boxes

```
┌──────────────────────────── r1 = 24px ─────────────────────────────┐
│  gap = 8px                                                          │
│  ┌───────────────────────── r2 = 16px ──────────────────────────┐  │
│  │  gap = 6px                                                    │  │
│  │  ┌──────────────────── r3 = 10px ──────────────────────┐     │  │
│  │  │  gap = 4px                                          │     │  │
│  │  │  ┌──────────────── r4 = 6px ──────────────────┐    │     │  │
│  │  │  │                                             │    │     │  │
│  │  │  │  ← innermost content lives here →           │    │     │  │
│  │  │  │                                             │    │     │  │
│  │  │  └─────────────────────────────────────────────┘    │     │  │
│  │  │                                                      │     │  │
│  │  └──────────────────────────────────────────────────────┘     │  │
│  │                                                                │  │
│  └────────────────────────────────────────────────────────────────┘  │
│                                                                       │
└───────────────────────────────────────────────────────────────────────┘
```

| Box | Radius | Gap to next box |
|-----|--------|-----------------|
| **Box 1** (outermost — window / container) | **r1 = 24px** | 8px |
| **Box 2** (card / panel) | **r2 = 16px** | 6px |
| **Box 3** (inner card / input group) | **r3 = 10px** | 4px |
| **Box 4** (innermost — value / chip) | **r4 = 6px** | — |

**Rule in code:**
```slint
box1.border-radius = 24px;
box2.border-radius = box1.border-radius - gap1;  // 24 - 8 = 16
box3.border-radius = box2.border-radius - gap2;  // 16 - 6 = 10
box4.border-radius = box3.border-radius - gap3;  // 10 - 4 = 6
```

> If your gap is always 8px, the sequence becomes: **24 → 16 → 8 → 4** (r5 = 4px, use Quinary).

---

## 4 · Leonux Spacing System

Based on an **8px base unit** aligned with both macOS HIG and GNOME HIG.

### Named Scale

```
micro   =  4px   — icon–label gap, badge padding, hairline insets
xs      =  8px   — tight field padding, dense list row padding
sm      = 12px   — form field internal padding
md      = 16px   — standard gap between elements  ← BASE UNIT × 2
lg      = 24px   — section spacing
xl      = 32px   — view-level padding
2xl     = 48px   — hero/feature sections
3xl     = 64px   — full-page padding on wide screens
```

### Rationale

```
Every value = 4px × n   (so: 4, 8, 12, 16, 24, 32, 48, 64)
```

Half-steps (4px, 12px) are allowed only for tight/dense contexts.
Use `micro = 4px` for internal component padding (e.g., badge inset).
Never use odd values (5px, 7px, 11px) — they break the grid.

### Component-Specific Metrics (Leonux standard)

| Token | Value | Applies to |
|-------|-------|-----------|
| `control-sm` | 24px | Compact controls (macOS small) |
| `control-md` | 32px | Standard controls (macOS regular) |
| `control-lg` | 40px | Large controls (GNOME button height) |
| `input-sm`   | 28px | Compact inputs |
| `input-md`   | 36px | Standard inputs (GNOME entry) |
| `input-lg`   | 44px | Large / touch-friendly inputs |
| `headerbar`  | 47px | GNOME headerbar |
| `titlebar`   | 38px | macOS compact titlebar |
| `sidebar`    | 260px| Standard sidebar width |
| `nav-rail`   | 72px | Collapsed icon-only rail |
| `traffic`    | 12px | Traffic light dot diameter |
| `traffic-gap`| 8px  | Gap between traffic light dots |

### Padding Inside Nested Boxes (matches radius rule)

| Context | Padding |
|---------|---------|
| Window to first card | `lg = 24px` |
| Card to inner panel | `md = 16px` |
| Panel to input group | `sm = 12px` |
| Input group to value chip | `xs = 8px` |
| Chip internal | `micro = 4px` |

---

## 5 · Quick Reference (Slint globals)

```slint
export global LeonuxRadius {
    out property <length> primary:    24px;
    out property <length> secondary:  16px;
    out property <length> tertiary:   10px;
    out property <length> quaternary:  6px;
    out property <length> quinary:     3px;
    out property <length> full:     9999px;
}

export global LeonuxSpace {
    out property <length> micro:  4px;
    out property <length> xs:     8px;
    out property <length> sm:    12px;
    out property <length> md:    16px;
    out property <length> lg:    24px;
    out property <length> xl:    32px;
    out property <length> xl2:   48px;
    out property <length> xl3:   64px;
}
```

---

*Leonux Design Reference · LionOS · © 2026 Lion*
