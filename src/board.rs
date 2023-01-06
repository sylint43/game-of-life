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
pub enum State {
    Alive,
    Dead,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            State::Alive => '\u{2588}',
            State::Dead => ' ',
        };

        write!(f, "{0}{0}", ch)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Board(Vec<Vec<State>>);

impl Board {
    pub fn dead_state(width: usize, height: usize) -> Self {
        Self(vec![vec![State::Dead; width]; height])
    }

    pub fn random_state(width: usize, height: usize) -> Self {
        let mut board = Self::dead_state(width, height);
        let mut rng = rand::thread_rng();

        for row in board.0.iter_mut() {
            for state in row.iter_mut() {
                if rng.gen::<f32>() > 0.85 {
                    *state = State::Alive;
                }
            }
        }

        board
    }

    pub fn next_board_state(self) -> Self {
        let width = self.0.len();
        let height = self.0[0].len();
        let mut next_state = Board::dead_state(width, height);

        for x in 0..width {
            for y in 0..height {
                next_state.0[x][y] = Board::next_cell_state(
                    (x as isize, y as isize),
                    (width as isize, height as isize),
                    &self,
                );
            }
        }

        next_state
    }

    fn next_cell_state(coords: (isize, isize), size: (isize, isize), state: &Board) -> State {
        let (x, y) = coords;
        let (width, height) = size;
        let mut live_neighbours = 0;

        for i in x - 1..=x + 1 {
            if i < 0 || i >= width {
                continue;
            }

            for j in y - 1..=y + 1 {
                if (j < 0 || j >= height) || (x == i && y == j) {
                    continue;
                }

                if state.0[i as usize][j as usize] == State::Alive {
                    live_neighbours += 1;
                }
            }
        }

        match state.0[x as usize][y as usize] {
            State::Alive => match live_neighbours {
                2 | 3 => State::Alive,
                _ => State::Dead,
            },
            State::Dead => match live_neighbours {
                3 => State::Alive,
                _ => State::Dead,
            },
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let print = self
            .0
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
            .0
            .iter()
            .all(|row| row.iter().all(|state| *state == State::Dead)))
    }

    #[test]
    fn test_random_state() {
        let random_state = Board::random_state(3, 3);
        assert!(random_state
            .0
            .iter()
            .any(|row| row.iter().any(|state| *state == State::Alive)))
    }

    #[test]
    fn test_dead_stay_dead() {
        let inital_state = Board(vec![
            vec![State::Dead, State::Dead, State::Dead],
            vec![State::Dead, State::Dead, State::Dead],
            vec![State::Dead, State::Dead, State::Dead],
        ]);
        let expected_state = Board(vec![
            vec![State::Dead, State::Dead, State::Dead],
            vec![State::Dead, State::Dead, State::Dead],
            vec![State::Dead, State::Dead, State::Dead],
        ]);
        let next_state = inital_state.next_board_state();

        assert!(next_state == expected_state);
    }

    #[test]
    fn test_should_come_alive() {
        let inital_state = Board(vec![
            vec![State::Dead, State::Dead, State::Alive],
            vec![State::Dead, State::Alive, State::Alive],
            vec![State::Dead, State::Dead, State::Dead],
        ]);
        let expected_state = Board(vec![
            vec![State::Dead, State::Alive, State::Alive],
            vec![State::Dead, State::Alive, State::Alive],
            vec![State::Dead, State::Dead, State::Dead],
        ]);
        let next_state = inital_state.next_board_state();

        assert!(next_state == expected_state);
    }

    #[test]
    fn test_should_die_and_come_alive() {
        let inital_state = Board(vec![
            vec![State::Alive, State::Alive, State::Alive],
            vec![State::Dead, State::Alive, State::Alive],
            vec![State::Dead, State::Dead, State::Dead],
        ]);
        let expected_state = Board(vec![
            vec![State::Alive, State::Dead, State::Alive],
            vec![State::Alive, State::Dead, State::Alive],
            vec![State::Dead, State::Dead, State::Dead],
        ]);
        let next_state = inital_state.next_board_state();

        assert!(next_state == expected_state);
    }
}
