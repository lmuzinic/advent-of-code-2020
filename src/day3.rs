use crate::Err;

pub fn first(input: &[String]) -> Result<u64, Err> {
    let width = input.first().unwrap().len();
    traverse(input, Position::new(3, 1, width as u32))
}

pub fn second(input: &[String]) -> Result<u64, Err> {
    let width = input.first().unwrap().len();
    Ok(traverse(input, Position::new(1, 1, width as u32))?
        * traverse(input, Position::new(3, 1, width as u32))?
        * traverse(input, Position::new(5, 1, width as u32))?
        * traverse(input, Position::new(7, 1, width as u32))?
        * traverse(input, Position::new(1, 2, width as u32))?)
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

    width: u32,
}

impl Position {
    pub fn slide(&mut self) {
        self.x += self.move_x;
        if self.x >= self.width {
            self.x -= self.width;
        }

        self.y += self.move_y;
    }

    pub fn new(move_x: u32, move_y: u32, width: u32) -> Self {
        Position {
            x: 0,
            y: 0,
            move_x,
            move_y,
            width,
        }
    }
}

#[allow(unused_imports, dead_code)]
mod tests {
    use super::*;

    fn input() -> Vec<String> {
        let input = vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ];

        input.iter().map(|s| s.to_string()).collect::<Vec<String>>()
    }

    #[test]
    fn day3_part1_sample() -> Result<(), Err> {
        let path = input();

        assert_eq!(first(&path)?, 7);

        Ok(())
    }

    #[test]
    fn day3_part2_sample() -> Result<(), Err> {
        let path = input();

        assert_eq!(second(&path)?, 336);

        Ok(())
    }
}
