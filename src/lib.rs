//! A crate offering a cursor (or highlight) for [`str`] slices.
//!
//! # Description
//!
//! This crate intends to ease the handwritting of parsers by providing
//! [`str`] slice with a notion of "current highlight". It should help you write
//! anything that needs to look forward in such a slice, may need to backtrack,
//! and will eventually move on from the current highlight without the need to
//! come back to it.
//!
//! # Abstract view
//! The user may see a [`StrCursor`] as a tape with two pointer on it, `head` and
//! `tail`, delimiting a (current) highlight `[tail,head[`. 
//! Once tail has moved forward, it cannot move backward.

pub mod pattern;
pub mod spanner;
#[doc(inline)]
pub use spanner::Spanner;

/// A cursor on `str` slices
///
/// It is parametred by a type that should implement the [`Spanner`] trait. A spanner
/// is used to keep track of the position in the input. Different types of spanner count
/// different positions, at different costs. Different spanner can be found in [`spanner`].
#[derive(Debug, Clone, Copy)]
pub struct StrCursor<'s, S> {
    base: &'s str,
    highlight_length: usize,
    pub spanner_tail: S,
    pub spanner_head: S,
}

impl<'s, S: Spanner + Default> StrCursor<'s, S> {
    /// Creates a new cursor with the default value of the spanner
    pub fn new(s: &'s str) -> Self {
        Self::with_spanner(s, Default::default())
    }
}

impl<'s, S: Spanner> StrCursor<'s, S> {
    /// Creates a cursor with the provided spanner
    pub fn with_spanner(s: &'s str, spanner: S) -> Self {
        Self {
            base: s,
            highlight_length: 0,
            spanner_head: spanner.clone(),
            spanner_tail: spanner,
        }
    }

    /// Indicates if the current highlight is empty
    pub fn highlight_empty(&mut self) -> bool {
        self.highlight_length == 0
    }

    /// Indicates if the post slice (that is `[tail..]`)  is empty
    pub fn post_empty(&mut self) -> bool {
        self.highlight_length == self.base.len()
    }

    /// Returns the current highlight
    pub fn highlight(&self) -> &'s str {
        unsafe { self.base.get_unchecked(..self.highlight_length) }
    }

    /// Returns the post slice
    pub fn post(&self) -> &'s str {
        unsafe { self.base.get_unchecked(self.highlight_length..) }
    }

    /// Advances `head` by one character
    ///
    /// Returns `None` if `head` is already at the end of the underlying slice.
    pub fn step(&mut self) -> Option<char> {
        let c = self.post().chars().next()?;
        self.highlight_length += c.len_utf8();
        self.spanner_head.forward(c);
        Some(c)
    }

    /// Backtracks `head` by one character
    ///
    /// Returns `None` if `head` is already at `tail`
    pub fn unstep(&mut self) -> Option<char> {
        let (c_idx, c) = self.highlight().char_indices().next_back()?;
        self.highlight_length = c_idx;
        self.spanner_head.backward(c);
        Some(c)
    }

    /// Advances `head` until `pat` is found.
    ///
    /// `pat` behaves as in [`std::str::pattern::Pattern`].
    ///
    /// The returned slice may be empty if character at head matches the pattern.
    /// If no character matching the pattern is found, then the returned slice
    /// will contain all the post slice.
    pub fn step_until<P>(&mut self, pat: P) -> &'s str
    where
        P: pattern::Pattern,
    {
        let rem_str = self.post();
        let offset = pat.find(rem_str).unwrap_or(rem_str.len());
        let res = unsafe { rem_str.get_unchecked(..offset) };
        self.highlight_length += offset;
        self.spanner_head.forward_str(res);
        res
    }

    /// Validate the current highlight
    /// 
    /// Validating the current highlight means bringing `tail` to `head`
    pub fn validate(&mut self) {
        self.base = unsafe { self.base.get_unchecked(self.highlight_length..) };
        self.highlight_length = 0;
        self.spanner_head.validate();
        self.spanner_tail = self.spanner_head.clone();
    }

    /// Runs the closure on the current highlight, validating it if the result is ok.
    pub fn then_validate<T, E, F>(&mut self, f: F) -> Result<T, E>
    where
        F: FnOnce(&'s str) -> Result<T, E>,
    {
        let res = f(self.highlight());
        if res.is_ok() {
            self.validate();
        }
        res
    }
}
