//! # veclite
//!
//! A lightweight, ergonomic wrapper around Rust’s `Vec<T>` that implements `Display` for easy printing,
//! and provides extra list-like utility methods.
//!
//! ## Features
//! - Implements `Display` for pretty, space-separated formatting
//! - Utility methods: `.add()`, `.prepend()`, `.remove()`, `.get()`, `.iter()` etc.
//! - Short alias [`Vel`] for ergonomic use, similar to how `vec![]` is used for `Vec<T>`
//!
//! ## Example
//! ```
//! use veclite::Vel;
//! let mut v = Vel::new();
//! v.add(10);
//! v.add(20);
//! v.prepend(5);
//! println!("{}", v); // prints: 5 10 20
//! ```

use std::fmt::{self, Display, Formatter};

/// `Veclite<T>` is a lightweight wrapper around the standard library's `Vec<T>`
/// that provides ergonomic formatting and additional list-like methods.
///
/// # Example
/// ```
/// use veclite::Veclite;
/// let mut v = Veclite::new();
/// v.add(1);
/// v.add(2);
/// v.prepend(0);
/// println!("{}", v); // prints: 0 1 2
/// ```
pub struct Veclite<T>(pub Vec<T>);

impl<T> Veclite<T> {
    /// Constructs a new, empty `Veclite<T>`.
    ///
    /// # Example
    /// ```
    /// let v: veclite::Veclite<i32> = veclite::Veclite::new();
    /// assert!(v.is_empty());
    /// ```
    pub fn new() -> Self {
        Veclite(Vec::new())
    }

    /// Appends a value to the end of the list.
    ///
    /// # Example
    /// ```
    /// let mut v = veclite::Veclite::new();
    /// v.add(10);
    /// assert_eq!(format!("{}", v), "10");
    /// ```
    pub fn add(&mut self, value: T) {
        self.0.push(value);
    }

    /// Prepends a value to the front of the list, shifting existing elements to the right.
    ///
    /// # Example
    /// ```
    /// let mut v = veclite::Veclite::new();
    /// v.add(2);
    /// v.prepend(1);
    /// assert_eq!(format!("{}", v), "1 2");
    /// ```
    pub fn prepend(&mut self, value: T) {
        self.0.insert(0, value);
    }

    /// Removes and returns the element at the specified index, if it exists.
    ///
    /// # Returns
    /// * `Some(T)` if the index is valid and the element was removed
    /// * `None` if the index is out of bounds
    ///
    /// # Example
    /// ```
    /// let mut v = veclite::Veclite::new();
    /// v.add(1);
    /// v.add(2);
    /// assert_eq!(v.remove(0), Some(1));
    /// assert_eq!(v.remove(5), None);
    /// ```
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.0.len() {
            Some(self.0.remove(index))
        } else {
            None
        }
    }

    /// Returns the number of elements in the `Veclite`.
    ///
    /// # Example
    /// ```
    /// let mut v = veclite::Veclite::new();
    /// v.add(42);
    /// assert_eq!(v.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the `Veclite` contains no elements.
    ///
    /// # Example
    /// ```
    /// let v: veclite::Veclite<i32> = veclite::Veclite::new();
    /// assert!(v.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns a reference to the element at the specified index, if it exists.
    ///
    /// # Example
    /// ```
    /// let mut v = veclite::Veclite::new();
    /// v.add(99);
    /// assert_eq!(v.get(0), Some(&99));
    /// assert_eq!(v.get(2), None);
    /// ```
    pub fn get(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }

    /// Returns an iterator over the elements of the `Veclite`.
    ///
    /// # Example
    /// ```
    /// let mut v = veclite::Veclite::new();
    /// v.add(1);
    /// v.add(2);
    /// let sum: i32 = v.iter().sum();
    /// assert_eq!(sum, 3);
    /// ```
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.0.iter()
    }

    /// Returns a mutable iterator over the elements of the `Veclite`.
    ///
    /// # Example
    /// ```
    /// let mut v = veclite::Veclite::new();
    /// v.add(1);
    /// for x in v.iter_mut() {
    ///     *x += 1;
    /// }
    /// assert_eq!(format!("{}", v), "2");
    /// ```
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.0.iter_mut()
    }
}

impl<T: Display> Display for Veclite<T> {
    /// Formats the `Veclite` for display using the `{}` formatter.
    ///
    /// Elements are separated by a single space, and each element is formatted
    /// using its own [`Display`] implementation.
    ///
    /// # Example
    /// ```
    /// let mut v = veclite::Veclite::new();
    /// v.add(1);
    /// v.add(2);
    /// assert_eq!(format!("{}", v), "1 2");
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

// --- IntoIterator implementations ---

impl<T> IntoIterator for Veclite<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Veclite<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Veclite<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

// --- Clone implementation ---

impl<T: Clone> Clone for Veclite<T> {
    /// Clones the `Veclite`, cloning all contained elements.
    ///
    /// # Example
    /// ```
    /// let mut v = veclite::Veclite::new();
    /// v.add(1);
    /// let v2 = v.clone();
    /// assert_eq!(format!("{}", v2), "1");
    /// ```
    fn clone(&self) -> Self {
        Veclite(self.0.clone())
    }
}

/// Short alias for [`Veclite`], for ergonomic use, similar to how `vec!` is used for `Vec<T>`.
///
/// # Example
/// ```
/// use veclite::Vel;
/// let mut v = Vel::new();
/// v.add(42);
/// println!("{}", v);
/// ```
pub type Vel<T> = Veclite<T>;