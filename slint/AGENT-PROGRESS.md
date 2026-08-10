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
| `data-display/` | 60 | ✅ Done, pending VS Code Problems-panel check | `data-display.zip` |
| everything else | ~661 | ⬜ Not started | — |

Untouched categories, roughly by size:
`text-input` (45), `charts` (43), `cards` (39),
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
