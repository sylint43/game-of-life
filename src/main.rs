use std::{thread::sleep, time::Duration};

use game_of_life::board::Board;
fn main() {
    let mut board = Board::random_state(40, 40);
    loop {
        print!("\x1B[2J\x1B[1;1H"); // Clear screen, put cursor at top left
        println!("{}", board);
        board = board.next_board_state();
        sleep(Duration::from_millis(100))
    }
}
