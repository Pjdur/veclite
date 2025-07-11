---
title: Examples
nav_order: 3
---

# Veclite Examples

## Basic Initialization

```rust
use veclite::Vel;

fn main() {
    let v = Vel:new()
}
```

## Adding Elements

```rust
use veclite::Vel;

fn main() {
    let mut v = Vel:new()
    v.add(1)
    v.add(2)
    v.add(3)
    println!("{}", v)
}
```

## Add an item to the front

```rust
use veclite::Vel;

fn main() {
    let v = Vel:new()
    v.add(2)
    v.prepend(1)
    println!("{}", v)
}
```

## Removing Elements

```rust
use veclite::Vel

fn main() {
    let mut names = Veclite::new();
    names.add("Alice".to_string());
    names.add("Bob".to_string());
    names.add("Carol".to_string());
    println!("{}", names); // Output: Alice Bob Carol

    // Remove the first element
    names.remove(0);
    println!("{}", names); // Output: Bob Carol
}
```

## Check list length

```rust
use veclite::Vel

fn main() {
    let mut v = Vel::new();
    v.add(42);
}
```

---

More examples will be added later. You can contribute at [https://github.com/Pjdur/Veclite/contribute](https://github.com/Pjdur/Veclite/contribute). Make sure to follow the [contributing guidelines](https://github.com/Pjdur/veclite/blob/main/contributing.md).