#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# Unit and regression tests for scripts/check-lock-sync.sh and its workflow gate.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
VALIDATOR="$REPO_ROOT/scripts/check-lock-sync.sh"
GATE_WORKFLOW="$REPO_ROOT/.github/workflows/lock-sync-gate.yml"
TEST_ROOT="$(mktemp -d "${TMPDIR:-/tmp}/check-lock-sync-test.XXXXXX")"

trap 'rm -rf "$TEST_ROOT"' EXIT

PASS_COUNT=0
FAIL_COUNT=0
LAST_STATUS=0
LAST_OUTPUT=""
FIXTURE=""

new_fixture() {
    FIXTURE="$TEST_ROOT/$1"
    mkdir -p "$FIXTURE"
}

capture_validator() {
    local workflow_dir="$1"

    set +e
    LAST_OUTPUT="$($VALIDATOR "$workflow_dir" 2>&1)"
    LAST_STATUS=$?
    set -e
}

expect_pass() {
    local workflow_dir="$1"

    capture_validator "$workflow_dir"
    if [ "$LAST_STATUS" -ne 0 ]; then
        printf 'expected success, got status %d:\n%s\n' "$LAST_STATUS" "$LAST_OUTPUT" >&2
        return 1
    fi
}

expect_failure_containing() {
    local workflow_dir="$1"
    local expected="$2"

    capture_validator "$workflow_dir"
    if [ "$LAST_STATUS" -eq 0 ]; then
        printf 'expected failure containing %q, but command succeeded:\n%s\n' \
            "$expected" "$LAST_OUTPUT" >&2
        return 1
    fi
    if [[ "$LAST_OUTPUT" != *"$expected"* ]]; then
        printf 'expected output to contain %q:\n%s\n' "$expected" "$LAST_OUTPUT" >&2
        return 1
    fi
}

run_test() {
    local name="$1"
    local test_function="$2"

    if "$test_function"; then
        printf 'PASS: %s\n' "$name"
        PASS_COUNT=$((PASS_COUNT + 1))
    else
        printf 'FAIL: %s\n' "$name" >&2
        FAIL_COUNT=$((FAIL_COUNT + 1))
    fi
}

test_accepts_synchronised_lockfile() {
    new_fixture synchronised
    cat > "$FIXTURE/build.yml" <<'YAML'
name: Build
jobs:
  build:
    steps:
      - uses: actions/checkout@v4
YAML
    cat > "$FIXTURE/actions.lock" <<'YAML'
version: 1
workflows:
    '.github/workflows/build.yml':
        - 'actions/checkout@v4'
dependencies:
    'actions/checkout@v4':
        ref: 'v4'
YAML

    expect_pass "$FIXTURE" || return 1
    [[ "$LAST_OUTPUT" == *"0 dangling edges"* ]] || {
        printf 'success output did not confirm transitive closure:\n%s\n' "$LAST_OUTPUT" >&2
        return 1
    }
}

test_parses_yaml_job_refs_comments_subpaths_and_case() {
    new_fixture parsing-boundaries
    cat > "$FIXTURE/reusable.yaml" <<'YAML'
name: Reusable workflow caller
jobs:
  call:
    uses: "Example/Reusable/.github/workflows/build.yml@Feature" # pinned caller
YAML
    cat > "$FIXTURE/actions.lock" <<'YAML'
version: 1
workflows:
    '.github/workflows/reusable.yaml':
        - 'example/reusable@Feature'
dependencies:
    'EXAMPLE/REUSABLE@Feature':
        ref: 'Feature'
YAML

    expect_pass "$FIXTURE"
}

test_ignores_valid_local_actions() {
    new_fixture local-action
    cat > "$FIXTURE/local.yml" <<'YAML'
name: Local action
jobs:
  build:
    steps:
      - uses: ./actions/build
YAML
    cat > "$FIXTURE/actions.lock" <<'YAML'
version: 1
workflows:
    '.github/workflows/local.yml': []
dependencies:
YAML

    expect_pass "$FIXTURE"
}

test_rejects_ref_case_mismatch() {
    new_fixture ref-case
    cat > "$FIXTURE/build.yml" <<'YAML'
name: Build
jobs:
  build:
    steps:
      - uses: actions/checkout@V4
YAML
    cat > "$FIXTURE/actions.lock" <<'YAML'
version: 1
workflows:
    '.github/workflows/build.yml':
        - 'actions/checkout@v4'
dependencies:
    'actions/checkout@v4':
        ref: 'v4'
YAML

    expect_failure_containing "$FIXTURE" "actions/checkout@V4"
}

