use crate::Err;

pub fn first(input: &[String]) -> Result<u64, Err> {
    traverse(input, Position::new(3, 1))
}

pub fn second(input: &[String]) -> Result<u64, Err> {
    Ok(traverse(input, Position::new(1, 1))?
        * traverse(input, Position::new(3, 1))?
        * traverse(input, Position::new(5, 1))?
        * traverse(input, Position::new(7, 1))?
        * traverse(input, Position::new(1, 2))?)
}

fn traverse(input: &[String], mut p: Position) -> Result<u64, Err> {
    let mut tree_count = 0;

    p.slide();
    while let Some(line) = input.get(p.y as usize) {
        if line.chars().nth(p.x as usize).unwrap().eq(&'#') {
            tree_count += 1;
        }

        p.slide();
    }

    Ok(tree_count)
}

struct Position {
    x: u32,
    y: u32,

    move_x: u32,
    move_y: u32,
}

impl Position {
    pub fn slide(&mut self) {
        self.x += self.move_x;
        if self.x >= 31 {
            self.x -= 31;
        }

        self.y += self.move_y;
    }

    pub fn new(move_x: u32, move_y: u32) -> Self {
        Position {
            x: 0,
            y: 0,
            move_x,
            move_y,
        }
    }
}
