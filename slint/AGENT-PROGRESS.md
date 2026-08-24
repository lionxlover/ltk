# LTK Slint Library — Agent Progress Log

Working log of the ongoing pass through this component library: fixing real
bugs, filling gaps, and bringing every file up to a consistent standard.
Read this before continuing the work in a new conversation — it saves
re-discovering things the hard way.

## Status by category

| Category | Files | Status | Delivered as |
|---|---|---|---|
| `buttons/` | 42 | ✅ Done, verified | `buttons.zip` |
| `inputs/` | 11 | ✅ Done, verified | `inputs.zip` |
| `forms/` | 4 | ✅ Done, verified | `forms.zip` |
| `layout-containers/` | 52 | ✅ Done, verified via VS Code Problems panel (3 rounds of fixes) | `layout-containers.zip` |
| `data-display/` | 60 | ✅ Done, verified via VS Code Problems panel (10 rounds of fixes) | `data-display.zip` |
| `text-input/` | 45 | ✅ Done, pending VS Code Problems-panel check | `text-input.zip` |
| `charts/` | 43 | ✅ Done, verified via VS Code Problems panel (2 rounds of post-delivery fixes) | `charts.zip` |
| `cards/` | 39 | ✅ Done, verified via VS Code Problems panel + live screenshots (1 round of post-delivery fixes) | `cards.zip` |
| `navigation/` | 39 | ✅ Done, verified via VS Code Problems panel + live screenshots (2 rounds of post-delivery fixes) | `navigation.zip` |
| `feedback/` | 36 | ✅ Done, verified via VS Code Problems panel (1 round of post-delivery fixes) | `feedback.zip` |
| `typography/` | 36 | ✅ Done, pending VS Code Problems-panel check | `typography.zip` |
| `media/` | 32 | ✅ Done, verified via live screenshots (1 round of post-delivery fixes, also surfaced a cross-cutting bug fixed in 7 other categories) | `media.zip` |
| `mobile/` | 30 | ✅ Done, verified via live screenshots (1 round of post-delivery fixes) | `mobile.zip` |
| `forms2/` | 31 | ✅ Done, verified in a follow-up pass | `forms2.zip` |
| `social2/` | 26 | ✅ Done, pending VS Code Problems-panel check | `social2.zip` |
| everything else | ~259 | ⬜ Not started | — |

Untouched categories, roughly by size:
`animation` (24), `overlays` (24), `selection-controls` (25), `utility` (23),
`indicators` (20), `desktop2` (17), `theming2` (12), `accessibility2`
(12), `desktop` (10), `range-value` (10), `social` (8), `accessibility`
(8), `theming` (6), `layout` (3), `progress` (1 component total —
`ProgressSpinner` — already reviewed and fixed; this category is
complete, not partial, despite how this note used to read).

Note: `desktop2/SplashScreen.slint`, `indicators/HealthBar.slint`,
`range-value/Slider.slint`, `range-value/SteppedSlider.slint`,
`range-value/OpacitySlider.slint`, `social2/InChatPollWidget.slint`, and
`utility/SuspenseBoundary.slint` each already had ONE specific bug
(the fractional-width-no-x progress-fill issue) pre-fixed during the
`media/` cross-cutting hotfix round — see that section below. Each
category still needs its own full pass for everything else.

`core/Theme.slint`, `core/Backend.slint`, `core/FaIcon.slint`, and
`progress/ProgressSpinner.slint` were all modified as shared dependencies
and are bundled into whichever zip needs them. `layout-containers/` needed
no core changes.

## Hard-won Slint rules (apply these from the start — don't rediscover them)

1. **`width`/`height` are aliases for `min-width`+`max-width` /
   `min-height`+`max-height` internally.** Never set both an explicit
   `width`/`height` AND a separate `min-width`/`max-width`/`min-height`/
   `max-height` — even if the values match, even if they don't. It's a
   real "Cannot specify both" compile error, not redundancy. If you need
   a fixed size, `width: 40px;` alone already locks it. If you need a
   range, only bind `min-`/`max-`, never a plain `width`/`height` too.
2. **`radius-full` (9999px) only on genuinely square/circular elements**
   (width == height). On an elongated shape it renders as a full ellipse,
   not a stadium/pill. Use `self.height / 2` for pill shapes instead —
   exact by construction, no clamping-behavior guesswork.
