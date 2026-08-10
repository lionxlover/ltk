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

### Unconfirmed — don't guess, verify first if you need these

- Whether a `FocusScope` wrapping a `TextInput` or a `PopupWindow`
  receives key events while that child holds real focus. Hit this twice
  (`NumberInput` arrow-key stepping, `Select` in-popup navigation) and
  both times chose to leave a documented gap rather than ship a guess.
- `if (cond) { A } else { B }` as a property-binding *expression*
  (versus imperative statements inside a callback, which is confirmed).
  Always used the ternary form instead — never needed to resolve this.
