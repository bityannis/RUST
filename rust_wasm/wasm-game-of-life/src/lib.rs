#![allow(unused_variables)]
use std::fmt;
mod utils;
extern crate js_sys;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    // Public methods, exported to JavaScript.
    #[wasm_bindgen]
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    // Public methods, exported to JavaScript.
    #[wasm_bindgen]
    pub fn new() -> Universe {
        //Patterns
        let width = 100;
        let height = 100;
        // let h_offset = 40;
        // let v_offset = 45;

        //HI Pattern
        // let cells = (0..width * height)
        //     .map(|i| {
        //         let row = i / width;
        //         let col = i % width;

        //         // bityannis stable pattern
        //         if (row >= 20 && row < 25) && (col >= 20 && col < 40)
        //             || (row >= 20 && row < 25) && (col >= 41 && col < 42)
        //             || (row >= 25 && row < 30) && (col >= 60 && col < 80)
        //             || (row >= 30 && row < 35) && (col >= 80 && col < 100)
        //             || (row >= 35 && row < 40) && (col >= 100 && col < 110)
        //             || (row >= 35 && row < 40) && (col >= 120 && col < 130)
        //         {
        //             Cell::Alive
        //         } else {
        //             Cell::Dead
        //         }
        //     })
        //     .collect();

        ////Predefine pattern generation
        // let cells = (0..width * height)
        //     .map(|i| {
        //         if i % 2 == 0 || i % 7 == 0 {
        //             Cell::Alive
        //         } else {
        //             Cell::Dead
        //         }
        //     })
        //     .collect();

        ////Random Generation
        let cells = (0..width * height)
            .map(|_| {
                if js_sys::Math::random() < 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        ////Single spaceship
        // let cells = vec![Cell::Dead; (width * height) as usize];
        // let mut cells = vec![Cell::Dead; (width * height) as usize];
        // cells[1] = Cell::Alive;
        // cells[width as usize + 2] = Cell::Alive;
        // cells[2 * width as usize] = Cell::Alive;
        // cells[2 * width as usize + 1] = Cell::Alive;
        // cells[2 * width as usize + 2] = Cell::Alive;

        Universe {
            width,
            height,
            cells,
        }
    }

    #[wasm_bindgen]
    pub fn render(&self) -> String {
        self.to_string()
    }

    #[wasm_bindgen]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[wasm_bindgen]
    pub fn height(&self) -> u32 {
        self.height
    }

    #[wasm_bindgen]
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)] //Import console.log built-in JS function
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("{} greet you 🌍", name));
    log(&format!("{} say hi from here aswell 👨🏻‍💻", name));
}
