use rand::seq::SliceRandom;

mod field;
use field::Field;
use field::PositionedField;

#[derive(Debug)]
pub struct Board {
    fields: Vec<Vec<Field>>,
    mines: u8,
}

impl Board {
    pub fn new(width: u8, height: u8, mines: u8) -> Board {
        Board {
            fields: gen_fields(width, height),
            mines: mines,
        }
    }

    pub fn width(&self) -> usize {
        self.fields[0].len()
    }

    pub fn height(&self) -> usize {
        self.fields.len()
    }

    fn fields(&mut self) -> Vec<&mut Field> {
        self.fields.iter_mut().flat_map(|line| line).collect()
    }

    pub fn positioned_fields(&mut self) -> Vec<PositionedField> {
        self.fields
            .iter_mut()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter_mut()
                    .enumerate()
                    .map(move |(x, field)| PositionedField::new(x, y, field))
            })
            .collect()
    }

    pub fn field(&mut self, x: usize, y: usize) -> &mut Field {
        &mut self.fields[y][x]
    }

    /*
    fn populate_mines(&mut self) {
        let fields = self.fields();
        let fields_to_mine: Vec<_> = fields
            .choose_multiple(&mut rand::thread_rng(), self.mines.into())
            .collect();
    }
    */
}

fn gen_fields(width: u8, height: u8) -> Vec<Vec<Field>> {
    let mut lines = Vec::new();

    for _ in 0..height {
        lines.push(make_line(width));
    }

    lines
}

fn make_line(width: u8) -> Vec<Field> {
    let mut columns = Vec::new();

    for _ in 0..width {
        columns.push(Field::new());
    }

    columns
}
