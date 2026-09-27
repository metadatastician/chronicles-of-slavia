#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
# Build the exact scanner used by the fleet gate. Unavailability is a failure,
# never an empty findings array. Keep this pin separate from check context names.
set -euo pipefail

revision=43124f025af26dadc9d268460410a205cfccefe8
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
