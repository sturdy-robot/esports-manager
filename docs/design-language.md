# eSports Manager — Design Language

## Identity: "Esports Editorial"

A premium sports-magazine aesthetic applied to esports management. Clean, confident,
typographically rich. The personality comes from the contrast between a distinctive
geometric display face and refined body text — like The Athletic meets competitive gaming.
Data-dense screens feel curated, not cluttered; key moments carry editorial punch.

---

## Core Principles

1. **Information density first** — multiple data streams visible simultaneously; no wasted space
2. **Typographic contrast drives hierarchy** — the interplay between display and body typefaces creates visual rhythm without decoration
3. **Energy on interaction** — static UI is calm and editorial; hover/active/live states bring competitive intensity
4. **Light and dark as equals** — both modes are first-class; neither is an afterthought

---

## Color System

### Dark Mode (Primary)

| Token              | Hex       | Usage                                      |
|--------------------|-----------|--------------------------------------------|
| `--bg-base`        | `#0A0A0F` | App background                             |
| `--bg-surface`     | `#14141F` | Card/panel surfaces                        |
| `--bg-elevated`    | `#1E1E2E` | Modals, dropdowns, popovers               |
| `--border-subtle`  | `#2A2A3C` | Card borders, dividers                     |
| `--border-focus`   | `#3B82F6` | Focus rings (solid blue)                   |
| `--text-primary`   | `#F1F5F9` | Headings, primary content                  |
| `--text-secondary` | `#94A3B8` | Labels, descriptions, secondary content    |
| `--text-muted`     | `#475569` | Placeholders, disabled text                |

### Light Mode

| Token              | Hex       | Usage                                      |
|--------------------|-----------|--------------------------------------------|
| `--bg-base`        | `#E8EDF2` | App background (muted blue-gray)           |
| `--bg-surface`     | `#F0F3F7` | Card/panel surfaces                        |
| `--bg-elevated`    | `#F5F7FA` | Modals, dropdowns, popovers               |
| `--border-subtle`  | `#CDD5DF` | Card borders, dividers                     |
| `--border-focus`   | `#3B82F6` | Focus rings                                |
| `--text-primary`   | `#1E293B` | Headings, primary content                  |
| `--text-secondary` | `#475569` | Labels, descriptions                       |
| `--text-muted`     | `#64748B` | Placeholders, disabled text                |

### Accent — Gradient System

The signature visual element. Used on primary CTAs, active navigation items,
live indicators, and key stat highlights. The gradient runs from emerald into the
project's trademark cyan — fresh, competitive, and instantly recognizable.

| Token                | Value                               | Usage                                |
|----------------------|-------------------------------------|--------------------------------------|
| `--accent-gradient`  | `linear-gradient(135deg, #10B981, #06B6D4)` | Primary buttons, active tab underlines, hero highlights |
| `--accent-emerald`   | `#10B981`                           | Secondary standalone accent          |
| `--accent-cyan`      | `#06B6D4`                           | Primary standalone accent (links, icons, logo) |
| `--accent-glow`      | `0 0 20px rgba(6, 182, 212, 0.3)`  | Box-shadow on hover/active states    |

### Semantic Colors

| Token              | Dark Hex  | Light Hex | Usage                            |
|--------------------|-----------|-----------|----------------------------------|
| `--color-win`      | `#22C55E` | `#16A34A` | Wins, positive deltas, gains     |
| `--color-loss`     | `#EF4444` | `#DC2626` | Losses, negative deltas, drops   |
| `--color-warning`  | `#F59E0B` | `#D97706` | Warnings, urgent inbox items     |
| `--color-info`     | `#3B82F6` | `#2563EB` | Informational badges, tooltips   |

---

## Typography

**Font stack:**
- **Display / Headings:** `Space Grotesk` (600/700) — geometric sans with distinctive details (asymmetric `a`, signature `g`). Gives the UI an immediately recognizable voice.
- **Body / Data:** `DM Sans` (400/500) — modern geometric sans with a wide x-height, clean at small sizes. Excellent built-in tabular figures.
- **Numbers:** No monospace font. All numeric data uses `DM Sans` with `font-variant-numeric: tabular-nums` for column alignment. This keeps numbers visually unified with surrounding text.

### Scale

| Level     | Size   | Font          | Weight | Line Height | Usage                         |
|-----------|--------|---------------|--------|-------------|-------------------------------|
| `display` | 32px   | Space Grotesk | 700    | 1.2         | Page titles (Dashboard, Draft)|
| `h1`      | 24px   | Space Grotesk | 700    | 1.3         | Section headings              |
| `h2`      | 20px   | Space Grotesk | 600    | 1.3         | Card titles                   |
| `h3`      | 16px   | Space Grotesk | 600    | 1.4         | Subsection headings           |
| `body`    | 16px   | DM Sans       | 400    | 1.5         | Default text (base size)      |
| `small`   | 14px   | DM Sans       | 400    | 1.5         | Labels, captions, timestamps  |
| `caption` | 12px   | DM Sans       | 400    | 1.4         | Fine print, badges            |
| `stat`    | 28px   | Space Grotesk | 700    | 1.1         | KPI callout values (big bold numbers) |

---

## Spacing System

Base unit: **4px**. All spacing is a multiple of 4.

| Token  | Value | Usage                                |
|--------|-------|--------------------------------------|
| `xs`   | 4px   | Inline icon gaps, tight padding      |
| `sm`   | 8px   | Intra-component padding              |
| `md`   | 12px  | Card internal padding                |
| `lg`   | 16px  | Between cards/sections               |
| `xl`   | 24px  | Major section gaps                   |
| `2xl`  | 32px  | Page-level margins                   |
| `3xl`  | 48px  | Hero/header spacing                  |