test_rejects_missing_lockfile() {
    new_fixture missing-lockfile
    cat > "$FIXTURE/build.yml" <<'YAML'
name: Build
jobs:
  build:
    steps:
      - run: true
YAML

    expect_failure_containing "$FIXTURE" "FATAL: no lockfile"
}

test_rejects_directory_without_workflows() {
    new_fixture no-workflows
    cat > "$FIXTURE/actions.lock" <<'YAML'
version: 1
workflows:
dependencies:
YAML

    capture_validator "$FIXTURE"
    if [ "$LAST_STATUS" -eq 0 ]; then
        printf 'expected a directory with no workflows to fail\n' >&2
        return 1
    fi
}

test_rejects_unonboarded_workflow() {
    new_fixture unonboarded
    cat > "$FIXTURE/build.yml" <<'YAML'
name: Build
jobs:
  build:
    steps:
      - uses: actions/checkout@v4
YAML
    cat > "$FIXTURE/actions.lock" <<'YAML'
version: 1
workflows:
dependencies:
    'actions/checkout@v4':
        ref: 'v4'
YAML

    expect_failure_containing "$FIXTURE" "not onboarded"
}

test_rejects_missing_ref_for_onboarded_workflow() {
    new_fixture missing-ref
    cat > "$FIXTURE/build.yml" <<'YAML'
name: Build
jobs:
  build:
    steps:
      - uses: actions/checkout@v4
YAML
    cat > "$FIXTURE/actions.lock" <<'YAML'
version: 1
workflows:
    '.github/workflows/build.yml': []
dependencies:
    'actions/checkout@v4':
        ref: 'v4'
YAML

    expect_failure_containing "$FIXTURE" "refs missing from the lockfile"
}

test_rejects_stale_workflow_ref() {
    new_fixture stale-ref
    cat > "$FIXTURE/build.yml" <<'YAML'
name: Build
jobs:
  build:
    steps:
      - run: true
YAML
    cat > "$FIXTURE/actions.lock" <<'YAML'
version: 1
workflows:
    '.github/workflows/build.yml':
        - 'actions/checkout@v4'
dependencies:
    'actions/checkout@v4':
        ref: 'v4'
YAML

    expect_failure_containing "$FIXTURE" "stale lockfile entries"
}

test_rejects_deleted_workflow_entry() {
    new_fixture deleted-workflow
    cat > "$FIXTURE/build.yml" <<'YAML'
name: Build
jobs:
  build:
    steps:
      - run: true
YAML
    cat > "$FIXTURE/actions.lock" <<'YAML'
version: 1
workflows:
    '.github/workflows/build.yml': []
    '.github/workflows/deleted.yml':
        - 'actions/checkout@v4'
dependencies:
    'actions/checkout@v4':
        ref: 'v4'
YAML

    expect_failure_containing "$FIXTURE" "workflow file that does not exist"
}

test_rejects_dangling_workflow_dependency() {
    new_fixture dangling-workflow-edge
    cat > "$FIXTURE/build.yml" <<'YAML'
name: Build
jobs:
  build:
    steps:
      - uses: actions/checkout@v4
YAML
    cat > "$FIXTURE/actions.lock" <<'YAML'
version: 1
workflows:
    '.github/workflows/build.yml':
        - 'actions/checkout@v4'
dependencies:
YAML

    expect_failure_containing "$FIXTURE" "DANGLING EDGES"
}

test_rejects_dangling_nested_dependency() {
    new_fixture dangling-nested-edge
    cat > "$FIXTURE/build.yml" <<'YAML'
name: Build
jobs:
  build:
    steps:
      - uses: example/parent@v1
YAML
    cat > "$FIXTURE/actions.lock" <<'YAML'
version: 1
workflows:
    '.github/workflows/build.yml':
        - 'example/parent@v1'
dependencies:
    'example/parent@v1':
        ref: 'v1'
        uses:
            - 'example/child@v2'
YAML

    expect_failure_containing "$FIXTURE" "example/child@v2"
}

test_rejects_dollar_local_action_rewrite() {
    new_fixture dollar-local-action
    cat > "$FIXTURE/build.yml" <<'YAML'
name: Build
jobs:
  build:
    steps:
      - uses: $/actions/build
YAML
    cat > "$FIXTURE/actions.lock" <<'YAML'
version: 1
workflows:
    '.github/workflows/build.yml': []
dependencies:
YAML

    expect_failure_containing "$FIXTURE" "invalid local-action rewrite"
}

