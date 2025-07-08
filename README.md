# veclite

**A lightweight, ergonomic wrapper around Rust’s `Vec<T>` that implements `Display` for pretty printing, and provides extra list-like utility methods.**

---

## Features

- **Display Implementation:** Print your vector contents with `{}` formatting, producing a clean, space-separated string (unlike the default debug output for `Vec<T>`).
- **Utility Methods:** `add`, `prepend`, `remove`, indexed access, and more — for convenient, list-like manipulation.
- **Short Alias:** Use the ergonomic alias `Vel` for concise, idiomatic code.
- **Familiar Semantics:** Works like a `Vec<T>`, but with extra display and usability power.

---

## Example Usage

```rust
use veclite::Vel;

fn main() {
    let mut v = Vel::new();
    v.add(10);
    v.add(20);
    v.prepend(5);
    println!("{}", v); // Output: 5 10 20

    // Access by index
    if let Some(x) = v.get(1) {
        println!("Second element is: {}", x); // Output: Second element is: 10
    }

    // Remove an element
    v.remove(0);
    println!("{}", v); // Output: 10 20
}
```

---

## Quick Start

### Add to your Cargo.toml

```toml
[dependencies]
veclite = "0.1"
```

### Import and use

```rust
use veclite::Vel; // Short alias, like Vec but with a capital "L"

let mut v = Vel::new();
v.add("hello");
v.add("world");
println!("{}", v); // Output: hello world
```

---

## API Overview

### Constructors and Methods

- `Vel::new()` — Create an empty vector.
- `add(value)` — Append a value to the end.
- `prepend(value)` — Insert a value at the beginning.
- `remove(index)` — Remove and return the value at the given index.
- `get(index)` — Get a reference to the value at the given index.
- `iter()` — Iterate over the values.
- `len()` — Number of elements.
- `is_empty()` — Check if empty.

### Display

Implements [`std::fmt::Display`], so you can:

```rust
let mut v = Vel::new();
v.add(1);
v.add(2);
println!("{}", v); // Output: 1 2
```

---

## Why use veclite?

- **Ergonomic display:** No more ugly debug output — just print your list, nicely formatted.
- **Convenience:** Common list operations at your fingertips.
- **Lightweight:** Zero dependencies, just a thin wrapper around `Vec<T>`.
- **Familiar:** If you know Vec, you already know how to use veclite.

---

## Examples

### 1. Integers

```rust
use veclite::Vel;

let mut v = Vel::new();
v.add(1);
v.add(2);
v.add(3);
println!("{}", v); // Output: 1 2 3
```

### 2. Strings

```rust
use veclite::Vel;

let mut names = Vel::new();
names.add("Alice".to_string());
names.add("Bob".to_string());
println!("{}", names); // Output: Alice Bob
```

### 3. Iteration

```rust
use veclite::Vel;

let mut v = Vel::new();
for i in 1..=5 {
    v.add(i);
}
for n in v.iter() {
    print!("{}-", n);
}
// Output: 1-2-3-4-5-
```

---

## FAQ

**Q: Does `veclite` re-export all `Vec` methods?**  
A: Only a subset of common list-like operations are provided. For advanced features, use `.0` to access the inner Vec directly.

**Q: Can I use `veclite` with non-displayable types?**  
A: The display implementation (`Display`) requires `T: Display`. Methods like `add`, `prepend`, etc. work for any `T`.

**Q: Why not just implement `Display` for `Vec<T>`?**  
A: The Rust standard library intentionally avoids this, to prevent accidental slow/large prints. `veclite` lets you opt in.

---

## Contributing

Contributions, bug reports, and feature requests are welcome! Please open an issue or PR on [GitHub](https://github.com/Pjdur/veclite).

---

## License

MIT OR Apache-2.0

---

## See Also

- [`Vec<T>` documentation (std)](https://doc.rust-lang.org/std/vec/struct.Vec.html)
