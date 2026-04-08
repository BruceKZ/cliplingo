# ClipLingo

<p align="center">
  <img src="./apps/desktop/src/assets/app-icon.png" alt="ClipLingo icon" width="96" height="96" />
</p>

<p align="center">
  Lightweight desktop translation for macOS and Windows, built around a fast clipboard-first workflow.
</p>

<p align="center">
  <a href="https://github.com/BruceKZ/cliplingo/stargazers">
    <img alt="GitHub stars" src="https://img.shields.io/github/stars/BruceKZ/cliplingo?style=flat-square" />
  </a>
  <a href="https://github.com/BruceKZ/cliplingo/network/members">
    <img alt="GitHub forks" src="https://img.shields.io/github/forks/BruceKZ/cliplingo?style=flat-square" />
  </a>
  <a href="https://github.com/BruceKZ/cliplingo/releases">
    <img alt="GitHub release" src="https://img.shields.io/github/v/release/BruceKZ/cliplingo?style=flat-square" />
  </a>
  <a href="https://github.com/BruceKZ/cliplingo/issues">
    <img alt="GitHub issues" src="https://img.shields.io/github/issues/BruceKZ/cliplingo?style=flat-square" />
  </a>
  <a href="./LICENSE">
    <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-black?style=flat-square" />
  </a>
</p>

## Overview

ClipLingo is a desktop translator focused on one job:

1. Select text in any app.
2. Trigger translation with `Cmd/Ctrl+C+C` or a fallback shortcut.
3. Read clipboard text safely.
4. Send the request to a configurable OpenAI-compatible provider.
5. Show the result quickly in a lightweight desktop window.

The project is intentionally narrow in scope: fast text translation, low friction, local-first configuration, and minimal UI noise.

## Features

- Clipboard-first translation workflow
- Double-copy trigger plus fallback hotkey
- Configurable OpenAI-compatible provider support
- Local settings and provider management
- Translation popup and full desktop workspace
- Light and dark theme support
- Local history and logging foundation
- Tauri-based native desktop packaging for macOS and Windows

## Tech Stack

<p>
  <img alt="Tauri" src="https://img.shields.io/badge/Tauri-v2-24C8DB?style=flat-square&logo=tauri&logoColor=white" />
  <img alt="Vue 3" src="https://img.shields.io/badge/Vue-3-42B883?style=flat-square&logo=vuedotjs&logoColor=white" />
  <img alt="TypeScript" src="https://img.shields.io/badge/TypeScript-5-3178C6?style=flat-square&logo=typescript&logoColor=white" />
  <img alt="Vuetify" src="https://img.shields.io/badge/Vuetify-UI-1867C0?style=flat-square&logo=vuetify&logoColor=white" />
  <img alt="Tailwind CSS" src="https://img.shields.io/badge/Tailwind_CSS-3-06B6D4?style=flat-square&logo=tailwindcss&logoColor=white" />
  <img alt="Pinia" src="https://img.shields.io/badge/Pinia-State-FFD859?style=flat-square&logo=pinia&logoColor=black" />
  <img alt="Vue Router" src="https://img.shields.io/badge/Vue_Router-4-42B883?style=flat-square&logo=vuedotjs&logoColor=white" />
  <img alt="Rust" src="https://img.shields.io/badge/Rust-Core-000000?style=flat-square&logo=rust&logoColor=white" />
  <img alt="Vite" src="https://img.shields.io/badge/Vite-Build-646CFF?style=flat-square&logo=vite&logoColor=white" />
  <img alt="pnpm" src="https://img.shields.io/badge/pnpm-Workspace-F69220?style=flat-square&logo=pnpm&logoColor=white" />
</p>

| Area | Choice |
| --- | --- |
| Desktop shell | Tauri v2 |
| Frontend | Vue 3 + TypeScript |
| UI system | Vuetify + Tailwind CSS |
| State management | Pinia |
| Routing | Vue Router |
| Native/backend | Rust |
| Shared contracts | `packages/shared-types` |
| Tooling | Vite, pnpm, ESLint, Prettier |

## Installation

### Option 1: Install From GitHub Releases

1. Open [GitHub Releases](https://github.com/BruceKZ/cliplingo/releases).
2. Download the package for your platform.
3. Install and launch ClipLingo.

Recommended for most users.

### Option 2: Run From Source

Prerequisites:

- Node.js 20+
- `pnpm`
- Rust toolchain
- Tauri platform dependencies
- macOS or Windows development environment

Clone and install:

```bash
git clone git@github.com:BruceKZ/cliplingo.git
cd cliplingo
pnpm install
```

Start the desktop app in development mode:

```bash
cd apps/desktop
pnpm tauri dev
```

Build the frontend:

```bash
pnpm --dir apps/desktop build
```

Run Rust checks:

```bash
pnpm --dir apps/desktop check:rust
pnpm --dir apps/desktop test:rust
```

## How To Use

### Basic Flow

1. Open ClipLingo.
2. Configure a provider in `Providers`.
3. Set your API endpoint, model, and API key.
4. Select text in another app.
5. Trigger translation with:
   - `Cmd+C+C` on macOS
   - `Ctrl+C+C` on Windows
6. Read, copy, and retry from the translation window.

### Manual Translation

You can also use the main translation workspace:

- choose source language
- choose target language
- paste or type text
- translate and copy directly

## Project Structure

```text
apps/
  desktop/
    src/           # Vue desktop UI
    src-tauri/     # Rust backend, app shell, native integration
packages/
  shared-types/    # Shared frontend/backend types and defaults
docs/              # Project docs
scripts/           # Packaging and verification scripts
task.md            # Product requirements and execution guide
agents.md          # Agent and implementation guardrails
```

## Architecture

Core modules in the current architecture:

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

Flow at a high level:

```text
User selection
  -> clipboard trigger
  -> clipboard read
  -> language routing
  -> provider request
  -> translation result
  -> popup / workspace UI
```

## Development Notes

Important constraints:

- no Electron
- no OCR, browser extension, TTS, or cloud sync in phase 1
- no raw HTML rendering for user content
- no plaintext API keys in config files
- OpenAI-compatible provider support must remain first-class

Product requirements source:

- [task.md](./task.md)

## License

This project is licensed under the [MIT License](./LICENSE).

## Contributing

Contributions are welcome, especially in these areas:

- provider configuration UX
- translation workspace readability
- multi-target translation support
- dark mode polish
- stability, packaging, and testing

Suggested contribution flow:

1. Open an issue or discuss the change first.
2. Create a focused branch from `develop`.
3. Keep the change scoped.
4. Run the relevant checks before opening a PR.
5. Submit a PR against `develop`.

Branch and commit conventions are documented in:

- [agents.md](./agents.md)

## Roadmap

Current priorities:

1. Refine the provider configuration experience.
2. Improve provider-page readability and information hierarchy.
3. Support editing provider display names cleanly.
4. Support multi-target translation in the manual translation workspace.
5. Continue dark mode and deferred TODO cleanup.

## Star History

If ClipLingo is useful to you, consider starring the repo:

- [Star ClipLingo on GitHub](https://github.com/BruceKZ/cliplingo)
