# Glossa branding pack — design

**Date:** 2026-10-30  
**Status:** Approved (user: hurry through remaining sections; implement)

## Direction

- Personality: **Manuscript terminal** (academic archive + CLI instrument)
- Approach: **Engraved specimen** — custom capital **G** as type-specimen mark
- Palette: cool vellum + graphite + iron-blue accent (high-contrast / colour-blind-safe)
- Icon: custom capital **G**
- Terminal: **dual** — quiet by default; compact banner via `glossa about` or `GLOSSA_BANNER=1`
- Letterhead: system for **print correspondence and future research exports**
- Lockups chosen in companion: **letterhead A** (horizontal header), **GitHub OG A** (archive spread)

## Tokens

| Token | Hex | Role |
|-------|-----|------|
| `vellum` | `#E8E6E1` | Light paper / backgrounds |
| `graphite` | `#1C1F24` | Primary ink |
| `iron-blue` | `#2F4A6E` | Accent only |
| `rule` | `#9AA1AB` | Hairlines |
| `ink-muted` | `#5C6370` | Secondary / meta |

Dark surfaces invert: graphite field, vellum ink, iron-blue highlight. Never encode meaning by colour alone.

## Typography

- Product title in prose: **Glossa**
- Binary / wordmark: lowercase **`glossa`**
- Brand surfaces: IBM Plex Mono (or embedded mono path in SVG)
- Letterhead/PDF body may use a readable serif later; mark never uses display serif flourishes

## Mark rules

- Single engraved **G**; open aperture; slight thick/thin; no anatomical tongue; no SaaS gradients
- Clear space ≈ crossbar gap height; simplified stroke at 16×16
- Lockups: icon-only, horizontal `G + glossa`, stacked for squares

## Asset compositions

### Icon

SVG master + PNG raster set (1024 → 16). Light (vellum) and dark (graphite) variants.

### Logo

Horizontal and stacked SVG lockups.

### Letterhead (A)

A4-oriented SVG: left `G` + `glossa` + short tagline, graphite hairline, footer `glossa.tools` / “offline-first linguistics”. Reusable as PDF export chrome later.

### GitHub social (A · Archive spread)

1280×640: vellum field, large G, wordmark, full tagline between rules, repo URL + “offline · reproducible · CLI/TUI”.

### In-terminal (dual)

- Default interactive / bare `glossa`: quiet one-liner (version + Stage message) — unchanged tone
- Banner: compact 5–7 line ASCII/ANSI G + wordmark + tagline
- Triggers: `glossa about`, or env `GLOSSA_BANNER=1` on bare invocation
- Honour `--no-color` / `NO_COLOR` (plain ASCII, no ANSI)

## Repo layout

```
brand/
  README.md          # brand guide
  tokens.json
  icon/
  logo/
  letterhead/
  social/
  terminal/
docs/superpowers/specs/2026-10-30-branding-pack-design.md
```

Wire README to brand guide. Do not commit `.superpowers/` brainstorm sessions (gitignore).

## Out of scope

- Website redesign at glossa.tools
- Full TUI theme implementation (tokens only inform later Stage 4)
- Trademark filing
