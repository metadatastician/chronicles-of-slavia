#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
# Regression coverage for #100: a skipped scanner must never mean a clean scan.
set -euo pipefail
root="$(cd "$(dirname "$0")/../.." && pwd)"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
mkdir "$tmp/bin"

cat > "$tmp/bin/git" <<'MOCK'
#!/usr/bin/env bash
set -euo pipefail
dir="$2"; shift 2
case "$1" in
  fetch) [ "${FAIL_STAGE:-}" != fetch ] ;;
  checkout)
    if [ "${FAIL_STAGE:-}" != source ]; then
      touch "$dir/mix.exs"
      printf '#!/bin/sh\nexit 0\n' > "$dir/hypatia-cli.sh"
      chmod +x "$dir/hypatia-cli.sh"
    fi ;;
  rev-parse) echo 9f2f62f5c9463c79b33a5ebf54372166ce56f349 ;;
esac
MOCK
cat > "$tmp/bin/mix" <<'MOCK'
#!/usr/bin/env bash
set -euo pipefail
if [ "$1" = escript.build ]; then
  [ "${FAIL_STAGE:-}" != build ] || exit 1
  if [ "${FAIL_STAGE:-}" != binary ]; then
    printf '#!/bin/sh\nexit 0\n' > hypatia
    chmod +x hypatia
  fi
fi
MOCK
chmod +x "$tmp/bin/"*
for stage in fetch source build binary; do
  if PATH="$tmp/bin:$PATH" FAIL_STAGE="$stage" \
    bash "$root/scripts/build-hypatia.sh" "$tmp/$stage" >"$tmp/log" 2>&1; then
    echo "FAIL: scanner $stage failure returned success" >&2; exit 1
  fi
  echo "PASS: scanner $stage failure is fatal"
done
PATH="$tmp/bin:$PATH" bash "$root/scripts/build-hypatia.sh" "$tmp/success" >/dev/null
[ -x "$tmp/success/hypatia" ]
echo 'PASS: complete scanner setup succeeds'

awk '/^  hypatia-scan:/ { scan=1 } /^  patch-bridge-triage:/ { scan=0 } scan' \
  "$root/.github/workflows/static-analysis-gate.yml" > "$tmp/hypatia-job"
! grep -Eq 'continue-on-error|Create stub findings|outputs.ready|\|\| true' "$tmp/hypatia-job"
grep -q 'name: Hypatia neurosymbolic scan' "$tmp/hypatia-job"
grep -q 'bash scripts/build-hypatia.sh' "$tmp/hypatia-job"
grep -q 'steps.scan.outputs.critical > 0' "$tmp/hypatia-job"
echo 'PASS: required Hypatia context stays fail-closed and keeps its critical gate'

# Exercise the actual local SPDX step, including an actions-lock header and
# a mutant with the SPDX text only in its workflow body.
awk '/      - name: Check SPDX Headers/ { step=1; next }
     step && /      - name:/ { exit }
     step && /        run: \|/ { next }
     step { sub(/^          /, ""); print }' \
  "$root/.github/workflows/workflow-linter.yml" > "$tmp/headers.sh"
mkdir -p "$tmp/headers/.github/workflows"
printf '# This workflow is managed by gh actions-lock.\n# SPDX-License-Identifier: MPL-2.0\nname: Valid\n' \
  > "$tmp/headers/.github/workflows/valid.yml"
(cd "$tmp/headers" && bash "$tmp/headers.sh") >/dev/null
printf 'name: Invalid\n# SPDX-License-Identifier: MPL-2.0\n' \
  > "$tmp/headers/.github/workflows/invalid.yml"
if (cd "$tmp/headers" && bash "$tmp/headers.sh") >"$tmp/log" 2>&1; then
  echo 'FAIL: SPDX in the body satisfied the header check' >&2; exit 1
fi
echo 'PASS: leading-comment SPDX policy accepts lock markers but rejects body-only text'

automerge="$root/.github/workflows/dependabot-automerge.yml"
! grep -Eq 'github.actor|pull_request_target:' "$automerge"
grep -Fq "github.event.pull_request.user.login == 'dependabot[bot]'" "$automerge"
grep -Fq 'github.event.pull_request.head.repo.full_name == github.repository' "$automerge"
echo 'PASS: Dependabot authorization uses the PR author and same-repo source'

echo '8 triage regression checks passed'
