use crossterm::terminal;

/// A simple struct representing a rectangular region of the terminal.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    /// The coordinate of the top side of the rectangle.
    pub top: u16,
    /// The coordinate of the left side of the rectangle.
    pub left: u16,
    /// Height of the rectangle.
    pub height: u16,
    /// Width of the rectangle
    pub width: u16,
}

impl Rect {
    /// Get a rect representing the current size of the terminal being written to.
    pub(super) fn get_size() -> Self {
        let (width, height) =
            terminal::size().expect("unable to get the dimensions of the terminal");
        Self {
            top: 0,
            left: 0,
            height,
            width,
        }
    }

    #[inline]
    pub fn partition<S: Partition>(self, partition: S) -> Vec<Rect> {
        partition.partition(self)
    }
}

// TODO: Is there some way to return something like [Rect; 4]
// or maybe Iterator<Item = Rect>?
pub trait Partition {
    fn partition(&self, area: Rect) -> Vec<Rect>;
}

pub struct Bottom;

impl Partition for Bottom {
    fn partition(&self, area: Rect) -> Vec<Rect> {
        vec![Rect {
            top: area.top + area.height - 1,
            height: 1,
            ..area
        }]
    }
}
