# Project: markit (CLI Snippet Manager)

A Rust-based command-line tool to save, view, and run command snippets (primarily Bash, with future support for SQL and more).

---

## 🥇 MVP (Milestone 1) – Core Bash Command Support

**Goal:** Save and run named shell commands from a YAML file.

### Features:
- `record <name>`: Start an interactive session to save a multi-line bash command.
- `run <name>`: Execute the stored command.
- `list`: Show a simple table of stored commands (name + description).
- `show <name>`: Print the full saved command.
- YAML-based storage (`~/.markit/bookmarks.yml`).
- Graceful error handling for missing, invalid, or duplicate entries.
- CLI built with `clap`

### Tech:
- Rust
- `clap` for CLI parsing
- `serde_yaml` for storage
- `dirs` crate for resolving config directory
- `std::process::Command` for execution

---

## 🥈 Milestone 2 – Non-Executable Snippet Support (e.g., SQL)

**Goal:** Add support for storing and retrieving non-executable snippets like SQL queries.

### Features:
- Support `executable: true/false` flag in YAML.
- `copy <name>`: Copy snippet to clipboard (via `arboard` or `copypasta`).
- `record <name> --type snippet`: Save a snippet that won't be executed.
- `list-full`: Show all metadata and full content.

---

## 🥉 Milestone 3 – Usability Enhancements

### Features:
- `edit <name>`: Open snippet in `$EDITOR`
- `delete <name>`: Remove a snippet
- `search <term>`: Fuzzy match name/content
- `list --tag <tag>`: Tag-based filtering
- Auto-backup of `bookmarks.yml` to `~/.markit/backups/`
- `export` and `import` for YAML files
- Metadata: `created_at`, `updated_at`

---

## 🖥️ Milestone 4 – TUI Mode

### Features:
- `ui`: Interactive terminal app using `ratatui`
  - Snippet list (selectable)
  - Command preview pane
  - Execute or copy with keypress
- Keyboard shortcuts (Enter to run, `c` to copy, `q` to quit)

---

## 🧪 Milestone 5 – Polish for Distribution

### Features:
- `--dry-run` flag for `run`
- Shell completion (zsh, bash) using `clap_complete`
- Generate man page using `clap_mangen`
- GitHub Actions CI for building release binaries
- Homebrew formula + custom tap repo
- Backup system separate from user-driven `export` (stored in `~/.markit/backups/`)
- Backup list + restore support

---

## 📦 Naming + Branding

- Final name selected: `markit`
- Rename crate and binary accordingly
- Set up GitHub repo with Graphite for stacked PR workflow

---

## 🚧 Notes

- Storage format should remain simple YAML (no DB dependency)
- Designed to be stateless, fast, and extensible
- MVP should be usable within 1–2 days of dev effort

---

## 🏁 Summary

Start with Bash command recording and execution. Once stable, add snippet viewing/copying, interactive UI, and export/import tooling. Finally, polish the distribution and prepare for wider release via Homebrew.
