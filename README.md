# markit 🧠✨

**A blazing-fast CLI to save, search, run, and manage your shell command snippets.**

---

## 🚀 Features

- ✅ Save shell commands with descriptions, tags, and metadata
- 🔍 Fuzzy search and interactive selection via `dialoguer`
- 🧠 Metadata tracking (`created_at`, `updated_at`)
- 📝 In-place YAML editing with your preferred editor
- 🧪 Clipboard support (`--copy`)
- 📂 Backup/restore with auto-snapshots on every change
- 🔁 Import/export snippet collections
- 🔐 Optional exec safety flag (`executable: true/false`)
- 💥 Fast and lightweight — just Rust and YAML

---

## 📦 Installation

### 🦀 With Cargo

```bash
cargo install markit
```

### 🍺 With Homebrew (once published)

```bash
brew tap Nightstack/markit
brew install markit
```

> _Homebrew formula will be hosted in a separate [tap repo](https://docs.brew.sh/Taps)._

---

## 📖 Usage

```bash
markit --help
```

### Save a new snippet

```bash
markit save "docker-clean"
# Prompts for description, tags, and command content
```

### List all snippets

```bash
markit list
```

### Show a snippet by name

```bash
markit show "docker-clean"
```

### Run a command

```bash
markit run "docker-clean"
```

### Copy command to clipboard

```bash
markit copy "docker-clean"
```

### Edit snippet

```bash
markit edit "docker-clean"
```

### Delete snippet (with confirmation)

```bash
markit delete "docker-clean"
```

### Filter by tag

```bash
markit list --tag "k8s"
```

### Export all snippets

```bash
markit export snippets.yml
```

### Import from file

```bash
markit import snippets.yml --force
```

### Restore from backup

```bash
markit restore
```

---

## 🧰 Data Format

All data is stored as human-readable YAML in:

```bash
~/.markit/bookmarks.yml
```

Automatic backups are saved in:

```bash
~/.markit/backups/
```

---

## 🛠️ Development

### Clone

```bash
git clone https://github.com/Nightstack/markit
cd markit
cargo run -- <command>
```

### Build

```bash
cargo build --release
```

---

## 🧠 Inspiration

Built for people who Google the same commands 20 times a week. Now you don’t have to.

---

## 📄 License

MIT
