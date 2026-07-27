#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
# Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>
#
# scan-dangerous.sh — flag dangerous/unsafe constructs USED in proof code.
#
# Dangerous constructs (believe_me, assert_total, postulate, sorry, Admitted,
# unsafeCoerce, Obj.magic) escape the proof obligation and must not appear in
# real proofs.  BUT the previous `proof-scan-dangerous` recipe grepped raw
# lines, so a comment that merely NAMED a banned construct —
#     -- All proofs MUST be constructive (no believe_me, no assert_total).
# — tripped the gate.  A check that fires on its own documentation is a
# false-positive gate: it cries wolf, trains people to override it (violating
# "squabble, don't bypass"), and wired into CI it turns the tree red for
# nothing.  This version strips comments first, so only real usage is flagged.
#
# Comment syntax handled: `--` line + `{- -}` block (Idris2/Lean4/Agda),
# `(* *)` block (Coq), `\*` line + `(* *)` block (TLA+ — chronicles-added;
# the upstream template has no TLA+ content to scan). Comment bodies are
# blanked in place, so reported line numbers still match the source.
#
# Exit: 0 = clean; 1 = a proof uses a dangerous construct in code.

set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

# OMITTED is TLA+/TLAPS's proof-obligation escape hatch — the same category
# of hazard as believe_me/sorry/Admitted in the dependently-typed provers.
PATTERNS='believe_me|assert_total|postulate|sorry|Admitted|unsafeCoerce|Obj\.magic|\bOMITTED\b'
dangerous=0

# Blank comment content while preserving line count (so grep -n stays accurate).
strip_comments() {
  local ext="${1##*.}" lc lc2 bo bc
  # lc2 is a second line-comment token. Idris2 documentation comments start
  # `|||`, which is a comment but does not start with `--`, so without this a
  # doc comment explaining *why* a construct was avoided is flagged as using
  # it. Matched as the full three characters, so `data X = A | B` is untouched.
  case "$ext" in
    idr)           lc='--'; lc2='|||'; bo='{-'; bc='-}' ;;
    lean|agda)     lc='--'; lc2='';    bo='{-'; bc='-}' ;;
    v)             lc='';   lc2='';    bo='(*'; bc='*)' ;;   # Coq block only
    tla)           lc='\\*'; lc2='';   bo='(*'; bc='*)' ;;   # TLA+ line + block
    *)             lc='';   lc2='';    bo='';   bc=''   ;;
  esac
  awk -v lc="$lc" -v lc2="$lc2" -v bo="$bo" -v bc="$bc" '
    BEGIN { inblk = 0 }
    {
      line = $0; out = ""; i = 1; n = length(line)
      while (i <= n) {
        if (inblk) {
          if (bc != "" && substr(line,i,length(bc)) == bc) { inblk = 0; i += length(bc) }
          else { i++ }
        } else if (bo != "" && substr(line,i,length(bo)) == bo) {
          inblk = 1; i += length(bo)
        } else if (lc != "" && substr(line,i,length(lc)) == lc) {
          break                       # rest of the line is a line-comment
        } else if (lc2 != "" && substr(line,i,length(lc2)) == lc2) {
          break                       # rest of the line is a doc comment
        } else {
          out = out substr(line,i,1); i++
        }
      }
      print out
    }' "$1"
}

while IFS= read -r f; do
  [ -z "$f" ] && continue
  matches="$(strip_comments "$f" | grep -nE "$PATTERNS" || true)"
  if [ -n "$matches" ]; then
    echo "  DANGEROUS (used in code): $f"
    printf '%s\n' "$matches" | sed 's/^/    /'
    dangerous=$((dangerous + 1))
  fi
done < <(find verification/proofs \
           \( -name '*.idr' -o -name '*.lean' -o -name '*.agda' -o -name '*.v' -o -name '*.tla' \) \
           -not -path '*/build/*' 2>/dev/null | sort)

echo
if [ "$dangerous" -gt 0 ]; then
  echo "FAIL: $dangerous file(s) use dangerous constructs in proof CODE (not comments)"
  exit 1
fi
echo "PASS: no dangerous constructs used in proof code"
