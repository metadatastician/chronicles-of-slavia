# Chronicles of Slavia — Assets

This directory contains all production and reference assets for the game.

## Structure

- `art/` — Concept art, production sprites, tilesets, and embroidery patterns
- `audio/` — Music, sound effects, ambient sound, and voice lines
- `video/` — Cutscenes and reference video
- `fonts/` — Typefaces (PT Serif OFL-1.1, plus any custom fonts)
- `branding/` — Logos, colour palette, and style guide

## Art pipeline

1. Concept art lives in `art/concept/` — reference images, sketches, mood boards
2. Production sprites go in `art/sprites/` — character rigs, environment tiles, UI elements
3. Tilesets in `art/tiles/` — per-zone tile data
4. Embroidery patterns in `art/patterns/` — Berehynia, Kanatitsa, Elbetitsa lines

## Audio pipeline

1. Music in `audio/music/` — per-zone themes, menu music, chronicle themes
2. Sound effects in `audio/sfx/` — abilities, environment, UI, animals, rift
3. Ambient sound in `audio/ambient/` — per-zone ambient beds
4. Voice (if added) in `audio/voice/`

## Naming conventions

- Lowercase with hyphens: `anya-dash-burst.ogg`
- Zone-prefixed where applicable: `zone-a-forest-ambient.ogg`
- Frame-numbered for animations: `anya-run-01.png`, `anya-run-02.png`
