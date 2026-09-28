#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
# Build the exact scanner used by the fleet gate. Unavailability is a failure,
# never an empty findings array. Keep this pin separate from check context names.
#
# Pin: last revision whose Escript Soundness passed in hypatia CI (2026-09-25).
# HEAD at 43124f02 (and everything since 4654d7a3) does NOT compile —
# MismatchedDelimiterError in lib/rules/pin_integrity.ex:56 — so building HEAD
# made this required check red no matter what this repo did. Re-point this pin
# to a newer revision only after that revision's own Escript Soundness is green.
# Upstream: hyperpolymath/hypatia#869.
set -euo pipefail

revision=9f2f62f5c9463c79b33a5ebf54372166ce56f349
scanner="${1:?usage: build-hypatia.sh <new-scanner-directory>}"
mkdir "$scanner"
git -C "$scanner" init -q
git -C "$scanner" remote add origin https://github.com/hyperpolymath/hypatia.git
git -C "$scanner" fetch --depth 1 origin "$revision"
git -C "$scanner" checkout --detach FETCH_HEAD
[ "$(git -C "$scanner" rev-parse HEAD)" = "$revision" ]
cd "$scanner"
test -f mix.exs
mix local.hex --force
mix local.rebar --force
mix deps.get
mix escript.build
# Require the real escript, not the wrapper's reduced-coverage shell fallback.
test -x hypatia
test -x hypatia-cli.sh
printf 'Built Hypatia at %s\n' "$revision"
