use crate::Err;

pub fn first(input: &mut [u64]) -> Result<u64, Err> {
    input.sort_unstable();

    let mut prev = 0;
    let mut ones = 0;
    let mut threes = 0;
    for adapter in input {
        match *adapter - prev {
            1 => {
                ones += 1;
            }
            3 => {
                threes += 1;
            }
            _ => {}
        }

        prev = *adapter;
    }

    threes += 1;

    Ok(ones * threes)
}

pub fn second(input: &mut [u64]) -> Result<u64, Err> {
    input.sort_unstable();

    let last = input.last().unwrap();
    let size = (last + 1) as usize;

    let mut possibilities = vec![0; size];
    for &adapter in input.iter() {
        if adapter <= 3 {
            possibilities[adapter as usize] += 1;

            for x in (1..adapter).rev() {
                possibilities[adapter as usize] += possibilities[x as usize];
            }

            continue;
        }

        possibilities[adapter as usize] += possibilities[adapter as usize - 3];
        possibilities[adapter as usize] += possibilities[adapter as usize - 2];
        possibilities[adapter as usize] += possibilities[adapter as usize - 1];
    }

    Ok(possibilities[possibilities.len() - 1])
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn day10_part1_sample() -> Result<(), Err> {
        let mut list = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

        assert_eq!(first(&mut list.as_mut_slice())?, 7 * 5);

        Ok(())
    }

    #[test]
    fn day10_part1_sample2() -> Result<(), Err> {
        let mut list = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];

        assert_eq!(first(&mut list.as_mut_slice())?, 22 * 10);

        Ok(())
    }

    #[test]
    fn day10_part2_sample() -> Result<(), Err> {
        let mut list = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];

        assert_eq!(second(&mut list.as_mut_slice())?, 8);

        Ok(())
    }

    #[test]
    fn day10_part2_sample2() -> Result<(), Err> {
        let mut list = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];

        assert_eq!(second(&mut list.as_mut_slice())?, 19208);

        Ok(())
    }
}
