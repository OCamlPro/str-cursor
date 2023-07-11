//! A polyfill module to have similar functionality to 
//! [`std::str::pattern::Pattern`] on stable

/// A polyfill trait to have similar functionality to 
/// [`std::str::pattern::Pattern`] on stable
pub trait Pattern {
    fn find(self, s: &str) -> Option<usize>;
}

impl Pattern for char {
    fn find(self, s: &str) -> Option<usize> {
        s.find(self)
    }
}
impl<'b> Pattern for &'b str {
    fn find(self, s: &str) -> Option<usize> {
        s.find(self)
    }
}
impl<'b> Pattern for &'b [char] {
    fn find(self, s: &str) -> Option<usize> {
        s.find(self)
    }
}
impl<'b, 'c> Pattern for &'c &'b str {
    fn find(self, s: &str) -> Option<usize> {
        s.find(self)
    }
}
impl<'b, const N: usize> Pattern for &'b [char; N] {
    fn find(self, s: &str) -> Option<usize> {
        s.find(self)
    }
}
impl<F> Pattern for F
where
    F: FnMut(char) -> bool,
{
    fn find(self, s: &str) -> Option<usize> {
        s.find(self)
    }
}
impl<const N: usize> Pattern for [char; N] {
    fn find(self, s: &str) -> Option<usize> {
        s.find(self)
    }
}
impl<'b> Pattern for &'b String {
    fn find(self, s: &str) -> Option<usize> {
        s.find(self)
    }
}
