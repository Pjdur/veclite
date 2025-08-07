//! # veclite
//!
//! A lightweight, ergonomic wrapper around Rust’s `Vec<T>` that implements `Display` for easy printing,
//! and provides extra list-like utility methods. Veclite also does not need `std`, and so can run without an operating system.
//!
//! ## Features
//! - Implements `Display` for space-separated formatting
//! - Retains all `Vec<T>` methods via `Deref`
//! - Adds `.prepend()` for list-style front insertion
//! - Short alias [`Vel`] for ergonomic use
//! - `vel![]` macro for concise construction
//!
//! ## Example
//! ```
//! use veclite::{Vel, vel};
//! let mut v = vel![1, 2, 3];
//! v.push(4);       // Vec method
//! v.prepend(0);    // Custom method
//! println!("{}", v); // prints: 0 1 2 3 4
//! ```

#![no_std]
extern crate alloc;

use alloc::vec::Vec;
use core::fmt::{self, Display, Formatter};
use core::ops::{Deref, DerefMut};

/// A lightweight wrapper around `Vec<T>` that provides pretty printing and list-like ergonomics.
///
/// Use [`Vel`] for a short alias, and [`vel![]`](macro@vel) for convenient construction.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Veclite<T>(pub Vec<T>);

impl<T> Veclite<T> {
    /// Creates a new, empty `Veclite<T>`.
    ///
    /// # Example
    /// ```
    /// use veclite::Vel;
    /// let v: Vel<i32> = Vel::new();
    /// assert!(v.is_empty());
    /// ```
    pub fn new() -> Self {
        Veclite(Vec::new())
    }

    /// Prepends a value to the front of the list.
    ///
    /// # Example
    /// ```
    /// use veclite::Vel;
    /// let mut v = Vel::new();
    /// v.push(2);
    /// v.prepend(1);
    /// assert_eq!(format!("{}", v), "1 2");
    /// ```
    pub fn prepend(&mut self, value: T) {
        self.0.insert(0, value);
    }
}

impl<T: Display> Display for Veclite<T> {
    /// Formats the list with space-separated elements.
    ///
    /// # Example
    /// ```
    /// use veclite::Vel;
    /// let v = Vel::from(vec![1, 2, 3]);
    /// assert_eq!(format!("{}", v), "1 2 3");
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, item) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", item)?;
        }
        Ok(())
    }
}

impl<T> Deref for Veclite<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Veclite<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<Vec<T>> for Veclite<T> {
    /// Converts a `Vec<T>` into a `Veclite<T>`.
    ///
    /// # Example
    /// ```
    /// use veclite::Vel;
    /// let v = Vel::from(vec![1, 2, 3]);
    /// ```
    fn from(v: Vec<T>) -> Self {
        Veclite(v)
    }
}

impl<T> IntoIterator for Veclite<T> {
    type Item = T;
    type IntoIter = alloc::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Veclite<T> {
    type Item = &'a T;
    type IntoIter = alloc::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Veclite<T> {
    type Item = &'a mut T;
    type IntoIter = alloc::slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

/// Short alias for [`Veclite<T>`], for ergonomic use.
///
/// # Example
/// ```
/// use veclite::Vel;
/// let mut v = Vel::new();
/// v.push(42);
/// println!("{}", v);
/// ```
pub type Vel<T> = Veclite<T>;

/// Macro to construct a `Vel<T>` just like `vec![]`.
///
/// # Example
/// ```
/// use veclite::vel;
/// let v = vel![1, 2, 3];
/// assert_eq!(format!("{}", v), "1 2 3");
/// ```
#[macro_export]
macro_rules! vel {
    ($($x:expr),* $(,)?) => {
        $crate::Vel::from(vec![$($x),*])
    };
}
