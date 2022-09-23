mod board;
use board::Board;

fn main() {
    let mut board = Board::new(3, 4, 5);
    board.field(1, 1).set_mine();

    for mut positioned_field in board.positioned_fields() {
        if positioned_field.x() == 1 && positioned_field.y() == 2 {
            positioned_field.field().set_mine();
        }
    }

    for positioned_field in board.positioned_fields() {
        println!("Field: {:#?}", positioned_field);
    }
}
