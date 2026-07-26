import { useState, useMemo } from "react";

// ─── Data ─────────────────────────────────────────────────────────────────────
// 23 categories · 744+ components · desktop + mobile + tablet

const REG = [
  {
    id:"tok", icon:"⬡", label:"Design Tokens", accent:"#a855f7",
    desc:"Foundational variables — color, type, spacing, motion, elevation",
    items:[
      "Color Primitives (50–950 scales per hue)",
      "Semantic — Background",
      "Semantic — Surface (3 elevation tiers)",
      "Semantic — On-Surface / Foreground Text",
      "Semantic — Border / Outline",
      "Semantic — Accent / Brand",
      "Semantic — Error / Destructive",
      "Semantic — Warning / Caution",
      "Semantic — Success / Positive",
      "Semantic — Info / Informational",
      "Alpha / Transparency tokens",
      "Typography — Font Family (sans / mono / display)",
      "Typography — Type Scale (rem-based, 10 steps)",
      "Typography — Font Weight scale",
      "Typography — Line Height scale",
      "Typography — Letter Spacing",
      "Spacing scale (4px base, 0–128)",
      "Component inner spacing (8px base)",
      "Border Radius scale (none → full)",
      "Border Width tokens",
      "Shadow / Elevation scale (0–6 levels)",
      "Glow / Luminance effect tokens",
      "Backdrop Blur tokens",
      "Z-Index layer stack",
      "Motion Duration (5 tiers: instant → slowest)",
      "Motion Easing — ease / ease-in / ease-out / ease-in-out",
      "Motion Easing — spring / bounce / overshoot",
      "Spring Preset tokens (stiff / default / gentle / wobbly)",
      "Breakpoints (xs / sm / md / lg / xl / 2xl)",
      "Grid Columns token",
      "Grid Gutter token",
      "Grid Margin token",
      "Icon Size scale (12 / 16 / 20 / 24 / 32 / 48)",
      "Touch Target minimum (44 × 44 px)",
      "Focus Ring width / offset",
      "Opacity scale",
    ]
  },
  {
    id:"type", icon:"Aa", label:"Typography", accent:"#3b82f6",
    desc:"Every text style from jumbo display to fine print",
    items:[
      "Display — Jumbo (hero headline)",
      "Display — XL",
      "Display — LG",
      "Display — MD",
      "Heading H1",
      "Heading H2",
      "Heading H3",
      "Heading H4",
      "Heading H5",
      "Heading H6",
      "Title — LG",
      "Title — MD",
      "Title — SM",
      "Title — XS",
      "Body — LG",
      "Body — MD (default reading)",
      "Body — SM",
      "Body — XS",
      "Label — LG",
      "Label — MD",
      "Label — SM",
      "Label — XS",
      "Caption",
      "Overline (uppercase track)",
      "Eyebrow / Kicker",
      "Tagline",
      "Footnote",
      "Legal / Fine Print",
      "Code / Monospace block",
      "Inline Code Span",
      "Blockquote",
      "Pull Quote (editorial)",
      "Drop Cap",
      "Running Text (justified body)",
      "Highlight / Mark",
      "Numeric / Tabular Figures",
    ]
  },
  {
    id:"inp", icon:"⌨", label:"Text & Input", accent:"#14b8a6",
    desc:"All input primitives for data entry, search, and capture",
    items:[
      "Text Input — single line",
      "Password Input (reveal toggle)",
      "Search Input (with clear ✕)",
      "Email Input",
      "URL Input",
      "Tel / Phone + country prefix select",
      "Number Input",
      "Number Stepper (inline ± buttons)",
      "Currency Input (formatted)",
      "Percentage Input",
      "Measurement Input (unit toggle)",
      "Textarea — fixed height",
      "Textarea — auto-resize",
      "Rich Text / WYSIWYG Editor",
      "Code Editor (syntax highlighting + line numbers)",
      "Markdown Editor (split preview)",
      "OTP / PIN Input (4–8 cells)",
      "Credit Card Input (formatted mask)",
      "Date Picker Input",
      "Time Picker Input",
      "Date-Time Picker",
      "Date Range Picker",
      "Month / Year Picker",
      "Week Picker",
      "Color Picker (hex / RGB / HSL / OKLCH)",
      "Gradient Builder Input",
      "File Upload — drag & drop zone",
      "Image Upload — with preview crop",
      "Multi-file Upload zone",
      "Camera Capture Input",
      "Voice / Speech Input",
      "QR / Barcode Scanner",
      "Signature Pad",
      "Emoji Picker Input",
      "Icon Picker Input",
      "Font Picker Input",
      "Tags / Chips Input (create + remove)",
      "Mention Input (@user autocomplete)",
      "Hashtag Input (#tag autocomplete)",
      "Autocomplete Input (async API)",
      "Command / Spotlight Input",
      "Address Input (structured fields)",
      "Masked / Regex Input",
      "Geolocation Input (map-based pick)",
      "JSON / YAML Editor (validated)",
    ]
  },
  {
    id:"sel", icon:"☑", label:"Selection Controls", accent:"#f59e0b",
    desc:"Controls for choosing one or many options from a set",
    items:[
      "Checkbox",
      "Checkbox — Indeterminate state",
      "Checkbox Group",
      "Radio Button",
      "Radio Group",
      "Toggle / Switch",
      "Toggle Group (multiple on/off)",
      "Segmented Control (mutually exclusive)",
      "Button Toggle Group",
      "Chip Select (single)",
      "Multi-Chip Select",
      "Select / Dropdown (single)",
      "Select — Multi (tags in trigger)",
      "Combobox (editable select)",
      "Listbox (single-select)",
      "Listbox (multi-select)",
      "Option Group",
      "Cascading / Nested Select",
      "Country Picker",
      "Language Picker",
      "Timezone Picker",
      "Currency Picker",
      "Flag Picker",
      "Color Swatch Picker",
      "Size / Variant Picker",
    ]
  },
  {
    id:"rng", icon:"⇔", label:"Range & Value", accent:"#f43f5e",
    desc:"Continuous and discrete value selection controls",
    items:[
      "Slider — single handle",
      "Slider — dual / range handles",
      "Slider — stepped (snapping)",
      "Slider — vertical orientation",
      "Slider — with live value tooltip",
      "Circular Knob / Dial",
      "Star Rating",
      "Heart Rating",
      "Thumb Rating (👍 / 👎)",
      "Numeric Rating input",
      "Emoji / Mood Rating (5-point)",
      "Number Spinner / Stepper",
      "Color Wheel",
      "Hue Slider",
      "Opacity / Alpha Slider",
      "Saturation-Lightness 2D Gradient Map",
    ]
  },
  {
    id:"btn", icon:"◉", label:"Buttons & Actions", accent:"#E63950",
    desc:"Every variant of pressable, tappable action element",
    items:[
      "Primary Button",
      "Secondary Button",
      "Tertiary / Ghost Button",
      "Outlined Button",
      "Text / Link Button",
      "Destructive / Danger Button",
      "Success / Confirm Button",
      "Warning Button",
      "Icon Button — square",
      "Icon Button — circle",
      "Floating Action Button (FAB)",
      "Extended FAB (icon + label)",
      "Mini FAB",
      "Speed Dial (FAB group)",
      "Split Button (action + dropdown caret)",
      "Dropdown Button",
      "Loading / Async Button (spinner state)",
      "Toggle Button (on / off state)",
      "Chip / Filter Button",
      "Pill Button",
      "Copy Button (auto clipboard)",
      "Share Button",
      "Bookmark / Save Button",
      "Like / Heart Button",
      "Reaction Button (emoji picker)",
      "Close / Dismiss × Button",
      "Back / Up Navigation Button",
      "Submit Button (with progress state)",
      "Swipe-to-Confirm (mobile slider)",
      "Long-press Action Button",
      "CTA / Hero Button",
      "Play / Pause Button",
      "Record Button",
      "Mute / Unmute Button",
      "Social Auth — Google",
      "Social Auth — Apple",
      "Social Auth — GitHub",
      "Social Auth — Facebook",
      "Social Auth — Twitter / X",
      "Back-to-Top Button",
      "Scroll-to-Section Anchor Button",
      "Download / Export Button",
    ]
  },
  {
    id:"nav", icon:"⊶", label:"Navigation", accent:"#06b6d4",
    desc:"All patterns for routing, wayfinding, and movement",
    items:[
      "Top App Bar — standard",
      "Top App Bar — large / collapsing",
      "Sticky Header",
      "Scroll-triggered transparent-to-solid Header",
      "Bottom Navigation Bar (3–5 items)",
      "Bottom Navigation Rail (tablet)",
      "Persistent Sidebar (desktop)",
      "Temporary Drawer (mobile)",
      "Modal Drawer",
      "Mini Sidebar (icon-only collapsed)",
      "Floating Navigation Panel",
      "Tab Bar — horizontal primary",
      "Tab Bar — horizontal secondary",
      "Tab Bar — vertical side",
      "Scrollable / Overflow Tabs",
      "Nested Tabs (2-level)",
      "Breadcrumb",
      "Breadcrumb — ellipsis-collapsing",
      "Pagination — numbered",
      "Pagination — prev / next only",
      "Cursor / Token-based Pagination",
      "Load More Button",
      "Infinite Scroll Trigger",
      "Stepper — horizontal",
      "Stepper — vertical",
      "Stepper — dot compact",
      "Stepper — icon labeled",
      "Nav Item",
      "Nav Group Header / Section Label",
      "Nav Collapse / Accordion",
      "Nav Badge",
      "Mega Menu (desktop hover panel)",
      "Flyout / Nested Dropdown Menu",
      "Hamburger / Menu Toggle Button",
      "Command Palette",
      "Launchpad / App Grid",
      "Dock (macOS-style icon launcher)",
      "Taskbar (Windows-style)",
      "Workspace / Virtual Desktop Switcher",
      "On-page Jump / Anchor Nav Bar",
    ]
  },
  {
    id:"ovr", icon:"⧉", label:"Overlays & Popups", accent:"#8b5cf6",
    desc:"Floating layers rendered above base content",
    items:[
      "Modal Dialog — standard",
      "Alert Dialog — destructive confirm",
      "Full-screen Dialog (mobile)",
      "Bottom Sheet — snap points (mobile)",
      "Peek Sheet — partial height reveal",
      "Side Sheet — right panel (desktop)",
      "Popover",
      "Tooltip — simple text",
      "Tooltip — rich (title + body + actions)",
      "Hover Card",
      "Context Tooltip",
      "Floating Label",
      "Dropdown Menu",
      "Nested / Sub-Dropdown Menu",
      "Action Sheet (iOS-style)",
      "Notification Drawer Panel",
      "Lightbox — image viewer",
      "Lightbox — video",
      "Inline Confirmation (popover)",
      "Floating Toolbar",
      "Floating Action Panel",
      "Scrim / Modal Overlay Backdrop",
      "Spotlight Overlay (feature highlight)",
      "Onboarding Step Highlight Overlay",
    ]
  },
  {
    id:"fb", icon:"◎", label:"Feedback & Status", accent:"#10b981",
    desc:"Loading, progress, result, and system-state communicators",
    items:[
      "Toast — info",
      "Toast — success",
      "Toast — warning",
      "Toast — error",
      "Toast — with action button",
      "Toast — multi-line",
      "Persistent Banner — top",
      "Persistent Banner — bottom",
      "Inline Alert — info",
      "Inline Alert — success",
      "Inline Alert — warning",
      "Inline Alert — error",
      "Callout / Highlighted Info Box",
      "Progress Bar — linear determinate",
      "Progress Bar — linear indeterminate",
      "Progress Bar — segmented steps",
      "Progress Bar — striped animated",
      "Circular Progress — determinate",
      "Circular Progress — indeterminate",
      "Skeleton — text line",
      "Skeleton — paragraph block",
      "Skeleton — heading",
      "Skeleton — avatar",
      "Skeleton — image / media",
      "Skeleton — card",
      "Skeleton — table row",
      "Skeleton — list item",
      "Shimmer Overlay wrapper",
      "Spinner — ring",
      "Dots Loader",
      "Pulse Loader",
      "Wave / Bar Loader",
      "Empty State (illustrated)",
      "Error / Crash State (500)",
      "Offline / No Connection State",
      "No Search Results State",
      "Permission Denied / 403 State",
      "Success / Completion State",
      "Confetti Celebration Burst",
    ]
  },
  {
    id:"bdg", icon:"⬟", label:"Indicators & Badges", accent:"#f97316",
    desc:"Small status chips, labels, and inline indicators",
    items:[
      "Badge — numeric count",
      "Badge — dot (unread indicator)",
      "Badge — status (online / offline / away / busy)",
      "Badge — text label",
      "Badge — New",
      "Badge — Beta",
      "Badge — Alpha",
      "Badge — Pro / Paid tier",
      "Badge — Verified ✓",
      "Badge — Deprecated",
      "Chip — input (removable tag)",
      "Chip — filter (toggle state)",
      "Chip — assist (suggestion)",
      "Chip — status",
      "Tag / Label (colored fill)",
      "Pill (colored outline)",
      "Status Dot",
      "Activity Indicator",
      "Health / Score Progress Bar",
      "Signal Strength Bars",
      "Battery Level Indicator",
      "Version String Badge",
      "Environment Badge (dev / staging / prod)",
    ]
  },
  {
    id:"dat", icon:"⊞", label:"Data Display", accent:"#38bdf8",
    desc:"Components presenting structured data at every scale",
    items:[
      "Table — basic",
      "Data Table (sortable + filterable)",
      "Virtualized / Windowed Table",
      "Frozen-column Table",
      "Expandable Row Table",
      "Editable Inline Table",
      "Tree / Hierarchical Table",
      "Summary / Footer Row",
      "Column Header (with sort icon)",
      "Table Toolbar (search + bulk actions)",
      "Bulk Action Bar",
      "Feature Comparison Matrix Table",
      "List — simple",
      "List — dense",
      "List — two-line with meta",
      "List — with avatar",
      "List — with leading icon",
      "List — with trailing action",
      "Virtualized / Windowed List",
      "Grouped List (section headers)",
      "Sortable / Drag-to-reorder List",
      "Selection List (checkbox per row)",
      "Checklist",
      "Definition / Description List",
      "Timeline — vertical",
      "Timeline — horizontal",
      "Activity Feed",
      "Chat Message Thread",
      "Nested Comment Section",
      "Code Block (syntax + line numbers + copy)",
      "Diff Viewer (side-by-side / unified)",
      "Log Viewer (real-time streaming)",
      "Terminal / Console Output",
      "JSON Tree Viewer",
      "Key-Value Display Row",
      "Property Inspector Panel",
      "Stat / Metric Display Block",
      "KPI Tile",
      "Leaderboard Table",
      "Price Display",
      "Price Comparison Table",
      "Receipt / Invoice View",
      "Tree View / File Explorer",
      "Network / Dependency Graph",
      "Gantt / Timeline View",
      "Org Chart",
      "Calendar — Month View",
      "Calendar — Week View",
      "Calendar — Day / Agenda View",
    ]
  },
  {
    id:"crd", icon:"▭", label:"Cards", accent:"#ec4899",
    desc:"Contained surface components grouping related content",
    items:[
      "Card — basic",
      "Card — elevated (shadow)",
      "Card — outlined",
      "Card — glass / blur effect",
      "Card — interactive / pressable",
      "Card — media (image header)",
      "Card — horizontal (side image)",
      "Card — feature (icon + heading + body)",
      "Card — pricing tier",
      "Card — profile / person",
      "Card — contact / vCard",
      "Card — stat tile",
      "Card — metric with sparkline",
      "Card — notification",
      "Card — alert",
      "Card — blog post preview",
      "Card — article preview",
      "Card — product (e-commerce)",
      "Card — product comparison",
      "Card — testimonial / quote",
      "Card — review (stars + text)",
      "Card — event (date + location)",
      "Card — location / place",
      "Card — map embed",
      "Card — weather (daily)",
      "Card — music / track",
      "Card — podcast episode",
      "Card — video thumbnail",
      "Card — file (type icon + metadata)",
      "Card — link preview (OG meta)",
      "Card — app listing (store-style)",
      "Card — team member",
      "Card — job listing",
      "Card — course / tutorial",
      "Card — achievement / trophy",
      "Card — swipeable (dismissible)",
      "Card — flip (front / back)",
      "Card — expandable / collapsible",
      "Card — empty / placeholder",
    ]
  },
  {
    id:"med", icon:"⬒", label:"Media", accent:"#0ea5e9",
    desc:"Image, video, audio, avatar, and icon display",
    items:[
      "Image (lazy load + error fallback)",
      "Responsive Image (srcset / sizes)",
      "Image Carousel / Slider",
      "Image Gallery Grid",
      "Image Masonry Layout",
      "Lightbox Viewer",
      "Image Crop Tool",
      "Before / After Comparison Slider",
      "Avatar — circle",
      "Avatar — rounded square",
      "Avatar Group / Overlap Stack",
      "Avatar with Status Ring",
      "Avatar with Badge overlay",
      "Avatar Fallback (initials / icon)",
      "Icon — mono / filled / duotone",
      "Animated / Lottie Icon",
      "SVG Illustration Slot",
      "Logo / Brand Mark",
      "Video Player (full controls)",
      "Video Thumbnail (play overlay)",
      "Audio Player (waveform + scrubber)",
      "Podcast Player UI",
      "Livestream Live Indicator Pill",
      "Story / Reel Viewer",
      "GIF Player (hover autoplay)",
      "Map Embed (static / interactive)",
      "QR Code Display",
      "Barcode Display",
      "Camera Preview / Viewfinder",
      "3D Model Viewer",
    ]
  },
  {
    id:"viz", icon:"⊿", label:"Charts & Data Viz", accent:"#22d3ee",
    desc:"Visualization components for quantitative and relational data",
    items:[
      "Line Chart",
      "Multi-series Line Chart",
      "Area Chart (filled)",
      "Stacked Area Chart",
      "Bar Chart — vertical",
      "Bar Chart — horizontal",
      "Grouped Bar Chart",
      "Stacked Bar Chart",
      "100% Stacked Bar",
      "Pie Chart",
      "Donut Chart",
      "Multi-ring Donut",
      "Scatter Plot",
      "Bubble Chart",
      "Heatmap / Calendar Heatmap",
      "Treemap",
      "Sunburst / Radial Treemap",
      "Funnel Chart",
      "Waterfall / Bridge Chart",
      "Gauge / Speedometer",
      "Radial Bar Chart",
      "Radar / Spider / Web Chart",
      "Candlestick (OHLC)",
      "Volume Chart",
      "Histogram",
      "Box Plot",
      "Violin Plot",
      "Sankey Diagram",
      "Chord Diagram",
      "Network / Force-directed Graph",
      "Flowchart / Process Diagram",
      "Gantt / Resource Timeline Chart",
      "Choropleth / Geo Map",
      "Geo Heatmap",
      "Dot Density Map",
      "Sparkline — line (inline)",
      "Sparkline — bar (inline)",
      "Micro Area Chart",
      "Progress Ring Chart",
      "Correlation Matrix",
      "Word Cloud",
      "Slope Chart",
      "Bump / Ranking Chart",
    ]
  },
  {
    id:"lay", icon:"⊡", label:"Layout & Containers", accent:"#84cc16",
    desc:"Structural components organizing spatial relationships",
    items:[
      "Page Root / App Shell",
      "Single Column Layout",
      "Two Column Layout",
      "Three Column Layout",
      "Holy Grail Layout (header + 3-col + footer)",
      "12-Column Grid System",
      "Auto-layout Flexbox Container",
      "CSS Grid Container",
      "Masonry Grid",
      "Bento Grid",
      "Pinterest / Waterfall Layout",
      "Responsive Max-Width Container",
      "Full-Width Section",
      "Content Region",
      "Sidebar + Content Pane",
      "Master-Detail Split View",
      "Dual Panel (resizable sash)",
      "Quad Panel Layout",
      "Accordion / Collapse Group",
      "Disclosure Widget",
      "Tab Panel Container",
      "Wizard / Multi-step Wrapper",
      "Card Grid Container",
      "Feature Grid (2 / 3 / 4 col)",
      "Tile Grid",
      "Wrap / Cluster Container",
      "Stack — vertical (gap-aware)",
      "Stack — horizontal (gap-aware)",
      "Inline Cluster",
      "Z-Stack (absolute-layered)",
      "Spacer (fixed-size gap)",
      "Divider — horizontal",
      "Divider — vertical",
      "Labeled Divider",
      "Scrollable Region",
      "Overflow Scroll X container",
      "Overflow Scroll Y container",
      "Custom Scrollbar",
      "Virtualized Scroll Container",
      "Infinite Scroll Wrapper",
      "Sticky Wrapper",
      "Fixed Container",
      "Portal / Teleport",
      "Aspect Ratio Box",
      "Full-Bleed / Stretch Box",
      "Safe Area Inset Wrapper",
      "Focus Trap Region",
      "Drag-and-Drop Zone",
      "Resizable Panel Group",
      "Snap-scroll Container",
    ]
  },
  {
    id:"frm", icon:"⊟", label:"Forms", accent:"#fbbf24",
    desc:"Form anatomy, validation patterns, and template layouts",
    items:[
      "Form Root (validation context)",
      "Form Section",
      "Form Group",
      "Form Row — horizontal fields",
      "Field Wrapper",
      "Field Label",
      "Field Description / Helper Text",
      "Field Error Message",
      "Field Success Message",
      "Field Character Counter",
      "Fieldset",
      "Legend",
      "Form Divider",
      "Form Header",
      "Form Footer / Submit Area",
      "Multi-step Form",
      "Wizard Form (branching logic)",
      "Dynamic Field Array (repeater)",
      "Inline / Compact Form",
      "Horizontal Label-Input Form",
      "Vertical Stacked Form",
      "Search + Filter Form",
      "Login Form (template)",
      "Registration Form (template)",
      "Password Reset / Recovery Form",
      "Profile Edit Form",
      "Payment / Billing Form",
      "Address Form",
      "Survey Form",
      "Quiz / Assessment Form",
      "Contact Form (template)",
    ]
  },
  {
    id:"dsk", icon:"⬚", label:"Desktop-Specific", accent:"#a78bfa",
    desc:"Components exclusive to or optimized for desktop environments",
    items:[
      "Window Chrome (title bar + traffic-light controls)",
      "Resizable + Draggable Window",
      "Minimized Window Chip (taskbar)",
      "Maximized / Fullscreen Window",
      "Floating Panel Window",
      "Inspector / Properties Side Panel",
      "Toolbar (icon + label groups)",
      "Overflow Toolbar (… more)",
      "Ribbon Bar (Office-style multi-tab)",
      "Menu Bar (macOS-style app menu)",
      "App Menu Item (with keyboard shortcut)",
      "Status Bar",
      "System Tray / Notification Area",
      "Desktop Widget Container",
      "Notification Center Panel",
      "App Switcher overlay (Cmd+Tab)",
      "Dock (icon launcher)",
      "Taskbar (Windows-style)",
      "Workspace / Virtual Desktop Switcher",
      "Spotlight / Quick Launch dialog",
      "Control Panel Layout",
      "Preferences / Settings Window",
      "About Dialog",
      "Splash Screen",
      "Shortcut Reference Sheet",
      "Drag Handle",
      "Panel Sash / Resize Rail",
      "Column Resizer Handle",
      "Right-click Context Menu",
      "Shell / OS Notification toast",
    ]
  },
  {
    id:"mob", icon:"▱", label:"Mobile-Specific", accent:"#34d399",
    desc:"Touch-first components for iOS, Android, and responsive mobile",
    items:[
      "Pull-to-Refresh Control",
      "Swipe-to-Delete Row",
      "Swipe-to-Archive Row",
      "Swipe-to-Action (custom labels)",
      "Long-press Context Action Sheet",
      "Haptic Feedback Wrapper",
      "Safe Area Top Spacer (notch)",
      "Safe Area Bottom Spacer (home bar)",
      "Dynamic Island Slot",
      "Status Bar Overlay",
      "Lock Screen Widget — small",
      "Lock Screen Widget — medium",
      "Home Screen Widget — 2×2",
      "Home Screen Widget — 2×4",
      "Home Screen Widget — 4×4",
      "App Icon (with badge dot)",
      "Splash / Launch Screen",
      "Onboarding Pager (swipeable screens)",
      "Biometric Auth Prompt (Face ID / Touch ID)",
      "Bottom Sheet — multi-snap-point",
      "Half-sheet Modal",
      "Action Sheet (iOS-style destructive)",
      "Document / File Picker",
      "Share Sheet",
      "Photo / Media Picker",
      "Contact Picker",
      "App Clip Mini-Card",
      "Reachability Mode Helper",
      "Floating Island Pill",
      "Swipe Navigation Gesture Area",
    ]
  },
  {
    id:"a11", icon:"⊛", label:"Accessibility", accent:"#60a5fa",
    desc:"Inclusive, WCAG-compliant components and a11y utilities",
    items:[
      "Skip to Content Link",
      "Focus Ring — browser default",
      "Focus Ring — custom branded",
      "Screen Reader Only Text (.sr-only)",
      "ARIA Live Region — polite",
      "ARIA Live Region — assertive",
      "Focus Trap Guard",
      "Roving Tab Index Group",
      "High-Contrast Mode Toggle",
      "Font Size Adjuster (A− / A+)",
      "Line Height Adjuster",
      "Dyslexia-Friendly Font Toggle",
      "Reduce Motion Toggle",
      "Color Blindness Simulation Toggle",
      "Keyboard Shortcut Legend",
      "Keyboard Navigation Mode Indicator",
      "Focus Visible Indicator (pointer vs keyboard)",
      "Accessible Error Summary (linked to fields)",
      "ARIA Announce (programmatic message queue)",
    ]
  },
  {
    id:"soc", icon:"◑", label:"Social & Communication", accent:"#fb7185",
    desc:"Chat, social interaction, and collaborative interface patterns",
    items:[
      "Chat Bubble — sent",
      "Chat Bubble — received",
      "Chat Bubble — system / status message",
      "Typing Indicator (animated …)",
      "Read Receipt Icon",
      "Message Reactions Strip",
      "Emoji Picker Panel",
      "GIF Picker Panel",
      "Attachment Preview (in-chat)",
      "Voice Message Player (waveform)",
      "Chat Input Bar",
      "Mention Autocomplete Popup (@)",
      "Hashtag Autocomplete Popup (#)",
      "Thread / Reply Preview",
      "Group Chat Header",
      "Direct Message Header",
      "User Presence Indicator",
      "Notification Bell + Dropdown Panel",
      "Mute / Unmute Toggle",
      "In-chat Poll Widget",
      "Comment Box",
      "Nested Comment Thread",
      "Like / Dislike Buttons",
      "Reaction Emoji Bar",
      "Share Count Display",
      "View Count Display",
      "Social Share Buttons Row",
      "Follow / Unfollow Button",
      "Friend Request Button",
      "Upvote / Downvote Buttons",
      "Leaderboard Row",
      "Block / Report Action Sheet",
    ]
  },
  {
    id:"mot", icon:"⟳", label:"Animation & Motion", accent:"#c084fc",
    desc:"Transition, gesture, and animation primitives",
    items:[
      "Page Transition — Fade",
      "Page Transition — Slide (directional)",
      "Page Transition — Scale / Zoom",
      "Shared Element Transition",
      "Hero Morph / Layout animation",
      "List Stagger Entrance",
      "Scroll-triggered Reveal",
      "Parallax Scroll Layer",
      "Spring Physics Container",
      "Inertia Momentum Scroll",
      "Drag-to-Dismiss gesture",
      "Swipe Gesture Handler",
      "Pinch-to-Zoom Handler",
      "Rubber-band / Bounce Overscroll",
      "Confetti Burst Overlay",
      "Ripple / Press Ink Effect",
      "Shimmer Keyframe wrapper",
      "Lottie / Rive Animation Player",
      "CSS Variable Spring wrapper",
      "Intersection Observer Trigger",
      "FLIP Animation Helper",
      "View Transition API Wrapper",
      "Morphing SVG Path",
      "Counter Number Tween",
    ]
  },
  {
    id:"thm", icon:"✦", label:"Theming & Customization", accent:"#fde047",
    desc:"Tools for managing, switching, and exporting design systems",
    items:[
      "Theme Provider",
      "Dark Mode Toggle",
      "System prefers-color-scheme Adapter",
      "High-Contrast Adapter",
      "Custom Theme Builder UI",
      "CSS Custom Property Injector",
      "Typography Scale Configurator",
      "Spacing Scale Configurator",
      "Color Palette Builder / Editor",
      "Brand Kit Switcher",
      "Icon Set Switcher",
      "Font Loader / Font Swap",
      "Animation Preset Selector",
      "Token Export (JSON / CSS / SCSS / Tailwind)",
      "Preview / Sandbox Canvas",
      "Component Playground",
      "WCAG Contrast Checker",
      "Accessibility Audit Overlay",
    ]
  },
  {
    id:"util", icon:"⚙", label:"Utility & Headless", accent:"#94a3b8",
    desc:"Logic-only primitives, hooks, and headless engine components",
    items:[
      "Portal (DOM teleport)",
      "Slot / Content Projection",
      "Error Boundary",
      "Suspense / Loading Boundary",
      "Intersection Observer Hook",
      "Resize Observer Hook",
      "Scroll Position Observer Hook",
      "Mutation Observer Hook",
      "Keyboard Shortcut Listener",
      "Idle Detector",
      "Network / Online Status Detector",
      "Clipboard Manager",
      "Drag Source primitive",
      "Drop Target primitive",
      "Virtual List Engine",
      "Virtual Grid Engine",
      "Infinite Loader Engine",
      "State Machine Provider",
      "Debounce Hook",
      "Throttle Hook",
      "Media Query Observer Hook",
      "Pointer / Touch Event Normalizer",
      "Focus Manager",
    ]
  },
];

