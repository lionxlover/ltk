# Slint Syntax Gotchas — Discovered This Session

Companion to this project's own `gotchas.md`. Everything here was found by
hitting a real compiler error (via the person's VS Code Slint Preview) or by
deliberately verifying against Slint's official docs before use — not by
assumption. Each entry shows the wrong code, why it's wrong, and the fix.

---

### `width`/`height` silently alias `min-`/`max-width`/`height`

```slint
// WRONG — "Cannot specify both 'width' and 'min-width'"
width: 40px;
min-width: 40px;
max-width: 40px;
```
`width: 40px;` already sets both `min-width` and `max-width` to `40px`
internally (confirmed via slint-ui/slint discussion #6287 — a maintainer
explains why: "width is also an alias to min-width and max-width"). Adding
either again is two bindings for the same constraint. This applies even
when the values match, and even when `width` is a computed expression, not
a literal.

```slint
// RIGHT — fixed size
width: 40px;
height: 40px;

// RIGHT — elastic range, no separate `width` binding at all
min-width: 64px;
max-width: 480px;

// RIGHT — a fixed value with a cap folded into one binding, instead of
// width: X; max-width: Y;
width: min(480px, max(220px, content.preferred-width));
```

---

### `radius-full` only works as a circle-clamp on square elements

```slint
// WRONG on a wide, short Rectangle — renders as a full ellipse, not a pill
height: 32px;
border-radius: Theme.radius-full; // 9999px
```
An oversized radius clamps correctly to a circle on a square element
(width == height), but on an elongated shape it does not clamp into a
flat-sided stadium the way CSS `border-radius` would.

```slint
// RIGHT
border-radius: self.height / 2; // exact by construction
```

---

### Rotation: `transform-rotation`, not `rotation-angle` (usually)

```slint
// WRONG on a Rectangle — "Unknown property rotation-angle"
rotation-angle: 45deg;
rotation-origin-x: self.width / 2;
rotation-origin-y: self.height / 2;
```
`rotation-angle`/`rotation-origin-x`/`rotation-origin-y` are real, but
**`Image`-only** (confirmed: Slint's builtin-elements docs describe them
under `Image` specifically, noting "the Image can't have children" once
set). For anything else — Rectangle, layouts, any general visual item —
use the common properties instead, which default to rotating around the
element's own center:

```slint
// RIGHT — works on any visual item, no origin properties needed
transform-rotation: 45deg;
```

---

### Infinite animation loop needs a nudge, not a static binding

```slint
// WRONG — never animates. angle is 360 from the very first frame, so
// there's no value *change* for `animate` to play.
property <bool> spinning: true;
property <float> angle: spinning ? 360 : 0;
transform-rotation: angle * 1deg;
animate angle { duration: 900ms; iteration-count: -1; }
```
```slint
// RIGHT — init bumps the value once, giving animate an actual transition
// to replay forever. The 360→0 seam is invisible on a symmetric shape.
property <float> spin-angle: 0;
transform-rotation: spin-angle * 1deg;
animate spin-angle { duration: 900ms; easing: linear; iteration-count: -1; }
init => { spin-angle = 360; }
```
Confirmed: `iteration-count: -1` means infinite reruns (Slint's animation
docs). Only works inside an unconditionally-animating element, or inside
`if condition: Element { ... }` if you want the loop to reset cleanly each
time `condition` becomes true (each mount reruns `init`).

---

### `@children` cannot be inside a conditional element

```slint
// WRONG — real compiler error: "The @children placeholder ..."
if expanded: VerticalLayout {
    @children
}
```
This is a genuine, currently-open Slint limitation (issue #6354, "Support
for conditional children wrappers"), not a syntax mistake.

```slint
// RIGHT — keep the wrapper unconditional, animate height instead
content-clip := Rectangle {
    clip: true;
    height: expanded ? content-inner.preferred-height : 0px;
    animate height { duration: 200ms; easing: ease; }
    content-inner := VerticalLayout {
        @children
    }
}
```
Bonus: this also animates, which the `if`-based version could never do
anyway (instant mount/unmount).

---

### `PopupWindow`, not a nested `Rectangle`, for anything that must escape clipping

A plain `Rectangle` positioned with `y: parent.height + ...` gets clipped
by any ancestor's `clip: true` and renders **under** later siblings (which
render on top by z-order). `PopupWindow` (confirmed, official example:
`popup := PopupWindow { ... } TouchArea { clicked => { popup.show(); } }`)
is the actual mechanism, and Slint 1.9+ auto-places it to stay on-window.

```slint
// close-on-click is deprecated (real warning, not just a style note) —
// use close-policy instead. These two are the exact 1:1 replacements:
close-policy: PopupClosePolicy.no-auto-close;        // was: close-on-click: false
close-policy: PopupClosePolicy.close-on-click-outside; // was: close-on-click: true
```
Caveat: if you use `close-on-click-outside`, nothing notifies your
component that the popup closed itself — don't pair it with your own
`expanded`-style flag unless you have a confirmed way to resync it.

---

### Arrays: read and reassign, don't index-assign

```slint
// LIKELY WRONG — no Slint documentation confirms this exists.
// The language reference describes array[index] as retrieval only.
root.checked-items[i] = !root.checked-items[i];
```
```slint
// RIGHT — report the change, let the host reassign the whole array
callback changed(int, bool);
// in the click handler:
root.changed(i, !is-checked(i));
// host does: my_array[i] = new_state; component.set_checked_items(my_array);
```
`.length` and `array[i]` (reading) are both confirmed and safe.

---

### Strings have no length, indexing, or slicing

Confirmed against Slint's full primitive-types member list: `to-float()`,
`is-float()`, `is-empty`, `to-lowercase()`, `to-uppercase()` — that's the
complete set. No `.length`, no `str[i]`, no substring. For anything
per-character (PIN entry, etc.), use separate statically-named properties
instead of trying to index into one string:

```slint
// RIGHT pattern for e.g. a 6-digit PIN field
private property <string> d0: ""; private property <string> d1: ""; /* ...d2-d7 */
property <string> combined: d0 + d1 + d2 + d3 + d4 + d5 + d6 + d7;
```

---

### `TouchArea.pressed-x`/`pressed-y` are a snapshot, not a live position

```slint
// WRONG for continuous drag tracking — pressed-x is frozen at the moment
// the press started; the handle jumps once and then stops following you.
moved => { root.value = compute-from(self.pressed-x); }
```
```slint
// RIGHT — mouse-x/mouse-y update continuously while dragging
moved => { root.value = compute-from(self.mouse-x); }
```

---

### Every `TouchArea`/`FocusScope` needs an explicit `enabled:` binding

```slint
// WRONG — dimming via opacity does NOT disable hit-testing
opacity: enabled ? 1.0 : 0.4;
TouchArea { clicked => { ... } }
```
```slint
// RIGHT
TouchArea {
    enabled: root.enabled;
    mouse-cursor: root.enabled ? MouseCursor.pointer : MouseCursor.default;
    clicked => { ... }
}
```
Found this exact bug repeated across 10+ components (checkboxes, switches,
radios, sliders, several buttons) — all stayed fully clickable while
visually "disabled."

---

### Two-way text binding needs `<=>`, not `:`

```slint
// RISKY — a persistent one-way binding keeps re-deriving input.text from
// root.text, which never changes as the user types. Can fight typing.
input := TextInput { text: root.text; }
```
```slint
// RIGHT
in-out property <string> text <=> input.text;
```

---

### Confirmed-real APIs worth knowing (all verified against official docs this session)

- `AccessibleRole`: `none, button, checkbox, combobox, groupbox, image,
  list, list-item, progress-indicator, radio-button, slider, spinbox,
  switch, tab, tab-list, tab-panel` — plus `accessible-label`,
  `accessible-enabled`, `accessible-checkable`/`-checked`,
  `accessible-expandable`/`-expanded`, `accessible-value` +
  `-minimum`/`-maximum`. Must be a **constant**, never a reactive
  expression like `cond ? A : B` — that's its own "must be constant"
  compile error.
- `Key.Return`, `.Backspace`, `.Home`, `.End`, arrow keys — all confirmed
  in the official Key namespace (and Slint's changelog literally says
  their own Slider widget "React[s] to Home and End keys" — matches this
  convention).
- `Timer { interval: ...; running: ...; triggered => {...} }` and
  `changed <property> => {...}` — both real, added in Slint 1.8.
- `SwipeGestureHandler { handle-swipe-right: true; swiped => {...} }`
  with `.swiping`/`.current-position`/`.pressed-position` — real, same
  1.8 release. Full property surface beyond what's shown in the one
  official example isn't confirmed — don't assume more than that.
- `.darker()`, `.brighter()`, `.transparentize()`, `.with-alpha()`,
  `.mix()` on colors — all confirmed.
- `MouseCursor` full enum includes `pointer, grab, grabbing, text,
  col-resize, ew-resize, ...` — confirmed via Slint's own source commit.
- `input.focus()` directly on a `TextInput` (not just `FocusScope`) —
  confirmed via official focus-handling docs example.
- `border-top-left-radius` / `-top-right-` / `-bottom-left-` /
  `-bottom-right-radius` — real, per-corner, for asymmetric shapes
  (a panel flush against one edge, etc.).

### `mouse-cursor` only exists on `TouchArea` (and a few input widgets) — not on `Rectangle`

```slint
// WRONG — real compiler error: "Unknown property mouse-cursor"
divider := Rectangle {
    mouse-cursor: MouseCursor.ew-resize;   // Rectangle has no such property

    TouchArea {
        mouse-cursor: parent.mouse-cursor; // compounds the mistake — reads
                                            // a property that doesn't exist
                                            // on the parent either
    }
}
```
Caught live in VS Code's Slint Preview (Problems panel: `Unknown property
mouse-cur...`, pointing at the `Rectangle`'s binding). `mouse-cursor` is a
`TouchArea` property (see the confirmed-APIs list below for the full enum)
— it doesn't exist on plain visual elements like `Rectangle`, and setting
it there compiles as an "unknown property" error, not a silent no-op.
Reading it back via `parent.mouse-cursor` from inside the `TouchArea` is
the same mistake in reverse: the parent never had the property to read.

```slint
// RIGHT — the ternary (or whatever expression) lives directly on the
// TouchArea itself, not on an ancestor
divider := Rectangle {
    TouchArea {
        mouse-cursor: horizontal ? MouseCursor.ew-resize : MouseCursor.ns-resize;
    }
}
```
This is the same shape as the `TouchArea.pressed-x`/`pressed-y` and
`enabled:` gotchas above: properties that only exist on `TouchArea`
(`mouse-cursor`, `pressed`, `has-hover`, `mouse-x`/`mouse-y`,
`pressed-x`/`pressed-y`) have to be read and set *on* the `TouchArea`
itself — never assumed onto, or read back from, whatever element wraps it.

---

### A property binding cannot read its own value — not even as a ternary fallback

```slint
// WRONG — real compiler error (VS Code Problems panel: "The binding for
// the property..."), flagged even though the self.height branch is only
// reached when use-width is false
height: use-width ? (self.width / 1px * ratio-denominator / ratio-numerator) * 1px
                   : self.height;
```
This is the same "binding loop" class Slint's own debugging docs mention
(a property depending on itself through a chain), just in its most direct
form: `height`'s binding expression contains `self.height`. It doesn't
matter that the offending branch is never the one that fires for a given
`use-width` value — Slint's dependency graph is built from the *textual*
bindings, not evaluated per-branch, so any mention of the property's own
name inside its own binding is a cycle. This is a narrower case of the
more general reciprocal-cross-reference bug below (`width` reading
`height` while `height` reads `width`): even fixing the cross-reference by
only ever binding one of the pair isn't enough if that one binding's
"unimplemented" fallback branch reads itself.

```slint
// RIGHT — fall back to a different element's property, e.g. the parent's,
// never the same property on the same element
height: use-width ? (self.width / 1px * ratio-denominator / ratio-numerator) * 1px
                   : parent.height;
```
Reading `root.<same-prop>` instead of `self.<same-prop>` has the identical
problem when `root` *is* the element being bound (it doesn't when `root`
refers to a different, outer component than the element currently being
positioned — e.g. a `PopupWindow`'s `width: root.width` reading its
enclosing component's width is a normal cross-element binding, not a
self-reference).

---

### Reciprocal width/height ternary bindings are still a binding loop, even across mutually-exclusive branches

```slint
// WRONG — width's binding depends on height, height's binding depends on
// width; Slint flags this as a cycle even though at runtime only one
// direction is ever active for a given `use-width` value
height: use-width ? (root.width / 1px * ratio-denominator / ratio-numerator) * 1px : root.height;
width: use-width ? root.width : (root.height / 1px * ratio-numerator / ratio-denominator) * 1px;
```
Same root cause as the self-reference case above, one level less obvious:
the dependency graph doesn't know the two ternaries are logically
mutually exclusive — it just sees "`width` binding mentions `height`" and
"`height` binding mentions `width`" and calls that a loop.

```slint
// RIGHT — only ever bind ONE of width/height on a given element. If the
// component genuinely needs to support deriving in both directions,
// that's two different components (or leave the second dimension for the
// host to set explicitly), not one component with reciprocal formulas.
height: use-width ? (parent.width / 1px * ratio-denominator / ratio-numerator) * 1px : parent.height;
// (width is left unbound here — the host either sets it explicitly, or a
// parent layout fills it — never derived from this element's own height)
```

---

### `parent` doesn't exist when a component is the top-level/root element

```slint
// WRONG — fine when nested inside something, but a real runtime error
// ("Cannot access id 'parent'") when this exported component is itself
// the top-level element — e.g. previewed standalone in Slint Preview, or
// instantiated directly as a window's content
export component AspectRatioBox inherits Rectangle {
    height: use-width ? (...) : parent.height;   // <- fails at root
}
```
Caught live via the person's Slint Preview (Problems panel: `Cannot
access id 'parent'`, Ln 28). Every exported component in a library like
this one is a *candidate* top-level element — the Preview tool loads
"the last exported" component from a file as the window root (per this
project's own `debugging-and-mcp.md`), and any consuming app could
likewise drop one of these components straight into a `Window`. `parent`
is only guaranteed to exist for elements that are actually nested inside
something. A `parent.foo` reference written directly in the *root*
component's own bindings breaks the moment that component has no
enclosing element — which, for anything meant to be reusable/exported,
is a real, expected use case, not a corner case.

Nested children referencing `parent.foo` are unaffected — inside a child
element, `parent` resolves to the *root* component itself (which always
exists once the component is instantiated), not to whatever contains the
whole component. The risk is specific to `parent.*` written directly on
the root element's own top-level bindings.

```slint
// RIGHT — an explicit `in property` fallback has no dependency on
// anything existing outside the component, and (being a distinct,
// unrelated property) can't become self-referential either
export component AspectRatioBox inherits Rectangle {
    in property <length> fallback-height: 200px;
    height: use-width ? (...) : fallback-height;
}
```

---

### `width: cond ? X : 0px` fighting `horizontal-stretch` on the same element

```slint
// WRONG — visually confirmed via VS Code Slint Preview: non-first columns
// collapse to zero width and their Text content renders on top of each
// other (garbled/doubled-looking text), regardless of horizontal-stretch
Rectangle {
    horizontal-stretch: i == 0 ? 0 : 1;
    width: i == 0 ? 120px : 0px;   // <- the 0px arm is the problem
}
```
This is the width/min-max aliasing gotcha above in a less obvious shape:
`width: 0px` still sets `min-width` and `max-width` to `0px` even when
`0px` is meant as a "don't care, let stretch decide" placeholder for the
non-fixed columns. Once min-width=max-width=0, `horizontal-stretch: 1`
has no room to redistribute into — the element is clamped to exactly zero
width no matter its stretch factor. Any `Text`/content inside that
zero-width element that uses explicit `x:` positioning (rather than being
laid out) isn't clipped by default, so it still renders — just at the same
collapsed x-position as every sibling column, overlapping them all.

```slint
// RIGHT — preferred-width is a distinct property that does NOT alias
// min-width/max-width, so it cooperates with horizontal-stretch instead
// of fighting it. This is the correct, idiomatic mechanism for "one fixed
// column + N stretchy columns" in a single HorizontalLayout.
Rectangle {
    horizontal-stretch: i == 0 ? 0 : 1;
    preferred-width: i == 0 ? 120px : 0px;
}
```

---

### An explicit `x:` on a layout-managed child doesn't reliably reposition it — use a real spacer element instead

```slint
// WRONG — visually confirmed via the person's Slint Preview screenshots:
// no staircase indentation appeared at all, despite depth data being
// correct and the binding being syntactically valid
VerticalLayout {
    for cmt[idx] in root.item-count: Rectangle {
        x: root.demo-depths[idx] * 24px;   // <- silently has no visible effect
        HorizontalLayout { /* avatar, text, ... */ }
    }
}
```
This project's own `gotchas.md` says an explicit `x: 0;` inside a layout
"even overrides the computed position" — true for `0`, but empirically a
*non-zero* explicit `x:` on a direct child of `VerticalLayout`/
`HorizontalLayout` doesn't reliably show up in the render the way the
same binding would on a child of a plain `Rectangle` with no enclosing
layout. Confirmed by a direct side-by-side in the same screenshot set:
`HorizontalTimeline`'s `for`-loop items *do* use `x:` successfully to
place themselves along a line — but its `for` loop is a direct child of
a plain `Rectangle` root with no `VerticalLayout`/`HorizontalLayout`
wrapping it (the "reserve x/y for overlays and custom drawing" pattern
from `language-and-layout.md`). The moment a `VerticalLayout` or
`HorizontalLayout` actually owns the child's positioning, don't fight it
with `x:`/`y:` — add a real spacer element instead, exactly the pattern
`TreeTable` already uses correctly for its own depth-based indentation:

```slint
// RIGHT — a real Rectangle that consumes layout space and pushes
// everything after it to the right, inside the row's own HorizontalLayout
VerticalLayout {
    for cmt[idx] in root.item-count: Rectangle {
        HorizontalLayout {
            Rectangle {
                width: root.demo-depths[idx] * 24px;
                horizontal-stretch: 0;
            }
            /* avatar, text, ... */
        }
    }
}
```

---

### `vertical-stretch: 1` collapses to near-zero when the only content is absolutely-positioned

```slint
// WRONG — visually confirmed: all 5 week rows compressed into a tiny
// strip, with the 30px selected-day circles overlapping each other
// between adjacent weeks
for row_idx in 5: HorizontalLayout {
    vertical-stretch: 1;   // <- no explicit height, and nothing else
                            //    forces real space either
    for col_idx in 7: Rectangle {
        horizontal-stretch: 1;
        Rectangle {
            x: (parent.width - 30px) / 2;
            y: (parent.height - 30px) / 2;
            width: 30px; height: 30px;   // <- absolutely positioned,
        }                                 //    doesn't count toward the
    }                                     //    row's preferred size
}
```
`vertical-stretch: 1` only distributes *already-available* leftover
space among stretchy siblings — it doesn't itself create space. A row's
own preferred height (what it asks for before stretching) normally comes
from its content, but content placed via absolute `x:`/`y:` (rather than
through normal layout flow) doesn't reliably contribute to that
preferred-size calculation. With no ancestor forcing extra height either
(the component was placed directly in a `VerticalLayout` with no fixed
wrapper `height:`, unlike sibling components in the same file), every row
collapsed toward ~0px while the fixed-size circles inside still rendered
at full size — producing overlap between weeks instead of a gap.

```slint
// RIGHT — an explicit height reserves real space per row; the column
// Rectangle underneath it still just uses normal Rectangle fill-by-default
// behavior to take that space, no changes needed there
for row_idx in 5: HorizontalLayout {
    height: 38px;
    for col_idx in 7: Rectangle {
        horizontal-stretch: 1;
        /* ... 30px circle now centers with (38-30)/2 = 4px margin ... */
    }
}
```
Not a bug whenever the stretchy row/column is nested inside *some*
ancestor that already has an explicit `height:`/`width:` (a very common,
safe pattern throughout this codebase, e.g. `FeatureComparisonMatrix`'s
and `GanttTimeline`'s header rows) — the collapse only happens when
nothing anywhere in the chain establishes real space and the only content
is absolutely positioned.

---

### Unconfirmed — don't guess, verify first if you need these

- Whether a `FocusScope` wrapping a `TextInput` or a `PopupWindow`
  receives key events while that child holds real focus. Hit this twice
  (`NumberInput` arrow-key stepping, `Select` in-popup navigation) and
  both times chose to leave a documented gap rather than ship a guess.
- `if (cond) { A } else { B }` as a property-binding *expression*
  (versus imperative statements inside a callback, which is confirmed).
  Always used the ternary form instead — never needed to resolve this.
