# ClipLingo Agent Guide

This file is the execution context for subagents working on ClipLingo.

## Product Summary

- Product name: `ClipLingo`
- Repo name: `cliplingo`
- Goal: build a lightweight desktop translator for macOS and Windows
- Primary trigger: `Cmd/Ctrl+C+C` within a configurable time window
- Fallback trigger: global hotkey such as `Cmd/Ctrl+Shift+Y`
- Translation source: system clipboard text only
- Provider priority: OpenAI-compatible APIs with fully custom base URL support

## Phase 1 Scope

- Tauri v2 desktop app
- Vue 3 + TypeScript + Tailwind CSS frontend
- Translation popup window
- Settings window
- Local history
- Config persistence
- Secure API key storage
- Language detection and routing
- Single-target and dual-target translation
- Logging and security hardening
- Packaging and release validation for macOS and Windows

## Hard Constraints

- Do not add product scope beyond `task.md`.
- Do not use Electron.
- Do not use gradients, glassmorphism, or flashy animations.
- Do not render translation text with `v-html` or any raw HTML path.
- Do not store API keys in plaintext config files.
- Do not trust user text, clipboard content, prompt content, or custom headers.
- Do not bypass Rust-side validation for security-sensitive flows.
- Do not add heavyweight dependencies unrelated to the core flow.

## Required Stack

- Desktop: Tauri v2
- Frontend: Vue 3, TypeScript, Composition API, Tailwind CSS
- State: Pinia preferred
- Routing: Vue Router preferred
- Backend/core: Rust
- History store: SQLite preferred
- Config file: TOML preferred
- Secret storage: macOS Keychain and Windows Credential Manager

## Target Architecture

Required modules:

- `trigger-manager`
- `clipboard-service`
- `translation-orchestrator`
- `language-router`
- `provider-openai-compatible`
- `config-service`
- `secure-storage`
- `history-repository`
- `window-manager`
- `logging-service`

Suggested structure:

```text
apps/
  desktop/
    src/
    src-tauri/
packages/
  shared-types/
docs/
```

## GitHub Workflow

- Default branches:
  - `main`: stable branch
  - `develop`: integration branch
- Working branches:
  - `feature/<issue>-<slug>`
  - `fix/<issue>-<slug>`
- Commit style: Conventional Commits
- Delivery loop per issue:
  1. create or switch to issue branch
  2. implement one scoped change only
  3. add or update tests first for security boundary changes
  4. run relevant lint and test commands
  5. self-review diff for regressions and scope creep
  6. open PR to `develop`
  7. review PR findings
  8. address feedback
  9. merge after checks pass

## Issue Breakdown

Execution order must follow:

1. Task 01: initialize project skeleton
2. Task 02: implement windows and tray foundation
3. Task 03: define shared types and config models
4. Task 04: implement config and secure storage abstractions
5. Task 05: implement clipboard service
6. Task 06: implement double-copy trigger state machine
7. Task 07: implement provider abstraction and OpenAI-compatible provider
8. Task 08: implement language detection and routing
9. Task 09: implement translation orchestrator
10. Task 10: implement translation popup UI
11. Task 11: implement settings page
12. Task 12: implement history and logging
13. Task 13: implement security hardening
14. Task 14: complete testing and packaging

## Definition of Done

For each issue, subagents must report:

- files changed
- new types or interfaces
- implementation summary
- risks or follow-up items
- validation performed

## Testing Expectations

Must cover:

- language routing
- parameter validation
- URL validation
- text sanitization
- multi-target parsing
- history trimming
- request timeout handling
- offline behavior
- config corruption recovery
- malicious HTML input
- prompt injection attempts
- invalid headers and base URLs

## Security Rules

- Base URLs must be `https://` by default.
- `http://localhost` is dev-only.
- Reject `file://`, `javascript://`, and malformed URLs.
- Reject header injection and newline characters in header names or values.
- Redact secrets and full text in logs by default.
- Keep user text in structured prompt slots only.
- Parse model output into constrained structures for multi-target responses.

## UI Rules

- Flat visual style
- High readability
- One main accent color plus one danger color at most
- Support light and dark themes
- System fonts preferred
- Popup must support ESC close, pin, scroll, retry, and copy

## Current Repo State

- Initial repository bootstrap in progress
- GitHub issues will mirror the 14 tasks in `task.md`
- Development should proceed issue by issue through PRs into `develop`
