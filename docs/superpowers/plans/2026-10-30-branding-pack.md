# Branding Pack Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Ship Glossa engraved-specimen brand assets plus dual in-terminal branding.

**Architecture:** Static masters under `brand/`; CLI banner module in `glossa-cli` triggered by `about` / `GLOSSA_BANNER`.

**Tech Stack:** SVG/PNG assets, Rust clap CLI, IBM Plex Mono path in SVG.

## Global Constraints

- Palette and lockups per `docs/superpowers/specs/2026-10-30-branding-pack-design.md`
- Quiet CLI by default; banner only on `about` or `GLOSSA_BANNER=1`
- Honour `--no-color` / `NO_COLOR`
- Ignore `.superpowers/`

## File map

- `brand/**` — tokens, icon, logo, letterhead, social, terminal, guide
- `crates/glossa-cli/src/brand.rs` — banner text + colour helper
- `crates/glossa-cli/src/main.rs` — `About` subcommand + banner env
- `README.md`, `.gitignore`

## Tasks

- [x] Write design spec
- [ ] Add `brand/` assets + guide + tokens
- [ ] Add CLI `about` + `GLOSSA_BANNER` dual branding
- [ ] Wire README + gitignore
- [ ] Test CLI quiet vs banner; open PR; merge to main
