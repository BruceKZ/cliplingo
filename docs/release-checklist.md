# ClipLingo Release Checklist

## Automated verification

- Run `./scripts/verify-release.sh`
- Confirm `pnpm --dir apps/desktop build` completes without Vue or Vite warnings that change runtime behavior
- Confirm `cargo test --manifest-path apps/desktop/src-tauri/Cargo.toml` passes
- Confirm `cargo check --manifest-path apps/desktop/src-tauri/Cargo.toml` passes

## Packaging

- macOS: run `./scripts/package-macos.sh`
- Windows: run `powershell -ExecutionPolicy Bypass -File .\scripts\package-windows.ps1`
- Confirm the generated release bundle contains no debug console windows and no development server dependency
- Confirm `apps/desktop/src-tauri/tauri.conf.json` keeps bundle mode active and uses the production frontend dist

## Manual product checks

- Double-copy trigger opens the translation popup with clipboard content
- Fallback shortcut opens the translation popup with clipboard content
- Popup supports ESC close, copy per target, retry, and pin/unpin
- Settings page validates provider URL, headers, model parameters, and routing rules inline
- History list can load and clear successfully

## Security checks

- Base URLs reject `javascript:`, `file:`, embedded credentials, non-loopback `http`, query strings, and fragments
- Custom headers reject newline injection
- UI rendering uses plain-text interpolation only and does not introduce `v-html`
- Release config does not enable raw network debug logging by default
