# Contributing to markit

Thanks for your interest in contributing!

## How to Contribute

- Found a bug? Please [open an issue](https://github.com/Nightstack/markit/issues).
- Have an idea for an improvement? Open an issue or pull request!
- Want to submit a fix or feature? PRs welcome.

## Setup for Development

```bash
git clone https://github.com/Nightstack/markit.git
cd markit
cargo build
cargo test
```

## Code Style

- Follow Rust formatting:

```bash
cargo fmt
```

- Check for warnings:

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

## Notes

- This project targets **Rust stable**.
- Cross-platform support is encouraged.
- Please update tests if needed!

---

Happy coding ☕️
