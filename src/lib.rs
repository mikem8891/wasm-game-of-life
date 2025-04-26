#[allow(unused)]
#[macro_use]
mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width:  u32,
    height: u32,
    cells: Box<[Cell]>,
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {

    pub fn new(width: u32, height: u32) -> Universe {
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn clear(&mut self) {
        for cell in self.cells.iter_mut() {
            *cell = Cell::Dead;
        }
    }

    pub fn randomize(&mut self) {
        for cell in self.cells.iter_mut() {
            let rand = js_sys::Math::random();
            *cell = if rand < 0.3 {
                Cell::Alive
            } else {
                Cell::Dead
            };
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
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

        let mut next = self.cells.clone();

        for row in 0..self.height as i32 {
            for col in 0..self.width as i32 {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
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
                    (_, 3) => Cell::Alive,
                    _ => Cell::Dead,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    fn get_index(&self, row: i32, column: i32) -> usize {
        let width  = self.width  as i32;
        let height = self.height as i32;

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

    fn live_neighbor_count(&self, row: i32, column: i32) -> u8 {
        let mut count = 0;
        const NEIGHBOR_DELTA: [[i32; 2]; 8] = [
            [-1, -1],
            [-1,  0],
            [-1,  1],
            [ 0, -1],
            [ 0,  1],
            [ 1, -1],
            [ 1,  0],
            [ 1,  1],
        ];
        for [delta_row, delta_col] in NEIGHBOR_DELTA {
            let neighbor_row = row    + delta_row;
            let neighbor_col = column + delta_col;
            let idx = self.get_index(neighbor_row, neighbor_col);
            count += self.cells[idx] as u8;
        }
        count
    }

    pub fn toggle_cell(&mut self, row: i32, col: i32) {
        let idx = self.get_index(row, col);
        self.cells[idx].toggle();
    }
}

impl Universe {
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(i32, i32)]) {
        for &(row, col) in cells.iter() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }

}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.chunks(self.width as usize) {
            for &cell in line {
                let symbol = match cell {
                    Cell::Dead  => '◻',
                    Cell::Alive => '◼',
                };
                write!(f, "{symbol}")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Cell {
    fn toggle(&mut self) {
        *self = match self {
            Cell::Alive => Cell::Dead,
            Cell::Dead  => Cell::Alive,
        };
    }
}