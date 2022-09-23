mod board;
use board::Board;

fn main() {
    let mut board = Board::new(3, 4, 5);

    for positioned_field in board.positioned_fields() {
        println!("Field: {:#?}", positioned_field);
    }

    board.populate_mines();

    for positioned_field in board.positioned_fields() {
        println!("Field: {:#?}", positioned_field);
    }
}
