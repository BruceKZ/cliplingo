# ClipLingo

ClipLingo is a lightweight desktop translation client for macOS and Windows.

Core flow:

1. User selects text in any app.
2. User presses `Cmd/Ctrl+C+C` within a short time window.
3. The app reads clipboard text.
4. The app calls a configurable OpenAI-compatible API.
5. A lightweight popup shows translated output quickly.

Phase 1 scope:

- Tauri v2 desktop app
- Vue 3 + TypeScript + Tailwind CSS frontend
- Clipboard-based translation
- Double-copy trigger and fallback hotkey
- OpenAI-compatible provider support
- Local config, history, and secure API key storage
- Translation popup, settings, and local history

Non-goals:

- OCR
- browser extension
- document batch translation
- TTS
- cloud sync
- account system

Primary product constraints:

- security-first input handling
- no Electron
- no gradients
- no HTML rendering of user content
- API keys must not be stored in plaintext config files

Product requirements source: [`task.md`](./task.md)
