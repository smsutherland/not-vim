//! Module for mainly for the [`Rect`] struct, the [`Partition`] and any implimentors of
//! `Partition`.
//!
//! A [`Rect`] represents a region of the terminal screen.

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
    /// Get a [`Rect`] representing the current size of the terminal being written to.
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

    /// Take a [`Partition`]er and use it to split the current [`Rect`].
    ///
    /// This is mainly a convenience function and so
    /// ```
    /// rect.partition(some_partitioner);
    /// ```
    /// is equivalent to
    /// ```
    /// some_partitioner.partition(rect);
    /// ```
    ///
    ///  
    #[inline]
    pub fn partition<S: Partition>(self, partition: S) -> Vec<Rect> {
        partition.partition(self)
    }
}

// TODO: Is there some way to return something like [Rect; 4]
// or maybe Iterator<Item = Rect>?
/// Turn a single [`Rect`] into many smaller [`Rect`]s.
///
/// The [`Rect`]s returned shoud be, but are not required to be a non-overlapping, complete covering of the
/// provided [`Rect`], with no spill out beyond the bounds of the provided [`Rect`].
///
/// The following is an example of partitioning a [`Rect`] using [`Bottom`].
/// ```
/// let initial_rect = Rect {
///     top: 0,
///     left: 10,
///     height: 5,
///     width: 3,
/// };
/// let parts = initial_rect.partition(Bottom);
/// assert_eq!(parts[0], Rect {
///     top: 4,
///     left: 10,
///     height: 1,
///     width: 3,
/// });
/// assert_eq!(parts[1], Rect {
///     top: 0,
///     left: 10,
///     height: 4,
///     width: 3,
/// });
/// ```
pub trait Partition {
    /// Split a [`Rect`] into individual parts.
    /// See the trait documentation for mode.
    fn partition(&self, area: Rect) -> Vec<Rect>;
}

/// A [`Partition`]er which splits a [`Rect`] into the bottom row and the rest.
///
/// The returned Vec has two elements.
/// `return[0]` is the bottom row of the [`Rect`].
/// `return[1]` is the remainder of the [`Rect`].
///
/// See [`Partition`] for more information about how to use this struct.
pub struct Bottom;

impl Partition for Bottom {
    fn partition(&self, area: Rect) -> Vec<Rect> {
        vec![
            Rect {
                top: area.top + area.height - 1,
                height: 1,
                ..area
            },
            Rect {
                height: area.height - 1,
                ..area
            },
        ]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn using_bottom() {
        let initial_rect = Rect {
            top: 0,
            left: 10,
            height: 5,
            width: 3,
        };
        let parts = initial_rect.partition(Bottom);
        assert_eq!(
            parts[0],
            Rect {
                top: 4,
                left: 10,
                height: 1,
                width: 3,
            }
        );
        assert_eq!(
            parts[1],
            Rect {
                top: 0,
                left: 10,
                height: 4,
                width: 3,
            }
        );
    }
}
