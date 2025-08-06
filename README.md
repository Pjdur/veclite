# veclite

**A lightweight, ergonomic wrapper around Rust’s `Vec<T>` that implements `Display` for pretty printing and provides extra list-like utility methods.**

---

## Features

- **Display Implementation:** Print vector contents with `{}` formatting — clean, space-separated output instead of debug-style brackets.
- **Utility Methods:** `prepend`, `remove`, `get`, and more — for convenient, list-like manipulation.
- **Short Alias:** Use the ergonomic alias `Vel` for idiomatic code.
- **Macro Support:** Construct lists with the `vel![]` macro, just like `vec![]`.
- **Familiar Semantics:** Works like a `Vec<T>`, but with extra display and usability power.
- **Trait Derivations:** Implements `Debug`, `PartialEq`, `Clone`, and `Default`.

---

## Quick Start

### Add to your `Cargo.toml`

```toml
[dependencies]
veclite = "1.0.0"
```

### Import and use

```rust
use veclite::{Vel, vel};

let mut v = vel!["hello", "world"];
v.prepend("greetings");
println!("{}", v); // Output: greetings hello world
```

---

## API Overview

### Constructors and Methods

| Method           | Description                                 |
|------------------|---------------------------------------------|
| `Vel::new()`     | Create an empty list                        |
| `Vel::from(Vec)` | Convert from a standard `Vec<T>`            |
| `prepend(value)` | Insert a value at the beginning             |
| `push(value)`    | Append a value to the end (via `Vec`)       |
| `remove(index)`  | Remove and return value at index            |
| `get(index)`     | Get reference to value at index             |
| `iter()`         | Iterate over values                         |
| `len()`          | Number of elements                          |
| `is_empty()`     | Check if empty                              |
... Other Vec methods are available directly

### Display

Implements [`std::fmt::Display`] for space-separated output:

```rust
let v = vel![1, 2, 3];
println!("{}", v); // Output: 1 2 3
```

---

## Example Usage

```rust
use veclite::Vel;

fn main() {
    let mut v = Vel::new();
    v.push(10);
    v.push(20);
    v.prepend(5);
    println!("{}", v); // Output: 5 10 20

    if let Some(x) = v.get(1) {
        println!("Second element is: {}", x); // Output: Second element is: 10
    }

    v.remove(0);
    println!("{}", v); // Output: 10 20
}
```

---

## More Examples

### Integers

```rust
let v = vel![1, 2, 3];
println!("{}", v); // Output: 1 2 3
```

### Strings

```rust
let names = vel!["Alice".to_string(), "Bob".to_string()];
println!("{}", names); // Output: Alice Bob
```

### Iteration

```rust
let v = vel![1, 2, 3, 4, 5];
for n in &v {
    print!("{}-", n);
}
// Output: 1-2-3-4-5-
```

---

## FAQ

**Q: Does `veclite` re-export all `Vec` methods?**  
A: Most methods are accessible via `Deref`, so you can use `push`, `iter`, `sort`, etc. directly. For advanced features, use `.0` to access the inner `Vec`.

**Q: Can I use `veclite` with non-displayable types?**  
A: Yes, but `Display` formatting requires `T: Display`. Other methods work for any `T`.

**Q: Why not implement `Display` for `Vec<T>` directly?**  
A: The Rust standard library avoids this to prevent accidental large or slow prints. `veclite` lets you opt in explicitly.

---

## Contributing

Bug reports, feature requests, and pull requests are welcome!  
Open an issue or PR on [GitHub](https://github.com/Pjdur/veclite).

---

## License

Licensed under either of:

- MIT
- Apache-2.0

---

## See Also

- [`Vec<T>` documentation (std)](https://doc.rust-lang.org/std/vec/struct.Vec.html)
```
