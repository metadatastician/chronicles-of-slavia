# SPDX-License-Identifier: AGPL-3.0-or-later
# Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>
#
# FORMAL VERIFICATION (PROOFS)
#
# Imported by ../../Justfile via `import? "build/just/proofs.just"`.
# Checks the formal proofs under verification/proofs/ across Idris2, Lean4,
# Agda, Coq and TLA+, and scans them for soundness-defeating constructs.
# Run the full sweep with `just proof-check-all`.
#
# CONVERGED (issue #14) onto the canonical estate shape landed in
# hyperpolymath/rsr-template-repo (`fix(proofs): make proof gates honest —
# missing toolchain is fatal, not a skip`, #147): the four dependently-typed
# provers now run through the shared `scripts/check-proofs.sh <prover>` +
# `verification/proofs/<prover>/MANIFEST` mechanism instead of bespoke inline
# recipes, so a proof absent from its MANIFEST is an error (gated by default)
# and a quarantined module that starts passing fails the gate until promoted.
#
# chronicles keeps two things beyond the template, since it has content the
# template doesn't: TLA+ (`proof-check-tla`, unchanged bespoke recipe — the
# template has no TLA+ specs to converge against) and TLA+ coverage in the
# dangerous-construct scan (`scripts/scan-dangerous.sh`, chronicles-extended
# to also scan `.tla` files for the `OMITTED` proof-obligation escape hatch).
#
# Everything PR #10 established is preserved by the converged scripts:
#   * A missing prover is FATAL, never a skip.
#   * The verdict is `exit 0 AND no error marker in the captured output`.
#   * Provers are invoked from the correct source root (see each MANIFEST).
#   * An empty proof directory / a proof file absent from its MANIFEST is
#     FATAL too — "no files, so nothing failed" is false green in disguise.

# Full sweep: every prover + the dangerous-pattern scan. Runs them ALL (no
# fail-fast) so one invocation shows the complete picture, then fails if any
# stage did not verify.
proof-check-all:
    #!/usr/bin/env bash
    set -uo pipefail
    echo "════════════════════════════════════════════════════════════"
    echo " Formal proof sweep — verification/proofs/"
    echo "════════════════════════════════════════════════════════════"
    recipes=(proof-check-idris2 proof-check-lean4 proof-check-agda proof-check-coq proof-check-tla proof-scan-dangerous)
    declare -A result
    fail=0
    for r in "${recipes[@]}"; do
        echo ""
        echo "──────── ${r} ────────"
        if just "${r}"; then
            result[$r]="PASS"
        else
            result[$r]="FAIL"
            fail=1
        fi
    done
    echo ""
    echo "════════════════════════════════════════════════════════════"
    echo " SUMMARY"
    echo "════════════════════════════════════════════════════════════"
    for r in "${recipes[@]}"; do
        printf '  %-24s %s\n' "${r}" "${result[$r]}"
    done
    echo ""
    if [ "$fail" -ne 0 ]; then
        echo "RESULT: FAIL — one or more proof stages did not verify (see above)." >&2
        exit 1
    fi
    echo "RESULT: PASS — every proof stage verified."

# ── Idris2 ─────────────────────────────────────────────────────────────────
proof-check-idris2:
    ./scripts/check-proofs.sh idris2

# ── Lean4 ──────────────────────────────────────────────────────────────────
proof-check-lean4:
    ./scripts/check-proofs.sh lean4

# ── Agda ───────────────────────────────────────────────────────────────────
proof-check-agda:
    ./scripts/check-proofs.sh agda

# ── Coq ────────────────────────────────────────────────────────────────────
proof-check-coq:
    ./scripts/check-proofs.sh coq

# ── TLA+ ───────────────────────────────────────────────────────────────────
# Not part of the converged template mechanism — chronicles-only content, kept
# as its own bespoke recipe. Parses (SANY) the specs. Requires java +
# tla2tools.jar via $TLA2TOOLS_JAR. Absent toolchain is FATAL, NOT a skip: the
# spec is otherwise UNVERIFIED. (SANY validates syntax + level/semantics; full
# TLC model-checking needs a .cfg per spec and is a deeper follow-up, tracked
# separately.)
proof-check-tla:
    #!/usr/bin/env bash
    set -uo pipefail
    echo "=== TLA+ specs (SANY parse) ==="
    root="verification/proofs/tlaplus"
    if [ ! -d "$root" ]; then
        echo "FATAL: proof source root '$root' does not exist." >&2
        exit 1
    fi
    mapfile -t files < <(find "$root" -name '*.tla' | sort)
    if [ "${#files[@]}" -eq 0 ]; then
        echo "FATAL: no .tla files under '$root' — nothing was checked." >&2
        exit 1
    fi
    jar="${TLA2TOOLS_JAR:-}"
    if ! command -v java >/dev/null 2>&1 || [ -z "$jar" ] || [ ! -f "$jar" ]; then
        echo "FATAL: TLA+ toolchain not provisioned — need java and tla2tools.jar" >&2
        echo "  via \$TLA2TOOLS_JAR. This is NOT a skip: the following spec(s) are" >&2
        echo "  UNVERIFIED and this gate declines to certify them:" >&2
        printf '    %s\n' "${files[@]}" >&2
        exit 1
    fi
    errors=0
    for f in "${files[@]}"; do
        printf '  %-40s ' "$f"
        out=$(java -cp "$jar" tla2sany.SANY "$f" 2>&1); rc=$?
        if [ "$rc" -eq 0 ] && ! grep -qiE 'error|fatal' <<<"$out"; then
            echo "OK (parsed)"
        else
            echo "FAIL (exit=$rc)"
            sed 's/^/      │ /' <<<"$out" | head -25 >&2
            errors=$((errors + 1))
        fi
    done
    if [ "$errors" -gt 0 ]; then
        echo "FAIL: $errors of ${#files[@]} TLA+ spec(s) did not parse." >&2
        exit 1
    fi
    echo "PASS: all ${#files[@]} TLA+ spec(s) parsed."

# ── Dangerous-construct scan ───────────────────────────────────────────────
# Converged onto scripts/scan-dangerous.sh (issue #14), chronicles-extended to
# also cover .tla files (the template has no TLA+ content). Comment-aware:
# matches inside comments are stripped first, so a file that merely
# *documents* "no believe_me" is not falsely flagged.
proof-scan-dangerous:
    ./scripts/scan-dangerous.sh

# ── Informational status (NOT a gate) ──────────────────────────────────────
proof-status:
    #!/usr/bin/env bash
    set -uo pipefail
    echo "=== Proof inventory ==="
    for lang in idris2:idr lean4:lean agda:agda coq:v tlaplus:tla; do
        dir="verification/proofs/${lang%%:*}"; ext="${lang##*:}"
        n=$(find "$dir" -name "*.$ext" 2>/dev/null | wc -l)
        printf '  %-9s %s file(s)\n' "${lang%%:*}:" "$n"
    done