3. **Rotation**: `transform-rotation`/`transform-scale`/`transform-origin`
   are the general-purpose properties on any visual item (default origin
   = center, don't need to set it separately). `rotation-angle`/
   `rotation-origin-x/y` are real but **`Image`-only** — using them on a
   `Rectangle` is an "Unknown property" error.
4. **`PopupWindow`** is the correct tool for dropdowns/popovers that must
   not be clipped by an ancestor or drawn under later siblings (a plain
   nested `Rectangle` will have both problems). Use `.show()`/`.close()`.
   `close-on-click` is **deprecated** — use `close-policy:
   PopupClosePolicy.no-auto-close` (full manual control) or
   `.close-on-click-outside` (auto-dismiss, but nothing notifies you if
   it self-closes, so don't pair it with your own `expanded`-style flag
   unless you've confirmed a resync path).
5. **`@children` cannot live inside a conditional (`if`) element** —
   confirmed via a real compiler error and matching the open Slint issue
   #6354 ("Support for conditional children wrappers"). Keep the wrapper
   unconditionally instantiated; animate its `height` to 0 instead of
   using `if` to show/hide it.
6. **Arrays**: `.length` and `array[i]` (read) are real and confirmed.
   **Index *assignment* (`array[i] = x`) is not confirmed anywhere in
   Slint's docs** and the language reference explicitly frames
   `array[index]` as retrieval only. Don't build core interactions on it
   — report the change via callback and let the host reassign the whole
   array (this is also just the correct architecture per this project's
   own `interop.md`: host owns state).
7. **Strings have no length/indexing/slicing** in `.slint` — confirmed
   against the full primitive-types member list (`to-float`, `is-float`,
   `is-empty`, `to-lowercase`, `to-uppercase`, nothing else). For
   per-character logic (PIN entry, etc.), use statically-named separate
   properties instead of trying to index into one string.
8. **`TouchArea.pressed-x`/`pressed-y`** are real but are a **frozen
   snapshot of the initial press position**, not live-updating. For
   continuous drag tracking (sliders, etc.) use `mouse-x`/`mouse-y`
   instead, which do update continuously.
9. Every interactive `TouchArea`/`FocusScope` needs an explicit
   `enabled: root.enabled;` binding. Dimming via `opacity` alone does
   **not** disable hit-testing — found this exact bug repeated across
   10+ files (checkboxes, switches, radios, sliders, buttons all stayed
   clickable while visually "disabled").
10. Two-way text bindings should use `<=>` (`text <=> input.text;`), not
    a one-way `text: root.text;` — the latter risks fighting the user's
    own typing since it keeps re-deriving from a value nothing updates
    as they type.
11. When genuinely unsure whether a nested `FocusScope` receives key
    events while a child `TextInput`/`PopupWindow` holds real focus:
    don't guess. Document the gap instead of shipping unverified
    event-routing behavior. (Happened twice — `NumberInput`'s arrow-key
    stepping, `Select`'s in-popup keyboard nav — both deliberately
    left out for this reason.)

## Verification workflow used for every file

No `slint-viewer`/`cargo` available in this sandbox (network egress
blocked) — verification is: (a) brace balance, (b) every `Theme.*`
reference checked against the real token list in `core/Theme.slint`,
(c) the width/height alias-conflict scanner (see below), (d) grep for
`radius-full` + check the shape is actually square, (e) grep for
array-index-write patterns, (f) check what else in the repo actually
imports the file before assuming an API change is safe. The *real*
compiler is the person's own VS Code / vscode.dev Slint Preview —
several bugs in this list were only caught that way, not by any of the
above. Ask for Problems-panel screenshots after each batch.

```python
# Width/height alias-conflict scanner — catches width+min-width (etc.)
# co-occurring within ~15 lines, regardless of whether the values match.
import re, glob
def check_file(path):
    lines = open(path, encoding='utf-8').read().split('\n')
    width_at, height_at = None, None
    findings = []
    for i, line in enumerate(lines):
        if re.match(r'^\s*width:\s*.+;\s*$', line): width_at = i
        if re.match(r'^\s*height:\s*.+;\s*$', line): height_at = i
        if re.match(r'^\s*min-width:\s*.+;\s*$', line) and width_at is not None and i-width_at<=15: findings.append((i+1,'min-width'))
        if re.match(r'^\s*max-width:\s*.+;\s*$', line) and width_at is not None and i-width_at<=15: findings.append((i+1,'max-width'))
        if re.match(r'^\s*min-height:\s*.+;\s*$', line) and height_at is not None and i-height_at<=15: findings.append((i+1,'min-height'))
        if re.match(r'^\s*max-height:\s*.+;\s*$', line) and height_at is not None and i-height_at<=15: findings.append((i+1,'max-height'))
    return findings
```

```python
# @children-inside-conditional scanner — walks brace depth and flags any
# @children line whose enclosing block was opened by an `if`/`for`. Added
# after shipping this exact bug twice in the layout-containers batch
# despite already knowing the rule from an earlier file in the same batch.
import re, glob
def check_children_conditional(path):
    lines = open(path, encoding='utf-8').read().split('\n')
    depth, cond_depths, findings = 0, set(), []
    for i, l in enumerate(lines):
        stripped = l.strip()
        is_if_open = bool(re.match(r'^(if|for)\b.*:\s*\S*\s*{', stripped))
        opens, closes = l.count('{'), l.count('}')
        if '@children' in l and cond_depths:
            findings.append((i + 1, sorted(cond_depths)))
        for _ in range(opens):
            depth += 1
            if is_if_open: cond_depths.add(depth); is_if_open = False
        for _ in range(closes):
            cond_depths.discard(depth); depth -= 1
    return findings
```

```python
# self-referential width/height scanner — flags a `width:`/`height:`
# binding whose own expression reads `self.<same-prop>` or
# `root.<same-prop>`. Added after AspectRatioBox shipped this exact bug
# twice: first as a width<->height cross-reference cycle, then again (after
# the first "fix") as height falling back to reading itself. A binding
# that reads root.<prop> on a genuinely *different* element (e.g. a nested
# PopupWindow's width reading its parent component's root.width) is fine —
# this only flags the same element reading its own property back.
import re, glob
def check_self_referential_dimension(path):
    findings = []
    for i, l in enumerate(open(path, encoding='utf-8').read().split('\n')):
        m = re.match(r'^\s*(width|height)\s*:\s*(.+);\s*$', l)
        if m and re.search(rf'\b(self|root)\.{m.group(1)}\b', m.group(2)):
            findings.append((i + 1, l.strip()))
    return findings
```

```python
# root-level `parent` scanner — flags any `parent.` reference written
# directly in the root component's own bindings (brace depth 1, i.e. not
# inside a nested child element). Added after AspectRatioBox failed with
# "Cannot access id 'parent'" in Slint Preview: an exported component is a
# candidate top-level element (Preview loads "the last exported" component
# as the window root), and `parent` only exists once something actually
# encloses it. `parent.foo` inside a *nested child* is fine — there it
# resolves to the root component itself, which always exists.
import glob
def check_root_level_parent(path):
    depth, findings = 0, []
    for i, l in enumerate(open(path, encoding='utf-8').read().split('\n')):
        stripped = l.strip()
        if not stripped.startswith('//') and 'parent.' in l and depth == 1:
            findings.append((i + 1, stripped))
        depth += l.count('{') - l.count('}')
    return findings
```

## Notable bugs fixed (the ones worth remembering the *shape* of)

- **Structural/non-functional**: `PillButton`, `LoadingButton`,
  `BookmarkButton`, `LikeHeartButton`, `MuteUnmuteButton`,
  `PlayPauseButton`, `RecordButton`, `BackToTopButton`,
  `ScrollAnchorButton`, `TextLinkButton`, `PinInput`, `CheckboxGroup` —
  all had a click handler that changed internal visual state but never
  told the host anything happened (no callback, or an unused one).
- **Fake gestures**: `LongPressButton` ("Hold to confirm") and
  `SwipeToConfirm` were plain tap-toggles with zero hold/drag detection.
  Rebuilt on real `Timer` (hold) and `SwipeGestureHandler` (swipe).
- **`CloseDismissButton`**: hover/pressed/focus state was hardcoded
  `true`, never wired to the real `TouchArea`/`FocusScope` — permanently
  rendered as focused.
- **`SegmentedControl`**: sliding indicator's geometry was bound to a
  completely empty `TouchArea`, which defaults to filling its parent —
  the indicator never moved, it always covered the whole control. Fixed
  by computing position arithmetically from `current-index` and
  `segments.length` instead of trying to address a `for`-loop instance.
- **`FishSlider`**: used the frozen `pressed-x`/`pressed-y` for drag
  tracking (see rule 8 above) — handle would jump once and stop
  following the pointer.
- **`AccordionItem`**: content area was an empty named `VerticalLayout`
  with no `@children` — no caller could ever put anything inside it.
- **`ProgressSpinner`**: rendered a static, non-animating ring despite
  being the `loading` state for ~15 button files. Fixed with a real
  `transform-rotation` loop (`iteration-count: -1`, triggered via
  `init =>`).
- **`SocialAuthTwitter`**: said "Continue with X", rendered the old
  Twitter bird icon — the correct `x-twitter.svg` was sitting unused in
  the same asset folder.
- **`PillButton`/`ReactionButton`**: rendered as full ellipses instead
  of pills (see rule 2).

## `layout-containers/` batch — findings

The single biggest, most repeated bug in this category, worse than in any
prior batch: **almost every "container" component was missing
`@children` entirely.** As layout/wrapper components their whole purpose
is to hold content, but ~40 of the 47 non-trivial files had a `Rectangle`/
`VerticalLayout`/`HorizontalLayout`/`Flickable` with nothing placed inside
it — anything a host tried to nest inside these components would silently
vanish. Fixed across: `AccordionGroup`/`AccordionItem`, `AppShell`,
`AspectRatioBox`, `BentoGrid`, `CardGridContainer`, `ColumnGrid`/
`GridColumn`, `ContentRegion`, `CssGridContainer` (was previously *just a
bag of unused properties with no layout element at all*), `DisclosureWidget`,
`DualPanel`, `FeatureGrid`, `FixedContainer` (same "no layout at all" bug
as CssGridContainer), `FlexContainer`, `FocusTrapRegion`, `FullBleedBox`,
`FullWidthSection` (same bug again), `HolyGrailLayout`, `HorizontalStack`,
`InfiniteScrollWrapper`, `InlineCluster`, `MasonryGrid`, `MasterDetailSplit`,
`MaxWidthContainer`, `OverflowScrollX/Y`, `PortalTeleport`,
`QuadPanelLayout`, `ResizablePanelGroup`, `SafeAreaWrapper`,
`ScrollableRegion` (`Flickable` had no content element at all — nothing to
scroll), `SidebarContentPane`, `SingleColumn`, `SnapScrollContainer`,
`StickyWrapper`, `TabPanelContainer`, `ThreeColumn`, `TileGrid`,
`TwoColumn`, `VerticalStack`, `VirtualizedScroll`, `WaterfallLayout`,
`WizardWrapper`, `WrapCluster`, `ZStack` (had a comment saying children
stack on top of each other but no actual `@children`).

**Multi-region layouts and the single-`@children` ceiling.** Components
like `AppShell`, `HolyGrailLayout`, `TwoColumn`/`ThreeColumn`, `DualPanel`,
`MasterDetailSplit`, `QuadPanelLayout`, `SidebarContentPane`,
`ResizablePanelGroup`, `TabPanelContainer`'s per-tab pane, and
`WizardWrapper`'s per-step pane all *look* like they should offer several
independent content slots (sidebar + main, four quadrants, one pane per
tab...). Slint only allows **one** `@children` placeholder per component,
and it can't be routed conditionally to different panes — there is no
named-slot mechanism. Rather than fake it, each of these now exposes
exactly one real content slot (the primary/largest pane) and says so in a
comment; the other panes are chrome-only. If a host genuinely needs
independent content in every pane, it has to compose the layout directly
instead of wrapping it in one of these.

**`@children` inside a conditional, twice more.** Caught and fixed in
`DisclosureWidget` (open/closed content) and initially *introduced by me*
in `PortalTeleport` and the first drafts of `FlexContainer`/
`SnapScrollContainer` before a repo-wide grep caught it — see the mistakes
section below.

**New binding-loop shape**: `AspectRatioBox` had `height` derived from
`root.width` when `use-width` was true, but silently returned `0px` for
`use-width: false` instead of deriving `width` from `height` — that's the
kind of gap this project's rule 11 says to document rather than guess at.
My first fix attempt tried to derive `width` from `height` *and* `height`
from `width` in the two ternary branches, which is a **new binding-loop
shape** not in the existing rules list: even though only one branch is
ever taken at runtime, Slint's dependency graph is built from the textual
bindings, so a property that references its sibling property (and vice
versa) in different branches of the *same two* properties is a static
cycle regardless of which branch actually fires. Fixed by only ever
binding `height`, never `width`, and documenting `use-width: false` as an
unimplemented direction (host must compute and set `width` explicitly).
Added this as a candidate rule 12 below.

**No true masonry/waterfall/bento packing is possible.** `MasonryGrid`,
`WaterfallLayout`, `BentoGrid`, `CardGridContainer`, `FeatureGrid`,
`TileGrid`, and `CssGridContainer` all had non-functional skeletons (empty
`HorizontalLayout`s meant to be "columns", or in `CssGridContainer`'s case
no layout element at all). Real masonry/waterfall packing needs each
item's rendered height up front to balance columns, and Slint has no way
to introspect the size of opaque `@children`. Converted all of these to a
real `GridLayout` + `@children` (children set their own `row`/`col`/
`rowspan`/`colspan`, which is the actual Slint mechanism for CSS-grid-like
placement) — a documented, honest reduction to uniform grid packing rather
than a fake uneven-height masonry effect.

**No CSS-style flex-wrap.** `FlexContainer`'s `wrap` property and
`WrapCluster`'s name both implied wrapping, but Slint's box layouts don't
wrap at all — items that overflow just get clipped/oversized by the
layout's own sizing. `WrapCluster` is now an honestly-named horizontally
scrolling single-row `Flickable` instead. `FlexContainer` had a worse
problem too (next item).

**No dynamic axis-switching for opaque children.** `FlexContainer` (a
runtime `horizontal` bool meant to switch between a `HorizontalLayout` and
`VerticalLayout`) and `SnapScrollContainer` (`horizontal` meant to flip
scroll axis) both hit the same wall as the masonry components: since
`@children` must be unconditional and can only be written once per
component, the *same* children can't be routed to two different layout
element types based on a runtime flag. Both now commit to one fixed axis
(the documented default) and point to the sibling component
(`VerticalStack`) for the other axis, rather than silently dropping
content on one branch — see the mistake below for how close this came to
shipping broken.

**`VirtualizedScroll` doesn't actually virtualize.** It reserves the
correct scrollable range (`total-items * item-height`) but, like the
masonry family, has no way to only render the children currently in
frame — that requires a real data model + item template
(`ListView` from `std-widgets.slint`, per this project's own
`debugging-and-mcp.md`), not raw `@children`. Documented rather than
faked.

**Real bugs (not just missing `@children`) fixed along the way:**
- `DragDropZone`: labeled "or click to browse" but had no `TouchArea` at
  all — impossible to click. Added one plus a `browse-clicked` callback
  (real OS file-drag detection is host-side; Slint has no native file-drag
  API surfaced in `.slint`).
- `ResizablePanelGroup`: was a static split with no actual resize
  handle — the divider didn't respond to drag at all. Rebuilt on
  `mouse-x`/`mouse-y` (not the frozen `pressed-x`/`pressed-y`, rule 8) with
  a real `split-changed` callback.
- `WizardWrapper`: Back/Next buttons had no `enabled:` binding on their
  `TouchArea` — pattern from rule 9, just for buttons that should be
  disabled at the first/last step rather than a persistent `enabled` prop.
  Added `enabled: current-step > 0` / `< total-steps - 1` and
  `step-changed`/`finished` callbacks (the in-out `current-step` alone
  isn't enough signal for "wizard completed").
- `InfiniteScrollWrapper`, `TabPanelContainer`: added `load-more`/
  `tab-changed` callbacks — same "changes internal state, tells nobody"
  shape as the buttons-batch bugs.
- `CssGridContainer` and `FixedContainer` (and separately
  `FullWidthSection`): had `int`-typed properties for what are really
  pixel/length values (`column-gap`, `row-gap`, `padding-x`, `padding-y`)
  that were declared but never consumed anywhere in the file. Retyped to
  `length` and actually wired them up.

## Candidate new rule (12): reciprocal width/height ternary bindings are a binding loop

If a component conditionally derives `height` from `width` in one ternary
branch and `width` from `height` in another (even for mutually-exclusive
conditions, even referencing `self`/`root` rather than the *other*
property directly), that's still a static cycle in Slint's dependency
graph — the graph is built from the textual bindings, not per-branch at
runtime. Only ever bind **one** of the pair; if the component needs to
support deriving in both directions, that's actually two different
components (or one property left for the host to set explicitly), not one
component with reciprocal formulas.

**Correction (caught by the person's VS Code Problems panel after
delivery):** the first fix for this in `AspectRatioBox` still failed —
`height: use-width ? (...) : self.height;` — because the *unimplemented*
ternary branch fell back to `self.height`, which is `height` reading
**itself**. That's the same binding-loop error in a narrower shape:
not cross-referencing a sibling property, but a property directly
depending on its own value. The "only ever bind one of the pair" rule
above isn't sufficient on its own — the bound property also can't
fall back to reading itself in any branch. Fixed by falling back to
`parent.height` (a different element's property) instead. Repo-wide
grep for `width:`/`height:` bindings that read `self.width`/`self.height`
or `root.width`/`root.height` on the *same* property found this was the
only real instance (one other hit, `Select.slint`'s popup
`width: root.width`, is a false positive — `root` there is the outer
`Select` component, a different element from the `PopupWindow` whose
`width` is being set, so it's a normal cross-element binding, not a
self-reference).

**Second correction (also from the Slint Preview, same file):** the
`parent.height` fallback above still failed at runtime — `Cannot access
id 'parent'` — because `AspectRatioBox` is the file's only export, so
Slint Preview loads it as the *top-level* window content, where it has no
`parent` at all. Any exported, reusable component is a candidate
top-level element, not just something that will always be nested inside a
host layout. Fixed by replacing the `parent.height` fallback with an
explicit `in property <length> fallback-height: 200px;` — a property with
no dependency on anything existing outside the component, and (being an
unrelated, distinct property) immune to the earlier self-reference bug
too. Swept the whole batch for the same shape (a `parent.*` reference
written directly in the *root* component's own bindings, as opposed to
inside a nested child element, where `parent` safely resolves to the root
component instead) — no other instances found.

## `data-display/` batch — findings

The dominant systemic bug here was different from (but parallel to)
`layout-containers/`'s missing-`@children` epidemic: **10 empty
`TouchArea {}` elements across 8 files** (`EditableInlineTable`,
`FrozenColumnTable`, `LeaderboardTable`, `SortableDataTable`, `TreeTable`,
`VirtualizedList`, `VirtualizedTable`, plus `MonthCalendar`'s nav arrows).
Every one of them had hover/press background styling wired up
(`row-ta.pressed ? ... : row-ta.has-hover ? ... : transparent`), which
*implies* the row is clickable, but no `clicked =>` handler and — in most
cases — no callback even declared on the component. Visually these looked
completely functional (rows highlight on hover, darken on press) while
doing nothing at all on click. Fixed by adding a `row-clicked`/
`item-clicked` callback to each and wiring it, using absolute indices
(`visible-start + idx`) for the two virtualized components since their
loop index is relative to the visible window, not the full dataset.

**Bigger structural bugs, one per file, each worth remembering the shape
of:**

- **`EditableInlineTable`** — named for inline editing, had *zero* editing
  capability: empty cell `TouchArea`s, no `TextInput`, no property to read
  edited values back from, no callback. Rebuilt with a real `TextInput`
  per cell and a `cell-edited(row, col, value)` callback, using the same
  flat row-major indexing (`row * columns.length + col`) already
  established by `FeatureComparisonMatrix`/`PriceComparisonTable` in this
  same batch for their `matrix` property — reused an existing pattern
  instead of inventing a new one.
- **`GroupedList`** — a whole section's items were never actually looped.
  The section body was a single `Rectangle` sized to fit N items
  (`height: items-per-section[si] * 40px`) but contained exactly one
  non-repeating "Item {si+1}" row (using the *section* index, not an item
  index) and one `TouchArea` covering the whole area reporting item index
  `0` regardless of where it was clicked. Fixed with a real nested `for
  item_idx in section-count` loop, correct per-item click reporting via
  `item-clicked(si, item_idx)`, and a bounds check on `items-per-section`
  against `sections` (they could legitimately be different lengths if the
  host hasn't populated both yet).
- **`SortableList`** — the row body's `TouchArea` called
  `reorder-request(idx, idx)`, a no-op self-reorder — almost certainly a
  copy-paste of the drag handle's `reorder-request(idx, idx + 1)` with the
  `+ 1` dropped. Replaced with a distinct `item-clicked` callback so the
  row body does something coherent (select) instead of a meaningless
  reorder-to-self.
- **`Timeline`** — same disease as `layout-containers/`'s worst offenders:
  a container component with a `vertical` orientation property and *no
  layout element or `@children` at all*. Nothing placed inside it could
  ever render. Strong independent evidence this was broken: this
  category's own `test.slint` didn't use `Timeline` to demonstrate
  `TimelineItem` — it manually reimplemented Timeline's own job
  (`VerticalLayout { padding: ...; TimelineItem { ... } }`) inline as a
  workaround. Fixed using the same "commit to one axis, document the
  single-`@children` limitation, point to the sibling `HorizontalTimeline`
  for the other axis" pattern established for `FlexContainer`/
  `SnapScrollContainer` last batch, and updated `test.slint` to actually
  exercise the real component instead of working around it.
- **`MonthCalendar`** — three separate gaps in one file: the ◀/▶
  month-navigation arrows were empty `TouchArea`s (added `previous-month`/
  `next-month` callbacks — actually recomputing the visible month is the
  host's job, this component only tracks a static day grid); day cells had
  *no* `TouchArea` at all despite `selected-day` being `in-out` (implying
  the user should be able to click a day) — added click-to-select; and a
  `day-val > 0 ? day-val : ""` ternary mixed an `int` arm with a `string`
  arm (see the note on this below).
- **`WeekCalendar`** — day-column headers were pure display despite
  `selected-day` being `in-out` — added click-to-select with hover
  feedback, mirroring the fix pattern used for `MonthCalendar`.
- **`NetworkGraph` / `OrgChart` / `TerminalOutput` / `JsonTreeViewer`** —
  all four declared configuration properties (`node-count`, `item-count`,
  `demo-titles`, `demo-labels`, …) that the component body never
  consulted at all — a fixed number of hardcoded nodes/rows regardless of
  what was passed in. Genuine N-node graph/org-chart auto-layout is real
  algorithmic work outside the scope of a bug-fix pass, so `NetworkGraph`
  and `OrgChart` were wired to use their declared label/title/color arrays
  *within* their existing fixed topology (documented as a fixed topology,
  not a general graph renderer) rather than attempting arbitrary-N
  layout. `TerminalOutput` and `JsonTreeViewer` had no fixed-topology
  excuse (they're just row lists), so both were converted to genuinely
  loop over `demo-*` arrays, matching the pattern already established by
  `ActivityFeed`/`GanttTimeline`/`LogViewer` elsewhere in this same
  category. Also deleted a dead 0-height "connector" `Rectangle` in
  `OrgChart` left over from a decoration that was never actually visible.
- **Unbounded secondary-array indexing**: `LeadingIconList`,
  `TrailingActionList`, `TwoLineList`, `AvatarList`, `DefinitionList` all
  drive their `for` loop off one array (`titles`, `keys`, …) but then index
  a *second*, independently-sized array (`icons`, `values`, …) with the
  same loop index and no bounds check — safe only if the caller happens to
  keep both arrays the same length, which nothing enforces. Added the same
  `idx < root.other-array.length ? root.other-array[idx] : ""` guard
  already used correctly elsewhere in this category (e.g. `ActivityFeed`,
  `ChatMessageThread`). Cross-checked `GanttTimeline` and
  `PriceComparisonTable`, which looked similar in an automated scan but
  turned out to already have the guard in a form the regex didn't
  recognize (a wrapping `if i < ...length:` instead of an inline ternary).

## Note on `int`/`string` ternary mismatches (unconfirmed, fixed defensively)

Two spots (`Badge`, `MonthCalendar`) had a ternary where one arm was a bare
`int` (`root.count`, `day-val`) and the other was a `string` literal
(`"99+"`, `""`), assigned directly to a `text:` property. This project's
own `buttons/ReactionButton.slint` already established as *confirmed*
that a bare `int` assigned **directly** to `text:` is valid Slint (int
converts implicitly to string). But a ternary is a different type-checking
context — both arms need to unify to one type before the assignment ever
happens, and that's not the same guarantee as the direct-assignment case.
Rather than assume implicit conversion also reaches inside a ternary arm
(unconfirmed either way, no compiler access to check), both were wrapped
in string interpolation (`"\{day-val}"`), which is unambiguously valid
regardless of the answer. Two further instances of the identical shape
were spotted in already-shipped, out-of-scope categories —
`navigation/NavBadge.slint:54` and `indicators/NumericBadge.slint:16` —
noted here rather than fixed now, since those categories haven't come up
in the rotation yet; worth a quick check when `navigation`/`indicators`
are reached.

## Post-delivery fix: `MarkdownEditor` — an id inside one `if` block isn't visible outside it

Two more real compile errors from the person's Slint Preview: "Cannot
access id 'input'" at both `has-focus: input.has-focus;` (a root-level
property) and a sibling `if !root.preview-mode: TouchArea`'s `clicked`
handler. `input` was declared inside a *different*
`if !root.preview-mode: Flickable { input := TextInput {...} }` block —
even though both conditionals share the identical condition, Slint
doesn't treat them as the same branch; an id given inside one `if`
simply isn't a valid reference from anywhere outside that specific
conditional subtree, including another `if` with the same-looking
condition. Fixed by making the `Flickable`+`TextInput` (and, for safety,
the `TouchArea` too) unconditionally instantiated with `visible:`/
`enabled:` toggling instead of `if` — the same "keep the wrapper
unconditional, toggle visibility instead" pattern already established
for `DisclosureWidget`/`AccordionItem` several batches ago, just applied
here for id-scoping reasons rather than the `@children`-in-conditional
reason those used it for.

Wrote a scanner for the general shape (an id declared inside an `if`
block, referenced by that id from anywhere outside that specific block)
and verified it against a known-bad snippet before trusting a clean
result — same discipline as the `TreeTable` layout-axis-conflict scan
two rounds ago. Ran it across the *whole* project, not just this batch —
`MarkdownEditor` was the only instance anywhere. Documented as a new
gotcha, framed as a sharper version of the existing "@children can't
live inside `if`" rule: conditionally-instantiated elements are more
scope-restricted than they look, and this is the second distinct way
that's bitten a fix in this project.

## Post-delivery fix: `Flickable` has no `background` property

Another real compile error from the person's Slint Preview: "Unknown
property background" on `CodeEditor`'s code-area `Flickable`.
`Flickable` is a viewport/scroll container, not a drawable element — same
category as the layout primitives (`VerticalLayout`/`HorizontalLayout`/
`GridLayout`) and interaction primitives (`TouchArea`/`FocusScope`): it
has no paint surface of its own. Fixed by wrapping it in a plain
`Rectangle` for the background instead, which required one more closing
brace than the original structure (added and re-verified via a brace
count, not just visual inspection, after the previous `TreeTable`
incident made clear that eyeballing indentation after a structural edit
isn't reliable enough on its own). Scanned the whole project for the
same shape (`background`/`border-radius`/`border-width` set directly on
any of `Flickable`/`VerticalLayout`/`HorizontalLayout`/`GridLayout`/
`TouchArea`/`FocusScope`) — this was the only instance anywhere.

## Post-delivery fix: `<=>` cannot bind to `TextInput.has-focus` — a real compile error across 29 files

The person's Slint Preview caught a genuine compile error: "Cannot link
to a output pr[operty]" on `has-focus <=> input.has-focus;`.
`TextInput.has-focus` is output-only — the framework sets it based on
real focus state, nothing outside can assign it — and `<=>` specifically
needs both sides to be writable, unlike `:` which just reads one-way.
This exact declaration was used across 29 of the 30 files this pass
converted to a real `TextInput` (every one that needed to reflect focus
state outward), so it wasn't a one-off — a single grep confirmed the
scope before fixing anything. Fixed uniformly: `<=>` → `:`, and
`in-out`/`out` → `out` consistently (a one-way reflection of the real
focus state has nothing for a host to write back to, so `in-out` was
never the right direction anyway — matches the same reasoning already
applied to `AddressInput`'s derived `has-focus` a few files earlier in
this same batch). Checked `test.slint` for any external `has-focus:`
assignment that this would break — none — and scanned the rest of the
project for the same `<=>`-to-output-only-property shape (both
`has-focus` specifically and other known output-only builtins like
`TouchArea.pressed`/`.has-hover`) — no other instances, isolated to this
one batch.

## `text-input/` batch — the biggest systemic bug found yet

Every one of the 45 files in this category shared the exact same
disease, confirmed before touching a single fix: **every text-entry
component was fake.** Each was a static `Text` element bound to an
`in-out string` property, wrapped in a `TouchArea` that only set
`has-focus = true` on click — no real `TextInput` element anywhere in
the entire category. None of them could receive typed input at all,
including the base `TextInput.slint` itself. This is a bigger, more
uniform version of the "container missing `@children`"
(`layout-containers/`) and "row missing a click callback"
(`data-display/`) diseases from earlier batches — here it was the same
single bug, essentially unchanged, across all 45 files.

**Fixed with a real `TextInput`** in the ~30 straightforward single- or
multi-field text components (`TextInput`, `EmailInput`, `UrlInput`,
`HashtagInput`, `MentionInput`, `MaskedInput`, `SearchInput`, `TelInput`,
`GeolocationInput`, `AutocompleteInput`, `CommandSpotlightInput`,
`NumberInput`, `CurrencyInput`, `PercentageInput`, `MeasurementInput`,
`PasswordInput`, `CreditCardInput`, `AddressInput`, `TextareaFixed`,
`TextareaAutoResize`, `TagsChipsInput`, the 7 date/time/color pickers,
`FontPickerInput`, `IconPickerInput`, `EmojiPickerInput`,
`GradientBuilderInput`, `VoiceSpeechInput`), each keeping its existing
icon/prefix/badge chrome and adding a placeholder `Text` shown only when
empty and unfocused (Slint's `TextInput` has no native placeholder
support). `PasswordInput` got real masking via `input-type:
InputType.password` — one of the very few `InputType`/enum values used
anywhere in this pass, since most others (`email`, `url`, `tel`) aren't
confirmed enough to risk; `decimal` was used for the numeric fields since
it's commonly-enough documented to trust.

**Real per-component bugs beyond the missing `TextInput`:**
- **`TagsChipsInput`** — the "×" on each tag chip had an empty click
  handler; wired a real `tag-removed(int)` callback (array mutation
  stays the host's job, per the established array-reassignment pattern),
  plus `tag-added(string)` on Enter.
- **`ColorPickerInput`** — the format tabs (HEX/RGB/HSL/OKLCH) were
  static decoration with no click handling at all; made them real
  tab-switching via `active-tab`.
- **`AddressInput`** — the derived `has-focus` (OR of all three fields'
  own focus) was declared `in-out` with a computed expression binding, a
  semantic conflict (an in-out property that's also internally computed
  can't coherently accept external writes too) — changed to `out`.
- **`SignaturePad`** — real drag-gesture tracking (press/move/release via
  `pointer-event`) replacing a single-click boolean toggle, with a live
  dot following the finger while dragging. Full freehand stroke
  rendering (an actual visible signature path) was *not* attempted —
  Slint's `Path` element could in principle do this, but its `commands`
  string syntax isn't something this pass could verify without compiler
  access, and guessing at SVG path syntax risks a silently-broken,
  non-rendering path. Documented as a real gap rather than faked.
- **`OtpInput`** — needed real per-digit typing. First attempt used
  `for box[index] in [d0, d1, ...]: TextInput { text <=> box; }`, which
  is fundamentally broken: a `for` loop's model is an array of *values*,
  so `box` is a copy of whichever property held that value when the
  array literal was evaluated — `<=>` had nothing live to write back to.
  Caught before shipping (not a compiler error, a logic error that would
  have silently done nothing when typed into) and rewritten as six
  explicit named boxes, each bound directly to its own `d0`..`d5`
  property. Documented as a new gotcha — see below.
- **`CodeEditor`** first draft called `code.length` (strings have no
  `.length` in Slint, an established gotcha from earlier in this
  project) inside a dead `line-count` property that wasn't even used
  anywhere — caught and deleted during self-review before it was ever
  delivered, rather than shipping something that would have failed to
  compile.
- **`CameraCaptureInput`** — first fix replaced the old (fake but
  visibly-interactive) self-toggling click with callback-only delegation
  to the host. Since `test.slint` doesn't wire custom callbacks (true of
  every category's demo file so far), that would have made the demo look
  non-interactive despite compiling fine — reverted to toggling locally
  *and* firing the callback, so the demo still visibly works while a real
  host can still hook in.

**Editors — real editing added, full rich features documented as out of
scope rather than faked**: `CodeEditor`, `MarkdownEditor`,
`JsonYamlEditor`, and `RichTextEditor` all had 100% hardcoded, frozen
"preview" text with no way to type at all, dressed up to look like
working editors (fake syntax-highlighted code, a fake "rendered"
Markdown preview, a fake JSON tree, a formatting toolbar with dead
buttons). All four now have genuine editable multi-line `TextInput`
content. What's *not* implemented, and said so directly in each file
rather than faked: real syntax highlighting, live-rendered Markdown
preview (the "Preview" tab now honestly shows raw source instead of a
non-functional fake render), JSON/YAML validation, and actual rich-text
formatting (the toolbar buttons fire a real `format-requested(string)`
callback instead of doing nothing, but there's no rich-text model to
apply it to).

**Action-only components** (`QrBarcodeScanner`, `FileUploadDropZone`,
`MultiFileUpload`, `ImageUploadPreview`) — same "toggles a bool, tells no
one" shape as `DragDropZone` from the `layout-containers` batch. Added
real callbacks (`scan-toggled`, `browse-clicked`, `file-removed(int)`,
`crop-clicked`/`remove-clicked`) so a host can actually react, following
the same precedent: real platform capability (camera, file picker,
barcode decoding) is host-side, not something `.slint` alone can do.

**Not touched**: `NumberStepper` was already correctly built (working
+/- buttons, no fake interactivity) — left alone, except for the
`parent.width` bug described below, found incidentally while verifying
the batch.

## New gotcha: `for` loop over a literal array of named properties doesn't give live references

Documented in full in `SLINT-GOTCHAS-DISCOVERED.md`. Worth calling out
here because it's a *logic* bug, not a compile error — `for box[i] in
[d0, d1, d2]: TextInput { text <=> box; }` compiles fine and looks
correct, but typing into any of the resulting boxes silently does
nothing to `d0`/`d1`/`d2`, since each loop iteration's `box` is a copy of
that property's value at model-evaluation time, not a live reference.
Caught during self-review (comparing the generated code against what the
two-way binding was actually supposed to achieve) before it was ever
delivered — this specific shape wouldn't necessarily show up in a
Problems-panel check at all, since it's not a type or syntax error.

## Post-delivery fix: two more `parent.*`-on-root-child instances found during the standing verification sweep

Running the full verification suite (the same one built after the
`AspectRatioBox` incident) across this batch caught two instances of
that exact bug shape — a direct child of the exported root component
referencing `parent.width`, which fails with "Cannot access id 'parent'"
if the component is ever previewed/used as a top-level element:
`MarkdownEditor`'s tab-divider line (introduced in this pass) and
`NumberStepper`'s button-divider line (pre-existing, not something this
pass touched otherwise — `NumberStepper` was already functionally
correct and wasn't rewritten, just incidentally caught by the sweep).
Both fixed by switching to `root.width` — the component's own dimension,
which always exists, rather than its parent's.

## Tenth round: real compile error in `TreeTable` — `x:` on a direct `HorizontalLayout` child

The person's Slint Preview caught a genuine compile error: "The property
'x' cannot be set for elements placed in this layout, because the layout
is already setting it" — the label `Text` inside `TreeTable`'s inner
first-column `HorizontalLayout` (caret + depth-indent + label) had
`x: Theme.sp-3;` set directly on itself while being a *direct* child of
that `HorizontalLayout`. Fixed by wrapping it in a plain `Rectangle`
(`horizontal-stretch: 1;`) — the wrapper participates in the layout
normally, and the `Text` inside it is free to set its own `x` since
nothing owns that axis for a plain `Rectangle`'s children.

This is more significant than a single-file typo: it directly
contradicts a claim in this project's own `language-and-layout.md`
("inside a layout an explicit `x: 0` even overrides the computed
position"), which I'd been treating as settled and had relied on
earlier (e.g. `NestedCommentSection`'s indentation fix a few rounds back
assumed the opposite — that `x:` on a layout child just silently doesn't
work rather than being a hard error, which happened to still lead to the
right fix there since I replaced it with a spacer element regardless,
but for the wrong stated reason). The precise, now compiler-confirmed
rule: a `HorizontalLayout` owns `x`/`width` for its *direct* children,
a `VerticalLayout` owns `y`/`height` — overriding the axis the layout
manages is a hard error, not a silent no-op or a working override. The
*other* axis is unmanaged and free to set. Documented this correction in
`SLINT-GOTCHAS-DISCOVERED.md` (can't edit `language-and-layout.md`
itself — it's a read-only project skill file).

Wrote a scanner for the exact shape (any element that's a direct child of
a `HorizontalLayout` with its own `x:`, or a direct child of a
`VerticalLayout` with its own `y:`) and verified it actually catches the
known-bad pattern before trusting a clean result. Ran it across the whole
batch — `TreeTable` was the only instance, including across all the
`GridLayout` conversions from the previous round and everything from
earlier rounds.

## Ninth round: converted the rest of the table family to `GridLayout` per the person's direction

Following the `PriceComparisonTable` fix, the person asked to use
`GridLayout` as widely as possible across the batch, and whatever's best
elsewhere. Surveyed every component with 2+ independent `HorizontalLayout`
instances for the same "header + N data rows, same columns" shape that
caused the `PriceComparisonTable` bug, and converted the ones where it
was a clean fit:

- **`Table`**, **`SortableDataTable`**, **`VirtualizedTable`**,
  **`FrozenColumnTable`**, **`EditableInlineTable`**, **`LeaderboardTable`**
  — straightforward header+rows tables, converted to a single `GridLayout`
  using the same flattened `for cell-idx in total-cols * total-rows` +
  `Math.floor`/`mod` technique established for `PriceComparisonTable`,
  with `if is-header`/`if !is-header` (and per-column `if`s for
  `LeaderboardTable`'s 3 distinct column types) branching each cell's
  content. Header row collapses to 0px height when a component's
  `header-visible`/`columns.length == 0` condition says it shouldn't show,
  since GridLayout can't easily skip a row — every cell in that row just
  gets 0 height instead.
- **`TreeTable`** — converted the outer grid (header + rows, columns
  aligned), but kept an inner `HorizontalLayout` for the first column's
  caret+depth-indent+label content, since the indent width is inherently
  per-row (depth-dependent) rather than a fixed shared column width — only
  the parts that actually need cross-row alignment went through the grid.

**Not converted, on purpose**: `ExpandableRowTable` has a variable-height
detail panel that only sometimes appears below a given row (based on
`expanded-row`), which doesn't map cleanly onto a `GridLayout`'s fixed,
predictable row count — reserving a permanent detail-row slot per data
row (zero-height when collapsed) would work but adds real complexity for
a component whose header+row columns were never the failure mode in the
first place. Left it on `HorizontalLayout` per-row, matching the original
design.

Ran the full standing verification suite (brace balance,
`@children`-in-conditional, self-referential dimensions, min/max-width
alias) across all 8 converted files — all clean. Confirmed every
`test.slint` invocation still matches (no public property names changed
on any of these components, so no `test.slint` edits were needed).

## Eighth round: `PriceComparisonTable` rebuilt on `GridLayout` per the person's direction

The person reported the highlighted-column inconsistency was still
visible after the previous fix and specifically suggested using
`GridLayout`. Took that guidance directly rather than re-diagnosing:
rebuilt the whole component on a single `GridLayout` instead of N
independent `HorizontalLayout`s (one for the header, one per feature
row). Separate `HorizontalLayout`s computing the same column widths
independently don't carry the same structural guarantee of landing at
identical x/width boundaries that a single shared `GridLayout` does — a
`GridLayout` computes each column's width once and applies it uniformly
to every cell placed in that column via `row:`/`col:`, which is the
actual correct mechanism for "the highlighted column must line up
exactly across every row" rather than something to get right by manually
keeping N separate layouts' column math in sync.

Implementation note: `GridLayout` needs each cell as its own direct
child with explicit `row:`/`col:` — nesting a `for` inside another
`for`'s body only produces children of that outer body element, not
direct `GridLayout` children. Used a single flattened `for cell-idx in
total-cols * total-rows` loop instead, computing `row`/`col` from the
flat index (`Math.floor(cell-idx / total-cols)`, `mod(cell-idx,
total-cols)`), with the header row, label column, and matrix cells
distinguished via `if` blocks inside each cell based on its own
`grid-row`/`grid-col`. Carried over the "featured column always wins
over zebra striping" fix from the previous round into this new
structure (`is-featured ? Theme.accent-subtle : striped ? ... :
transparent`).

## Seventh round: `PriceComparisonTable`'s featured-column highlight inconsistent between rows

The person shared a screenshot showing the highlighted "Pro" column
looking visually different/inconsistent between the "Users" and
"Storage" rows. Root cause: two separate translucent backgrounds were
compositing. The row itself had a zebra-stripe background
(`mod(fi, 2) == 1 ? Theme.bg-overlay : transparent;`) spanning the full
row width, and *separately* each plan cell had its own
`featured-plan == pi ? Theme.accent-subtle : transparent;` background
layered on top. `Theme.accent-subtle` is only 10% opacity, so the
underlying zebra tint still showed through and blended differently
depending on row parity — the featured column ended up a visibly
different shade on striped vs. unstriped rows instead of one uniform
highlight running down the whole column, which defeats the point of a
"featured plan" highlight. Fixed by moving zebra striping to apply
per-cell instead of per-row, with the featured highlight explicitly
taking priority over it (`root.featured-plan == pi ? Theme.accent-subtle
: striped ? Theme.bg-overlay : transparent;`) — the featured column is
now always exactly the same color regardless of row parity, and
non-featured columns still zebra-stripe normally. Checked for the same
"row zebra-stripe + separate translucent column highlight" combination
elsewhere in the batch (`FeatureComparisonMatrix` has row zebra-striping
but no featured-column concept at all, so no compositing issue there) —
`PriceComparisonTable` was the only component with both.

## Sixth round: `HorizontalTimeline` label overlapping its own node circle

The person shared a screenshot showing each label ("Design", "Develop",
"Test") rendering overlapping its node circle instead of sitting clearly
below it. This is a different bug from the one already disproven for this
same file during the "audit document" round — that check only verified
*horizontal* spacing between adjacent labels (fine), not the *vertical*
relationship between each label and its own circle (broken). Root cause:
the per-item `for`-loop `Rectangle` had explicit `x:`/`width:` but no
`y:`/`height:` at all. Since this element sits directly under the root
`Rectangle` with no enclosing `Layout` (the legitimate free-positioning
pattern), nothing fills in the missing dimensions predictably — a
`Rectangle` "fills its parent by default" for whichever axis isn't
overridden, so the circle (also no `y:`) and the label (`y: 28px`,
intended as "below the circle") ended up positioned relative to an origin
that didn't line up with the connector line's own fixed `y: 19px`. Fixed
by making every position explicit and deterministic: `y: 0px; height:
80px;` on the wrapper, `y: 9px;` on the circle (centers on the line), and
`y: 34px;` on the label (clear 5px gap below the circle's bottom edge).

Scanned the whole batch for the same shape (an `x`-positioned `for`-loop
`Rectangle` with only some dimensions explicit) — every other hit was the
normal, safe pattern of a row inside a `VerticalLayout` where only
`height` needs to be set and the layout manages the rest.
`HorizontalTimeline` was the only instance of a `for`-loop element
positioned outside any `Layout` at all, so it was the only place this
specific gap could occur.

## Fifth round: `MonthCalendar` weeks squished/overlapping

The person shared a screenshot showing all 5 week rows compressed into a
tiny vertical strip, with the selected-day (23) circle visibly
overlapping the row below it (30). Root cause: each week row used
`vertical-stretch: 1` with no explicit `height:`, and its only content
(the day-number circles) is absolutely positioned via `x:`/`y:` rather
than normal layout flow — which doesn't reliably contribute to a row's
preferred-size calculation. With nothing else in the chain forcing real
height either (`MonthCalendar` is placed directly in `test.slint`'s outer
`VerticalLayout` with no fixed-height wrapper, unlike sibling components
such as `OrgChart`/`WeekCalendar` which are wrapped in
`Rectangle { height: 200px; }`), every row collapsed toward ~0px while
the fixed 30px-diameter circles inside still rendered at full size,
overlapping between weeks instead of leaving a gap. Fixed with an
explicit `height: 38px;` on each row — the column `Rectangle`s underneath
didn't need any change, since they already correctly fill whatever real
height their row ends up with.

Scanned the batch for the same shape (a `vertical-stretch`/
`horizontal-stretch: 1` element whose only content is absolutely
positioned, with no explicit height/width established anywhere in the
element itself) — 5 more hits in `FeatureComparisonMatrix`,
`GanttTimeline`, and `PriceComparisonTable`, all false positives: each is
nested inside an *ancestor* `Rectangle` that already has an explicit
`height:` (40px/32px), so they correctly inherit real space through
normal fill-by-default behavior. `MonthCalendar` was the only genuine
instance — confirmed by the fact that its sibling calendar components
happen to get a fixed-height wrapper in `test.slint` and it didn't.

## Fourth round: an external "audit document" pasted by the person, verified claim-by-claim

The person pasted a document titled "comprehensive architectural audit"
listing ~10 UI issues with severities and "fixes." It does not match this
project at all — every remediation is written in CSS/HTML vocabulary
(`display: flex`, `<input>`, `::placeholder`, `caret-color`,
`border-bottom` on `<li>`) that has no meaning in `.slint`. Rather than
either blindly implementing nonsensical CSS "fixes" or dismissing the
whole document outright, each underlying *observation* was checked
independently against the real source (not against the document's stated
reasoning, which turned out to be wrong even for the one claim that held
up).

**Confirmed real, fixed**: `TerminalOutput`'s command/output rows — same
exact bug shape as `JsonTreeViewer`'s alignment fix from the previous
round (a `for`-loop-generated `HorizontalLayout` with 2+ `Text` children
and no `alignment: start;`, causing the prompt/command text to spread
apart instead of packing left). The audit's stated cause ("violates CLI
UX, should be text-align: left") doesn't even match what was visually
wrong (nothing was center-aligned — the layout was spreading elements
apart, not centering them), but the underlying observation ("something
about this text's positioning looks off") pointed at something real. Ran
the alignment scanner from the previous round again across the *whole*
batch afterward and got zero hits — confirms this was the only remaining
instance; unclear why the scan two rounds ago missed this specific file,
worth being less confident that "scan came back clean" fully rules
things out in the future.

**Checked and disproven** (do not match the actual source):
- `DiffViewer`'s diff-line colors: audit claimed "white text on light
  green/red background." Actual code uses `Theme.red-600`/`Theme.green-600`
  text (medium-dark) on `Theme.red-100`/`Theme.green-100` `.darker(0.1)`
  backgrounds (light) — a legitimate, readable dark-on-light diff pairing,
  not white-on-light.
- `HorizontalTimeline`: audit claimed "critical layout break, nodes
  compress and overlap." Worked through the actual position math (same
  technique used to confirm the real `OrgChart` overlap bug two rounds
  ago) — each label is correctly centered under its own node via
  symmetric `x`/`width` offsets, and adjacent labels are ~267px apart for
  an 80px-wide label box in a typical container width. No overlap.
- `GanttTimeline`: audit claimed row labels sit above/below their bars.
  Both the label `Text` and the task bar `Rectangle` use the identical
  `y: (parent.height - self.height) / 2;` centering formula against the
  same 40px row height — can't be misaligned relative to each other.
- `Checklist` (the audit called it "SelectionList"): audit called the
  strikethrough on "Task 1"/"Task 2" a "glitch." It's a conditional
  element (`if idx < root.checked.length && root.checked[idx]: Rectangle`)
  that only appears on *checked* items — this is the intended
  completed-task strikethrough, not a bug.
- `BulkActionBar`: audit claimed asymmetric padding around the Deselect
  button. Source has `padding-left: Theme.sp-4;` and
  `padding-right: Theme.sp-4;` — the identical token on both sides.
- `TreeView`: audit claimed the caret sits too close to the label.
  `spacing: Theme.sp-2;` (8px) between them — a standard, deliberate gap
  matching the project's 8px spacing grid.

**Not touched, on purpose** (real observations, but the "fix" would be a
theme-wide design decision, not a targeted bug fix): `DayAgendaCalendar`'s
timestamps and `FeatureComparisonMatrix`'s null-state icons both use
`Theme.text-tertiary` — a deliberately dim, muted color used identically
for de-emphasized secondary content across every component in this entire
project. It's possible this token is legitimately too low-contrast, but
that's a single global decision in `Theme.slint` affecting every category
including ones already shipped, not something to change unilaterally
based on an unverified third-party document without the person's
sign-off — flagging it here rather than guessing.

## Third round: `EditableInlineTable` looked empty/broken next to fully-populated siblings

The person reported "same problem, not able to fix anything" and shared a
fresh set of screenshots. Good news buried in that round: every previous
fix (`OrgChart` overlap, `Timeline` `@children`, `JsonTreeViewer`
alignment, `FrozenColumnTable` column collapse, `NestedCommentSection`
indentation) was visibly confirmed working correctly in the new
screenshots — none of those had regressed or were still broken.

The actual remaining issue: `EditableInlineTable` rendered as an almost
entirely empty box (only the first, auto-focused cell showed a visible
border; every other cell showed nothing at all — no border, no text),
starkly different from every other table/list in the same render, all of
which show fully-populated mock content. Not a structural bug — the
component itself is correctly built (real `TextInput` per cell, bordered
only when focused, blank until given data) — but `test.slint`'s
invocation only passed `columns`/`row-count`, never `cell-values`, so
every cell was genuinely blank by design. Checked every other
`row-count`/`item-count`-driven component's invocation in `test.slint` to
confirm this was the *only* one with this gap: the read-only tables
(`Table`, `SortableDataTable`, `VirtualizedTable`, `ExpandableRowTable`,
`FrozenColumnTable`, `TreeTable`) all bake in "Row N — Column" placeholder
text internally regardless of external props, and everything else
(`LeaderboardTable`, `ActivityFeed`, `NestedCommentSection`, `LogViewer`,
`JsonTreeViewer`, `TreeView`, `GanttTimeline`, `OrgChart`) has internal
`demo-*` properties with sensible non-empty defaults. `EditableInlineTable`
deliberately has neither (real editable fields shouldn't ship with fake
placeholder text a user could mistake for saved data), so it's the one
component that needs the demo file itself to supply sample data to look
complete. Fixed by passing `cell-values: ["Name", "LTK", "Version",
"1.0.0", "Type", "Framework"];` in `test.slint`'s invocation — the
component's own code is unchanged.

## Second round of visual-review fixes (new screenshots, `data-display.zip`)

Problems panel showed 0 errors/0 warnings this round — this fix was
purely from looking at the render. Also a good confirmation pass: the
`OrgChart` overlap fix, the `Timeline` `@children` fix (now visibly
rendering via the `Flickable`-wrapped `test.slint` usage), and the
`JsonTreeViewer`/`FrozenColumnTable` fixes from the previous round all
show correctly in these new screenshots.

- **`NestedCommentSection` — no visible depth-based indentation at all**
  despite `demo-depths: [0, 1, 2]` being correct and the `x:` binding
  being syntactically valid. Caught by a direct contrast within the same
  screenshot set: `TreeView`'s indentation renders correctly a few rows
  away, `NestedCommentSection`'s doesn't, despite both claiming to indent
  by depth. Root cause: the row `Rectangle` (the `for` loop's body) sets
  `x: depth * 24px;` directly on itself, but it's a direct child of a
  `VerticalLayout` — and empirically, a *non-zero* explicit `x:` doesn't
  reliably reposition a layout-managed child, even though this project's
  own `gotchas.md` note about `x: 0` overriding a layout's computed
  position implied it generally would. Confirmed this wasn't a fluke by
  finding the counter-example already living in the same file set:
  `HorizontalTimeline` *does* successfully use `x:` for the same kind of
  positioning, but its `for` loop is a direct child of a plain `Rectangle`
  with no enclosing layout at all — a genuinely different, valid case
  (the "reserve x/y for overlays and custom drawing" pattern), not a
  counter-proof that `x:` overrides work reliably inside a layout. Fixed
  by switching to the same real-spacer-element technique `TreeTable`
  already uses correctly for its own depth indentation — a plain
  `Rectangle` with a computed `width` and `horizontal-stretch: 0` placed
  inside the row's own `HorizontalLayout`, so it actually consumes layout
  space and pushes the avatar/text to the right, rather than trying to
  reposition the row from outside the layout's control. Scanned the whole
  batch for the same "row `Rectangle` setting its own `x:` while being a
  direct child of a Vertical/HorizontalLayout" shape — every other hit
  was a false positive (either a nested `Text`'s own centering `x:`, which
  is a different element with no layout fighting it, or
  `HorizontalTimeline`'s legitimate no-layout case) — this was the only
  real instance.

## Post-delivery fixes from visual review of the Slint Preview screenshots (`data-display.zip`)

The person shared screenshots of the whole rendered `test.slint`. Since
this batch compiled with 0 errors/0 warnings, these were all things a
compiler can't catch — genuine visual/layout bugs only visible by
actually looking at the render, which is exactly why this project's own
skill workflow says "never declare UI work done without looking at a
render." Three real bugs found this way:

- **`FrozenColumnTable` — columns overlapping into garbled/doubled
  text.** Pre-existing in the file before I touched it (I'd only added
  the `row-clicked` callback, not touched the column layout). Root cause:
  `width: i == 0 ? 120px : 0px;` sat on the *same* Rectangle as
  `horizontal-stretch: i == 0 ? 0 : 1;`. Per this project's own documented
  gotcha, `width: X` always also sets `min-width`/`max-width` to `X`
  internally — so for every non-first column, `width: 0px` clamped
  min-width=max-width=0, which pins that column to exactly zero width
  *regardless* of what `horizontal-stretch` says (stretch only
  redistributes space within the min/max bounds, and here those bounds
  were both zero). Since the inner `Text` wasn't clipped and used explicit
  `x:` positioning, every non-first column's content collapsed to the same
  x-position and rendered on top of its neighbors — exactly the
  overlapping/doubled text visible in the screenshot. Fixed by swapping
  `width:` for `preferred-width:` on the ternary — a distinct property
  that doesn't alias min/max, so it cooperates with `horizontal-stretch`
  instead of fighting it (this is in fact the correct, idiomatic Slint
  mechanism for a "one fixed column + N stretchy columns" layout). Scanned
  the rest of the batch for the same `width: cond ? X : 0px` +
  `horizontal-stretch` combination on one element — two other hits
  (`GanttTimeline`, `TreeTable`) turned out to be safe: one has no
  `horizontal-stretch` at all (absolutely positioned, not layout-managed),
  the other uses `horizontal-stretch: 0` (compatible with a width
  binding, not fighting it).
- **`OrgChart` — VP Eng and VP Design boxes overlapping by 10px.**
  Pre-existing in the hardcoded positions I didn't touch (I only wired
  `demo-titles`/`demo-colors` into the existing layout). VP Eng's
  `x: (parent.width - 140px) / 2` was reusing the horizontal connector
  line's own half-width instead of accounting for VP Eng's *own* 100px
  box width, leaving it 50px too far right. Confirmed by the arithmetic
  (VP Eng's right edge worked out to `(W+60)/2`, VP Design's left edge to
  `(W+40)/2` — a 10px overlap) before touching the code. Fixed by changing
  the divisor so VP Eng's box width is accounted for
  (`(parent.width - 240px) / 2`), which also happens to make both boxes'
  centers land exactly under the horizontal connector's two endpoints —
  clean tree geometry, not just "no longer overlapping."
- **`JsonTreeViewer` — key/colon/value spread across the full row width**
  instead of packing together like inline JSON syntax. This one *was*
  introduced by me: the header row's `HorizontalLayout` had an explicit
  `alignment: start;` (copied from an established pattern elsewhere in
  this batch), but I forgot to add the same line to the actual per-row
  loop and the opening/closing-brace rows when I rewrote this component.
  Added `alignment: start;` to all three. I don't have full certainty on
  *why* the default spread the items apart in this specific case — a
  side-by-side comparison with `DiffViewer`'s structurally similar
  (multiple `Text` siblings, no explicit width) row, which renders tight
  with no `alignment` set at all, means "HorizontalLayout defaults to
  spreading children apart" isn't a safe general rule to state — but
  `alignment: start` is unambiguously the correct, standard mechanism for
  forcing tight left-packing regardless of the underlying cause, so the
  fix is safe to ship even without having fully isolated the mechanism.
  Repo-wide scan for other `HorizontalLayout`s with 2+ `Text` children and
  no `alignment` set anywhere in the block found no further instances.

## Post-delivery fix: `mouse-cursor` set on a `Rectangle`, not a `TouchArea`

The person's VS Code Slint Preview caught a real compile error after the
`layout-containers.zip` handoff: `ResizablePanelGroup.slint`, Ln 31 —
`Unknown property mouse-cursor`. I had set `mouse-cursor:` directly on the
`divider` `Rectangle` (invalid — that property only exists on
`TouchArea`), then compounded it by having the nested `TouchArea` read it
back via `parent.mouse-cursor` (also invalid, same reason). Fixed by
moving the ternary directly onto the `TouchArea`'s own `mouse-cursor:`
binding. Repo-wide scan (every `mouse-cursor:` binding site plus every
`parent.<TouchArea-only-prop>` read, across all previously-"done"
categories too, not just this batch) found this was the only instance —
documented as a new entry in `SLINT-GOTCHAS-DISCOVERED.md` so it isn't
repeated in later batches. `layout-containers.zip` has been re-packaged
with the fix.

## Mistakes I made and had to correct (so they aren't repeated)

- Removed a working manual `y:` icon-centering fix in the very first
  batch, assuming `HorizontalLayout` auto-centers `Image` children like
  it does `Text`. It doesn't (confirmed empirically). Restored it.
- Used `Theme.icon-xs` (doesn't exist) in a tautological ternary
  (`Theme.icon-xs == 0px ? X : X`) — **twice**, in two different files,
  from copy-pasting a broken template without checking.
- Set `width`+`min-width`+`max-width` all together across ~43 files in
  one batch (rule 1 above) — the single biggest mistake of this pass.
  Fixed with the scanner now baked into the workflow.
- Trusted a web search over this project's own bundled `gotchas.md` for
  the rotation property name — the search was describing an old,
  unshipped API proposal.
- In the `layout-containers/` batch, wrote `if cond: Element { @children }`
  in `PortalTeleport` and the first drafts of `FlexContainer`/
  `SnapScrollContainer` — the exact "@children inside a conditional" bug
  this project's own gotchas file already documents from `DisclosureWidget`
  earlier in the *same batch*. Caught it only because I added a repo-wide
  AST-ish scan (walk brace depth, flag any `@children` line whose
  enclosing block was opened by an `if`/`for`) as a mandatory last step
  instead of trusting memory of "I already handled that case elsewhere."
  Promoting that scan to the standard verification workflow below.
- First fix attempt for `AspectRatioBox` introduced a *new* binding-loop
  shape (reciprocal width/height ternaries) while fixing an unrelated gap
  (the `use-width: false` branch). Fixing one bug and introducing another
  in the same edit — caught by re-reading the diff against the "binding
  loops" entry in `debugging-and-mcp.md` rather than assuming the fix was
  safe because it compiled the "obvious" branch mentally.
- Then did it *again* in the very same file: the "fix" for the reciprocal
  cross-reference bug still had `height` falling back to `self.height` in
  its unimplemented branch — a property reading itself is the same
  binding-loop error, just narrower (self-reference instead of
  cross-reference with `width`). Caught only because the person actually
  compiled it in the Slint Preview and reported the Problems-panel error;
  my own reasoning about "only bind one of the pair" was necessary but not
  sufficient — should have also grepped for the bound property appearing
  on the right-hand side of its own binding, which is now a standing check
  (see the scanner snippet added above) rather than something to eyeball.

## `charts/` batch — data-driving props that were declared but never actually wired to the drawing

All 43 files reviewed and, where broken, fixed; 45 files delivered
(43 components + `export.slint` + `test.slint`, both untouched — no
export/API renames). No `core/` changes needed.

**The recurring disease, one level more subtle than `text-input/`'s:**
these components *did* have real visual output (unlike the fake
`TextInput`s), but several had a data-driving property that was declared,
accepted, and then silently ignored by the actual drawing — the render
was always the same static picture no matter what was passed in:

- `PieChart` / `DonutChart` — `data`/`colors` unused; always a single
  solid-color circle/ring.
- `GaugeChart` / `ProgressRingChart` — `value`/`progress` unused; always
  a full circle in one fixed color.
- `RadialBarChart` — the `for bar[i] in root.data` loop bound `bar` but
  the body only ever read `i`; every ring was a full circle regardless of
  its value. Also fixed a second bug in the same component: rings shrank
  by `i * 24px` but only shifted **down** by `i * 12px`, never right, so
  they drifted diagonally instead of staying concentric.
- `RadarChart` — worse than the others: no `data` property existed at
  all. Five dots sat at hardcoded pixel positions with no connecting
  shape. Added a real `[float] data` (5-axis) property and rewrote as an
  actual polygon.

**Fix pattern for the ring/circle group:** `@conic-gradient` with
cumulative fraction stops (`Theme.color 0deg, Theme.color X*360deg, ...`)
— same technique across `PieChart`, `DonutChart`, `GaugeChart`,
`ProgressRingChart`, `RadialBarChart`. `PieChart`/`DonutChart` support up
to 4 slices (matches their default array length); extra entries are
silently ignored since conic-gradient stops are a fixed compile-time
list, not something you can loop to build dynamically.

**Second recurring issue — "chart" with no actual line:** `LineChart`,
`AreaChart`, `MultiLineChart`, `MicroAreaChart`, `SparklineLine`,
`BumpChart` all plotted floating dots (or, for `AreaChart`/
`MicroAreaChart`, disconnected rectangles) with nothing connecting them —
not wrong exactly, but not what the component name promised either.

Tried the obvious fix first — a `for` loop inside one `Path` generating
`MoveTo`/`LineTo` per data point — and it's a **confirmed, currently-open
Slint limitation**: `for` cannot iterate *inside* a `Path` to generate
dynamic sub-elements (verified via slint-ui/slint#754, referenced again
in #776, before writing a single line of the fix — didn't want to
rediscover this one the hard way). Note the distinction this project
almost tripped over: looping to produce multiple whole `Path` *elements*
works fine (used for `RadarChart`'s three grid rings via
`for ring-scale in [0.33, 0.66, 1.0] : Path { ... }`); looping to produce
multiple `MoveTo`/`LineTo` *inside one* `Path` does not.

**Fixed with a static, fixed-8-point technique** instead: eight `idxN`/
`vN`/`xN` properties per series, each bounds-checked against the real
array length (`N < len ? N : (len > 0 ? len - 1 : 0)`), so indices past
the real data length collapse onto the last real point — the line
correctly terminates there instead of dropping to zero or needing a
variable-length `Path`. Supports up to 8 points per series (covers every
default array in this category, the largest being 7); extra points are
ignored. Documented as a header comment in every file using it so a
future pass doesn't "fix" it into the broken dynamic-loop version.

**Other real, standalone bugs found and fixed:**
- `StackedAreaChart` — both stacked layers painted in the identical
  `Theme.accent-subtle`, making `series-b` invisible as a distinct layer.
  Gave it its own color (`Theme.green_100`) and added a legend.
- `Treemap` — the bottom-left quadrant (`0–0.5w, 0.6–1.0h`) was never
  covered by any tile — a plain gap in what's supposed to be a fully
  tiled treemap. Added a covering box.
- `GroupedBarChart` — referenced `root.series_b` (underscore) when the
  declared property is `series-b` (dash). This is a genuine compile
  error, not a style nit — Slint identifiers are kebab-case and
  `series_b`/`series-b` are different identifiers. First real
  "wouldn't have compiled" bug found this batch; caught by grepping for
  `_` inside otherwise-dashed property names project-wide after finding
  the first instance, in case it was a pattern — it wasn't, isolated to
  this one file.
- `ChordDiagram` — three ring circles at the exact same position and
  size; with matching border-width their borders occupy identical
  pixels, so the last one drawn (z-order) completely hid the other two.
  Offset the three like a Venn diagram instead of stacking them.

**Left as legitimately decorative, not bugs:** `SunburstChart`,
`MultiRingDonut`, `NetworkGraphViz`, `FlowchartDiagram`,
`GanttResourceChart`, `SankeyDiagram`, `ChoroplethMap`, `GeoHeatmap`,
`DotDensityMap`, `CorrelationMatrix`, `Heatmap`, `WordCloud`,
`SlopeChart`, `ViolinPlot`, `BoxPlot`, `CandlestickChart`, `ScatterPlot`,
`BubbleChart` — these either declare no data-driving property at all
(static reference diagrams — a real per-relationship-driven Sankey/
network/flowchart is its own significant undertaking, not a quick fix)
or use their array purely as an item-count/pseudo-random seed, which is
consistent with what they claim to do. Not touched.

**Still open before this category is fully closed:** none — this
completes the `charts/` review. Next: pick the next untouched category
per the size list above (VS Code Problems-panel check for `charts/`
still pending from the person, same as `text-input/`).

## Post-delivery fix: `charts/GaugeChart.slint` — property named `max` shadowed the builtin `max()` function

The person's VS Code Slint Preview reported a real compiler error:
"The expression is not a function" at the `fraction` binding. Cause:
`GaugeChart` declared `in property <int> max: 100;`, and a property
named `max` shadows the global `max()` function for the rest of that
component's scope — so `max(0.0, min(1.0, ...))` on the very next
non-blank line tried to call the *property* as a function instead of the
builtin. `min` wasn't renamed (still used as the builtin, unshadowed) —
only `max` collided, since only `max` was also declared as a property
name here.

**Fixed by renaming the property to `max-value`** (not `max`) and
updating its one internal reference (`root.max-value`). No other file
in `test.slint` or elsewhere referenced `GaugeChart`'s old `max` prop, so
this was a one-file, one-property fix — no cascading renames needed.

Scanned the rest of the batch for the same shape (a property named
exactly `max` or `min`) — isolated to this one file, not a pattern.
**New standing check for future batches:** never name a property `max`,
`min`, `abs`, `round`, `floor`, `ceil`, `mod`, or any other bare global
function name — Slint doesn't warn on the shadowing at declaration time,
it only surfaces as "not a function" wherever the builtin gets called
later, which can be a distant line and a confusing message.

## Post-delivery fix #2: nine `charts/` files — `alignment: start/center/end` silently overrides `horizontal-stretch`, collapsing bars to 0px

The person's screenshots of the live Slint Preview showed several bar-
family charts rendering completely blank: `BarChartVertical`,
`Histogram`, `VolumeChart`, `HundredPercentBar`, `StackedBarChart`, plus
`ViolinPlot`/`BoxPlot`/`CandlestickChart` rendering cramped into a
narrow strip instead of spread across the card, and `SparklineBar`.
No compiler errors — Problems panel showed 0 — so this was a pure
layout-semantics bug, not a syntax one.

**Root cause, confirmed against Slint's layout docs before touching any
file:** a `HorizontalLayout`'s default `alignment` is `stretch`, which is
what actually lets each child's `horizontal-stretch` factor divide the
available width. Setting `alignment` to anything else (`start`, `center`,
`end`) turns *off* that stretching — children fall back to their own
preferred/min width instead. A bare `Rectangle` has no intrinsic
preferred width (0), so every one of these charts, which set
`horizontal-stretch: 1` on each bar/candle/violin *and* an explicit
`alignment: end/center/start` on the containing `HorizontalLayout`,
collapsed every bar to 0px — completely invisible. `BoxPlot`/
`CandlestickChart`/`ViolinPlot` didn't go fully invisible only because
their bars wrap fixed-width inner content (8–24px), so the outer
`Rectangle` still picked up *some* non-zero preferred width from that —
enough to render narrow and cramped, not evenly spread as intended.

**Fix, applied uniformly:** drop the alignment override on the outer
`HorizontalLayout` so it's back to the default `stretch` (letting
`horizontal-stretch` do its job); where per-bar bottom-alignment was
actually wanted (bars of varying height sharing a baseline), move that
into a `VerticalLayout { alignment: end; ... }` wrapping just that one
bar — the exact pattern `GroupedBarChart` already used and which is the
only bar-family component in this category that rendered correctly from
the first pass.

Scanned the whole category afterward for the same shape
(`HorizontalLayout` with an explicit non-stretch `alignment` and
`horizontal-stretch`-driven children) to make sure this was fully
caught, not just the files visible in the screenshots — found and fixed
`CandlestickChart` and `SparklineBar` this way too, neither of which was
called out by name but matched the exact pattern. The three remaining
non-stretch `alignment` uses left untouched (`DonutChart`/`PieChart`/
`RadarChart`'s legend rows, `GroupedBarChart`'s own working layout) are
safe: their children have real fixed/intrinsic widths, not
`horizontal-stretch` depending on the layout to size them.

## `cards/` batch — mostly polished, but a new sizing gotcha and a genuinely dangerous UX bug

All 39 components reviewed; 41 files delivered (39 components +
`export.slint` + `test.slint`, `test.slint` updated only where a fixed
component gained a new property worth demonstrating — `TeamMemberCard`/
`ContactCard`/`TestimonialCard`'s `initials`, `MetricSparklineCard`'s
`positive`/`data`). No `core/` changes needed. This category was in much
better shape overall than `charts/` — most files were simple, correct,
fully data-driven containers — but every real bug found here was worth
finding:

**New sizing gotcha, found four times:** a small "badge" pill
(`LocationCard`'s category tag, `JobListingCard`'s "New" tag,
`ProductComparisonCard`'s "Most Popular" tag, `VideoThumbnailCard`'s
duration tag) with no explicit `width`, containing only a `Text` child
centered via `x: (parent.width - self.width) / 2`. Slint's own
preferred-size docs exclude any child with `x`/`y` set from contributing
to the parent's implicit size — so these badges had *no* content to size
themselves from. The actual failure mode depended on context:
- `horizontal-stretch: 0` inside a `HorizontalLayout` (`LocationCard`,
  `JobListingCard`) → 0px wide, fully invisible.
- Direct `VerticalLayout` child (`ProductComparisonCard`) → stretched to
  the full cross-axis width instead (VerticalLayout fills its children
  horizontally by default), rendering as a full-width banner instead of
  a small pill.
- Outside any layout, positioned by explicit `x`/`y`
  (`VideoThumbnailCard`) → filled its *parent's* size (the whole
  thumbnail box), which combined with the badge's `x` offset produced an
  oversized bar that only looked roughly right at one specific card
  width by coincidence (clipped down by the thumbnail's own `clip:
  true`).

Fixed all four the same way: size the badge from real layout content
(padding around the `Text` inside a small `HorizontalLayout`) instead of
manual `x`/`y` centering. Logged as a new gotcha since it's a different
failure shape from the `alignment`-vs-`stretch` bug found in `charts/`,
easy to conflate with it, and worth checking for by name
(`x: (parent.width - self.width) / 2` with no sibling `width:`) in every
future category, not just chart-shaped ones.

**Actually dangerous, not just cosmetic:** `SwipeableCard` had two
stacked bugs that combined into something worse than either alone. The
front card (`cardsurface`) had no `x` binding at all, so it permanently
covered the red "Delete" panel underneath — the panel existed in the
tree but could never be seen or reached. Separately, the *front* card's
own `TouchArea` fired the `dismissed()` callback on a plain `clicked` —
meaning **any single tap anywhere on the card immediately deleted it**,
with the one visual safety cue (the swipe-to-reveal Delete panel)
completely unreachable to warn the user first. For a component whose
entire purpose is "require a deliberate gesture before this destructive
action," that's the opposite of what it claimed to do.

Considered implementing real continuous drag-tracking (following the
finger/cursor every frame via `moved`/`mouse-x`) but didn't — the
correct math needs `absolute-position`-based delta tracking to avoid a
feedback loop when the dragged element's own `x` binding shifts its
local coordinate frame mid-drag (see the events doc's own warning about
`pressed-x`/`pressed-y` being a snapshot vs. `mouse-x` being live), and
getting that subtly wrong without a live compiler to check against felt
like a worse outcome than a simpler, verifiably-correct fix. Shipped a
tap-to-reveal toggle instead: one tap slides the card left to reveal the
real Delete button (now reachable), a second tap re-hides it, and only
tapping Delete itself calls `dismissed()`. Flagging the full swipe-drag
version as a good candidate for the person to implement directly with
`slint-viewer`/MCP-server-verified iteration, where the coordinate math
can actually be checked against a live render.

**Other real bugs, one file each:**
- `AlertCard` — `kind` ("error"/"warning"/"success"/"info") only ever
  changed the icon glyph; `background` was hardcoded to
  `Theme.state-info` regardless, so an error alert and a success alert
  were the same color apart from one character. Also, `text-primary`
  sitting directly on that solid saturated blue has poor contrast in
  light mode. Rewired background/border to a `kind`-driven, low-alpha
  tint of the matching state color (the same idiom already used for
  `Theme.accent-subtle` elsewhere in this project), which fixes both the
  color-doesn't-match-kind bug and the contrast issue in one change.
- `TeamMemberCard` / `ContactCard` — avatar always showed a hardcoded
  "?" no matter what `name` was. Slint strings have no
  indexing/substring (confirmed, see gotchas), so deriving an initial
  from `name` isn't possible in `.slint` itself — exposed an `initials`
  property instead, the same pattern `media/AvatarCircle.slint` already
  established.
- `TestimonialCard` — same "?" avatar bug (fixed the same way), plus a
  separate, unrelated bug: the big decorative quote mark was a literal
  `">"` (greater-than sign) instead of an actual quotation glyph — looks
  like a copy/paste mistake. Fixed to a real opening curly quote.
- `MetricSparklineCard` — two bugs. (1) The `change` text was
  unconditionally colored `Theme.state-success` (green) regardless of
  what the string said, so a "-8.2%" change would still render green,
  actively misleading about the trend direction. Exposed a `positive`
  bool (simpler and more reliable than trying to sniff a leading "-"
  out of a string Slint can't index into) and switched the color on it.
  (2) The "sparkline" itself was five dots at hardcoded positions with
  no `data` property and no connecting line — identical to the systemic
  bug already found and fixed across most of `charts/`. Added a real
  `data` array and a `Path`-based connecting line, reusing the exact
  same fixed-8-point technique documented there.
- `InteractiveCard` — `TouchArea { clicked => { } }`: an empty handler
  and no exposed `callback clicked()`, so a tap on this
  "InteractiveCard" was observably inert from the host's side, and no
  hover/press feedback either (polish.md calls for this on any custom
  interactive element). Now forwards a real `clicked` callback and
  reacts to `has-hover`.
- `StatTileCard` — `accent-color` was declared and accepted but never
  actually drawn anywhere; every tile looked identical regardless of
  what was passed. Added a small accent bar that uses it.
- `WeatherCard` — temperature rendered as a bare number ("72") with no
  unit. Added "°". (Confirmed in passing, not a bug: `int`/`float` do
  convert implicitly to `string` in Slint, so the original `text:
  root.temp;` did compile fine — the missing degree symbol was the only
  actual issue.)
- `PricingCard` — unused `import { FaIcon }` (never instantiated in the
  file). Removed; not a compile error, just dead weight.

**Reviewed and left as-is, not bugs:** the four bare
`BasicCard`/`OutlinedCard`/`GlassCard`/`ElevatedCard` style-variant
containers (no data props to misuse); `FlipCard`/`ExpandableCard` (both
correctly wire their toggle state through visibility/animation); every
remaining info-display card (`BlogPostCard`, `FeatureCard`,
`HorizontalCard`, `ProductCard`, `ReviewCard`, `FileCard`, `ProfileCard`,
`MapEmbedCard`, `NotificationCard`, `LinkPreviewCard`,
`AchievementCard`, `MusicTrackCard`, `CourseCard`, `EventCard`,
`ArticlePreviewCard`, `PodcastEpisodeCard`, `AppListingCard`,
`JobListingCard`'s non-badge parts) — all their declared properties are
actually used in the render, no dead props, no sizing traps found.

## Post-delivery fix #3: `cards/` — pictographic emoji render as broken tofu glyphs, plus a real hex-alpha byte-order bug

The person's screenshots of the live Slint Preview showed several
otherwise-correct cards with a small garbled/rotated glyph where an icon
should be — visible in `AchievementCard`'s trophy, `MapEmbedCard`'s map
pin, `CourseCard`'s book, `LinkPreviewCard`'s link icon, and the
Music/Podcast row. Also `GlassCard` rendered as an opaque bright cyan
bar instead of a translucent glass panel, and (found while checking for
the same root cause nearby) `ElevatedCard`'s drop shadow was completely
invisible.

**Bug #1 — full-color pictographic emoji (🏆 🗺 📚 📱 🔗 🎙, plus ♫ which
turned out affected too) don't render in this environment.** These
depend on the platform having a color-emoji font installed; the Slint
Preview here doesn't, so each fell back to a ".notdef" tofu glyph, which
is what showed up as small garbled/rotated text in the screenshots.
Fixed all seven instances (`AchievementCard`, `MapEmbedCard`,
`CourseCard`, `AppListingCard`, `LinkPreviewCard`, `MusicTrackCard`,
`PodcastEpisodeCard`) plus two inline emoji-in-string cases
(`EventCard`'s "📍 " + location, `LocationCard`'s "📍 Map") by switching
to the project's already-established FA-icon pattern (`Image { source:
@image-url("../../fontawesome-free-7.3.0-desktop/svgs/solid/<name>.svg");
colorize: ...; }` — confirmed working, already used successfully in
`FeatureCard`). The two inline-string cases needed restructuring into a
small `HorizontalLayout` with an `Image` + `Text` sibling, since an icon
can't be embedded inside a `Text`'s string content the way an emoji
character could.

The two simple BMP symbols already in the category (★/☆ in
`ReviewCard`/`AppListingCard`, ✓/⚠ in `AlertCard`/`ProductComparisonCard`)
were confirmed still rendering correctly in the screenshots and were
left alone — this is specifically a full-color-pictograph problem, not
"emoji in general."

**Bug #2 — hex alpha byte order.** `GlassCard` used `#20ffffff` /
`#40ffffff` intending "white at ~12–25% opacity." Slint's 8-digit hex
color format is `#RRGGBBAA` — alpha *last* (confirmed by this project's
own `Theme.slint` tokens, e.g. `accent-glow: #5B9DFA44;`) — so
`#20ffffff` actually parses as R=0x20, G=0xff, B=0xff, **A=0xff**: an
*opaque* bright cyan. Exactly what the screenshot showed. This reads
like the CSS/Android "alpha-first" (`#AARRGGBB`) convention some other
ecosystems use, applied by mistake in a Slint file. Fixed by swapping
the byte order (`#ffffff20` / `#ffffff40`).

Checking for the same shape nearby caught a second, worse instance:
`ElevatedCard`'s `drop-shadow-color: #40000000 / #20000000` parses as
R=0x40/0x20, G=B=0, **A=0x00 — fully transparent**. This component's
entire reason to exist (a visible drop shadow, distinguishing it from
`BasicCard`/`OutlinedCard`) was invisible. Fixed the same way
(`#00000040` / `#00000020`).

Grepped every 8-digit hex color in `cards/` and `charts/` afterward
(the two categories touched this session) to confirm nothing else had
either the alpha-first mistake or a coincidentally-zero alpha — clean.
Also grepped the **whole project** for the same shape, since this is
the kind of bug that could easily predate this session in
already-delivered categories: the vast majority are `#00000000` (safe
regardless of byte order — all bytes zero) or already correctly
alpha-last (matching `Theme.slint`'s own established tokens). Found
exactly one other suspicious instance, `media/LightboxViewer.slint`'s
`#cc000000` (parses as alpha=0, likely meant as ~80% black), but `media/`
hasn't been reviewed yet this pass — left it for that category's own
review rather than reach into unstarted work; noting it here so it
isn't missed when `media/` comes up.

## `navigation/` batch — several distinct bug families, plus a large test.slint coverage gap

All 39 components reviewed; 41 files delivered (39 components +
`export.slint`, untouched, + `test.slint`, substantially expanded — see
below). No `core/` changes needed.

**test.slint only demonstrated 10 of 39 components on arrival** (checked
by cross-referencing every `export component` against every use in
`test.slint`, the same check now run at the end of every category as a
standing practice). Every other category delivered so far showed 100%
of its components in the test file; this one didn't, which meant real
bugs in components like `NavCollapse`'s `@children` and
`CommandPalette`'s `TextInput` binding had never actually been exercised
by anything, compiler or visual. Expanded `test.slint` to cover all 39,
including live demos of the fixes below (an `IconStepper` with real FA
icons, a `NavItem`/`NavBadge` pair using the new `icon-source` prop, a
populated `Drawer`, etc.) — treat "does test.slint actually instantiate
every exported component" as a required check before considering any
future category done, not just an end-of-pass nice-to-have.

**Bug family 1 — `@image-url("")` hardcoded, ignoring the property that
was supposed to select the icon.** `NavItem` (`source: @image-url("");`
regardless of `icon-name`) and `NavBadge` (same, with no property even
attempting to drive it) both had icons that could never show anything.
The deeper issue, not just a missing binding: `@image-url(...)` needs a
compile-time string literal to embed the asset, so a *runtime*
`icon-name: string` could never resolve to a path through it regardless
of how the binding was written. Fixed both using this project's already-
established pattern for exactly this problem (`overlays/
FloatingActionPanel.slint`, `feedback/AboutDialog.slint`): expose
`in property <image> icon-source;` and let the *caller* supply a literal
`@image-url(...)` per instance.

**Bug family 2 — components missing `@children`, so nested content
would silently overlap instead of stacking.** Confirmed via Slint's own
container-component docs: elements placed inside a component with no
`@children` become direct, un-laid-out children of that component's
root, not routed anywhere in particular. Found in three separate files
— `FlyoutMenu` (a "Content slot" that had no slot mechanism at all),
`NavCollapse` (same, for its expandable body), and `Drawer` (same, for
its "Content slot" comment). None of these were caught by a compile
error, since omitting `@children` isn't itself invalid — the component
just silently doesn't do what it visually implies it should. `NavCollapse`
needed a second pass: the first fix attempt put `@children` inside an
`if expanded: VerticalLayout { @children }`, which is itself a confirmed,
separately-documented Slint limitation (this project's own gotchas.md:
"`@children` cannot be inside a conditional element"). Redone with the
established workaround — keep the wrapper unconditional, animate its
height to 0 when collapsed.

**Bug family 3 — dead click handlers**, same shape as the
`cards/InteractiveCard` bug from the previous category: `TouchArea {
clicked => { } }` (empty body, no callback) in `TopAppBar`'s back
button and three items in `FloatingNavPanel`. A sneakier variant turned
up in `EllipsisBreadcrumb`: the "•••" expand button's handler was
`clicked => { /* expand all */ }` — a comment describing what it should
do, standing in for code that was never written. Grepped the whole
category for both the empty-body and comment-only-body shapes after
finding the first instance of each, to make sure this was fully caught
rather than fixed once and assumed done.

**Bug family 4 — mixed-type ternaries and concatenation**, flagged
defensively rather than confirmed broken (no live compiler here to
check against, and Slint's own docs are explicit that int/float convert
implicitly to string in general — but relying on that inside a ternary
or a chained `+` where the *other* operand's type varies by position is
an unnecessary risk to leave in, especially now that this project has
working examples of the safe alternative everywhere). Found and
rewritten to explicit string interpolation: `NavBadge` (`root.count > 99
? "99+" : root.count` — string branch vs. int branch), `Steps`/
`HorizontalStepper`/`VerticalStepper` (identical shape, `"✓" : (i + 1)`,
in all three stepper components), and `PrevNextPagination`
(`root.current + " / " + root.total`, int leading a `+` chain into a
string literal).

**One-off, high-value fixes:**
- `FloatingNavPanel` — `visible: root.visible;`, a property bound to
  itself. This is a tautology (reads back whatever `visible`'s own
  default already is), not a way to expose visibility control — looks
  like a copy/paste mistake for what should have been a separate `open`
  property. The component also had no data property at all (hardcoded
  "Dashboard"/"Projects"/"Settings") and all three items were dead taps
  (family 3 above). Rewrote as data-driven with a real `open` property
  and working `item-clicked` callback.
- `CommandPalette` — `TextInput { text: root.query; }`, a one-way
  binding. This project's own gotchas.md already documents this exact
  failure mode ("a persistent one-way binding keeps re-deriving
  input.text from root.text, which never changes as the user types —
  can fight typing") from a previous category; this is the first time
  it's actually turned up live. Made `query` two-way (`in-out`) and
  switched to `<=>`.
- `ScrollableTabs` — no scrolling mechanism at all (no `Flickable`)
  despite the name, and no way to pass more than the three hardcoded
  tabs it shipped with. Made it data-driven (`[string] tabs`, matching
  the sibling `TabBar` component's own convention) and wrapped the row
  in a `Flickable` with `viewport-width` bound to the row's preferred
  width, so it now actually scrolls once tabs overflow.
- `Taskbar` — `system-tray-icons` was declared and accepted but never
  rendered anywhere in the component body; the system tray was simply
  absent regardless of what was passed in. Added a slot for it.
- `Dock`/`Taskbar` — default icon values were full-color pictographic
  emoji (🏠📁🌐✉️🎵🔊📶🔋), which render as broken tofu glyphs in this
  environment (same issue found and fixed across all of `cards/` last
  session). Swapped defaults for simple BMP symbols, confirmed safe.
  `Taskbar`'s own logo was a 🦞 (lobster) — also broken for the same
  reason, and also a mismatch for a project called "Lion Toolkit"
  regardless; replaced with the FA "fish" icon this project already
  established as its brand mark elsewhere (`media/LogoMark.slint`), for
  consistency rather than guessing at a different mark.

**Reviewed and left as-is, not bugs:** `NestedTabs` (fully wired, just
not parameterized with arrays — a legitimate static demo, consistent
with how this project already treats a few structural/diagram-style
components in other categories); `Pagination`'s `for page[i] in
root.total-pages` (dual-variable binding over a plain `int`, initially
looked unusual but is a confirmed, already-established pattern
elsewhere in this project — `data-display/ActivityFeed.slint` and
`data-display/JsonTreeViewer.slint` both do the same over an
`item-count`); `Drawer`'s `x: { if !open {...} else {...} }` (a
statement-block property binding — checked against Slint's own docs,
which confirm a binding can be "an expression or a block," before
assuming this was the *other*, actually-unconfirmed shape this
project's gotchas.md flags: an inline `if/else` used as a sub-expression
mid-binding. Different thing; this one's fine.); `MegaMenu`'s
`self.visible`-driven height/opacity (unlike `FloatingNavPanel`'s bug,
this ties into the real builtin `visible` property, which a caller can
validly set externally — not a self-reference tautology).

## Post-delivery fix: `navigation/FlyoutMenu.slint` — id declared inside a conditional referenced from outside its scope

The person's VS Code Problems panel caught two real compiler errors:
"Cannot access id 'ta'" at the `background:` binding. Cause: the
previous fix for `FlyoutMenuItem`'s dead click handler declared its
`TouchArea` as `if !show-separator: ta := TouchArea { ... }` — but
`background:` (declared earlier in the same component, unconditionally)
references `ta.pressed`/`ta.has-hover` to drive hover/press feedback.
An id declared inside a conditional element isn't in scope for bindings
outside that same conditional — a real, structural error, not caught by
the earlier read-through since nothing in this repo actually compiles
`.slint` outside the person's own VS Code Problems panel.

**Fixed by making the TouchArea unconditional** and using
`enabled: !show-separator;` instead of wrapping it in `if` — `ta` is now
always in scope, and separator rows just don't respond to hover/press
since their TouchArea is disabled rather than absent.

Scanned `navigation/`, `cards/`, and `charts/` afterward for the same
shape (an id declared inside `if cond: id := Element` and referenced via
`id.property` anywhere in the file) — isolated to this one file.
**New standing check:** an id declared inside a conditional (`if`)
element is only in scope within that same conditional block — if a
binding elsewhere in the component needs to reference that id, either
move the reference inside the same conditional, or make the element
unconditional and use `enabled:`/`opacity:`/similar instead of `if` to
express the "sometimes present" behavior.

## Post-delivery fix round 2: `navigation/` — layout-stacking overlaps, a data-less LaunchpadGrid, and confirmation that even "safe" text glyphs aren't safe here

Three live screenshots of the delivered category caught real bugs the
first pass missed — none were compiler errors (Problems panel showed
0/0), all purely visual.

**`CollapsingHeader` and `StickyHeader` rendered overlapping each
other.** Cause: `CollapsingHeader`'s root `Rectangle` had no explicit
`height` at all — unlike `StickyHeader`, which correctly sets `height:
48px;` on its own root. Its actual content lived entirely inside
`if expanded:`/`if !expanded:` conditional children, each with their
own fixed height, but that doesn't propagate up to size the parent.
Inside the `VerticalLayout` this category's test.slint uses to stack
`TopAppBar`/`StickyHeader`/`CollapsingHeader`, that meant
`CollapsingHeader` was allocated *zero* height — its content still
rendered at full size, just overflowing out of a zero-height box and
visually landing on top of the sibling above it. Fixed by binding the
root's height explicitly to whichever variant is showing
(`root.expanded ? 100px : 48px`), with an `animate` on it since the
value now genuinely changes.

Ran a heuristic scan afterward for the same shape (a `Rectangle`-rooted
component with no `height`/`vertical-stretch` at the top level, whose
only real content lives inside conditional children) across the rest of
the category. Two more matched the pattern (`Drawer`, `Steps`,
`VerticalStepper`) but turned out to be false positives on inspection —
`Drawer` is designed to fill an already-sized parent directly (confirmed
correct in the same screenshots), and `Steps`/`VerticalStepper` both
have substantial *unconditional* layout content sizing them correctly,
the scan just also noticed their (harmless) conditional connector-line
decorations.

**`Drawer`'s "Menu" header text rendered jumbled together with its nav
items instead of appearing above them.** Cause: the header `Rectangle`,
the 1px divider `Rectangle`, and `content-layout` (the `@children` slot
fixed in the previous round) were three separate direct children of
`panel` with no enclosing layout to stack them. Per Slint's "containers
fill their parent by default" rule, all three defaulted to filling the
*entire* panel and completely overlapped. Wrapped all three in a
`VerticalLayout` so they stack top-to-bottom as the "Header" / "Divider"
/ "Content slot" comments always implied they should.

**`LaunchpadGrid` had no per-app data at all** — just a `count`
producing that many identical tiles, each hardcoded to the literal text
"App" with an empty icon square. Added real `icons: [image]` and
`names: [string]` properties (`count` now defaults to `icons.length`,
still overridable) so each tile actually shows a distinct app.

**Confirmed — the previous round's "safe BMP symbol" fix for the
tofu-glyph bug was itself wrong.** The first round (delivered, not yet
screenshotted) swapped broken pictographic emoji (🏠📁✉️) for simple
geometric/dingbat Unicode symbols (◉▤◈✉♪⚙▮▲▭), reasoning these were
common enough to be safe. A live screenshot proved otherwise — every one
of those still rendered as the same broken tofu glyph. The only glyphs
confirmed actually rendering correctly across all three screenshots were
ordinary ASCII/near-ASCII: `✓` (checkmark), `✕`/`×` (multiplication
sign), `‹`/`›` (angle quotes) — all already in use elsewhere in this
category (steppers, TabButton, pagination) — plus, critically, every
*real FA SVG image* icon already in use (`BottomNavBar`, `BottomNavRail`,
`MiniSidebar`, `TopAppBar`, `IconStepper`) rendered perfectly. The actual
rule isn't "BMP vs. emoji," it's "this environment's bundled font has
very little Unicode symbol coverage beyond basic punctuation — a
freeform `Text`-based icon slot is fundamentally unreliable here
regardless of which glyph block it draws from."

Given that, converted every remaining freeform-text icon property to a
real `image`, matching the pattern already established for NavItem/
NavBadge (and consistent with why that pattern exists — `@image-url()`
needing a compile-time literal is a *second*, independent reason beyond
the font-coverage one): `Dock.icons`, `Taskbar.pinned-apps`,
`FlyoutMenuItem.icon` → `icon-source`, `SidebarItem.icon` → `icon-source`.
`Taskbar.system-tray-icons` and `EllipsisBreadcrumb`'s "•••" button
stayed as plain text but had their glyphs swapped for guaranteed-safe
plain text/ASCII (`"Wi-Fi · 100%"`, `"..."`) rather than converted to
images, since neither is really an "icon" in the same sense. Updated
`test.slint` throughout to match every changed property.

This is a real API-breaking change for four components
(`Dock`/`Taskbar`/`FlyoutMenuItem`/`SidebarItem`) delivered in the
previous round — flagging clearly here since anyone who already
integrated against the first delivery's `[string]` icon properties will
need to switch to `[image]`/`icon-source`. Checked for other consumers
of all four inside this repo (`grep`, cross-category) before making the
change — none exist yet outside this category's own `test.slint`, so
this is a clean break, not a breaking change against real usage.

**New standing check, going forward:** don't trust a "safe" Unicode
glyph choice without a screenshot confirming it actually renders in
*this specific* Slint Preview environment — prefer real FA SVG icons
for anything icon-shaped from the start, and reserve plain `Text` glyphs
for cases already proven safe by an existing, working screenshot
(`✓ ✕ × ‹ › ...` and plain alphanumerics).

## `feedback/` batch — loading indicators that don't load, and two more instances of known bug patterns

All 36 components reviewed; 38 files delivered (36 components +
`export.slint` + `test.slint`, the latter substantially expanded — see
below). No `core/` changes needed.

**The headline bug in this category: loading/progress indicators whose
animation binding never actually animates.** This project's own
gotchas.md already documents the root cause ("Infinite animation loop
needs a nudge, not a static binding") from a prior category, but it
turned up *seven separate times* here, in every shape that pattern can
take:

- `PulseLoader` — `opacity: 0.4;` (static; the attached `animate`
  had nothing to transition between, so it never played at all).
- `IndeterminateProgress` — `x: -parent.width * 0.4;` (static; the bar
  sat permanently parked off the left edge instead of sweeping).
- `ProgressBar`'s indeterminate mode — `x: indeterminate ? parent.width
  * 0.3 : 0px;` (this one *did* animate once, when `indeterminate`
  itself toggled — but then sat still forever after, since nothing else
  ever changed `x` again).
- `StripedProgress` — all five `stripeN.x` bindings were static
  literals with `animate x` attached; the "moving stripes" barber-pole
  effect never moved.
- `BusyIndicator` — no animation at all (a refresh-icon glyph that
  never rotates isn't "busy"); also had genuinely dead code, an
  `opacity: running ? 1.0 : 0.3;` toggle that could never matter since
  the root already has `visible: running;` hiding the whole thing.
- `WaveLoader` — three bars at completely fixed heights, no animation.
- `ConfettiBurst` — 20 particles at fixed positions, no animation —
  a "burst" that never bursts.

Fixed all seven with the confirmed nudge-and-repeat technique (an
internal property that gets bumped once via `init =>`, with `animate
... { iteration-count: -1; }` on it), plus `animation-tick()` for
`WaveLoader` (Slint's own sanctioned mechanism for continuously-running
animations, confirmed via docs — the same idiom this category's
`DotsLoader` already used correctly, which is why it wasn't in the list
above). `ConfettiBurst` intentionally plays once per mount rather than
looping, since a burst is a one-shot trigger by nature, not an ambient
loop — noted in its header comment that re-mounting (e.g. behind an
`if`) is how a host replays it.

**Two more instances of the "fake progress" disease from charts/:**
`CircularProgress` and `IndeterminateSpinner` both drew a full, complete
ring regardless of `value`/state — `CircularProgress`'s `value`/`max`
only toggled the second ring's *visibility*, never its extent. Fixed
both with the conic-gradient arc technique from charts/
(GaugeChart/ProgressRingChart), now a third time reused in this project.
Deliberately did *not* put the gradient directly on `border-color`
(which is brush-typed and would be the more natural fit) — Slint's
software renderer has a confirmed open bug where gradients on a
bordered Rectangle don't render (slint-ui/slint#6225) — used the
background-fill-plus-punch-out-circle technique instead, which needs to
know the surrounding surface color (exposed as `backdrop-color`,
overridable per instance) but sidesteps that renderer issue entirely.

**Other real, standalone bugs:**
- `EmptyState` — `callback action();` was declared but nothing in the
  component could ever trigger it: no button, no `TouchArea`
  referencing it anywhere. Added an actual action button (shown only
  when the host supplies `action-label`, so a host that doesn't want
  one can still omit it).
- `NoResultsState` — `"No results for \"{root.query}\""` — missing the
  backslash Slint interpolation requires (confirmed against this
  project's own gotchas.md: "Literal `{name}` shows up in the UI (no
  diagnostic)"). Would have displayed the literal text `{root.query}`
  instead of the actual search term. Fixed to `\{root.query}`.
- `OfflineState` — referenced `wifi-slash.svg`, which doesn't exist in
  this project's bundled FontAwesome set (confirmed against the actual
  file listing — a missing-asset bug, not a rendering one). Swapped for
  `plug-circle-xmark`, a real icon with the same "disconnected" meaning.
- `Snackbar` and `ActionToast` — both had an action-button `Rectangle`
  with no explicit width, centering its label via x/y (excluded from
  Slint's preferred-size computation — the same badge-sizing bug found
  repeatedly across `cards/` and `navigation/`). With
  `horizontal-stretch: 0`, both buttons had 0px preferred width and
  were invisible. `ErrorState`'s "Retry" button had the identical bug,
  compounded by sitting inside a `VerticalLayout { alignment: center;
  }` (the alignment-vs-stretch issue from charts/, reinforcing the same
  failure). All three fixed by sizing from real layout content instead.
- `AboutDialog` and `Dialog` — both ended with a stray `ta := TouchArea
  { }`, completely empty and never referenced anywhere in either file.
  Removed as dead code.
- `Alert`/`Notification` — used "ℹ" (U+2139) for the info-kind icon.
  Not confirmed broken by any screenshot, but given this category's own
  established caution (only trust glyphs a screenshot has actually
  confirmed — see the tofu-glyph correction from `navigation/`),
  swapped preemptively for plain ASCII "i", which needs no such
  confirmation at all.

**Gap, not a bug — added anyway:** every skeleton-loader primitive
(`Skeleton`, `SkeletonText`, `SkeletonHeading`, `SkeletonAvatar`,
`SkeletonImage`, `SkeletonTableRow`'s cells, `SkeletonCard`'s image
placeholder) was a static gray box with no animation at all. A skeleton
screen's entire purpose is signaling "loading" through motion — a
static block just reads as a gray box, not a loading state. Added the
same pulsing-opacity nudge animation used for the loader fixes above to
all of them. The composite skeletons (`SkeletonParagraph`,
`SkeletonListItem`, most of `SkeletonCard`) inherit this for free since
they're built from the leaf components.

**test.slint coverage:** arrived demonstrating 7 of 36 components — the
largest gap of any category so far. Expanded to cover all 36, including
live demos of every fix above (`EmptyState` with `action-label` set,
`NoResultsState` with a real `query`, etc.).

## Post-delivery fix: `feedback/CircularProgress.slint` — same `max` shadowing mistake as charts/GaugeChart, missed this time

The person's VS Code Problems panel caught a real compiler error: "The
expression is not a function" at line 23. Identical root cause to the
`charts/GaugeChart.slint` fix from an earlier round: `in property
<float> max: 100;` shadows the global `max()` function for the rest of
the component, breaking the `max(0.0, min(1.0, ...))` call a few lines
later. This project's own gotchas.md already documents this exact
pattern as a standing check ("never name a property `max`, `min`,
`abs`, `round`, `floor`, `ceil`, `mod`...") — it just wasn't re-checked
against `CircularProgress` specifically when that file was rewritten
during the `feedback/` pass, since the rewrite was modeled on
`GaugeChart`'s *fixed* version but the property name itself wasn't
re-verified against the checklist at the end.

**Fixed the same way**: renamed to `max-value`, updated its one internal
reference. No other file in `feedback/` (checked via grep) has the same
shadowing shape, and `test.slint`'s one `CircularProgress { value: 68;
}` usage doesn't reference `max` at all, so no cascading changes needed.

## `typography/` batch — mostly clean presets, but several components could never actually be reused with different content

All 36 components reviewed; 38 files delivered (36 components +
`export.slint`, untouched + `test.slint`, unchanged — it already
covered all 36 components with bare `{}` instantiations, and every one
still compiles and renders correctly against the new properties added
below since all defaults were preserved). No `core/` changes needed.

This category was the cleanest so far — 26 of the 36 components are
simple `inherits Text` style presets (`DisplayLG`, `HeadingH1`,
`BodyMD`, `LabelSM`, etc.) that need no fix at all: since they inherit
`Text` directly, the builtin `text` property is always overridable by
any caller (`DisplayLG { text: "My Heading"; }`), so a hardcoded default
string isn't a reusability bug the way it would be for a composite
component — it's just a sensible default, exactly like this project's
other `Text`-inheriting presets already established as a working
convention.

**The real, recurring bug in this category**: the other ten
components — the composite ones that inherit `Rectangle` and wrap one
or more `Text` children (`InlineCodeSpan`, `RunningText`,
`NumericTabular`, `DropCap`, `PullQuote`, `Blockquote`, `CodeBlock`) —
had their actual content hardcoded with **no exposed property to
override any of it**. Because they inherit `Rectangle`, not `Text`,
there's no builtin `text` property a caller could reach for the way
there is with the presets above; nothing was exposed in its place. A
"CodeBlock" component that can only ever show one specific four-line
Rust snippet, or a "PullQuote" that can only ever quote Charles Eames,
isn't really reusable — it's a screenshot. Fixed all seven by adding
explicit `in property <string>` (or `<[string]>`) overrides for every
piece of real content, while keeping the existing hardcoded values as
defaults so nothing about the current demo changes.

`DropCap` needed a slightly different shape than the rest: a drop cap
fundamentally requires the first letter split from the rest of the
paragraph, and Slint strings have no indexing/substring (confirmed
elsewhere in this project's own gotchas.md), so that split can't happen
inside `.slint` at all. Exposed it as two separate properties
(`drop-letter`, `rest-of-text`) for the host to pre-split — the same
pattern already established for `TeamMemberCard`/`ContactCard`'s avatar
`initials`.

**Other real, one-off bugs:**
- `Caption` — default text was corrupted: English grafted mid-word onto
  a Chinese clause with no space or punctuation between them
  ("Caption — small辅助文字用于图片说明、时间戳等。"), reading as
  nonsense in both languages. Every other component in this category
  has a clean, plain-English default — this reads like an encoding or
  copy-paste accident, not intentional bilingual content. Fixed to
  plain English.
- `HighlightMark` — inherited `Text` directly and only set color/font
  properties, so it couldn't actually highlight anything: confirmed
  against Slint's own `Text` docs that `Text` has no `background`
  property at all (that's `Rectangle`-only), so a "highlighted mark"
  span genuinely cannot be built as a bare `Text`. This is the one
  component in the category that couldn't follow the
  "inherits `Text` directly" convention every sibling preset uses, for
  exactly that reason — restructured to wrap a `Text` inside a
  `Rectangle` with a real background fill.

**Checked and not a bug:** `PullQuote`/`Blockquote`'s real, accurately-
attributed quotes from Charles Eames and Steve Jobs — both are
genuinely well-documented, correctly-attributed quotes (verified
against what's widely and consistently cited), used as ordinary demo
placeholder content in a private component library, not published
persuasive material — no misattribution concern here. Also checked
`CodeBlock`'s `println!("{x}")` line: the un-escaped `{x}` is *correct*
as written — it's Rust source text meant to display literally, not
Slint interpolation syntax, so it should NOT have a backslash the way a
real interpolation would.

## `media/` batch — a confirmation round more than a discovery round

All 32 components reviewed; 34 files delivered (32 components +
`export.slint`, unchanged + `test.slint`, updated — see below). No
`core/` changes needed. Unlike most previous batches, nothing here was a
*new* bug shape — every fix matched a pattern this project had already
confirmed and documented in an earlier category. Framing this
explicitly since it's a useful signal: the standing checklist below is
maturing into something that actually catches real bugs on the first
pass, not just after a person's screenshot.

**Real compile errors — underscore vs. dash, same shape as
`charts/GroupedBarChart`'s `series_b` bug:** `AvatarBadge`
(`root.badge_text` vs. the declared `badge-text`), `AvatarFallback`
(`root.text_color` vs. `text-color`), `IconDisplay`
(`root.icon_color` vs. `icon-color`). All three would have failed to
compile. (`LivestreamPill`'s `Theme.red_500` was *not* touched — that's
a real, correctly-named `Theme.slint` palette token, which legitimately
uses underscores for its primitive-palette entries; checked against
`core/Theme.slint` before assuming it was the same bug.)

**Hex alpha byte-order bug, the one flagged and deferred from the
`cards/` round:** `LightboxViewer`'s `#cc000000` backdrop parses as
R=0xcc, G=B=0x00, **A=0x00** — fully transparent, so the modal backdrop
never actually darkened anything behind it. Fixed to `#000000cc` (~80%
opaque black), matching the byte order this project's own `Theme.slint`
already establishes (alpha last). This was noted as a likely instance
back in the `cards/` batch write-up specifically to check when `media/`
came up — confirmed and fixed now.

**Pill-radius-renders-as-ellipse, rule 2 from
`SLINT-GOTCHAS-DISCOVERED.md`:** `LivestreamPill` (self-sizing width via
`self.preferred-width`, fixed 28px height — genuinely non-square) and
`AvatarBadge`'s notification badge (sized only by content, so a longer
`badge-text` like "99+" can exceed its own height) both used
`Theme.radius-full` on a non-square shape. Fixed both to
`self.height / 2`, exact by construction regardless of content width.

**Pictographic emoji tofu glyphs, same root cause as the `cards/`/
`navigation/` corrections — converted to real FA SVG icons:**
`CameraPreview` (camera emoji to `camera.svg`), `MapEmbed` (map emoji to
`map.svg`), `ModelViewer` (ice-cube emoji to `cube.svg`), `PodcastPlayer`
(microphone emoji to `microphone.svg`), `SvgIllustration` (palette emoji
to `palette.svg`). Also converted two components whose *entire purpose*
is displaying an arbitrary caller-supplied icon as freeform text —
`IconDisplay` (`icon: string`, defaulted to a star glyph) and
`AnimatedIcon` (`icon: string`, defaulted to a refresh-arrow glyph,
neither on the confirmed-safe list) — to the same
`in property <image> icon-source` pattern already established for
`NavItem`/`NavBadge`/`Dock`/`Taskbar`. This is the correct fix rather
than a narrower one for two independent reasons already on record: the
font-coverage problem, and `@image-url()` needing a compile-time literal
that a runtime string can't satisfy anyway. `IconDisplay`'s dead
`variant: string` property ("mono", never read anywhere in the component
body) was dropped rather than carried forward. `VideoPlayer`'s bare
play-triangle text glyph got the same treatment, converted to real
`FaIcon` play/pause images — bringing it in line with `AudioPlayer` in
the same category, which already used real icons correctly from the
start.

**Unwired animation — the "static binding, animate has nothing to
play" shape from `feedback/`'s loading-indicator round, found once
more:** `AnimatedIcon`'s `spinning: bool` was declared and accepted but
never actually connected to anything — the icon never rotated
regardless of its value. Fixed with the same confirmed nudge-and-repeat
technique (`init =>` bumps an internal `spin-angle` once, `animate`
with `iteration-count: -1` replays it forever), gated behind
`root.spinning` in the `transform-rotation` binding so the animation is
only visible when actually requested.

**Overlapping children + unwired configuration props, in the same two
files:** `ImageGallery` and `ImageMasonry` both declared `columns`/
`count` properties that the component body completely ignored — each
`for` loop was hardcoded to a literal `6` regardless of what was passed
in — *and* neither had any wrapping layout at all, so every generated
tile was an un-laid-out direct child of a plain `Rectangle` and fully
overlapped every other tile. Per this project's own precedent for the
`layout-containers/` masonry family, true variable-height masonry
packing isn't achievable in Slint (`GridLayout` normalizes every cell in
a row to the same height, and there's no way to introspect rendered
child size to balance columns) — rebuilt both on a real `GridLayout`
with `row:`/`col:` computed from the flattened loop index
(`Math.floor(i / root.columns)` / `Math.mod(i, root.columns)`), an
honest, documented reduction to uniform grid packing rather than a fake
uneven-height effect. `ImageMasonry` gained a `columns` property it
never had before, to match `ImageGallery`'s existing one.

**Dead click handlers, the `InteractiveCard`/`TopAppBar` shape again:**
`ImageCarousel`'s prev/next chevron buttons were empty `TouchArea {}`
elements — visually implying navigation that did nothing. Wired real
wrap-around index logic (`Math.mod(root.current - 1 + root.total,
root.total)` / `Math.mod(root.current + 1, root.total)`). Its dot
indicator was also hardcoded to `for idx in 5` regardless of
`root.total` — fixed to loop over the real property.
`VideoThumbnail`'s play button had the same empty-`TouchArea` shape;
added a real `play-clicked()` callback. `VideoPlayer`'s toolbar play
button fired its `play-toggle()` callback but never actually updated
`root.playing` (declared `in`, which can't be written internally
anyway) — changed to `in-out` and made the click handler toggle it
locally *and* fire the callback (the same dual approach already
established for `text-input/CameraCaptureInput`, so the demo stays
visibly interactive even though `test.slint` never wires custom
callbacks). Both the big center icon and the small toolbar icon now
actually swap between play/pause based on `root.playing`, instead of
showing a permanently-static glyph.

**Computed `in` property that should be `out`, the
`AddressInput.has-focus` shape again:** `AvatarGroupStack.display-count`
was declared `in` with a body computed entirely from `root.count`
(`Math.min(root.count, 4)`) — an `in` property that's also internally
computed can't coherently accept external writes, same semantic
conflict already fixed once in `text-input/`. Changed to `out`.

**`test.slint` gap:** `ImageMasonry` was imported but never actually
instantiated anywhere in the file — would have shipped with zero visual
or compile coverage. Added a real side-by-side `ImageGallery` /
`ImageMasonry` demo. Also enlarged `LightboxViewer`'s test invocation
(300x60 to 440x340) — the component centers a fixed 400x300 modal
inside its own bounds, and a 60px-tall host box put that math into
negative territory, guaranteeing overflow/clipping regardless of
whether the component itself was correct.

**Reviewed and left as-is:** `BeforeAfterSlider` and `ImageCropTool`
have no `TouchArea` at all despite names implying interactivity
("Slider", "Tool") — but per this project's own precedent (only fix an
*actually broken* promise, like an empty `TouchArea` that implies
interactivity through hover/press styling that goes nowhere), these
have no such half-finished promise: no hover state, no drag affordance,
nothing suggesting they should already respond to input. Adding real 2D
drag-repositioning to `ImageCropTool` in particular would need delta
tracking against `mouse-x`/`mouse-y` (never the frozen `pressed-x`/
`pressed-y`) without a live compiler to verify the math against — the
same reasoning `SignaturePad`'s freehand-stroke gap was left
undone for. Documented here as a good candidate for the person to
implement directly with `slint-viewer`/MCP-verified iteration, rather
than guessed at now.

## Post-delivery hotfix round — screenshot-driven, `media/` + cross-cutting

Triggered by two screenshots of the live `media/` Slint Preview render.
Per the established convention, these are individual-file hotfixes, not
a re-bundled zip.

**Confirmed via screenshot:** `BeforeAfterSlider`'s "Before" label was
visibly truncated to "Bef" right at the center divider. Root cause and
the general gotcha are written up in full in
`SLINT-GOTCHAS-DISCOVERED.md` — short version: the "Before" panel had
no `x:`, so it centered instead of anchoring left, which pushed its
right edge (and the second half of its own label) underneath the
"After" panel rendered on top of it. Fixed with `x: 0;`.

**Caught in my own code before it shipped further:** re-verifying
`VideoPlayer` (written earlier this same session) against the newly
understood mechanism turned up the identical bug in its own toolbar
progress bar — no `x:` on the fill, and the track/fill declared in the
wrong z-order on top of that. Fixed both.

**Found via project-wide grep prompted by the above:** the same exact
shape — a proportional-width free child with no `x:` — turned up in
`media/PodcastPlayer` (pre-existing, not something this session
touched before now) and in seven files across five categories not yet
reached in the normal workflow: `desktop2/SplashScreen`,
`indicators/HealthBar`, `range-value/Slider`,
`range-value/SteppedSlider`, `range-value/OpacitySlider`,
`social2/InChatPollWidget`, `utility/SuspenseBoundary`. All ten
confirmed instances are fixed now rather than deferred, since the fix
is a mechanical one-line addition (occasionally a declaration-order
swap) and there's no ambiguity once the shape is understood. Also
checked four look-alike "fill" components that turned out to already
be correct and were left untouched: `feedback/ProgressBar`,
`indicators/BatteryIndicator`, `range-value/RangeSlider`,
`range-value/VerticalSlider`.

**What this means for the status table above:** `desktop2/`,
`indicators/`, `range-value/`, `social2/`, and `utility/` are still
"Not started" as categories — this hotfix only resolved this one
specific bug shape in one file each. Each still needs its own full pass
(unwired properties, dead handlers, emoji, hex-alpha, the rest of the
standing checklist) when its turn comes up normally; the fix already
applied here should carry forward as done rather than being
rediscovered.

Files delivered this round, individually (not zipped):
`media/BeforeAfterSlider.slint`, `media/PodcastPlayer.slint`,
`media/VideoPlayer.slint`, `desktop2/SplashScreen.slint`,
`indicators/HealthBar.slint`, `range-value/Slider.slint`,
`range-value/SteppedSlider.slint`, `range-value/OpacitySlider.slint`,
`social2/InChatPollWidget.slint`, `utility/SuspenseBoundary.slint`.

## `mobile/` batch — the largest single-category fix count so far

All 30 components reviewed; 33 files delivered (30 components +
`export.slint`, unchanged + `test.slint`, updated). No `core/` changes
needed. 25 of the 30 component files needed a real fix — this category
skewed heavily toward "OS-chrome mockups with zero actual interactivity"
rather than subtle rendering issues.

**Underscore-vs-dash compile errors (2):** `HomeScreenWidget4x4`
(bare `photo_count` against the declared `photo-count`, not even
`root.`-qualified), `AppIconBadge` (`badge_count` used twice against
the declared `badge-count`).

**Pictographic emoji / unconfirmed-safe glyphs, converted to real FA
icons (11 files):** `HomeScreenWidget2x2` (gear), `LockScreenWidgetSmall`
(cloud-sun), `PullToRefresh` (down-arrow → chevron-down, see below),
`SplashLaunchScreen` (no literal "lion" icon exists in the FA set for
"Lion Toolkit" — used `layer-group` as a generic toolkit mark instead),
`StatusBarOverlay` (signal + battery), `AppClipMiniCard` (link),
`AppIconBadge` (mobile), `BiometricAuthPrompt` (face-smile /
fingerprint, switched on `auth-type`), `OnboardingPager` (per-page
icon array, converted `icons: [string]` → `icon-sources: [image]`,
same array-of-images technique as below), `ShareSheet` (per-app icon
array, same array-of-images conversion, plus a `box` fallback icon).
Left untouched: `LockScreenWidgetSmall.value`'s default `"72°"` — a
temperature *value* string, not an icon slot, and the degree sign is
basic Latin-1 (near-universal font coverage) rather than the
pictographic/symbol category this rule targets; no confirmed evidence
it's actually broken here the way the emoji were.

**Unwired animation, the `AnimatedIcon`/`spinning` shape from `media/`,
found twice more:** `PullToRefresh`'s "refreshing" state rendered a
hand-drawn `Path` arc that never actually rotated — also a second,
independent problem on its own per `icons-and-theming.md` ("don't
hand-draw glyphs as inline Path elements"). Replaced with a real FA
spinner icon and the confirmed nudge-and-repeat rotation technique.
`DynamicIslandSlot`/`FloatingIslandPill` had real width/height changes
on `expanded` but no `animate` at all, so the resize snapped instantly —
added `animate width, height` with a spring easing, since the animated
morph is this component's entire reason for existing.

**Components with literally zero interactivity despite the name
promising it — the most common issue this round.** Three files
(`SwipeToAction`, `SwipeToArchive`, `SwipeToDelete`) had no `TouchArea`
anywhere at all — the hidden action button was permanently unreachable.
Added real drag-to-reveal: a `TouchArea` on the *fixed* root tracking
`mouse-x` (not on the `front` layer being dragged — that would have its
own local `mouse-x` chase its own movement and never produce a usable
delta), clamped via `.clamp()` (the confirmed method form, not assumed
`Math.clamp(...)`), committing past a 40px threshold via a
`pointer-event` up handler rather than relying on unconfirmed
click-after-drag semantics. `SwipeToAction`'s `action-color-name`
property was also declared and never read anywhere — wired to a real
color switch. Five more sheet/picker components
(`ActionSheetDestructive`, `LongPressActionSheet`, `ContactPicker`,
`AppClipMiniCard`, `BiometricAuthPrompt`, `OnboardingPager`,
`PhotoMediaPicker`, `ShareSheet` — eight, not five) had rows, buttons,
or cancel affordances with no `TouchArea` at all; added real callbacks
(`action-selected`, `cancelled`, `contact-selected`, `type-selected`,
`opened`, `app-selected`, `photo-selected`, `completed`, etc.) plus
`has-hover`-based background feedback where already touching that
Rectangle's background.

**Overlapping children, the missing-layout shape:**
`BottomSheetMultiSnap` and `HalfSheetModal` both had a drag-handle row
and a conditional title `Text` as two free (non-layout) children of the
same parent — both default-center per the newly-confirmed rule from the
`media/` round, landing directly on top of each other. Wrapped both in
a `VerticalLayout`.

**Declared-and-ignored configuration properties, worst instance yet:**
`PhotoMediaPicker` declared `grid-columns`/`grid-rows`/`spacing` and its
body was just one blank placeholder `Rectangle` — none of the three
properties were used anywhere, and there was no actual photo grid at
all. Rebuilt on a real `GridLayout` with `row:`/`col:` from the
flattened index, `spacing` applied via `* 1px` (it's declared as an
`int`, not a `length`), and a `photo-selected(int)` callback per cell.

**Silently-ignored `padding-left` on a bare `Text`, a confirmed core
gotcha (`padding only has effect on layout elements`) hit for real:**
`DocumentFilePicker`'s "Select File" header had `padding-left` directly
on a `Text` with no enclosing layout — no visual effect at all. Wrapped
in a `HorizontalLayout`. Same file also had `vertical-stretch: 1`
instead of `horizontal-stretch: 1` on its list-row label — a
plausible axis-name typo — meaning the trailing "›" chevron never
actually got pushed to the row's right edge; fixed the axis.

**Bunched, uncentered header title (no stretch, no spacer), in two
files:** `ContactPicker` ("Cancel"/"Contacts") and `PhotoMediaPicker`
("Cancel"/"Photo Library"/"Done") both used a plain `alignment: center`
`HorizontalLayout` with no stretch factors, which centers the whole
row of texts *as a group* rather than positioning Cancel left / title
center. Fixed with `horizontal-stretch: 1` + `horizontal-alignment:
center` on the title; `ContactPicker` additionally gained an invisible
same-text ghost spacer on the right so the title centers on the *whole*
bar rather than just the leftover space after Cancel.

**Completely inert components, fixed to actually do their one job:**
`HapticFeedbackWrapper` had `enabled`/`feedback-type` properties and no
callback at all — impossible to ever invoke. Reconsidered its design:
since a wrapping wrapper with its own `TouchArea` would block touches
from reaching whatever real interactive content it wrapped, the correct
shape is a callable *service* a sibling invokes (`callback trigger()`),
not a `@children`-forwarding wrapper. `ReachabilityHelper` had the
opposite problem — it's a pure positioning wrapper with no competing
touch-capture concern, so its bug really was a missing `@children`
(wrapping any real screen content in it made that content vanish,
since nothing was ever forwarded); added `@children` plus an `animate`
on its existing `y` shift.

**`test.slint` gaps:** `HapticFeedbackWrapper`, `ReachabilityHelper`,
and `DocumentFilePicker` were all imported/exported but never actually
instantiated anywhere in the test file. Added real demos for all three
(the haptic demo shows the intended sibling-invokes-callback usage
pattern directly). Also switched the `Flickable`'s hardcoded
`viewport-height: 3600px` to `content.preferred-height` (the pattern
`core/test.slint` already uses) — a hardcoded guess that this round's
additions would have made stale immediately, cutting off the last
section from ever being reachable by scrolling.

**Reviewed and left as-is:** `SafeAreaBottomSpacer`, `SafeAreaTopSpacer`,
`SwipeNavigationGestureArea` — genuinely just static spacers/affordances
with nothing to wire up. `HomeScreenWidget2x4`'s `VerticalLayout` combines
an explicit `y: 16px` with `vertical-stretch: 1` as a direct
`HorizontalLayout` child; flagged as a *possible* subtle interaction
between an explicit cross-axis offset and stretch sizing, but left
untouched rather than guessed at — not confirmed against the docs, and
lower confidence than everything else fixed this round.

## `mobile/` post-delivery hotfix round — screenshot-driven

Triggered by three screenshots of the live render. Several were fixes
I should have caught during the original pass rather than after —
noted plainly rather than glossed over.

**Wrong Theme token, not a typo family I'd seen before:**
`DynamicIslandSlot` and `FloatingIslandPill` both used
`background: Theme.text-primary` — that token is near-white in dark
mode (it's meant for text readable on a dark surface), so both
rendered as blank white pills with no visible content at all. Fixed to
a fixed near-black (`#0a0a0c`), not tied to `Theme.dark-mode` at all,
since a Dynamic Island is a physical hardware cutout, not part of the
app's theme. Checked `SwipeNavigationGestureArea`'s identical-looking
`background: Theme.text-primary` and left it alone — that one really
does want to track the current theme (a home-indicator bar visible
against whatever surface it sits on), so it's correct as written.

**`radius-full` on a non-square shape — rule 2 from
`SLINT-GOTCHAS-DISCOVERED.md`, missed on first pass in five of my own
new components this round:** `DynamicIslandSlot`, `FloatingIslandPill`,
`AppClipMiniCard`'s Open button, `BiometricAuthPrompt`'s Cancel button,
`OnboardingPager`'s Next button — all non-square, all rendering as
ellipses instead of flat-sided pills. This is a checklist item I
personally added to the gotchas file after `media/`'s `LivestreamPill`
and clearly didn't apply to my own new code here; fixed all five to
`self.height / 2`.

**`AppIconBadge`'s notification badge had no explicit `width`,** only
`height` — relying on ambiguous default sizing gave an oversized,
overflowing badge that bled into neighboring icons in the same row
(and made the number look clipped). Gave it an explicit,
content-aware width instead of leaning on any assumption about
default-fill behavior.

**`horizontal-stretch: 1` directly on a `Text` didn't reliably widen its
box** — `ContactPicker`'s and `PhotoMediaPicker`'s headers rendered
"Cancel"/title/spacer completely concatenated with no separation at
all, not stretched-and-centered as intended. `DocumentFilePicker`'s
very similar-looking per-row use of the same property (pushing a
trailing chevron right) rendered correctly, so this isn't "stretch
never works on Text" — likely something about combining stretch with
`horizontal-alignment: center` specifically, not confirmed further.
Sidestepped the ambiguity entirely: wrapped the title `Text` in a
plain `Rectangle { horizontal-stretch: 1; }` (Rectangle's stretch
behavior is unambiguous, confirmed dozens of times this whole project)
and centered the `Text` inside it with the standard `x:
(parent.width - self.width) / 2` technique instead of relying on
`horizontal-alignment`.

**`SwipeToDelete`/`SwipeToArchive`/`SwipeToAction` rendered as nothing
at all** — an empty gap where the whole "Swipe Rows" section should
be. None of the three components had an explicit `height`, and
neither did any of their `test.slint` invocations; inside a
`VerticalLayout`, an unconstrained `Rectangle` with no content-based
sizing of its own has nothing to size itself from and collapses.
Added `height: 64px` to all three as a sensible default row height,
fixing it regardless of what a future caller does or doesn't specify.

**`test.slint` improvement:** the Dynamic Island / Floating Pill demo
always passed a fixed `width: 120px` override, so it never actually
exercised `expanded`/`content` or the animated resize at all — the
whole point of these two components. Changed to show one collapsed
(`DynamicIslandSlot`, defaults) and one expanded with real content
(`FloatingIslandPill { expanded: true; content: "2:34 remaining"; }`),
so the fix is now actually visible in a static render rather than
hidden behind an override that made both look identical either way.

Files touched this round: `DynamicIslandSlot.slint`,
`FloatingIslandPill.slint`, `AppClipMiniCard.slint`,
`BiometricAuthPrompt.slint`, `OnboardingPager.slint`,
`AppIconBadge.slint`, `ContactPicker.slint`, `PhotoMediaPicker.slint`,
`SwipeToDelete.slint`, `SwipeToArchive.slint`, `SwipeToAction.slint`,
`test.slint`.

## `forms2/` batch — the biggest structural finding of the whole project

All 30 components reviewed; 33 files delivered (30 components +
`export.slint`, unchanged + `test.slint`, heavily expanded). No `core/`
changes needed. 23 of 30 files needed a fix, and the dominant issue
here wasn't a small styling slip — it was foundational.

**Every single layout-wrapper primitive in this category was missing
`@children`, and `test.slint` itself was already trying to nest real
content inside several of them.** `FormRoot`, `FormGroup`,
`FormSection`, `FormRow`, `FieldWrapper`, `Fieldset`,
`HorizontalLabelForm`, `StackedForm` — eight components whose entire
purpose is wrapping arbitrary form content — had a `VerticalLayout`/
`HorizontalLayout` with nothing forwarded into it at all. Since
`test.slint`'s own "Form Anatomy" demo already nested a
`FormGroup`/`Rectangle`/`FieldHelperText` tree inside a `FormSection`
inside a `FormRoot`, this wasn't a hypothetical risk — that entire demo
section would have rendered as just two header lines and nothing else,
with every field silently discarded. This is the same missing-`@children`
shape as `mobile/HapticFeedbackWrapper`/`ReachabilityHelper`, just at a
much larger scale — every one of these needed a wrapper, not a service,
so `@children` was unambiguously the right fix for all eight (unlike
those two mobile components, none of these have their own `TouchArea`
competing for touches).

**The identical `Math.mod(question + 1, 10)` bug, copy-pasted into three
separate files:** `DynamicFieldArray`, `QuizForm`, `SurveyForm` all
showed "Item"/"Question" labels via `Math.mod(current + 1, 10)` — for
any 10th (or 10-item) entry this wraps to 0, showing "Item 0"/"Question
0" instead of 10. There's no reason for a modulo here at all; removed
it in all three.

**Static, non-editable text standing in for form fields, everywhere.**
`LoginForm`, `RegistrationForm`, `PasswordResetForm`, `PaymentForm`,
`AddressForm`, `ContactForm`, `ProfileEditForm` all rendered every field
as a bordered box containing plain `Text` — styled exactly like an
input, but with nothing behind it a person could actually type into.
Converted every field across all seven to a real `TextInput` (with a
placeholder `Text` shown only while empty, and `has-focus`-driven
border-color for real focus feedback), and wired every Cancel/Submit/
Save/Send button with a real callback carrying the field values.
Password fields are real and typable but intentionally not masked —
this project hasn't confirmed Slint's password `input-type` against a
live compiler, so guessing at an unconfirmed enum wasn't worth the risk
versus a plain, working field; switched the empty-state placeholder
character from `•••••••• ` to `********` too, since bullet isn't on the
confirmed-safe glyph list and asterisk is plain ASCII.

**`padding-left`/`padding-top` silently ignored on a bare `Text`, the
confirmed `gotchas.md` rule hit for real, repeatedly:**
`ProfileEditForm`'s First Name/Last Name/Email/Bio fields all had
`padding-left`/`padding-top` directly on a `Text` outside any layout —
no visual effect at all. Fixed as part of the `TextInput` conversion
above (the input now insets via `x: Theme.sp-3`, a binding rather than
a layout `padding` property, so it isn't subject to this gotcha at all).

**Unselectable options in both quiz-style components:** `QuizForm`'s
four answer buttons and `SurveyForm`'s five satisfaction-level buttons
had no `TouchArea` at all. Wired both with real selection, correct/
incorrect highlighting (`QuizForm`, against a new `correct-option`
property), and score/progress tracking. Also caught a subtler bug while
wiring `QuizForm`: an early version set `selected-option` and then
immediately reset it to advance the question, all inside one click
handler — since Slint only re-renders after a callback finishes, not
between statements inside it, the correct/incorrect flash would never
actually have been visible. Fixed with a `Timer` (`interval: 700ms;
running: root.answered;`) that delays the actual advance, so the
feedback has time to render first. Both components' `current-question`/
`score`/`completed` were declared `in` (not `in-out`), which would have
made this impossible regardless — an `in` property can't be written
from inside its own component — so both were promoted to `in-out` as
part of the same fix. Neither component had a `total-questions`
property before; `SurveyForm` hardcoded "of 5" and a fixed 20% progress
width regardless of the actual question count, `QuizForm` hardcoded "of
10" the same way — both now take a real `total-questions`/
`total-steps`-style property and compute the label and progress width
from it.

**Unwired configuration properties, but a different shape than "should
be fixed" — flagged instead as intentional, caller-facing config
surface, matching the `WizardForm.branching` precedent from `mobile/`:**
`HorizontalLabelForm`'s `label-width`/`gap` and `FormRow`'s `columns`/
`equal-width` are declared and never read internally, but there's no
way for a `@children`-forwarding wrapper to reach into its own opaque
forwarded children and assign them a width or a grid column — the
*caller's* own children have to reference the wrapper by its own id
(`hlf.label-width`) to use these meaningfully. Demonstrated that exact
pattern in the new `test.slint` coverage for both rather than inventing
internal logic that isn't achievable in Slint as written.

**Emoji / unconfirmed-safe glyphs, converted to real FA icons:**
`PasswordResetForm` (key emoji → `key.svg`), `PaymentForm` (lock emoji →
`lock.svg`), `QuizForm` (party emoji → `trophy.svg`, matching the
"complete" moment rather than a literal party icon), `SearchFilterForm`
(magnifying-glass emoji → `magnifying-glass.svg`, plus three `▾`
down-triangles across its filter pills → `chevron-down.svg`).
`AddressForm`'s "United States ▾" got the same chevron-down treatment.

**A near-miss caught before it shipped:** an early pass at making
"Forgot?" and "Sign up" tappable in `LoginForm` put a bare `TouchArea`
as a *sibling* after the `Text`, inside a `HorizontalLayout` — with no
content of its own to derive a preferred size from, that TouchArea
risked collapsing to zero width and never actually being clickable.
Reconsidered before shipping and restructured so the `TouchArea` wraps
the clickable `Text` as its own child instead (its size then derives
from that child, which is standard, reliable `Text`-in-a-layout
sizing), and re-audited every other `TouchArea` written this round to
confirm none of them share that shape — the AddressForm/SearchFilterForm
ones are all direct children of a plain `Rectangle` (which fills it
entirely by default), not layout siblings, so they're fine as written.

**`test.slint` gaps:** nine exported components — `FieldWrapper`,
`FormRow`, `HorizontalLabelForm`, `InlineForm`, `ProfileEditForm`,
`QuizForm`, `SearchFilterForm`, `StackedForm`, `WizardForm` — were
never actually instantiated anywhere in the file. Added real demos for
all nine.

**Reviewed and left as-is:** `MultiStepForm` was already fully and
correctly wired (progress dots, step titles, Back/Next with real
`current-step` updates) — no changes needed. `WizardForm`'s `branching`/
`branch-step` properties are pure host-facing configuration by design,
same reasoning as `mobile/HapticFeedbackWrapper`'s properties — a
subclass can't intercept a `TouchArea` callback body defined in its
base component, so the realistic, sound design is exposing these for
the host to read and act on (`current-step` is already `in-out`,
so external code can already override it after any click), not
something to rearchitect via an unconfirmed inheritance-override
mechanism.

## `social2/` batch — the `Math.mod` bug recurs a 4th and 5th time

All 26 components reviewed; 28 files delivered (26 components +
`export.slint`, unchanged + `test.slint`, expanded). No `core/` changes
needed beyond the earlier cross-cutting fix already applied to
`InChatPollWidget` (this round gave that file its first full pass —
previously it only got the one targeted fractional-width fix during
`media/`'s cross-cutting hotfix). 20 of 26 files needed a fix.

**The `Math.mod(x, N)` bug from `forms2/`, twice more:**
`LeaderboardRow` showed `Math.mod(rank, 10)` — rank 10 would show as
"0", rank 11 as "1" (indistinguishable from actual rank 1). Added a
`rank: 10` case to `test.slint` specifically to exercise this.
`NotificationBellDropdown` showed `Math.mod(notification-count, 100)` —
same shape, now capped honestly at "99+" instead (matching the
established `AppIconBadge` badge-count convention) rather than
wrapping. That's 5 confirmed instances of this exact copy-pasted bug
across the project now (`forms2/DynamicFieldArray`, `QuizForm`,
`SurveyForm`, plus these two) — genuinely worth a standing check in any
future category that displays a count, rank, or index.

**Compile errors — underscore vs. dash, 4 more instances across 3
files:** `GroupChatHeader` (`member_count`), `LikeDislikeButtons`
(`like_count` *and* `dislike_count`, both wrong in the same file), and
`VoiceMessagePlayer` (`duration_seconds`).

**`border-color` set with no `border-width` — a new, real gotcha, found
3 times:** `ThreadReplyPreview`, `DirectMessageHeader`,
`GroupChatHeader` all set only `border-color` with no `border-width`
anywhere on that element — Slint's border defaults to zero width, so
none of the three ever rendered any border at all regardless of the
color chosen. Fixed all three with a thin accent `Rectangle` (a left
bar for the reply preview, a bottom separator for the two chat headers)
rather than guessing at an unconfirmed per-side `border-width`
property — this project has confirmed per-corner *radius* properties
exist, but never a per-side *width* one.

**A broken MM:SS time format, caught while fixing the
`duration_seconds` compile error in the same line:**
`VoiceMessagePlayer`'s elapsed/duration display hardcoded a `"0:"`
minutes prefix and only ever mod-60'd the seconds — anything past 60
seconds would show a wrong, stuck-at-zero minute count (e.g. 90 seconds
as "0:30" instead of "1:30"). Fixed with a real `Math.floor(x / 60)`
for the minutes part. Its *other* two `Math.mod` uses (waveform bar
height variety, and the seconds-within-a-minute part of the same clock
format) are correct, intentional uses — documented inline so a future
pass doesn't mistake them for the bug above.

**Emoji / unconfirmed-safe glyphs, converted to real FA icons (12
files):** `MuteToggle`, `ShareCountDisplay`, `ViewCountDisplay`,
`AttachmentPreview`, `CommentBox`, `DirectMessageHeader`,
`GroupChatHeader`, `LikeDislikeButtons`, `VoiceMessagePlayer`,
`SocialShareButtonsRow` (this one got actual brand icons —
`fontawesome-free`'s `svgs/brands/` set has real Twitter/Facebook/
LinkedIn/Reddit-alien logos, not just generic substitutes),
`NestedCommentThread`. Two components — `ReactionEmojiBar` and
`MessageReactionsStrip` — needed the same fix despite the emoji being
the actual semantic *content* rather than decoration (a reaction pill
*is* the thumbs-up/heart/laugh), because the rendering problem doesn't
discriminate based on semantic role: raw emoji tofu in this environment
either way. `MessageReactionsStrip` additionally had emoji *embedded*
inside compound strings (`"👍 3"`) rather than as standalone icon
slots — split into parallel `reaction-icons: [image]` /
`reaction-counts: [int]` arrays instead of trying to parse a compound
string apart.

**`EmojiPickerPanel` deserves its own note:** unlike every other emoji
case this project has hit, this component's entire purpose is browsing
a *wide, open-ended* set of emoji — there's no finite FA icon set that
honestly represents "all emoji ever," so a literal 1:1 substitution
strategy doesn't really apply. Treated it the same way this whole
library treats other necessarily-simplified mockups (`ImageGallery`'s
numbered tiles standing in for real photos, `VideoPlayer`'s generic
video area): a curated 12-icon stand-in set, clearly commented as an
approximation rather than a real emoji keyboard, so the panel is at
least visually functional instead of a grid of tofu boxes.

**Zero interactivity despite the component's entire purpose being
interaction, four more instances of the shape found repeatedly in
`mobile/`:** `FriendRequestButton` (not one button in any of its three
states had a `TouchArea` — Add Friend, Accept, and Decline all did
nothing), `BlockReportSheet` (same as `mobile/ActionSheetDestructive` —
Mute/Block/Report/Cancel all inert), `HashtagAutocomplete` and
`MentionAutocomplete` (rows not selectable — the entire point of
"autocomplete" is picking a suggestion), `GroupChatHeader`'s
search/more buttons, `DirectMessageHeader`'s call/video buttons,
`AttachmentPreview`'s remove button, `VoiceMessagePlayer`'s play
button, `NotificationBellDropdown`'s rows and "view all," `GifPickerPanel`'s
category tabs. All wired with real callbacks (and `has-hover` feedback
where cheap to add alongside).

**`InChatPollWidget`'s `voted: bool` was declared and never read
anywhere** — voting was entirely non-functional, just a static results
display. Since `votes: [int]` is `in` (can't be index-assigned from
inside the component — a confirmed, already-documented gotcha), wired
real voting as a `vote-cast(int)` callback plus a `private` optimistic
local total purely for immediate visual feedback, leaving the actual
vote-count source of truth for the host to update. Also fixed
`CommentBox` and two picker search boxes (`EmojiPickerPanel`,
`GifPickerPanel`) the same way `forms2/` fixed static-text-pretending-
to-be-an-input: real `TextInput`, not decorative `Text`.

**`test.slint` fixes:** `MessageReactionsStrip`'s invocation still
passed the old `reactions: [string]` property after the rewrite to
`reaction-icons`/`reaction-counts` — caught and updated before
delivery. Also switched the hardcoded `viewport-height: 3200px` to
`content.preferred-height` (same stale-guess issue already fixed in
`mobile/`'s and `forms2/`'s `test.slint`), since this round's additions
would have made the fixed number stale immediately.
