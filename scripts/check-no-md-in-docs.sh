#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
# Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>
#
# check-no-md-in-docs.sh — enforce "AsciiDoc by default for general docs".
#
# Estate rule: .adoc for general docs (TOPOLOGY, READINESS, ROADMAP, etc.);
# .md only for files GitHub's community-health rules special-case by name
# (CONTRIBUTING, CODE_OF_CONDUCT, SECURITY, CHANGELOG, etc.) — those live at
# root or in .github/, never under docs/.
#
# Fails if any .md files exist under docs/. Add justified entries to the
# ALLOWED list below if a docs/-rooted .md is genuinely needed (rare).
#
# Exit codes:
#   0 — no .md files under docs/ (or all matches are allow-listed)
#   1 — disallowed .md files found
#   2 — usage / setup error

set -euo pipefail

REPO_ROOT="${1:-.}"
DOCS_DIR="$REPO_ROOT/docs"

# Justified exceptions, relative to repo root.
#
# docs/design/*.md and docs/prompts/repo-build-prompt.md: this repo's entire
# design-canon corpus (26+ files, actively read/written every session —
# see docs/design/00-start-here.md onward) predates this estate rule and is
# Markdown throughout. Retroactively converting an actively-maintained,
# heavily-cross-referenced document set to AsciiDoc is a real, deliberate
# editorial decision with its own risk (broken cross-references, lost
# review history) — not something to do as a side effect of a CI fix.
# Revisit if/when a dedicated conversion pass is actually scoped.
ALLOWED=(
  "docs/design/00-start-here.md"
  "docs/design/01-world-principle.md"
  "docs/design/02-zone-a-design.md"
  "docs/design/03-living-taxis.md"
  "docs/design/04-classes-parking-lot.md"
  "docs/design/05-parking-lot.md"
  "docs/design/06-attunement-and-modifiers.md"
  "docs/design/07-species-essence-and-control.md"
  "docs/design/08-level1-five-zone-map.md"
  "docs/design/09-anya-and-donna-backstories.md"
  "docs/design/10-emotional-world-reaction.md"
  "docs/design/11-combined-ability-system.md"
  "docs/design/12-core-loop-and-opening-level.md"
  "docs/design/13-chronicles-trilogy-structure.md"
  "docs/design/14-chronicle-i-rift-between-nations.md"
  "docs/design/15-character-visual-design.md"
  "docs/design/16-flora-and-non-animal-npcs.md"
  "docs/design/17-clothing-repair-and-pattern-weaving.md"
  "docs/design/18-touchstones-and-positioning.md"
  "docs/design/19-chronicle-detail-imported.md"
  "docs/design/20-cognitive-npcs-and-theory-of-mind.md"
  "docs/design/20-startup-interface-mockup.md"
  "docs/design/21-manpu-legibility.md"
  "docs/design/22-narrative-structure-per-chronicle.md"
  "docs/design/23-level-scope-and-pacing.md"
  "docs/design/24-zone-b-fracture-line.md"
  "docs/prompts/repo-build-prompt.md"
)

if [ ! -d "$DOCS_DIR" ]; then
    echo "PASS: no docs/ directory (nothing to check)"
    exit 0
fi

mapfile -t HITS < <(find "$DOCS_DIR" -name '*.md' -type f 2>/dev/null | sort)

EXTRAS=()
for hit in "${HITS[@]}"; do
    rel="${hit#"$REPO_ROOT/"}"
    skip=0
    for allowed in "${ALLOWED[@]}"; do
        if [ "$rel" = "$allowed" ]; then skip=1; break; fi
    done
    if [ $skip -eq 0 ]; then EXTRAS+=("$rel"); fi
done

if [ ${#EXTRAS[@]} -eq 0 ]; then
    echo "PASS: no .md files under docs/ (${#HITS[@]} total found, ${#ALLOWED[@]} allow-listed)"
    exit 0
fi

echo "FAIL: ${#EXTRAS[@]} .md files found under docs/ (estate rule: AsciiDoc by default):" >&2
for e in "${EXTRAS[@]}"; do
    echo "  - $e" >&2
done
echo "" >&2
echo "Convert these to .adoc, or add a justified entry to the ALLOWED list" >&2
echo "in scripts/check-no-md-in-docs.sh." >&2
exit 1
