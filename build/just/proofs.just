# SPDX-License-Identifier: MPL-2.0
# Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>
#
# FORMAL VERIFICATION (PROOFS)
#
# Imported by ../../Justfile via `import? "build/just/proofs.just"`.
# Checks the formal proofs under verification/proofs/ across Idris2, Lean4,
# Agda, Coq and TLA+, and scans them for soundness-defeating constructs.
# Run the full sweep with `just proof-check-all`.
#
# DESIGN — this gate must be able to FAIL (it previously could not):
#   * A missing prover is FATAL, never a skip. A checker that is not present
#     has verified nothing; reporting "SKIP" and exit 0 in that case is a lie.
#     If a recipe cannot run its tool, it exits non-zero and says why.
#   * The verdict is `exit 0 AND no error marker in the captured output`, not
#     the exit code alone. `idris2 --check` in particular can print `Error:`
#     on some unresolved names while still exiting 0, so exit-code-only
#     under-reports. stderr is captured and shown, never discarded.
#   * Provers are invoked from the correct source root so module/path
#     resolution matches the declared module names (e.g. an Agda file whose
#     module is `Foo` must be checked with its directory as the include root,
#     or it fails for reasons unrelated to the proof — this was a real
#     false-FAIL in the previous gate).
#   * An empty proof directory is FATAL too — "no files, so nothing failed"
#     is the same false green in a different disguise.

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
# Dependent-type proofs (ABI layout/pointer/platform, core types). Checked from
# the idris2 source root so `module ABI.Foo` resolves to ABI/Foo.idr.
proof-check-idris2:
    #!/usr/bin/env bash
    set -uo pipefail
    echo "=== Idris2 proofs ==="
    if ! command -v idris2 >/dev/null 2>&1; then
        echo "FATAL: idris2 is not installed. A proof checker that is absent has" >&2
        echo "  verified nothing — this is a gate FAILURE, not a skip. Provision" >&2
        echo "  idris2 (see verification/proofs/README.adoc) and re-run." >&2
        exit 1
    fi
    root="verification/proofs/idris2"
    if [ ! -d "$root" ]; then
        echo "FATAL: proof source root '$root' does not exist." >&2
        exit 1
    fi
    mapfile -t files < <(cd "$root" && find . -name '*.idr' | sed 's|^\./||' | sort)
    if [ "${#files[@]}" -eq 0 ]; then
        echo "FATAL: no .idr files under '$root' — nothing was checked." >&2
        exit 1
    fi
    errors=0
    for rel in "${files[@]}"; do
        printf '  %-28s ' "$rel"
        out=$(cd "$root" && idris2 --check "$rel" 2>&1); rc=$?
        if [ "$rc" -eq 0 ] && ! grep -q 'Error:' <<<"$out"; then
            echo "OK"
        else
            echo "FAIL (exit=$rc)"
            sed 's/^/      │ /' <<<"$out" | head -25 >&2
            errors=$((errors + 1))
        fi
    done
    rm -rf "$root/build"
    if [ "$errors" -gt 0 ]; then
        echo "FAIL: $errors of ${#files[@]} Idris2 proof(s) did not verify." >&2
        exit 1
    fi
    echo "PASS: all ${#files[@]} Idris2 proof(s) verified."

# ── Lean4 ──────────────────────────────────────────────────────────────────
proof-check-lean4:
    #!/usr/bin/env bash
    set -uo pipefail
    echo "=== Lean4 proofs ==="
    if ! command -v lean >/dev/null 2>&1; then
        echo "FATAL: lean is not installed / not on PATH. A proof checker that is" >&2
        echo "  absent has verified nothing — gate FAILURE, not a skip. Provision" >&2
        echo "  lean (elan) and ensure it is on PATH, then re-run." >&2
        exit 1
    fi
    root="verification/proofs/lean4"
    if [ ! -d "$root" ]; then
        echo "FATAL: proof source root '$root' does not exist." >&2
        exit 1
    fi
    mapfile -t files < <(cd "$root" && find . -name '*.lean' | sed 's|^\./||' | sort)
    if [ "${#files[@]}" -eq 0 ]; then
        echo "FATAL: no .lean files under '$root' — nothing was checked." >&2
        exit 1
    fi
    errors=0
    for rel in "${files[@]}"; do
        printf '  %-28s ' "$rel"
        out=$(cd "$root" && lean "$rel" 2>&1); rc=$?
        if [ "$rc" -eq 0 ] && ! grep -qi 'error:' <<<"$out"; then
            echo "OK"
        else
            echo "FAIL (exit=$rc)"
            sed 's/^/      │ /' <<<"$out" | head -25 >&2
            errors=$((errors + 1))
        fi
    done
    if [ "$errors" -gt 0 ]; then
        echo "FAIL: $errors of ${#files[@]} Lean4 proof(s) did not verify." >&2
        exit 1
    fi
    echo "PASS: all ${#files[@]} Lean4 proof(s) verified."

