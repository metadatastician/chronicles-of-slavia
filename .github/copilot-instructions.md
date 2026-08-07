<!--
SPDX-License-Identifier: CC-BY-SA-4.0
Copyright (c) Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
-->
<!-- Copyright (c) {{CURRENT_YEAR}} {{AUTHOR}} ({{OWNER}}) <{{AUTHOR_EMAIL}}> -->
<!-- Authoritative source: docs/AI-CONVENTIONS.md -->

# Copilot Instructions

## Before Writing Code

- Read `0-AI-MANIFEST.a2ml` in the repo root for canonical file locations.
- State files (.a2ml) live in `.machine_readable/` ONLY, never the root.

## License

- SPDX: `AGPL-3.0-or-later` on new code/config/scripts; `CC-BY-SA-4.0` on new prose docs (ADR-0008).
- The estate default denies AGPL, but **this repo overrides it**: ADR-0008
  (`docs/decisions/0008-licence-position.adoc`) rules AGPL-3.0-or-later for code
  and CC-BY-SA-4.0 for prose. Follow ADR-0008 here.
- Never edit a licence header automatically — licence changes are manual and
  owner-only. Third-party licences (MPL-2.0 components, OFL-1.1 fonts) are
  untouchable and keep their own terms.
- Copyright: `{{AUTHOR}} ({{OWNER}}) <{{AUTHOR_EMAIL}}>`

## Code Style

- Use descriptive variable names.
- Annotate and document all files.
- Add SPDX header to every source file.
- Use `just` for build/test/lint commands.

## Banned Patterns

- Idris2: no `believe_me`, no `assert_total`
- Haskell: no `unsafeCoerce`, no `unsafePerformIO`
- OCaml: no `Obj.magic`
- Coq: no `Admitted`
- Lean: no `sorry`
- Rust: no `transmute` unless FFI with `// SAFETY:` comment

## Banned Languages

- No TypeScript (use ReScript)
- No Node.js / npm / bun (use Deno)
- No Go (use Rust)
- No Python (use Julia or Rust)

## Containers

- Use Podman, never Docker.
- Name the file `Containerfile`, never `Dockerfile`.
- Base image: `cgr.dev/chainguard/wolfi-base:latest`.

## ABI/FFI

- ABI definitions in Idris2 (`src/interface/abi/`).
- FFI implementations in Zig (`src/interface/ffi/`).
- Generated C headers in `src/interface/generated/`.

## State Files

Never create these in the repo root:
STATE.a2ml, META.a2ml, ECOSYSTEM.a2ml, AGENTIC.a2ml, NEUROSYM.a2ml, PLAYBOOK.a2ml.
They belong in `.machine_readable/` only.
