use rand;
use rand::Rng;
use std::ops::Range;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0_u8,
    Alive = 1_u8,
}

pub struct GameField {
    pub height: i32,
    pub width: i32,
    current_field: Vec<Vec<Cell>>,
    next_field: Vec<Vec<Cell>>,
}

impl GameField {
    pub fn new(width: i32, height: i32) -> GameField {
        GameField {
            height,
            width,
            current_field: vec![vec![Cell::Dead; width as usize]; height as usize],
            next_field: vec![vec![Cell::Dead; width as usize]; height as usize],
        }
    }

    pub fn current_field(&self) -> &Vec<Vec<Cell>> {
        &self.current_field
    }

    pub fn fill_random(&mut self, fill_percent: u8) {
        let mut rng = rand::thread_rng();
        for row in self.current_field.iter_mut() {
            for cell in row.iter_mut() {
                if rng.gen_range(Range {
                    start: 0_u8,
                    end: 101_u8,
                }) <= fill_percent
                {
                    *cell = Cell::Alive
                } else {
                    *cell = Cell::Dead
                };
            }
        }
    }
    fn count_neighbours(&self, i: i32, j: i32) -> u8 {
        let mut neighbours_counter = 0_u8;

        for row_shift in [-1, 0, 1] {
            for col_shift in [-1, 0, 1] {
                if row_shift == 0 && col_shift == 0 {
                    continue;
                }
                let row_pos = ((i + row_shift + self.height) % self.height) as usize;
                let col_pos = ((j + col_shift + self.width) % self.width) as usize;
                neighbours_counter += self.current_field[row_pos][col_pos] as u8;
            }
        }
        neighbours_counter
    }

    pub fn update_field(&mut self) {
        for i in 0_i32..self.height {
            for j in 0_i32..self.width {
                let is_alive = self.current_field[i as usize][j as usize] == Cell::Alive;

                let count_neighbours = self.count_neighbours(i, j);
                let stay_alive = is_alive && (count_neighbours == 2 || count_neighbours == 3);
                let born = !is_alive && count_neighbours == 3;

                self.next_field[i as usize][j as usize] = if stay_alive || born {
                    Cell::Alive
                } else {
                    Cell::Dead
                };
            }
        }

        std::mem::swap(&mut self.current_field, &mut self.next_field);
    }
}
