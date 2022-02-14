use rand;
use rand::Rng;
use std::ops::Range;

pub struct GameField {
    pub height: i32,
    pub width: i32,
    current_field: Vec<Vec<u8>>,
    next_field: Vec<Vec<u8>>,
}

impl GameField {
    pub fn new(width: i32, height: i32) -> GameField {
        GameField {
            height,
            width,
            current_field: vec![vec![0; width as usize]; height as usize],
            next_field: vec![vec![0; width as usize]; height as usize],
        }
    }

    pub fn current_field(&self) -> &Vec<Vec<u8>> {
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
                    *cell = 1
                } else {
                    *cell = 0
                };
            }
        }
    }
    fn count_neighbours(&self, i: i32, j: i32) -> u8 {
        self.current_field[((i - 1 + self.height) % self.height) as usize][j as usize]
            + self.current_field[((i - 1 + self.height) % self.height) as usize]
                [((j - 1 + self.width) % self.width) as usize]
            + self.current_field[((i - 1 + self.height) % self.height) as usize]
                [((j + 1 + self.width) % self.width) as usize]
            + self.current_field[i as usize][((j - 1 + self.width) % self.width) as usize]
            + self.current_field[((i + 1 + self.height) % self.height) as usize][j as usize]
            + self.current_field[((i + 1 + self.height) % self.height) as usize]
                [((j + 1 + self.width) % self.width) as usize]
            + self.current_field[((i + 1 + self.height) % self.height) as usize]
                [((j - 1 + self.width) % self.width) as usize]
            + self.current_field[i as usize][((j + 1 + self.width) % self.width) as usize]
    }

    pub fn update_field(&mut self) {
        for i in 0_i32..self.height {
            for j in 0_i32..self.width {
                let is_alive = self.current_field[i as usize][j as usize] == 1_u8;

                let count_neighbours = self.count_neighbours(i, j);
                let stay_alive = is_alive && (count_neighbours == 2 || count_neighbours == 3);
                let born = !is_alive && count_neighbours == 3;

                self.next_field[i as usize][j as usize] =
                    if stay_alive || born { 1_u8 } else { 0_u8 };
            }
        }

        std::mem::swap(&mut self.current_field, &mut self.next_field);
    }
}
