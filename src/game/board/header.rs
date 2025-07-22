use std::fmt::{Display, Formatter};

/// Header of the board for displaying.
#[derive(Debug)]
pub struct Header {
    width: usize,
}

impl Header {
    #[must_use]
    pub const fn new(width: usize) -> Self {
        Self { width }
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, " │")?;
        let max_column = self.width.saturating_sub(1);

        for x in 0..self.width {
            write!(f, "{x:x}")?;

            if x < max_column {
                write!(f, " ")?;
            }
        }

        write!(f, "\n─┼")?;

        for _ in 0..=max_column.saturating_mul(2) {
            write!(f, "─")?;
        }

        Ok(())
    }
}