# ── Agda ───────────────────────────────────────────────────────────────────
# Each file is checked with its own directory as the include root, so a file
# declaring `module Foo` resolves correctly regardless of the tree it sits in.
proof-check-agda:
    #!/usr/bin/env bash
    set -uo pipefail
    echo "=== Agda proofs ==="
    if ! command -v agda >/dev/null 2>&1; then
        echo "FATAL: agda is not installed. A proof checker that is absent has" >&2
        echo "  verified nothing — gate FAILURE, not a skip. Provision agda." >&2
        exit 1
    fi
    root="verification/proofs/agda"
    if [ ! -d "$root" ]; then
        echo "FATAL: proof source root '$root' does not exist." >&2
        exit 1
    fi
    mapfile -t files < <(cd "$root" && find . -name '*.agda' | sed 's|^\./||' | sort)
    if [ "${#files[@]}" -eq 0 ]; then
        echo "FATAL: no .agda files under '$root' — nothing was checked." >&2
        exit 1
    fi
    errors=0
    for rel in "${files[@]}"; do
        printf '  %-28s ' "$rel"
        # --safe rejects postulates / unsafe pragmas outright; run from the
        # file's own dir so `module <name>` matches its path.
        dir="$root/$(dirname "$rel")"; base="$(basename "$rel")"
        out=$(cd "$dir" && agda --safe "$base" 2>&1); rc=$?
        if [ "$rc" -eq 0 ]; then
            echo "OK"
        else
            echo "FAIL (exit=$rc)"
            sed 's/^/      │ /' <<<"$out" | head -25 >&2
            errors=$((errors + 1))
        fi
    done
    find "$root" -name '*.agdai' -delete 2>/dev/null || true
    rm -rf "$root/_build" 2>/dev/null || true
    if [ "$errors" -gt 0 ]; then
        echo "FAIL: $errors of ${#files[@]} Agda proof(s) did not verify." >&2
        exit 1
    fi
    echo "PASS: all ${#files[@]} Agda proof(s) verified."

# ── Coq ────────────────────────────────────────────────────────────────────
proof-check-coq:
    #!/usr/bin/env bash
    set -uo pipefail
    echo "=== Coq proofs ==="
    if ! command -v coqc >/dev/null 2>&1; then
        echo "FATAL: coqc is not installed. A proof checker that is absent has" >&2
        echo "  verified nothing — gate FAILURE, not a skip. Provision coq." >&2
        exit 1
    fi
    root="verification/proofs/coq"
    if [ ! -d "$root" ]; then
        echo "FATAL: proof source root '$root' does not exist." >&2
        exit 1
    fi
    mapfile -t files < <(cd "$root" && find . -name '*.v' | sed 's|^\./||' | sort)
    if [ "${#files[@]}" -eq 0 ]; then
        echo "FATAL: no .v files under '$root' — nothing was checked." >&2
        exit 1
    fi
    errors=0
    for rel in "${files[@]}"; do
        printf '  %-28s ' "$rel"
        out=$(cd "$root" && coqc "$rel" 2>&1); rc=$?
        if [ "$rc" -eq 0 ] && ! grep -q 'Error' <<<"$out"; then
            echo "OK"
        else
            echo "FAIL (exit=$rc)"
            sed 's/^/      │ /' <<<"$out" | head -25 >&2
            errors=$((errors + 1))
        fi
    done
    # Coq compilation artifacts, kept out of the tree.
    find "$root" \( -name '*.vo' -o -name '*.glob' -o -name '*.vos' \
        -o -name '*.vok' -o -name '*.aux' -o -name '.*.aux' \) -delete 2>/dev/null || true
    if [ "$errors" -gt 0 ]; then
        echo "FAIL: $errors of ${#files[@]} Coq proof(s) did not verify." >&2
        exit 1
    fi
    echo "PASS: all ${#files[@]} Coq proof(s) verified."

# ── TLA+ ───────────────────────────────────────────────────────────────────
# Parses (SANY) the specs. Requires java + tla2tools.jar via $TLA2TOOLS_JAR.
# Absent toolchain is FATAL, NOT a skip: the spec is otherwise UNVERIFIED.
# (SANY validates syntax + level/semantics; full TLC model-checking needs a
# .cfg per spec and is a deeper follow-up, tracked separately.)
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
# Flags soundness escape hatches (believe_me, sorry, Admitted, postulate,
# axioms, unsafeCoerce, …) in the proof CODE. Comment-aware: matches inside
# comments are stripped first, so a file that merely *documents* "no believe_me"
# is not falsely flagged (a real hazard — the template stubs say exactly that).
proof-scan-dangerous:
    #!/usr/bin/env bash
    set -uo pipefail
    echo "=== Dangerous-construct scan ==="
    root="verification/proofs"
    pattern='believe_me|really_believe_me|assert_total|idris_crash|postulate|sorry|Admitted|\badmit\b|\bAxiom\b|unsafeCoerce|Obj\.magic|TERMINATING'
    mapfile -t files < <(find "$root" -type f \( -name '*.idr' -o -name '*.lean' \
        -o -name '*.agda' -o -name '*.v' -o -name '*.tla' \) | sort)
    if [ "${#files[@]}" -eq 0 ]; then
        echo "FATAL: no proof files under '$root' — nothing was scanned." >&2
        exit 1
    fi
    flagged=0
    for f in "${files[@]}"; do
        case "$f" in
            *.v)   stripped=$(perl -0777 -pe 's{\(\*.*?\*\)}{}gs' "$f") ;;          # Coq block comments only
            *.lean) stripped=$(perl -0777 -pe 's{/-.*?-/}{}gs; s{--[^\n]*}{}g' "$f") ;;  # Lean block + line
            *.idr|*.agda) stripped=$(perl -0777 -pe 's{\{-.*?-\}}{}gs; s{--[^\n]*}{}g' "$f") ;; # block + line
            *)     stripped=$(cat "$f") ;;
        esac
        matches=$(grep -nE "$pattern" <<<"$stripped" || true)
        if [ -n "$matches" ]; then
            echo "  DANGEROUS: $f"
            sed 's/^/      │ /' <<<"$matches"
            flagged=$((flagged + 1))
        fi
    done
    if [ "$flagged" -gt 0 ]; then
        echo "FAIL: $flagged file(s) contain soundness-defeating constructs." >&2
        exit 1
    fi
    echo "PASS: no dangerous constructs in proof code (${#files[@]} file(s) scanned)."

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
