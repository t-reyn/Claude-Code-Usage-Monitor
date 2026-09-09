# CLAUDE.md

Personal aesthetic fork of [CodeZeno/Claude-Code-Usage-Monitor](https://github.com/CodeZeno/Claude-Code-Usage-Monitor) — a Rust + raw Win32/GDI app that embeds a Claude usage widget (5-hour + weekly limits) **directly into the Windows taskbar**, plus tray icon(s) with a percentage number badge.

- Remotes: `origin` = t-reyn fork, `upstream` = CodeZeno. Work happens on `custom-theme`; keep `main` clean for upstream syncs.
- No upstream PRs intended. No tests — verify by building and running.

## Memory

Project memory for this repo lives in the claude-sync repo and is imported below. It loads only when a session works in this project. When a fact about this project changes, update that file in place (Status, Open items, Key decisions). Dated history goes in the ARCHIVE.md or dated project file beside it, never the workspace MEMORY.md. Cross-project rules stay in the root `claude-sync/memory/MEMORY.md`.

@../claude-sync/memory/Claude-Code-Usage-Monitor/MEMORY.md

## Build & run

```powershell
cargo build --release
.\target\release\claude-code-usage-monitor.exe            # normal
.\target\release\claude-code-usage-monitor.exe --diagnose # logs auth/poll issues
```

Requires MSVC toolchain (`stable-msvc`). Single-instance guard: kill the running instance before launching a rebuilt exe (`Stop-Process -Name claude-code-usage-monitor`).

## Architecture

| File | Role |
|---|---|
| `src/window.rs` | Almost all UI: taskbar embedding (child of `Shell_TrayWnd`), settings load/save, layout constants, GDI painting (`paint_content`, `draw_row`), context menu, drag-to-reposition |
| `src/tray_icon.rs` | Tray number-badge icon rendering + severity color ramp (`interpolated_fill`) |
| `src/poller.rs` | Usage polling. Claude: OAuth `GET api.anthropic.com/api/oauth/usage`, token from `~/.claude/.credentials.json`. Codex equivalent exists but is off by default |
| `src/theme.rs` | **Only** dark-mode detection via registry — NOT styling, despite the name |
| `src/native_interop.rs` | Win32 helpers, `Color` struct (`Color::from_hex("#RRGGBB")`) |
| `src/models.rs` | `UsageData { session, weekly }` per app |
| `src/localization/` | UI strings, 8 languages — new visible strings must be added to `Strings` and every language file |

## Theming surface (where the aesthetic lives)

All colors are inline `Color::from_hex` calls — there is no central palette file:

- **Claude accent (bar fill + brand):** `claude_accent_color()` in `window.rs` (~line 865) → `#D97757`
- **Usage % text:** `claude_usage_text_color()` (~line 877) → dark `#F09A7A` / light `#A94F32`
- **Widget bg / label text / bar track:** in `update_layered_window` (~lines 1193–1207) → dark: bg `#1C1C1C`, text `#888888`, track `#444444`; light: bg `#F3F3F3`, text `#404040`, track `#AAAAAA`. Same constants repeated in the `WM_PAINT` fallback path (~line 2601 area) — change both.
- **Divider:** hardcoded RGB tuples in `paint_content` (~line 1351)
- **Font:** Segoe UI, `sc(-12)`, `FW_MEDIUM`, created in `paint_content` (~line 1390)
- **Geometry:** consts at `window.rs` ~807–821 (`SEGMENT_W/H/GAP/COUNT`, `WIDGET_HEIGHT: 46`, label/text widths). Bars are 10 discrete segments, not continuous.
- **Tray badge severity ramp:** `interpolated_fill()` in `tray_icon.rs` — `#D97757` (≤50%) → `#D08540` (70) → `#CC8C20` (85) → `#C45020` (95) → `#B82020` (100)
- **Dark/light switch:** follows Windows theme (`theme::is_dark_mode()`), no in-app override

DPI: every pixel value must go through `sc()` (96-DPI base). GDI constraints: solid fills, rects/rounded rects, ClearType text — no blur/shadows/transparency effects, fonts must be installed on Windows.

## Settings

`%APPDATA%\ClaudeCodeUsageMonitor\settings.json` — poll interval (default 5 min; menu offers 1/5/15 min + 1 hr), `show_claude_code` (default true), `show_codex` (default **false**), tray offset, widget visibility, language. Right-click widget/tray for the menu (poll frequency, Start with Windows, model toggles, reset position). Note: usage % only refreshes on poll; the countdown text re-renders every minute from the last-fetched reset time.
