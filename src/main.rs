mod board;
use board::Board;

fn main() {
    let mut board = Board::new(3, 4, 5);
    board.field(1, 1).set_mine();

    for mut field in board.fields() {
        println!("Field: {:#?}", field);
    }
}