---

## Border Radius

| Token      | Value | Usage                             |
|------------|-------|-----------------------------------|
| `--r-sm`   | 4px   | Small badges, tags                |
| `--r-md`   | 8px   | Cards, buttons, inputs            |
| `--r-lg`   | 12px  | Modals, large panels              |
| `--r-full` | 9999px| Avatars, circular indicators      |

---

## Animation & Interaction

### Philosophy

Static UI is professional and calm. Interaction states introduce the **arena energy** —
glowing borders, gradient reveals, smooth transitions.

### Transition Defaults

- **Duration:** `150ms` for micro-interactions (hover, focus), `300ms` for layout shifts, `500ms` for page transitions
- **Easing:** `cubic-bezier(0.4, 0, 0.2, 1)` (standard Material easing)

### Hover States

- **Cards:** Subtle border glow (`--accent-glow`), border color shifts to `--accent-cyan`
- **Buttons (primary):** Gradient brightens, subtle scale `1.02`, glow shadow appears
- **Table rows:** Background lightens slightly, left border accent appears
- **Navigation items:** Gradient underline slides in from left

### Active/Live States

- **Live match indicator:** Pulsing cyan dot with `animation: pulse 2s infinite`
- **Simulating match:** Progress bar with gradient fill, shimmer animation
- **Urgent inbox:** Warning badge with subtle pulse

### Page Transitions

- **Enter:** Content slides up 12px + fades in over 300ms (staggered per card group)
- **Tab switch:** Cross-fade 200ms, no spatial movement

### Stat Animations

- **Stat bars:** Width animates from 0 to value on first render (400ms, ease-out)
- **KPI numbers:** Count up from 0 to value when entering viewport (300ms)
- **Delta badges:** Slide in from right + fade (200ms)

---

## Component Patterns

### Cards

- Surface color background, 1px `--border-subtle` border
- `--r-md` radius, `md` internal padding
- On hover: border transitions to `--accent-cyan`, `--accent-glow` shadow fades in

### Stat Bars (Player Attributes)

- 4px height, `--r-full` radius
- Track: `--border-subtle` background
- Fill: `--accent-gradient`
- Numeric value in `DM Sans` tabular-nums to the right, right-aligned

### Tables

- No outer border; thin `--border-subtle` row dividers
- Header row: `--text-secondary`, `small` size, uppercase tracking
- Alternating row backgrounds disabled — use hover highlight instead
- Numeric columns: `DM Sans` tabular-nums, right-aligned
- Sortable columns: chevron icon, active sort gets `--accent-cyan` color

### Buttons

- **Primary:** `--accent-gradient` background, white text, `--r-md`, glow on hover
- **Secondary:** Transparent bg, 1px `--border-subtle` border, `--text-primary` text, border glows on hover
- **Ghost:** No border, no bg, `--text-secondary` text, hover reveals subtle bg

### Navigation (Sidebar)

- Fixed left sidebar, 240px wide (collapsible to 56px icon-only)
- Logo at top, nav groups with section labels
- Active item: gradient left border (3px), `--bg-elevated` background, `--text-primary` text
- Inactive: `--text-secondary`, no background
- Hover: `--bg-surface` background slides in

### Badges / Tags

- `--r-sm` radius, `small` text, `xs` horizontal padding
- Win: `--color-win` text with 10% opacity background
- Loss: `--color-loss` text with 10% opacity background
- Role badges: `--accent-cyan` text with 10% opacity background

---

## Iconography

- **Style:** Lucide icons — 1.5px stroke, consistent 20px size in nav, 16px inline
- **Color:** Inherits `--text-secondary` by default; active/accent states use `--accent-cyan`
- **Usage rules:**
  - Always paired with text labels in nav (except collapsed state)
  - Stat icons (gold, kills, cs) use a fixed color from the semantic palette
  - Never decorative-only — every icon conveys meaning

---

## Layout Structure

```text
┌──────────────────────────────────────────────────┐
│  Sidebar (240px)  │  Main Content                │
│                   │                              │
│  [Logo]           │  ┌─ Top Bar ──────────────┐  │
│                   │  │ Page Title  [Clock] [⚙] │  │
│  ── Team ──       │  └────────────────────────┘  │
│  Dashboard        │                              │
│  Roster           │  ┌─ Content Grid ─────────┐  │
│  Schedule         │  │                        │  │
│                   │  │  Cards / Tables /       │  │
│  ── League ──     │  │  Data panels            │  │
│  Standings        │  │                        │  │
│  Results          │  │                        │  │
│                   │  └────────────────────────┘  │
│  ── Finance ──    │                              │
│  Budget           │                              │
│                   │                              │
│  ── Staff ──      │                              │
│  Coaching         │                              │
│  Scouting         │                              │
└──────────────────────────────────────────────────┘
```

---

## Implementation Notes

- **CSS framework:** TailwindCSS v4, extended with design tokens as CSS custom properties
- **Component library:** shadcn/ui (customized to match design language)
- **Icons:** Lucide React
- **Fonts:** Space Grotesk + DM Sans, bundled locally via `@fontsource` (no network dependency)
- **Theme switching:** CSS custom properties toggled via `data-theme="dark|light"` on `<html>`
- **Animations:** Tailwind `animate-*` utilities + custom keyframes for glow/pulse effects
- **Runtime:** Offline desktop app via Tauri — no external network dependencies for UI assets
