mod utils;

use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::console;

extern crate web_sys;

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty = 0,
    Sand = 1,
    Water = 2,
    Concrete = 3,
}

impl Cell {
    pub fn set_cell(&mut self, cell: Cell) {
        *self = cell;
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = 500;
        let height = 500;

        let cells = (0..width * height).map(|_i| Cell::Empty).collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Empty).collect();
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Empty).collect();
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

    pub fn render(&self) -> String {
        self.to_string()
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    pub fn tick(&mut self) {
        let mut next = { self.cells.clone() };

        {
            let mut visited: HashMap<usize, Cell> = HashMap::new();

            for row in 0..self.height {
                for col in 0..self.width {
                    let current_idx = self.get_index(row, col);
                    let cell = self.cells[current_idx];

                    if cell == Cell::Empty {
                        continue;
                    }

                    let visited_cell = visited.get(&current_idx);

                    if visited_cell.is_some() {
                        continue;
                    }

                    if cell == Cell::Water {

                        if row == self.height - 1 {
                            continue;
                        }

                        let neighbor_idx = self.get_index(row + 1, col);
                        let neighbor_cell = self.cells[neighbor_idx];

                        if visited.get(&neighbor_idx).is_some() {
                            continue;
                        }

                        if neighbor_cell == Cell::Empty {
                            // Do switch logic
                            visited.insert(neighbor_idx, neighbor_cell);
                            next[current_idx] = Cell::Empty;
                            next[neighbor_idx] = Cell::Water;
                            continue;
                        }

                        if col >= 1 {
                            let bl_neighbor_idx = self.get_index(row + 1, col - 1);
                            let bl_neighbor_cell = self.cells[bl_neighbor_idx];

                            if visited.get(&bl_neighbor_idx).is_some() {
                                continue;
                            }

                            if bl_neighbor_cell == Cell::Empty {
                                // Do switch logic
                                visited.insert(bl_neighbor_idx, bl_neighbor_cell);
                                next[current_idx] = Cell::Empty;
                                next[bl_neighbor_idx] = Cell::Water;
                                continue;
                            }

                            let l_neighbor_idx = self.get_index(row, col - 1);
                            let l_neighbor_cell = self.cells[l_neighbor_idx];

                            if visited.get(&l_neighbor_idx).is_some() {
                                continue;
                            }

                            if l_neighbor_cell == Cell::Empty {
                                // Do switch logic
                                visited.insert(l_neighbor_idx, l_neighbor_cell);
                                next[current_idx] = Cell::Empty;
                                next[l_neighbor_idx] = Cell::Water;
                                continue;
                            }
                        }

                        if col < self.width - 1 {
                            let br_neighbor_idx = self.get_index(row + 1, col + 1);
                            let br_neighbor_cell = self.cells[br_neighbor_idx];

                            if visited.get(&br_neighbor_idx).is_some() {
                                continue;
                            }

                            if br_neighbor_cell == Cell::Empty {
                                // Do switch logic
                                visited.insert(br_neighbor_idx, br_neighbor_cell);
                                next[current_idx] = Cell::Empty;
                                next[br_neighbor_idx] = Cell::Water;
                                continue;
                            }

                            let r_neighbor_idx = self.get_index(row, col + 1);
                            let r_neighbor_cell = self.cells[r_neighbor_idx];

                            if visited.get(&r_neighbor_idx).is_some() {
                                continue;
                            }

                            if r_neighbor_cell == Cell::Empty {
                                // Do switch logic
                                visited.insert(r_neighbor_idx, r_neighbor_cell);
                                next[current_idx] = Cell::Empty;
                                next[r_neighbor_idx] = Cell::Water;
                                continue;
                            }

                            
                        }
                    }

                    if cell == Cell::Sand {
                        // Handle sand logic

                        if row == self.height - 1 {
                            continue;
                        }

                        let neighbor_idx = self.get_index(row + 1, col);
                        let neighbor_cell = self.cells[neighbor_idx];

                        if visited.get(&neighbor_idx).is_some() {
                            continue;
                        }

                        if neighbor_cell == Cell::Empty {
                            // Do switch logic
                            visited.insert(neighbor_idx, neighbor_cell);
                            next[current_idx] = Cell::Empty;
                            next[neighbor_idx] = Cell::Sand;
                            continue;
                        }

                        if neighbor_cell == Cell::Water {
                            visited.insert(neighbor_idx, neighbor_cell);
                            next[current_idx] = Cell::Water;
                            next[neighbor_idx] = Cell::Sand;
                            continue;
                        }

                        if col > 1 {
                            let bl_neighbor_idx = self.get_index(row + 1, col - 1);

                            let bl_neighbor_cell = self.cells[bl_neighbor_idx];

                            if visited.get(&bl_neighbor_idx).is_some() {
                                continue;
                            }

                            if bl_neighbor_cell == Cell::Empty {
                                // Do switch logic
                                visited.insert(bl_neighbor_idx, bl_neighbor_cell);
                                next[current_idx] = Cell::Empty;
                                next[bl_neighbor_idx] = Cell::Sand;
                                continue;
                            }
                        }

                        if col < self.width - 1 {
                            let br_neighbor_idx = self.get_index(row + 1, col + 1);
                            let br_neighbor_cell = self.cells[br_neighbor_idx];

                            if visited.get(&br_neighbor_idx).is_some() {
                                continue;
                            }

                            if br_neighbor_cell == Cell::Empty {
                                // Do switch logic
                                visited.insert(br_neighbor_idx, br_neighbor_cell);
                                next[current_idx] = Cell::Empty;
                                next[br_neighbor_idx] = Cell::Sand;
                                continue;
                            }
                        }
                    }
                }
            }
        }
        self.cells = next;
    }

    pub fn set_cell(&mut self, row: u32, col: u32, cell: Cell) {
        let idx = self.get_index(row, col);
        self.cells[idx].set_cell(cell);
    }
}

impl Universe {
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Sand;
        }
    }
}

use std::fmt;

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Empty { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
