---
title: Installation
nav_order: 2
---

# ⚙️ Installing Veclite

Veclite is published on [crates.io](https://crates.io/crates/veclite) and can be easily added to your Rust project using Cargo.

## 🛠 Prerequisites

Make sure you have:

- [Rust](https://www.rust-lang.org/tools/install) installed (version 1.60 or newer recommended)
- Cargo set up and ready to use

You can verify your setup by running:

```bash
rustc --version
cargo --version
```

## 🚀 Add Veclite to Your Project

Open your project’s `Cargo.toml` and add Veclite under `[dependencies]`:

```toml
[dependencies]
veclite = "0.2"
```

## ✅ Verify Installation

To check if Veclite is working, try importing it in your code:

```rust
use veclite::Veclite;

fn main() {
    let v = Veclite::new();
    println!("{}", v);
}
```

This should compile and output a formatted representation of an empty Veclite vector.

## 📚 Next Steps

- [Explore Features](intro.md)
- [View Examples](examples.md) *(optional)*
- [Check API Docs](https://docs.rs/veclite)

---

Need help or want to contribute? Feel free to open an issue or pull request on [GitHub](https://github.com/Pjdur/veclite). Happy coding!
