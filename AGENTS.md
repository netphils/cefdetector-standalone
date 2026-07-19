# AGENTS.md

Project conventions for `cefdetector-standalone` — a Tauri 2 (React + TypeScript + Rust) app that scans the Windows registry for Electron/CEF (Chromium-kernel) applications and displays them.

## Stack
- **Frontend**: React + TypeScript, Vite (`pnpm dev` / `pnpm build`).
- **Backend**: Rust via Tauri 2 commands (`src-tauri/src/lib.rs`).
- **Package manager**: pnpm.

## Build / verify commands
- Frontend: `pnpm build` (runs `tsc && vite build`).
- Backend (Rust): `cd src-tauri && cargo build`.
- Run app: `pnpm tauri dev`.

## Architecture
- `src-tauri/src/lib.rs` exposes Tauri `#[tauri::command]` functions. `scan_apps` enumerates the three Uninstall registry keys, extracts `DisplayName`/`DisplayIcon`/`InstallLocation`/`UninstallString` into a nested `HashMap`, and emits `app-entry` events to the frontend (one per matched app). It returns `HashMap {"size": <total humansize string>}`.
- Frontend listens for `app-entry` events (`@tauri-apps/api/event`) and renders cards in `ResultsPage`. `invoke("scan_apps", { params: {} })` triggers the scan.
- `BROWSER_MATCH` constant maps browser type → filename patterns, ordered by priority (lower index = higher priority). `matches_browser` returns the highest-priority type plus the sum of matched file sizes.
- Version info via `getVersion` / `getTauriVersion` from `@tauri-apps/api/app`.

## Conventions
- Registry scanning is **Windows-only** (`winreg`, `windows-icons`, `walkdir`, `humansize` deps).
- Per-app `size` displayed = full install dir size (`dir_size`); the **grand total** = sum of only the matched browser-kernel file sizes.
- Skip any app missing the four critical registry values (`DisplayName`, `DisplayIcon`, `InstallLocation`, `UninstallString`) or with no valid install dir.
- RPC/IPC apps render icons as base64 PNG; missing icons fall back to `no-image-svgrepo-com.svg`.
- Git commits: Chinese messages, imperative mood (`feat:`/`fix:`/`chore:` style). Local git user: `zyizhuo` / `zyizhuo@outlook.com`. Default branch `master`.
- `.gitignore` excludes `node_modules`, `dist`, `/src-tauri/target/`, `/src-tauri/Cargo.lock`, `/src-tauri/gen/`, build artifacts, and opencode files (`.opencode/`, `.opencode.json`, `.opencode.jsonc`).
