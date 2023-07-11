//! A module containing different spanners made to keep track
//! of the location in a slice

pub trait Spanner : Clone {
    fn forward(&mut self, c: char);
    fn backward(&mut self, c: char);
    fn validate(&mut self);

    fn forward_str(&mut self, s: &str) {
        for c in s.chars() {
            self.forward(c)
        }
    }
}

/// A spanner that does nothing at zero cost
#[derive(Debug, Clone, Copy, Default)]
pub struct NoOpSpanner;

impl Spanner for NoOpSpanner {
    fn forward(&mut self, _c: char) {}

    fn backward(&mut self, _c: char) {}

    fn validate(&mut self) {}

    fn forward_str(&mut self, _s: &str) {}
}

/// A spanner that counts how many bytes have been passed
#[derive(Debug, Clone, Copy, Default)]
pub struct ByteSpanner {
    pub bytes: usize,
}

impl Spanner for ByteSpanner {
    fn forward(&mut self, c: char) {
        self.bytes += c.len_utf8();
    }

    fn backward(&mut self, c: char) {
        self.bytes -= c.len_utf8();
    }

    fn validate(&mut self) {}

    fn forward_str(&mut self, s: &str) {
        self.bytes += s.len();
    }
}

/// A spanner that counts how many characters have been passed
#[derive(Debug, Clone, Copy, Default)]
pub struct CharSpanner {
    kars: usize,
}

impl Spanner for CharSpanner {
    fn forward(&mut self, _c: char) {
        self.kars += 1;
    }

    fn backward(&mut self, _c: char) {
        self.kars -= 1;
    }

    fn validate(&mut self) {}

    fn forward_str(&mut self, s: &str) {
        self.kars += s.chars().count()
    }
}

/// A spanner keeping track of rows and columns
/// 
/// More expansive than the others.
#[derive(Debug, Clone, Default)]
pub struct RowColSpanner {
    row: usize,
    col: usize,
    old_cols: Vec<usize>,
}
impl Spanner for RowColSpanner {
    fn forward(&mut self, c: char) {
        match c {
            '\n' => {
                self.old_cols.push(self.col);
                self.row += 1;
                self.col = 0;
            }
            c if c.is_control() => (),
            _ => self.col += 1,
        }
    }

    fn backward(&mut self, c: char) {
        match c {
            '\n' => {
                self.row -= 1;
                self.col = self.old_cols.pop().unwrap();
            }
            c if c.is_control() => (),
            _ => self.col -= 1,
        }
    }

    fn validate(&mut self) {
        self.old_cols = vec![];
    }

    // forxard_str left to default implem
}
