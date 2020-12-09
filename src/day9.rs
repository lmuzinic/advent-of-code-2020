use crate::Err;

pub fn first(input: &[u128], preamble_size: usize) -> Result<u128, Err> {
    for i in preamble_size..input.len() {
        if !has(input, input[i], i - preamble_size, i) {
            return Ok(input[i]);
        }
    }

    Err(Err::Failed())
}

pub fn second(input: &[u128], prev: u128) -> Result<u128, Err> {
    'i: for i in 0..input.len() {
        for j in i..input.len() {
            let sum = input[i..=j].iter().sum::<u128>();
            if sum > prev {
                continue 'i;
            }

            if sum == prev {
                let slice = input[i..=j].iter();
                let min = slice.clone().min().unwrap();
                let max = slice.max().unwrap();

                return Ok(min + max);
            }
        }
    }

    Err(Err::Failed())
}

fn has(input: &[u128], candidate: u128, start: usize, end: usize) -> bool {
    for i in start..end {
        for j in start..end {
            if input[i] == input[j] {
                continue;
            }

            if candidate == input[i] + input[j] {
                return true;
            }
        }
    }

    false
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn day9_part1_sample() -> Result<(), Err> {
        let list = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 49, 100, 50,
        ];

        assert_eq!(first(&list, 25)?, 100);

        Ok(())
    }

    #[test]
    fn day9_part1_sample2() -> Result<(), Err> {
        let list = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(first(&list, 5)?, 127);

        Ok(())
    }

    #[test]
    fn day9_part2_sample() -> Result<(), Err> {
        let list = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(second(&list, 127)?, 62);

        Ok(())
    }
}