// ─── Component ─────────────────────────────────────────────────────────────────

const TOTAL = REG.reduce((s, c) => s + c.items.length, 0);

export default function ComponentRegistry() {
  const [q, setQ] = useState("");
  const [activeCat, setActiveCat] = useState(null);
  const [collapsed, setCollapsed] = useState({});

  const shown = useMemo(() => {
    const lq = q.toLowerCase();
    return REG
      .map(c => ({ ...c, items: c.items.filter(i => i.toLowerCase().includes(lq)) }))
      .filter(c => (!activeCat || c.id === activeCat) && c.items.length > 0);
  }, [q, activeCat]);

  const vis = shown.reduce((s, c) => s + c.items.length, 0);

  const toggle = id => setCollapsed(p => ({ ...p, [id]: !p[id] }));
  const catClick = id => setActiveCat(activeCat === id ? null : id);

  return (
    <div style={{
      minHeight:"100vh",
      background:"linear-gradient(160deg, #0A0C12 0%, #0D1018 60%, #0A0C14 100%)",
      fontFamily:"'Space Grotesk', system-ui, -apple-system, sans-serif",
      color:"#C0C8DC",
    }}>

      {/* ─── STICKY HEADER ─────────────────────────────────────────────────── */}
      <div style={{
        position:"sticky", top:0, zIndex:200,
        background:"rgba(9,11,16,0.88)",
        backdropFilter:"blur(24px)",
        WebkitBackdropFilter:"blur(24px)",
        borderBottom:"1px solid rgba(255,255,255,0.07)",
        padding:"16px 20px 12px",
      }}>
        <div style={{maxWidth:1340, margin:"0 auto"}}>

          {/* Title row */}
          <div style={{display:"flex", alignItems:"center", gap:12, marginBottom:12}}>
            <div style={{
              width:38, height:38, borderRadius:11, flexShrink:0,
              background:"linear-gradient(135deg, #E63950 0%, #8b5cf6 100%)",
              display:"flex", alignItems:"center", justifyContent:"center",
              fontSize:20, boxShadow:"0 0 18px rgba(230,57,80,0.35)",
            }}>🔩</div>
            <div style={{flex:1}}>
              <h1 style={{
                margin:0, fontSize:17, fontWeight:700, color:"#ECEEF8",
                letterSpacing:"-0.02em",
              }}>UI Component Registry</h1>
              <p style={{
                margin:0, fontSize:12,
                fontFamily:"'JetBrains Mono', monospace",
                color:"#384060",
              }}>
                {vis === TOTAL
                  ? <><span style={{color:"#E63950", fontWeight:700}}>{TOTAL}</span> components · {REG.length} categories</>
                  : <><span style={{color:"#E63950"}}>{vis}</span><span style={{color:"#384060"}}> / {TOTAL}</span> matching · {shown.length} categories</>
                }
              </p>
            </div>
          </div>

          {/* Search */}
          <div style={{position:"relative", marginBottom:11}}>
            <span style={{
              position:"absolute", left:12, top:"50%", transform:"translateY(-50%)",
              fontSize:14, color:"#384060", pointerEvents:"none",
            }}>⌕</span>
            <input
              value={q}
              onChange={e => setQ(e.target.value)}
              placeholder="Search any component, pattern, or concept…"
              style={{
                width:"100%", boxSizing:"border-box",
                background:"rgba(18,21,30,0.85)",
                border:"1px solid rgba(255,255,255,0.09)",
                borderRadius:10, padding:"9px 14px 9px 34px",
                color:"#D0D8F0", fontSize:13.5,
                fontFamily:"inherit", outline:"none",
              }}
            />
            {q && (
              <button onClick={() => setQ("")} style={{
                position:"absolute", right:10, top:"50%", transform:"translateY(-50%)",
                background:"none", border:"none", color:"#384060",
                cursor:"pointer", fontSize:16, padding:"0 4px",
              }}>✕</button>
            )}
          </div>

          {/* Category pills */}
          <div style={{
            display:"flex", gap:5, overflowX:"auto",
            scrollbarWidth:"none", paddingBottom:2,
          }}>
            <button
              onClick={() => setActiveCat(null)}
              style={{
                padding:"4px 13px", borderRadius:999, border:"1px solid",
                borderColor: activeCat === null ? "rgba(255,255,255,0.25)" : "rgba(255,255,255,0.08)",
                background: activeCat === null ? "rgba(255,255,255,0.1)" : "transparent",
                color: activeCat === null ? "#ECEEF8" : "#404868",
                fontSize:11, fontWeight:600, cursor:"pointer",
                whiteSpace:"nowrap", fontFamily:"inherit",
              }}
            >All · {TOTAL}</button>
            {REG.map(c => {
              const on = activeCat === c.id;
              return (
                <button key={c.id} onClick={() => catClick(c.id)} style={{
                  padding:"4px 11px", borderRadius:999, border:"1px solid",
                  borderColor: on ? c.accent+"80" : "rgba(255,255,255,0.07)",
                  background: on ? c.accent+"1C" : "transparent",
                  color: on ? c.accent : "#404868",
                  fontSize:11, fontWeight:500, cursor:"pointer",
                  whiteSpace:"nowrap", fontFamily:"inherit",
                }}>{c.icon} {c.label}</button>
              );
            })}
          </div>
        </div>
      </div>

      {/* ─── CONTENT ───────────────────────────────────────────────────────── */}
      <div style={{maxWidth:1340, margin:"0 auto", padding:"18px 18px 80px"}}>

        {shown.length === 0 ? (
          <div style={{
            textAlign:"center", padding:"80px 20px", color:"#252A3C",
          }}>
            <div style={{fontSize:52, marginBottom:12}}>◌</div>
            <p style={{margin:0, fontSize:15, color:"#3A4058"}}>
              No components match <em style={{color:"#5060A0"}}>"{q}"</em>
            </p>
          </div>
        ) : (
          shown.map(c => (
            <Block key={c.id} c={c} isOpen={collapsed[c.id] !== true} toggle={toggle} q={q} />
          ))
        )}

        {/* Footer count */}
        {shown.length > 0 && (
          <div style={{
            textAlign:"center", marginTop:40,
            fontSize:12, color:"#25293A",
            fontFamily:"'JetBrains Mono', monospace",
          }}>
            {vis} components shown · {REG.length} total categories · LionOS Toolkit Reference
          </div>
        )}
      </div>
    </div>
  );
}