test_allows_unreferenced_dependency_record() {
    new_fixture unreferenced-dependency
    cat > "$FIXTURE/build.yml" <<'YAML'
name: Build
jobs:
  build:
    steps:
      - uses: actions/checkout@v4
YAML
    cat > "$FIXTURE/actions.lock" <<'YAML'
version: 1
workflows:
    '.github/workflows/build.yml':
        - 'actions/checkout@v4'
dependencies:
    'actions/checkout@v4':
        ref: 'v4'
    'example/unused@v1':
        ref: 'v1'
YAML

    expect_pass "$FIXTURE" || return 1
    [[ "$LAST_OUTPUT" == *"1 dependencies: record(s) are unreferenced"* ]] || {
        printf 'expected an informational unreferenced dependency note:\n%s\n' "$LAST_OUTPUT" >&2
        return 1
    }
}

test_fails_closed_without_gnu_awk() {
    local fake_bin

    new_fixture no-gawk
    fake_bin="$TEST_ROOT/fake-bin"
    mkdir -p "$fake_bin"
    cat > "$fake_bin/gawk" <<'SH'
#!/bin/sh
exit 1
SH
    cat > "$fake_bin/awk" <<'SH'
#!/bin/sh
exit 1
SH
    chmod +x "$fake_bin/gawk" "$fake_bin/awk"

    set +e
    LAST_OUTPUT="$(PATH="$fake_bin" /usr/bin/bash "$VALIDATOR" "$FIXTURE" 2>&1)"
    LAST_STATUS=$?
    set -e

    if [ "$LAST_STATUS" -eq 0 ] || [[ "$LAST_OUTPUT" != *"need gawk"* ]]; then
        printf 'expected a fail-closed gawk prerequisite error:\n%s\n' "$LAST_OUTPUT" >&2
        return 1
    fi
}

test_repository_lockfile_is_synchronised() {
    expect_pass "$REPO_ROOT/.github/workflows"
}

test_gate_remains_self_protecting_and_wired() {
    if [ ! -f "$GATE_WORKFLOW" ]; then
        printf 'gate workflow is missing: %s\n' "$GATE_WORKFLOW" >&2
        return 1
    fi
    if grep -Eq '^[[:space:]]*(-[[:space:]]*)?uses:' "$GATE_WORKFLOW"; then
        printf 'gate workflow must not contain a uses: directive\n' >&2
        return 1
    fi
    if grep -Eq '^[[:space:]]*paths(-ignore)?:' "$GATE_WORKFLOW"; then
        printf 'gate workflow must run for every pull request, without path filters\n' >&2
        return 1
    fi
    if ! grep -Fq './scripts/check-lock-sync.sh' "$GATE_WORKFLOW"; then
        printf 'gate workflow does not execute the lock synchronisation validator\n' >&2
        return 1
    fi
    if ! grep -Fq '${{ github.event.pull_request.head.sha || github.sha }}' "$GATE_WORKFLOW"; then
        printf 'gate workflow does not check out the triggering revision\n' >&2
        return 1
    fi
}

run_test "accepts a synchronised lockfile" test_accepts_synchronised_lockfile
run_test "parses .yaml job refs, comments, subpaths, and repository-name case" \
    test_parses_yaml_job_refs_comments_subpaths_and_case
run_test "ignores valid local actions" test_ignores_valid_local_actions
run_test "keeps refs case-sensitive" test_rejects_ref_case_mismatch
run_test "rejects a missing lockfile" test_rejects_missing_lockfile
run_test "rejects a directory without workflow files" test_rejects_directory_without_workflows
run_test "rejects an unonboarded workflow" test_rejects_unonboarded_workflow
run_test "rejects a missing ref for an onboarded workflow" \
    test_rejects_missing_ref_for_onboarded_workflow
run_test "rejects a stale workflow ref" test_rejects_stale_workflow_ref
run_test "rejects a deleted workflow entry" test_rejects_deleted_workflow_entry
run_test "rejects a dangling workflow dependency" test_rejects_dangling_workflow_dependency
run_test "rejects a dangling nested dependency" test_rejects_dangling_nested_dependency
run_test "rejects a dollar-prefixed local-action rewrite" \
    test_rejects_dollar_local_action_rewrite
run_test "allows an unreferenced dependency record with a note" \
    test_allows_unreferenced_dependency_record
run_test "fails closed when GNU awk is unavailable" test_fails_closed_without_gnu_awk
run_test "accepts the repository's current lockfile" test_repository_lockfile_is_synchronised
run_test "keeps the gate self-protecting and wired" test_gate_remains_self_protecting_and_wired

printf '\n%d passed; %d failed\n' "$PASS_COUNT" "$FAIL_COUNT"
if [ "$FAIL_COUNT" -ne 0 ]; then
    exit 1
fi
