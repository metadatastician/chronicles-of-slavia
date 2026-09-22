#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
# Copyright (c) 2026 Jonathan D.A. Jewell (hyperpolymath) <j.d.a.jewell@open.ac.uk>

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
README_FILE="$SCRIPT_DIR/../README.adoc"
PASS=0
FAIL=0

if [ ! -r "$README_FILE" ]; then
    printf 'FAIL: README is not readable: %s\n' "$README_FILE" >&2
    exit 1
fi

pass() {
    printf 'PASS: %s\n' "$1"
    PASS=$((PASS + 1))
}

fail() {
    printf 'FAIL: %s\n' "$1" >&2
    FAIL=$((FAIL + 1))
}

assert_absent_literal() {
    local description="$1"
    local literal="$2"

    if grep -Fq "$literal" "$README_FILE"; then
        fail "$description"
    else
        pass "$description"
    fi
}

assert_absent_pattern() {
    local description="$1"
    local pattern="$2"

    if grep -Eq "$pattern" "$README_FILE"; then
        fail "$description"
    else
        pass "$description"
    fi
}

FALSE_OPENSSF_PROJECT="bestpractices.dev/projects/8509"
OPENSSF_BADGE_PATTERN='bestpractices\.dev/projects/[0-9]+/badge'
SCORECARD_BADGE='image:https://api.scorecard.dev/projects/github.com/metadatastician/chronicles-of-slavia/badge[OpenSSF Scorecard,link="https://scorecard.dev/viewer/?uri=github.com/metadatastician/chronicles-of-slavia"]'

assert_absent_literal \
    "README does not reference unrelated OpenSSF Best Practices project 8509" \
    "$FALSE_OPENSSF_PROJECT"
assert_absent_pattern \
    "README displays no unverified OpenSSF Best Practices project badge" \
    "$OPENSSF_BADGE_PATTERN"
assert_absent_literal \
    "README makes no OpenSSF Best Practices certification claim" \
    "OpenSSF Best Practices"

SCORECARD_COUNT=$(grep -Fc "$SCORECARD_BADGE" "$README_FILE" || true)
if [ "$SCORECARD_COUNT" -eq 1 ]; then
    pass "README retains one repository-specific OpenSSF Scorecard badge"
else
    fail "README contains exactly one canonical OpenSSF Scorecard badge (found $SCORECARD_COUNT)"
fi

if awk -v badge="$SCORECARD_BADGE" '
    index($0, badge) {
        found = 1
        if ((getline next_line) > 0 && next_line == "") separated = 1
    }
    END { exit !(found && separated) }
' "$README_FILE"; then
    pass "README keeps a blank separator after the compliance badge block"
else
    fail "README keeps a blank separator after the compliance badge block"
fi

printf 'Results: PASS=%d FAIL=%d\n' "$PASS" "$FAIL"
exit "$FAIL"
