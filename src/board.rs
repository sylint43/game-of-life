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

use rand::random;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Alive,
    Dead,
}

pub struct Board(Vec<Vec<State>>);

impl Board {
    pub fn dead_state(width: usize, height: usize) -> Self {
        Self(vec![vec![State::Dead; width]; height])
    }

    pub fn random_state(width: usize, height: usize) -> Self {
        let mut board = Self::dead_state(width, height);

        for row in board.0.iter_mut() {
            for state in row.iter_mut() {
                if random() {
                    *state = State::Alive;
                }
            }
        }

        board
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
}