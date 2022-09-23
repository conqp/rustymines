mod board;
use board::Board;

fn main() {
    let mut board = Board::new(3, 4, 5);
    board.test();
}
