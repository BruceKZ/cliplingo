# ClipLingo Release Checklist

## GitHub Release automation

- `Desktop CI` validates the desktop app and uploads workflow artifacts only. It does not create a GitHub Release.
- `Desktop Release` publishes a GitHub Release and uploads macOS and Windows bundles when you push a version tag such as `v0.1.0`.
- `apps/desktop/package.json` and `apps/desktop/src-tauri/tauri.conf.json` must keep the same version. The release workflow rejects mismatches.
- `workflow_dispatch` is also available for manual publishing from Actions. It uses the version committed in the selected ref and creates or updates the matching `v<version>` release.

## Release steps

1. Update the version in `apps/desktop/package.json`.
2. Update the same version in `apps/desktop/src-tauri/tauri.conf.json`.
3. Merge the release commit to `main`.
4. Create and push a tag that exactly matches the app version, for example `v0.1.0`.
5. Wait for the `Desktop Release` workflow to finish, then verify the generated GitHub Release assets.

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
