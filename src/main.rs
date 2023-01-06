use game_of_life::board::Board;
fn main() {
    let board = Board::random_state(20, 30);
    println!("{}", board);
}