function Block({ c, isOpen, toggle, q }) {
  const lq = q.toLowerCase();
  return (
    <div style={{ marginBottom: 12 }}>

      {/* ─ Category header ─ */}
      <button
        onClick={() => toggle(c.id)}
        style={{
          display:"flex", alignItems:"center", gap:10, width:"100%",
          background:`linear-gradient(90deg, ${c.accent}10 0%, transparent 55%)`,
          border:`1px solid ${c.accent}22`,
          borderRadius:12, padding:"10px 14px",
          cursor:"pointer", fontFamily:"inherit",
          textAlign:"left", marginBottom: isOpen ? 7 : 0,
          transition:"border-color 0.15s",
        }}
      >
        <span style={{
          width:30, height:30, borderRadius:8, flexShrink:0,
          background:`${c.accent}1A`,
          border:`1px solid ${c.accent}45`,
          display:"flex", alignItems:"center",
          justifyContent:"center",
          fontSize:14, color:c.accent,
        }}>{c.icon}</span>

        <div style={{flex:1, minWidth:0}}>
          <div style={{
            fontSize:13.5, fontWeight:650, color:"#EAEEF8",
            lineHeight:1.3,
          }}>{c.label}</div>
          <div style={{
            fontSize:11, color:"#343A52", marginTop:1,
            overflow:"hidden", textOverflow:"ellipsis", whiteSpace:"nowrap",
          }}>{c.desc}</div>
        </div>

        <span style={{
          background:`${c.accent}20`, color:c.accent,
          fontSize:11, fontWeight:700,
          padding:"2px 9px", borderRadius:999, flexShrink:0,
          fontFamily:"'JetBrains Mono', monospace",
        }}>{c.items.length}</span>

        <span style={{color:"#2A3048", fontSize:11, flexShrink:0, marginLeft:2}}>
          {isOpen ? "▾" : "▸"}
        </span>
      </button>

      {/* ─ Item grid ─ */}
      {isOpen && (
        <div style={{
          display:"grid",
          gridTemplateColumns:"repeat(auto-fill, minmax(200px, 1fr))",
          gap:5,
        }}>
          {c.items.map((item, i) => {
            const hl = q && item.toLowerCase().includes(lq);
            return (
              <div key={i} style={{
                background: hl ? `${c.accent}14` : "rgba(14,16,22,0.75)",
                border:`1px solid ${hl ? c.accent+"48" : "rgba(255,255,255,0.055)"}`,
                borderRadius:7, padding:"7px 11px",
                fontSize:12, lineHeight:1.45,
                color: hl ? "#E0E6F8" : "#5A6280",
                display:"flex", alignItems:"flex-start", gap:7,
              }}>
                <span style={{
                  width:4, height:4, borderRadius:999, marginTop:5,
                  background: hl ? c.accent : "#1E2338",
                  flexShrink:0,
                }}/>
                <span>{item}</span>
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
}
