#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
# Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>
#
# Regression tests for the foundation CI security pins.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

PASS_COUNT=0
FAIL_COUNT=0

pass() {
    printf 'PASS: %s\n' "$1"
    PASS_COUNT=$((PASS_COUNT + 1))
}

fail() {
    printf 'FAIL: %s\n' "$1" >&2
    FAIL_COUNT=$((FAIL_COUNT + 1))
}

assert_equals() {
    local description="$1"
    local expected="$2"
    local actual="$3"

    if [[ "$actual" == "$expected" ]]; then
        pass "$description"
    else
        fail "$description (expected '$expected', got '$actual')"
    fi
}

assert_matches() {
    local description="$1"
    local pattern="$2"
    local actual="$3"

    if grep -Eq "$pattern" <<<"$actual"; then
        pass "$description"
    else
        fail "$description (missing pattern '$pattern')"
    fi
}

workflow_uses() {
    local workflow="$1"

    awk '
        /^[[:space:]]*(-[[:space:]]*)?uses:[[:space:]]*/ {
            sub(/^[[:space:]]*(-[[:space:]]*)?uses:[[:space:]]*/, "")
            sub(/[[:space:]]*#.*/, "")
            gsub(/^['\''"]|['\''"]$/, "")
            print
        }
    ' "$workflow"
}

assert_all_refs_are_immutable() {
    local description="$1"
    shift
    local workflow ref refs_checked=0
    local invalid_refs=""

    for workflow in "$@"; do
        while IFS= read -r ref; do
            [[ -z "$ref" || "$ref" == ./* || "$ref" == docker://* ]] && continue
            refs_checked=$((refs_checked + 1))
            if [[ ! "$ref" =~ @[0-9a-f]{40}$ ]]; then
                invalid_refs+="${workflow#$REPO_ROOT/}: $ref"$'\n'
            fi
        done < <(workflow_uses "$workflow")
    done

    if [[ "$refs_checked" -eq 6 && -z "$invalid_refs" ]]; then
        pass "$description"
    else
        fail "$description (checked $refs_checked refs; expected 6)"
        [[ -n "$invalid_refs" ]] && printf '%s' "$invalid_refs" >&2
    fi
}

CODEQL_WORKFLOW="$REPO_ROOT/.github/workflows/codeql.yml"
GOVERNANCE_WORKFLOW="$REPO_ROOT/.github/workflows/governance.yml"
HYPATIA_WORKFLOW="$REPO_ROOT/.github/workflows/hypatia-scan.yml"
SCORECARD_WORKFLOW="$REPO_ROOT/.github/workflows/scorecard.yml"

for workflow in \
    "$CODEQL_WORKFLOW" \
    "$GOVERNANCE_WORKFLOW" \
    "$HYPATIA_WORKFLOW" \
    "$SCORECARD_WORKFLOW"; do
    if [[ ! -r "$workflow" ]]; then
        printf 'FATAL: workflow is not readable: %s\n' "$workflow" >&2
        exit 1
    fi
done

CODEQL_USES="$(workflow_uses "$CODEQL_WORKFLOW")"
EXPECTED_CODEQL_USES=$'actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1\ngithub/codeql-action/init@cdf488f595d80d6e07e03d4674febd5ab45fa938\ngithub/codeql-action/analyze@cdf488f595d80d6e07e03d4674febd5ab45fa938'
assert_equals \
    "CodeQL uses the reviewed checkout, init, and analyze revisions" \
    "$EXPECTED_CODEQL_USES" \
    "$CODEQL_USES"

CHECKOUT_BLOCK="$(awk '
    /^[[:space:]]*- name:[[:space:]]*Checkout[[:space:]]*$/ { capture = 1 }
    capture && /^[[:space:]]*- name:/ && $0 !~ /Checkout[[:space:]]*$/ { exit }
    capture { print }
' "$CODEQL_WORKFLOW")"
assert_matches \
    "CodeQL checkout disables persisted Git credentials" \
    "^[[:space:]]+persist-credentials:[[:space:]]+false[[:space:]]*$" \
    "$CHECKOUT_BLOCK"

assert_equals \
    "governance calls the reviewed standards revision" \
    "hyperpolymath/standards/.github/workflows/governance-reusable.yml@8f31a5a4ba591d544b65f91f6d78b136e07756f0" \
    "$(workflow_uses "$GOVERNANCE_WORKFLOW")"

assert_equals \
    "Hypatia calls the reviewed standards revision" \
    "hyperpolymath/standards/.github/workflows/hypatia-scan-reusable.yml@cc58c0cb23f73fc2019ce85a56a468e5248a93b3" \
    "$(workflow_uses "$HYPATIA_WORKFLOW")"

assert_equals \
    "Scorecard calls the reviewed standards revision" \
    "hyperpolymath/standards/.github/workflows/scorecard-reusable.yml@8750b94ac1bbe8c51ad13fe106669b13478f0b62" \
    "$(workflow_uses "$SCORECARD_WORKFLOW")"

# Negative/boundary coverage: reject tags, branches, abbreviated hashes, and
# uppercase pseudo-hashes by requiring every external ref to be 40 lowercase
# hexadecimal characters. The exact count also detects a removed or extra call.
assert_all_refs_are_immutable \
    "all changed external references are full immutable commit SHAs" \
    "$CODEQL_WORKFLOW" \
    "$GOVERNANCE_WORKFLOW" \
    "$HYPATIA_WORKFLOW" \
    "$SCORECARD_WORKFLOW"

printf '\n%d passed; %d failed\n' "$PASS_COUNT" "$FAIL_COUNT"
if [[ "$FAIL_COUNT" -ne 0 ]]; then
    exit 1
fi
