use std::io::{self, Write};

use crossterm::{cursor::MoveTo, execute, queue, style::Print, terminal};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Cell {
    symbol: char,
}

impl Default for Cell {
    fn default() -> Self {
        Self { symbol: ' ' }
    }
}

#[derive(Debug, Clone)]
struct Buffer {
    content: Vec<Cell>,
    area: Rect,
}

impl Buffer {
    fn diff(&self, other: &Buffer) -> Vec<(Cell, u16, u16)> {
        if self.area != other.area {
            enumerate_2d(&self.content, self.area).collect()
        } else {
            enumerate_2d(&self.content, self.area)
                .filter(|(cell, x, y)| {
                    let other_cell =
                        other.content[*y as usize * self.area.width as usize + *x as usize];
                    *cell != other_cell
                })
                .collect()
        }
    }
}

fn enumerate_2d(items: &Vec<Cell>, area: Rect) -> impl Iterator<Item = (Cell, u16, u16)> + '_ {
    assert_eq!(
        items.len(),
        area.width as usize * area.height as usize,
        "{area:?}"
    );
    items.iter().enumerate().map(move |(i, item)| {
        (
            *item,
            (i % area.width as usize) as u16,
            (i / area.width as usize) as u16,
        )
    })
}

impl Default for Buffer {
    fn default() -> Self {
        let (width, height) = terminal::size().unwrap();
        let area = Rect {
            top: 0,
            left: 0,
            height,
            width,
        };

        let content = vec![Cell::default(); height as usize * width as usize];
        Self { content, area }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Rect {
    pub top: u16,
    pub left: u16,
    pub height: u16,
    pub width: u16,
}

#[derive(Debug)]
pub struct Terminal<W: Write> {
    buffers: [Buffer; 2],
    current_buf: usize,
    writer: W,
}

impl<W: Write> Terminal<W> {
    pub fn new(writer: W) -> io::Result<Self> {
        Ok(Self {
            buffers: [Buffer::default(), Buffer::default()],
            current_buf: 0,
            writer,
        })
    }

    pub fn draw(&mut self) -> io::Result<()> {
        let diff = self.buffers[self.current_buf].diff(&self.buffers[1 - self.current_buf]);

        for (cell, x, y) in diff {
            queue!(self.writer, MoveTo(x, y))?;
            queue!(self.writer, Print(cell.symbol))?;
        }

        self.writer.flush()?;

        // swap buffers
        self.current_buf = 1 - self.current_buf;
        self.buffers[self.current_buf] = self.buffers[1 - self.current_buf].clone();

        Ok(())
    }
}

impl<W: Write> Terminal<W> {
    pub fn set(&mut self, c: char, i: usize) {
        self.buffers[self.current_buf].content[i] = Cell { symbol: c }
    }

    pub fn set_cursor(&mut self, i: usize) -> io::Result<()> {
        execute!(
            self.writer,
            MoveTo(
                (i % self.buffers[self.current_buf].area.width as usize) as u16,
                (i / self.buffers[self.current_buf].area.width as usize) as u16,
            )
        )?;
        Ok(())
    }
}
