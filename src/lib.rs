#![no_std]

#[allow(unused)]
#[macro_use]
mod utils;


use wasm_bindgen::prelude::*;

const WIDTH:  i32 = 128;
const HEIGHT: i32 = 80;
const SIZE: usize = (WIDTH * HEIGHT) as usize;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

struct DoubleBuffer<T> {
    buffer: [T; 2],
    write: usize,
}

impl<T: Copy> DoubleBuffer<T> {
    fn new(data: T) -> Self {
        let buffer = [data; 2];
        DoubleBuffer { buffer, write: 1}
    }

    fn borrow_read_write(&mut self) -> (&T, &mut T) {
        let (left, right) = self.buffer.split_at_mut(1);
        let left  = &mut left[0];
        let right = &mut right[0];
        match self.write {
            0 => (right, left),
            _ => (left, right),
        }
    }

    fn borrow_read(&self) -> &T {
        &self.buffer[1 - self.write]
    }
    fn borrow_write(&self) -> &T {
        &self.buffer[self.write]
    }

    fn swap_buffers(&mut self) {
        self.write = 1 - self.write;
    }
}

#[wasm_bindgen]
pub struct Universe {
    neighbors: [[usize; 8]; SIZE],
    cells: DoubleBuffer<[Cell; SIZE]>,
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {


    pub fn new() -> Universe {
        let mut cells = [Cell::Dead; SIZE];
        for (i, cell) in cells.iter_mut().enumerate() {
            if i % 2 == 0 || i % 7 == 0 {
                *cell = Cell::Alive
            }
        }
        let cells = DoubleBuffer::new(cells);

        let neighbors = core::array::from_fn::<_, SIZE, _>(|i| {
            let (row, col) = Universe::get_row_col(i);
            let north = Universe::get_index(row - 1, col);
            let ne    = Universe::get_index(row - 1, col + 1);
            let east  = Universe::get_index(row,     col + 1);
            let se    = Universe::get_index(row + 1, col + 1);
            let south = Universe::get_index(row + 1, col);
            let sw    = Universe::get_index(row + 1, col - 1);
            let west  = Universe::get_index(row,     col - 1);
            let nw    = Universe::get_index(row - 1, col - 1);
            [north, ne, east, se, south, sw, west, nw]
        });

        Universe {
            neighbors,
            cells,
        }
    }

    fn get_row_col(index: usize) -> (i32, i32) {
        let row = index as i32 / WIDTH;
        let col = index as i32 % WIDTH;
        (row, col)
    }

    pub fn clear(&mut self) {
        let (_, cells) = self.cells.borrow_read_write();
        for cell in cells.iter_mut() {
            *cell = Cell::Dead;
        }
    }

    pub fn randomize(&mut self) {
        let (_, cells) = self.cells.borrow_read_write();
        for cell in cells.iter_mut() {
            let rand = js_sys::Math::random();
            *cell = if rand < 0.3 {
                Cell::Alive
            } else {
                Cell::Dead
            };
        }
    }

    pub fn width() -> i32 {
        Universe::WIDTH
    }

    pub fn height() -> i32 {
        Universe::HEIGHT
    }

    pub fn cells(&mut self) -> *const Cell {
        let cells = self.cells.borrow_write();
        cells.as_ptr()
    }

    pub fn insert_glider_at(&mut self, row: i32, col: i32) {
        const GLIDER: [[i32; 2]; 5] = [
            [-1,  0],
            [ 0,  1],
            [ 1, -1],
            [ 1,  0],
            [ 1,  1]
        ];
        for point in GLIDER {
            self.toggle_cell(row + point[0], col + point[1]);
        }
    }
    pub fn insert_pulsar_at(&mut self, row: i32, col: i32) {
        const PULSAR: [[i32; 2]; 6] = [
            [ 1,  2],
            [ 1,  3],
            [ 1,  4],
            [ 2,  6],
            [ 3,  6],
            [ 4,  6],
        ];
        for point in PULSAR {
            self.toggle_cell(row + point[0], col + point[1]);
            self.toggle_cell(row + point[0], col - point[1]);
            self.toggle_cell(row - point[0], col + point[1]);
            self.toggle_cell(row - point[0], col - point[1]);
            self.toggle_cell(row + point[1], col + point[0]);
            self.toggle_cell(row + point[1], col - point[0]);
            self.toggle_cell(row - point[1], col + point[0]);
            self.toggle_cell(row - point[1], col - point[0]);
        }
    }

    pub fn tick(&mut self) {

        self.cells.swap_buffers();

        let (current, next) = self.cells.borrow_read_write();

        for (i, neighbors) in self.neighbors.iter().enumerate() {
            let live_neighbors = neighbors.iter().filter(|&i| current[*i] == Cell::Alive).count();
            next[i] = current[i].tick(live_neighbors);
        }

    }

    fn get_index(row: i32, column: i32) -> usize {
        let width  = Universe::WIDTH;
        let height = Universe::HEIGHT;

        let row = if row < 0 {
            row + height
        } else if row >= height {
            row - height
        } else {
            row
        };
        let column = if column < 0 {
            column + width
        } else if column >= width {
            column - width
        } else {
            column
        };

        (row * width + column) as usize
    }

    pub fn toggle_cell(&mut self, row: i32, col: i32) {
        let idx = Universe::get_index(row, col);
        let (_, cells) = self.cells.borrow_read_write();
        cells[idx].toggle();
    }
}

impl Universe {
    const WIDTH:  i32 = WIDTH;
    const HEIGHT: i32 = HEIGHT;
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &[Cell] {
        self.cells.borrow_read()
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(i32, i32)]) {
        for &(row, col) in cells.iter() {
            let idx = Universe::get_index(row, col);
            self.cells.borrow_read_write().1[idx] = Cell::Alive;
        }
    }

}

impl Cell {
    fn toggle(&mut self) {
        *self = match self {
            Cell::Alive => Cell::Dead,
            Cell::Dead  => Cell::Alive,
        };
    }

    fn tick(&self, live_neighbors: usize) -> Cell {
        match (&self, live_neighbors) {
            // Rule 1: Any live cell with fewer than two live neighbours
            // dies, as if caused by underpopulation.
            // Rule 2: Any live cell with two or three live neighbours
            // lives on to the next generation.
            // Rule 3: Any live cell with more than three live
            // neighbours dies, as if by overpopulation.
            // Rule 4: Any dead cell with exactly three live neighbours
            // becomes a live cell, as if by reproduction.
            // All other cells remain in the same state.
            (Cell::Alive, 2) => Cell::Alive,
            (_          , 3) => Cell::Alive,
            _                => Cell::Dead,
        }
    }
}