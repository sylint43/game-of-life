// Copyright (C) 2023 Sylvia Waldron
//
// This file is part of game_of_life.
//
// game_of_life is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// game_of_life is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with game_of_life.  If not, see <http://www.gnu.org/licenses/>.
use rand::Rng;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Alive,
    Dead,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Cell::Alive => '\u{2588}',
            Cell::Dead => ' ',
        };

        write!(f, "{0}{0}", ch)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Board {
    size: (usize, usize),
    state: Vec<Vec<Cell>>,
}

impl Board {
    pub fn dead_state(width: usize, height: usize) -> Self {
        Self {
            size: (width, height),
            state: vec![vec![Cell::Dead; width]; height],
        }
    }

    pub fn random_state(width: usize, height: usize) -> Self {
        let mut board = Self::dead_state(width, height);
        let mut rng = rand::thread_rng();

        for row in board.state.iter_mut() {
            for cell in row.iter_mut() {
                if rng.gen::<f32>() > 0.85 {
                    *cell = Cell::Alive;
                }
            }
        }

        board
    }

    pub fn next_board_state(&self) -> Self {
        let (width, height) = self.size;
        let mut next_state = Board::dead_state(width, height);

        for row in 0..height {
            for col in 0..width {
                next_state.state[row][col] = self.next_cell_state((col as isize, row as isize));
            }
        }

        next_state
    }

    fn next_cell_state(&self, coords: (isize, isize)) -> Cell {
        let (x, y) = coords;
        let (width, height) = (self.size.0 as isize, self.size.1 as isize);
        let mut live_neighbours = 0;

        for row in y - 1..=y + 1 {
            if row < 0 || row >= height {
                continue;
            }

            for col in x - 1..=x + 1 {
                if (col < 0 || col >= width) || (x == col && y == row) {
                    continue;
                }

                if self.state[row as usize][col as usize] == Cell::Alive {
                    live_neighbours += 1;
                }
            }
        }

        match self.state[y as usize][x as usize] {
            Cell::Alive => match live_neighbours {
                2 | 3 => Cell::Alive,
                _ => Cell::Dead,
            },
            Cell::Dead => match live_neighbours {
                3 => Cell::Alive,
                _ => Cell::Dead,
            },
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let print = self
            .state
            .iter()
            .map(|row| {
                row.iter()
                    .map(|state| format!("{}", state))
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", print)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dead_state() {
        let dead_state = Board::dead_state(4, 4);
        assert!(dead_state
            .state
            .iter()
            .all(|row| row.iter().all(|state| *state == Cell::Dead)))
    }

    #[test]
    fn test_random_state() {
        let random_state = Board::random_state(3, 3);
        assert!(random_state
            .state
            .iter()
            .any(|row| row.iter().any(|state| *state == Cell::Alive)))
    }

    #[test]
    fn test_dead_stay_dead() {
        let inital_state = Board {
            state: vec![
                vec![Cell::Dead, Cell::Dead, Cell::Dead],
                vec![Cell::Dead, Cell::Dead, Cell::Dead],
                vec![Cell::Dead, Cell::Dead, Cell::Dead],
            ],
            size: (3, 3),
        };
        let expected_state = Board {
            state: vec![
                vec![Cell::Dead, Cell::Dead, Cell::Dead],
                vec![Cell::Dead, Cell::Dead, Cell::Dead],
                vec![Cell::Dead, Cell::Dead, Cell::Dead],
            ],
            size: (3, 3),
        };
        let next_state = inital_state.next_board_state();

        assert!(next_state == expected_state);
    }

    #[test]
    fn test_should_come_alive() {
        let inital_state = Board {
            state: vec![
                vec![Cell::Dead, Cell::Dead, Cell::Alive],
                vec![Cell::Dead, Cell::Alive, Cell::Alive],
                vec![Cell::Dead, Cell::Dead, Cell::Dead],
            ],
            size: (3, 3),
        };
        let expected_state = Board {
            state: vec![
                vec![Cell::Dead, Cell::Alive, Cell::Alive],
                vec![Cell::Dead, Cell::Alive, Cell::Alive],
                vec![Cell::Dead, Cell::Dead, Cell::Dead],
            ],
            size: (3, 3),
        };
        let next_state = inital_state.next_board_state();

        assert!(next_state == expected_state);
    }

    #[test]
    fn test_should_die_and_come_alive() {
        let inital_state = Board {
            state: vec![
                vec![Cell::Alive, Cell::Alive, Cell::Alive],
                vec![Cell::Dead, Cell::Alive, Cell::Alive],
                vec![Cell::Dead, Cell::Dead, Cell::Dead],
            ],
            size: (3, 3),
        };
        let expected_state = Board {
            state: vec![
                vec![Cell::Alive, Cell::Dead, Cell::Alive],
                vec![Cell::Alive, Cell::Dead, Cell::Alive],
                vec![Cell::Dead, Cell::Dead, Cell::Dead],
            ],
            size: (3, 3),
        };
        let next_state = inital_state.next_board_state();

        assert!(next_state == expected_state);
    }
}
