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

    pub fn fields(&mut self) -> Vec<PositionedField> {
        let mut fields = Vec::new();
        let mut y = 0;

        for line in self.fields.iter_mut() {
            let mut x = 0;

            for field in line.iter_mut() {
                fields.push(PositionedField::new(field, x, y));
                x += 1;
            }

            y += 1;
        }

        fields
    }

    pub fn field(&mut self, x: usize, y: usize) -> &mut Field {
        &mut self.fields[y][x]
    }
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
