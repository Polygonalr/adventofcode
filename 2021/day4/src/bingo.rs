pub mod bingo_grid {
    use std::fmt;

    #[derive(Debug, Clone)]
    pub struct BingoGrid {
        number_grid: [[i32; 5]; 5],
        marked_grid: [[bool; 5]; 5],
    }

    impl BingoGrid {
        pub fn new(number_grid: [[i32; 5]; 5]) -> BingoGrid {
            BingoGrid { 
                number_grid,
                marked_grid: [[false; 5]; 5],
            }
        }
        // set marked_grid to true given a winning number
        // returns true if the number exists, false if not.
        pub fn add_winning_number(&mut self, winning_number: i32) -> bool {
            for i in 0..5 {
                for j in 0..5 {
                    if self.number_grid[i][j] == winning_number {
                    self.marked_grid[i][j] = true;
                    return true;
                    }
                }
            }
            false
        }
        // check whether there is any row or column in marked_grid full of true
        pub fn is_bingo(&self) -> bool {
            for i in 0..5 {
                if self.marked_grid[i].iter().all(|&x| x) {
                    return true;
                }
            }
            for j in 0..5 {
                if self.marked_grid.iter().map(|row| row[j]).all(|x| x) {
                    return true;
                }
            }
            false
        }
        // sum of all non-winning numbers in number_grid
        pub fn sum_of_non_winning_numbers(&self) -> i32 {
            let mut sum = 0;
            for i in 0..5 {
                for j in 0..5 {
                    if !self.marked_grid[i][j] {
                        sum += self.number_grid[i][j];
                    }
                }
            }
            sum
        }
    }

    impl fmt::Display for BingoGrid {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            writeln!(f, "Bingo Grid: {:?}", self.number_grid)
        }
    }
}