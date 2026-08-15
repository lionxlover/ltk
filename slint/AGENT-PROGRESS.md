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
| `charts/` | 43 | ✅ Done, pending VS Code Problems-panel check | `charts.zip` |
| everything else | ~528 | ⬜ Not started | — |

Untouched categories, roughly by size:
`cards` (39),
`navigation` (39), `feedback` (36), `typography` (36), `media` (32),
`mobile` (30), `forms2` (31), `social2` (26), `overlays` (24),
`animation` (24), `selection-controls` (25), `utility` (23),
`indicators` (20), `desktop2` (17), `theming2` (12), `accessibility2`
(12), `desktop` (10), `range-value` (10), `social` (8), `accessibility`
(8), `theming` (6), `layout` (3), `progress` (partially touched —
`ProgressSpinner.slint` fixed, rest untouched).

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
