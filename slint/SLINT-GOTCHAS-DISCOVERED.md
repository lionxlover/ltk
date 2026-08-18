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

### An `x`-positioned `for`-loop element outside any `Layout` needs *every* dimension explicit, not just `x`

```slint
// WRONG — visually confirmed: label text rendered overlapping the node
// circle instead of clearly below it
for item[idx] in root.items: Rectangle {
    x: 24px + idx * (...) - 10px;   // explicit
    width: 20px;                     // explicit
    // no y:, no height: — left to Slint's implicit default

    Rectangle { /* the circle, also no y: */ width: 20px; height: 20px; }
    Text { y: 28px; /* ... */ }      // meant to sit "28px below the circle"
}
```
Inside a `VerticalLayout`/`HorizontalLayout`, leaving a dimension unset
is completely normal — the layout fills it in. But this `for`-loop body
is a direct child of a plain `Rectangle` root with **no** enclosing
layout (the legitimate "reserve x/y for overlays and custom drawing"
pattern) — nothing manages the unset dimensions for it. A `Rectangle`
"fills its parent by default" for whichever dimension isn't overridden,
so leaving `y`/`height` unset here doesn't center or collapse
predictably the way it might elsewhere — it left the circle and label
positioned relative to an origin that didn't line up with the connector
line's own fixed `y: 19px`, so the label's `y: 28px` (meant as "well
below the circle") ended up overlapping it instead.

```slint
// RIGHT — every dimension explicit and deterministic once nothing
// external (a Layout) is managing positioning for this element
for item[idx] in root.items: Rectangle {
    x: 24px + idx * (...) - 10px;
    y: 0px;
    width: 20px;
    height: 80px;   // matches the root's own height

    Rectangle { y: 9px; width: 20px; height: 20px; /* ... */ }  // centers on the y:19px line
    Text { y: 34px; /* ... */ }  // clear gap below the circle's y:9+20=29 bottom edge
}
```

---

### `x:`/`y:` cannot be set on a *direct* child of a `Layout` that manages that axis — not even to override it

```slint
// WRONG — real compiler error: "The property 'x' cannot be set for
// elements placed in this layout, because the layout is already setting
// it"
HorizontalLayout {
    Rectangle { width: 24px; }
    Text {
        x: Theme.sp-3;   // <- HorizontalLayout owns x for its direct
        text: "...";     //    children; this is a hard error, not a
    }                    //    silently-ignored/overridden binding
}
```
This project's other reference file (`language-and-layout.md`) says "an
explicit `x: 0` even overrides the computed position" inside a layout —
that turned out to be too broad, confirmed wrong by an actual compiler
error caught in `TreeTable.slint`'s Slint Preview. The real rule is
per-axis and depends on which layout: `HorizontalLayout` manages `x`/
`width` for its direct children (so neither can be overridden there —
`width` is fine to override, only `x` errors, since stretch factors
still need to control width distribution); `VerticalLayout` manages `y`/
`height` the same way. The *other* axis (e.g. `y` inside a
`HorizontalLayout`) is fine to set freely, since the layout doesn't
manage it. This only applies to **direct** children of the layout
element itself — wrap the element needing custom positioning in a plain
`Rectangle` (which the layout is free to size via `width`/
`horizontal-stretch` as normal), and set `x`/`y` on something *inside*
that wrapper instead, where no layout owns that axis.

```slint
// RIGHT — the Rectangle participates in the layout normally (sized via
// horizontal-stretch); the Text inside it is free to set its own x,
// since nothing owns that axis for a plain Rectangle's children
HorizontalLayout {
    Rectangle { width: 24px; }
    Rectangle {
        horizontal-stretch: 1;
        Text { x: Theme.sp-3; text: "..."; }
    }
}
```
Scanned the whole batch afterward for the same shape (any direct
`HorizontalLayout`/`VerticalLayout` child with an explicit `x:`/`y:` on
itself) — `TreeTable` was the only instance.

---

### A `for` loop over `[prop1, prop2, ...]` iterates copies, not live references — can't two-way bind back

```slint
// WRONG — box is a COPY of whichever property's value was in the array
// at that index when the array literal was evaluated; two-way binding
// `text <=> box` has nothing live to write back to
in-out property <string> d0: "";
in-out property <string> d1: "";
for box[index] in [d0, d1]: TextInput {
    text <=> box;   // does not, and cannot, update root.d0 / root.d1
}
```
This looks like it should work — `box` seems to "be" `d0` or `d1` on
each iteration — but a `for` loop's model is evaluated as an array of
values, not an array of bindable references to the original properties.
Once `[d0, d1]` is evaluated into a model, each element is an independent
copy; there is no mechanism in Slint to iterate "the next named property"
by reference. This matters specifically when several *separately named*
properties (not a single array property) each need their own editable
`TextInput`, e.g. per-digit OTP boxes — see the existing note above on
per-character fields (strings have no indexing, so multi-box text entry
already has to use separate `d0`/`d1`/… properties instead of one
string).

```slint
// RIGHT — write out each box explicitly against its own named property;
// verbose, but each TextInput's <=> binding is real
TextInput { text <=> root.d0; }
TextInput { text <=> root.d1; }
```

---

### `<=>` fails on a builtin's output-only property — `TextInput.has-focus` can't be two-way bound

```slint
// WRONG — real compiler error: "Cannot link to a output pr[operty]"
in-out property <bool> has-focus <=> input.has-focus;
```
`<=>` requires both sides to be writable. `TextInput.has-focus` is
output-only (the framework sets it based on real focus state; nothing
outside can assign it), so aliasing an `in-out` (or even an `out`)
property to it with `<=>` fails — `<=>` isn't "alias to whatever this
is," it specifically needs a two-way-capable target. This is easy to
miss because plenty of *other* two-way bindings in this exact codebase
work fine with `<=>` against a builtin — `TextInput.text` genuinely is
bidirectionally writable, so `text <=> root.text;` is correct — the
failure is specific to properties the framework only ever writes to
from its own side.

```slint
// RIGHT — one-way binding (`:`) reads the real focus state without
// trying to make it writable from outside; `out`, not `in-out`, since a
// host has no business writing this property anyway (there's nothing
// for it to write TO now that it's a one-way reflection)
out property <bool> has-focus: input.has-focus;
```
Hit this across 29 of `text-input/`'s 30 fixed files at once, since they
all used the identical `<=>` pattern for exactly this property. Scanned
the rest of the project for the same shape afterward — no other
instances found, so it was isolated to this one batch.

---

### `Flickable` (and other non-drawable elements) have no `background`/`border-*` properties

```slint
// WRONG — real compiler error: "Unknown property background"
Flickable {
    background: Theme.bg-base;
    viewport-height: input.preferred-height + 24px;
    input := TextInput { ... }
}
```
`Flickable` is a viewport/scroll container — it has scroll-position and
viewport-size properties, but no drawable surface of its own (same
category as `VerticalLayout`/`HorizontalLayout`/`GridLayout`/
`TouchArea`/`FocusScope`: layout and interaction primitives, not visual
elements). Only elements that actually paint something (`Rectangle`,
`Text`, `Image`, `Path`) have `background`/`border-*`.

```slint
// RIGHT — wrap it in a plain Rectangle for the background; Flickable
// fills its wrapper by default (containers fill their parent unless
// overridden), so no extra width/height binding is needed
Rectangle {
    background: Theme.bg-base;
    Flickable {
        viewport-height: input.preferred-height + 24px;
        input := TextInput { ... }
    }
}
```
Scanned the whole project for the same shape (any of `Flickable`,
`VerticalLayout`, `HorizontalLayout`, `GridLayout`, `TouchArea`,
`FocusScope` with `background`/`border-radius`/`border-width` set
directly on itself) — `CodeEditor.slint` was the only instance.

---

### An id declared inside an `if` block isn't accessible outside that same conditional scope

```slint
// WRONG — real compiler error: "Cannot access id 'input'" (twice: once
// at the has-focus binding, once at the TouchArea's clicked handler)
out property <bool> has-focus: input.has-focus;   // <- root-level

if !root.preview-mode: TouchArea {
    clicked => { input.focus(); }   // <- a DIFFERENT if-block, same condition
}

if !root.preview-mode: Flickable {
    input := TextInput { ... }      // <- input only exists inside HERE
}
```
Even though both `if` blocks share the identical condition
(`!root.preview-mode`), they're separately, independently instantiated —
Slint has no guarantee (or mechanism to express) "these two conditionals
are logically the same branch." An id given to an element inside an `if`
is scoped to that specific conditional subtree; nothing outside it —
not a sibling `if` with the same condition, not a root-level property
binding evaluated unconditionally — can reference that id, because the
element might not exist at runtime.

```slint
// RIGHT — make the element unconditionally instantiated (toggle
// visibility/enabled instead of using `if`), so its id is a normal,
// always-valid reference from anywhere in the component
Flickable {
    visible: !root.preview-mode;
    input := TextInput { ... }   // now exists always; id always valid
}

TouchArea {
    visible: !root.preview-mode;
    enabled: !root.preview-mode;
    clicked => { input.focus(); }   // works: input always exists
}
```
This is the same underlying constraint already documented above for
`@children` (can't live inside `if`) and for the width/height
self-reference cycle — conditionally-instantiated elements are more
restricted than they look. Scanned the whole project for the same shape
(an id declared inside one `if` block, referenced by that id from
anywhere outside that specific block) — `MarkdownEditor` was the only
instance.

---

---

### `Path` cannot loop *inside itself* to generate `MoveTo`/`LineTo` — but looping to produce whole `Path` elements is fine

```slint
// WRONG — confirmed open Slint limitation (slint-ui/slint#754, #776):
// `for` cannot iterate inside a Path to generate dynamic sub-elements.
Path {
    MoveTo { x: data[0].x; y: data[0].y; }
    for pt[i] in data : LineTo { x: pt.x; y: pt.y; }   // does not work
}
```
Hit this while trying to draw a real polyline for `LineChart` et al. from
a dynamic-length `[float]` array. Verified via search *before* writing
the fix, not after hitting a compiler error — worth doing that check
first for anything Path+`for`-shaped.

```slint
// RIGHT (for a genuinely dynamic-length series) — a fixed maximum of
// static, indexed MoveTo/LineTo, each bounds-checked against the real
// array length so extra points collapse onto the last real value:
property <int> len: root.data.length;
property <int> idx1: 1 < len ? 1 : (len > 0 ? len - 1 : 0);
property <float> v1: len > 0 ? root.data[idx1] : 0;
// ...repeat per point up to whatever max you're committing to (8 covered
// every default array in the charts/ category)
Path {
    MoveTo { x: x0; y: v0; }
    LineTo { x: x1; y: v1; }   // ...
}
```
```slint
// RIGHT (looping to produce several independent, fully-static Path
// elements is a completely different thing and works fine — used for
// RadarChart's three grid rings):
for ring-scale in [0.33, 0.66, 1.0] : Path {
    MoveTo { x: cx + radius * ring-scale * ax0; y: cy + radius * ring-scale * ay0; }
    LineTo { x: cx + radius * ring-scale * ax1; y: cy + radius * ring-scale * ay1; }
    Close { }
}
```
The distinction: looping *elements* (each a complete, self-contained
`Path`) is ordinary `for` usage; looping to build up the *sub-element
list inside one* `Path` is the unsupported case.

---

### `Path` needs `viewbox-width`/`viewbox-height` set explicitly for predictable scaling

Confirmed real properties (official Path docs), alongside `viewbox-x`/
`viewbox-y`. Set `width`/`height` to the pixel box you want the shape
drawn into, and `viewbox-width`/`viewbox-height` to the coordinate range
your `MoveTo`/`LineTo` commands are written in — the shape scales from
one to the other. Left unset, the viewbox is inferred from the path
commands' own bounding box, which is harder to reason about when the
coordinates are computed from data rather than hand-picked.

```slint
Path {
    width: parent.width; height: parent.height;   // real pixel box
    viewbox-width: 100; viewbox-height: 100;       // coordinate system MoveTo/LineTo use
    MoveTo { x: 0; y: 100; }                       // bottom-left, in viewbox units
}
```
A useful trick when you want a Path's coordinate system to match real
pixels 1:1 (so other, non-Path elements in the same container can share
the same numbers): set `viewbox-width: parent.width / 1px;` — dividing a
`length` by `1px` strips the unit back to a plain number, same
conversion direction as the already-documented `value * 1px`.

---

### `.with-alpha()` works on a `brush`-typed property, not just literal colors

Confirmed via the docs: `.with-alpha()`/`.with-alpha()`-adjacent methods
are defined for both colors and brushes. This matters when a component's
property is declared `in property <brush> foo` (broader than `color`, so
callers can also pass gradients) — `foo.with-alpha(0.2)` still compiles
and works, no need to narrow the property to `color` just to use it.

---

### Underscore vs. dash in a property reference is a silent-until-compile bug, not a typo the language forgives

```slint
// Declared:
in property <[float]> series-b: [...];
// WRONG elsewhere in the same file — compiles to "Unknown property", not
// a fuzzy match to series-b:
root.series_b.length
```
Slint identifiers are kebab-case; `series_b` and `series-b` are different
identifiers, full stop — there's no automatic underscore/dash
normalization. Found one instance of this in `charts/GroupedBarChart.slint`
(`root.series_b` where the declared property was `series-b`); grepped the
rest of the project for the same shape afterward — isolated to this one
file, not a systemic pattern this time.

---

---

### A property named `max`/`min` (or any other bare global function name) shadows that function

```slint
// WRONG — compiles, but breaks the very next use of the builtin:
// "The expression is not a function"
in property <int> max: 100;
property <float> fraction: max(0.0, min(1.0, value / max));
//                          ^^^ tries to call the *property* max, not the builtin
```
Confirmed by hitting it live: `charts/GaugeChart.slint` declared
`in property <int> max`, which shadowed the global `max()` function for
the rest of that component — the error pointed at the `max(...)` call
site, not the property declaration, so the actual cause (a same-named
property several lines above) wasn't obvious from the error location
alone. Slint doesn't warn at the declaration; it only ever surfaces where
the builtin is later called.

```slint
// RIGHT
in property <int> max-value: 100;
property <float> fraction: max(0.0, min(1.0, value / max-value));
```
Applies to any bare global function name, not just `max`/`min` — `abs`,
`round`, `floor`, `ceil`, `mod`, etc. are equally shadowable. Safest to
just never name a property identically to a builtin function.

---

---

### `alignment: start/center/end` on a layout silently disables `*-stretch` for its children

```slint
// WRONG — every bar collapses to 0px wide. `horizontal-stretch: 1` only
// takes effect when the layout's own alignment is the default `stretch`;
// setting alignment to anything else makes children fall back to their
// own preferred width instead — 0 for a bare Rectangle.
HorizontalLayout { alignment: end;
    for bar[i] in data : Rectangle { horizontal-stretch: 1; height: bar * 100px; }
}
```
Confirmed against Slint's own layout docs/examples (the `alignment: start`
example explicitly notes children "retain their specified minimum width"
instead of stretching). Hit this live across nine `charts/` files —
several (`BarChartVertical`, `Histogram`, `VolumeChart`,
`HundredPercentBar`, `StackedBarChart`, `SparklineBar`) rendered
completely blank; three more (`BoxPlot`, `CandlestickChart`,
`ViolinPlot`) rendered cramped into a narrow strip rather than spread
across the row, because their bars wrapped fixed-width inner content
that gave the outer `Rectangle` *some* non-zero preferred width instead
of exactly zero. No compiler error either way — this is a pure
layout-semantics footgun, not a syntax mistake, and it only shows up
visually.

```slint
// RIGHT — leave the layout at its default alignment (stretch) so
// horizontal-stretch actually divides the width; if per-item
// bottom/top/center-alignment along the *other* axis is also wanted,
// nest a layout for just that:
HorizontalLayout {
    for bar[i] in data : VerticalLayout { horizontal-stretch: 1; alignment: end;
        Rectangle { height: bar * 100px; }
    }
}
```
The general rule this falls out of: `alignment` and `*-stretch` are two
different mechanisms for the *same axis* (the layout's main axis) and
only one wins — `alignment` at any non-default value always wins over a
sibling's stretch factor, for every child in that layout, not just the
ones without their own explicit size.

---

---

### A `Text` centered via `x`/`y` inside a width-less `Rectangle` doesn't size the `Rectangle` — with three different failure modes depending on context

```slint
// WRONG — Rectangle has no explicit width, and its only child sets x/y,
// which Slint's own preferred-size rules exclude from sizing the parent.
Rectangle {
    height: 20px;
    background: Theme.accent-subtle;
    Text {
        x: (parent.width - self.width) / 2;   // centering — but this
        y: (parent.height - self.height) / 2; // excludes Text from the
        text: "New";                          // parent's preferred size
    }
}
```
Confirmed against Slint's layout docs: preferred size is computed from
"the child that has the bigger preferred size, **whose x and y property
are not set**." A `Text` positioned this way never counts, so the
`Rectangle` has *no* content to size itself from. What that actually
renders as depends entirely on where the `Rectangle` sits, which is what
makes this one sneaky — it doesn't fail the same way twice:
- Inside a `HorizontalLayout` with `horizontal-stretch: 0` → preferred
  width resolves to 0 → **completely invisible**.
- Direct child of a `VerticalLayout` → cross-axis default is to stretch
  to the full available width → renders as a **full-width banner**
  instead of a small pill.
- Outside any layout, positioned by explicit `x`/`y` on the `Rectangle`
  itself → an unset width on a `Rectangle` **fills its parent** (per the
  already-documented "containers fill their parent by default" rule) →
  can produce an oversized shape that only looks right at one specific
  parent size by coincidence, if something else happens to clip it back
  down.

```slint
// RIGHT — size the Rectangle from real layout content instead of
// manual x/y centering, so its preferred size is well-defined:
Rectangle {
    height: 20px;
    background: Theme.accent-subtle;
    HorizontalLayout {
        padding-left: 8px;
        padding-right: 8px;
        Text { text: "New"; vertical-alignment: center; }
    }
}
```
Found this shape four times in one category (`cards/`) alone — grep for
`x: (parent.width - self.width) / 2` (or the `y` equivalent) with no
sibling `width:`/`height:` binding on the parent as a standing check in
any new category, the same way the `alignment`-vs-`stretch` check from
`charts/` is. The two are easy to conflate (both are "a bar/badge didn't
show up right") but are different root causes and different fixes.

---

---

### Full-color pictographic emoji (🏆🗺📚📱🔗🎙…) can render as broken tofu glyphs — use a real icon asset instead

```slint
// RISKY — depends on the platform/environment having a color-emoji font
// installed. Confirmed broken in the Slint Preview environment used for
// this project: falls back to a ".notdef" tofu glyph, which shows up as
// a small garbled/rotated block of text rather than the intended icon.
Text { text: "🏆"; font-size: 28px; }
```
Hit this live across seven files in `cards/` — the person's screenshot
showed the exact tofu-glyph symptom for every full-color pictograph
(U+1F300+ range) in the category. Simple BMP symbols (★ ☆ ✓ ⚠ from the
same category) were confirmed still rendering fine in that same
screenshot, which led to a wrong conclusion the next category acted on
— see the correction below.

**Correction, from `navigation/` one category later:** the "simple BMP
symbols are safe" takeaway above was wrong, or at least incomplete. A
follow-up fix swapped broken emoji for simple geometric/dingbat BMP
symbols (◉▤◈✉♪⚙▮▲▭) reasoning they'd be safer — a later screenshot
showed every one of *those* rendering as the same broken tofu glyph too.
The `★ ☆ ✓ ⚠` that worked in `cards/` weren't safe because they're BMP;
they were safe because they happen to be in whatever narrow glyph set
this environment's font actually bundles (mostly basic
ASCII/punctuation-adjacent symbols) — and that's not a rule you can
extrapolate from to guess whether some *other* symbol will work. Only
treat a glyph as safe if a working screenshot has actually confirmed it
in this environment; the running confirmed-safe list is `✓ ✕ × ‹ ›` plus
plain alphanumerics. For anything icon-shaped, skip `Text`-glyph icons
entirely and use a real image from the start — don't try to find a
"safer" fallback character.

```slint
// RIGHT — this project's own established pattern (already used in
// FeatureCard before this was ever an issue): a real vector SVG, not
// font-dependent.
Image {
    source: @image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/trophy.svg");
    width: 26px; height: 26px;
    colorize: Theme.accent;   // repaints the monochrome SVG to match theme
}
```
For an emoji embedded *inside* a larger string (e.g. `"📍 " + location`),
there's no way to keep it inline — split into a small `HorizontalLayout`
with an `Image` + `Text` sibling instead; a `Text`'s string content can't
contain an icon.

For a component whose icon needs to vary per-instance (not just a fixed
default), expose `in property <image> icon-source;` (plus a
`has-icon: bool` if the icon is optional) and let the *caller* supply a
literal `@image-url(...)` — `@image-url()` needs a compile-time string
literal, so it can't resolve a runtime `string` property to a path
regardless of the font issue either. This is now this project's
established convention for any per-instance-selectable icon (`NavItem`,
`NavBadge`, `SidebarItem`, `Dock`, `Taskbar`, `FlyoutMenuItem`, plus the
pre-existing `overlays/FloatingActionPanel.slint`/
`feedback/AboutDialog.slint`).

---

### Slint's 8-digit hex color is `#RRGGBBAA` — alpha *last*, not alpha-first

```slint
// WRONG — reads as R=0x20, G=0xff, B=0xff, A=0xff: an OPAQUE bright
// cyan, not "white at ~12% opacity" the author clearly intended (this
// is the CSS/Android "#AARRGGBB" convention, which Slint does not use).
background: #20ffffff;
```
Confirmed by this project's own `Theme.slint` tokens, which are
consistently alpha-last (`accent-glow: #5B9DFA44;`,
`accent-subtle: #5B9DFA1A;`). Hit this live in two `cards/` files:
`GlassCard` (`#20ffffff`/`#40ffffff` → opaque cyan instead of a glass
effect — directly visible in a screenshot) and, worse, `ElevatedCard`
(`drop-shadow-color: #40000000` → R=0x40, G=B=0, **A=0x00, fully
transparent** — the component's entire drop shadow, its whole reason to
exist as "Elevated," was invisible with zero visual symptom pointing at
the color itself since a *missing* shadow doesn't look obviously wrong
the way a wrong-colored one does).

```slint
// RIGHT
background: #ffffff20;                    // white, ~12% opacity
drop-shadow-color: #00000040;             // black, ~25% opacity
```
Worth a standing check in any new category: grep for 8-digit hex colors
and sanity-check the alpha byte is last, especially for anything using
`0x00`/near-zero alpha-looking values in the *first* two digits — that
specific shape (`#XX000000` or `#XXffffff`) is the tell.

---

---

### A component with no `@children` silently drops the layout of anything nested inside it

```slint
// WRONG — no compile error, but nested items become direct, un-laid-out
// children of MyContainer's root instead of being arranged by the
// VerticalLayout — they'll overlap instead of stacking.
export component MyContainer inherits Rectangle {
    VerticalLayout {
        padding: 8px;
        // no @children here
    }
}
// MyContainer { ItemA {} ItemB {} }  <- both land on top of each other
```
Confirmed via Slint's own container-component docs: "by default child
elements... become [direct] children of [the component]" when there's
no `@children` to redirect them — not an error, just silently not what
a container-shaped component almost certainly intends. Found this exact
shape three separate times in one category (`navigation/`): `FlyoutMenu`,
`NavCollapse`, and `Drawer` — all three had a comment or a name implying
a content slot ("Content slot", an `if expanded:` body) but no actual
`@children`.

```slint
// RIGHT
export component MyContainer inherits Rectangle {
    VerticalLayout {
        padding: 8px;
        @children
    }
}
```
Worth a standing check for any component whose whole purpose is to wrap
other content (menus, drawers, collapsible sections, cards with a
"slot"): grep the file for `@children` and confirm it's actually there
before assuming a container component works.

---

### `TouchArea { clicked => { } }` and `clicked => { /* comment */ }` both mean "does nothing" — check for the comment-only variant too

```slint
// WRONG — compiles fine either way; both are silently inert
TouchArea { clicked => { } }
TouchArea { clicked => { /* expand all */ } }
```
The second form is sneakier: it reads like something was implemented,
but the "implementation" is just a comment describing the intent. Found
both shapes across `cards/` and `navigation/` — `grep -n "=> { }"` catches
the first; `grep -n "=> { /\*"` catches the second. Worth running both on
any category with interactive components, not just the first one.

---

### Don't lean on implicit int→string conversion inside a ternary or a `+` chain when the branches/operands have different types

```slint
// RISKY — one branch is string, the other is int. int does convert
// implicitly to string in general (confirmed: Slint's own docs), but
// that's a different guarantee from "a ternary/concatenation with
// mismatched branch types resolves this automatically" — untested here,
// no live compiler to verify against, and the safe alternative costs
// nothing.
text: count > 99 ? "99+" : count;
text: current + " / " + total;   // int + string + int
```
```slint
// RIGHT — make every branch explicitly a string via interpolation
text: count > 99 ? "99+" : "\{count}";
text: "\{current} / \{total}";
```
Found this shape five times in one category (`navigation/`: `NavBadge`,
`Steps`, `HorizontalStepper`, `VerticalStepper`, `PrevNextPagination`) —
worth grepping for `? "` immediately followed later by a bare
non-string branch, and for `+ "` chains mixing types, in any new
category.

---

---

### An id declared inside a conditional `if` element is out of scope outside that conditional

```slint
// WRONG — real compiler error: "Cannot access id 'ta'"
background: ta.pressed ? Theme.surface-pressed : transparent;   // ← declared earlier, unconditionally
...
if some-cond: ta := TouchArea { }   // `ta` only exists when some-cond is true
```
Hit this live in `navigation/FlyoutMenu.slint`, caught by the person's
VS Code Problems panel. An id's scope is tied to the conditional it's
declared inside — a sibling binding elsewhere in the component (even
one declared earlier, at the component's top level) can't see into that
conditional. This is a different limitation from the already-documented
`@children`-inside-`if` restriction, but has the same shape of fix.

```slint
// RIGHT — make the element unconditional, express "sometimes inert"
// with `enabled:` (or `opacity:`) instead of `if`:
background: ta.pressed ? Theme.surface-pressed : transparent;
...
ta := TouchArea {
    enabled: !some-cond;
    clicked => { ... }
}
```
Worth checking for specifically whenever a `TouchArea`/other id'd
element that's conditionally shown also needs its state
(`pressed`/`has-hover`/etc.) read by a sibling binding elsewhere in the
same component — that combination is exactly when this bites.

---

---

### A component with no explicit `height`/`vertical-stretch` on its root, whose real content lives only inside conditional children, gets zero size in a layout

```slint
// WRONG — no explicit height on the root, and the only content is
// inside `if` blocks. Compiles fine, but inside a VerticalLayout this
// component is allocated ZERO height — its content still renders at
// its own conditional size, just overflowing out of that zero-height
// box and overlapping whatever's next to it.
export component MyHeader inherits Rectangle {
    in property <bool> expanded: true;
    if expanded: Rectangle { height: 100px; /* ... */ }
    if !expanded: Rectangle { height: 48px; /* ... */ }
}
```
Confirmed live via a screenshot: `navigation/CollapsingHeader.slint` had
exactly this shape and visibly overlapped the sibling above it
(`StickyHeader`) in a `VerticalLayout`. A sibling component in the same
file, `StickyHeader`, has an unconditional `height: 48px;` on its own
root and stacked correctly — the difference is exactly that explicit
root sizing.

```slint
// RIGHT — bind the root's own height explicitly to whichever variant
// is showing:
export component MyHeader inherits Rectangle {
    in property <bool> expanded: true;
    height: expanded ? 100px : 48px;
    if expanded: Rectangle { /* ... */ }
    if !expanded: Rectangle { /* ... */ }
}
```
Only relevant when a component's content is *entirely* conditional —
a component with substantial unconditional layout content alongside
some conditional decoration (checked a couple of steppers for this
exact shape and they were fine) sizes correctly from that unconditional
content regardless.

---

### Multiple direct children of a plain `Rectangle`, none inside a layout, all overlap instead of stacking

```slint
// WRONG — header, divider, and content are three separate children of
// `panel` with no layout wrapping them. Per the "containers fill their
// parent by default" rule, all three default to filling the ENTIRE
// panel and completely overlap.
panel := Rectangle {
    Rectangle { height: 56px; /* header content */ }
    Rectangle { height: 1px; /* divider */ }
    VerticalLayout { @children /* content */ }
}
```
Confirmed live via a screenshot: `navigation/Drawer.slint` had exactly
this shape — the header's "Menu" text rendered jumbled together with
the drawer's nav items instead of appearing above them, because nothing
told the header/divider/content to stack top-to-bottom rather than each
independently fill the whole panel.

```slint
// RIGHT — wrap the siblings in a layout so they actually stack:
panel := Rectangle {
    VerticalLayout {
        Rectangle { height: 56px; /* header content */ }
        Rectangle { height: 1px; /* divider */ }
        VerticalLayout { @children /* content */ }
    }
}
```
Easy to miss because each individual child *looks* correctly sized on
its own (explicit `height:` bindings and all) — the bug is purely about
the missing outer layout, not any one child's own properties.

---

### Unconfirmed — don't guess, verify first if you need these

- Whether a `FocusScope` wrapping a `TextInput` or a `PopupWindow`
  receives key events while that child holds real focus. Hit this twice
  (`NumberInput` arrow-key stepping, `Select` in-popup navigation) and
  both times chose to leave a documented gap rather than ship a guess.
- `if (cond) { A } else { B }` as a property-binding *expression*
  (versus imperative statements inside a callback, which is confirmed).
  Always used the ternary form instead — never needed to resolve this.
  **Partial update:** confirmed a *related* but different thing this
  session — a property binding can be a whole statement *block*
  (`x: { ... }`, not just a single expression; Slint's own docs say
  bindings are reactive "whether... an expression or a block"), and
  `navigation/Drawer.slint` uses exactly that (`x: { if !open {...}
  else {...} }`) successfully. That's a block containing an if/else
  *statement*, which is the already-confirmed callback-statement form,
  just used as a binding body instead of inside a `clicked =>`. Still
  unconfirmed: whether `if (cond) { A } else { B }` works as a bare
  sub-expression *inside* a larger expression (e.g. `x: 5 + (if cond
  {1} else {2})`) — didn't need that shape, so still hasn't been
  tested.
