use crate::Err;
use crate::Err::Failed;
use std::cmp::{max, min};

pub fn first(input: &[String]) -> Result<u64, Err> {
    let mut highest_seat_id: usize = 0;
    for boarding_pass in input {
        let id = seat_id(boarding_pass.as_str());
        highest_seat_id = max(highest_seat_id, id);
    }

    Ok(highest_seat_id as u64)
}

pub fn second(input: &[String]) -> Result<u64, Err> {
    let mut all: Vec<usize> = Vec::new();

    for boarding_pass in input {
        let id = seat_id(boarding_pass.as_str());
        all.push(id);
    }

    all.sort_unstable();

    let mut prev = all.first().unwrap() - 1;
    for id in &all {
        if id - 1 != prev {
            return Ok((id - 1) as u64);
        }

        prev = *id;
    }

    Err(Failed())
}

fn seat_id(pass: &str) -> usize {
    let mut row_start: usize = 0;
    let mut row_end: usize = 127;

    let mut col_start: usize = 0;
    let mut col_end: usize = 7;

    let mut row_seats: usize = diff(row_start, row_end);
    let mut col_seats: usize = diff(col_start, col_end);

    for x in pass.split("") {
        if x == "F" {
            row_end -= row_seats;
            row_seats = diff(row_start, row_end);
        }

        if x == "B" {
            row_start += row_seats;
            row_seats = diff(row_start, row_end);
        }

        if x == "L" {
            col_end -= col_seats;
            col_seats = diff(col_start, col_end);
        }

        if x == "R" {
            col_start += col_seats;
            col_seats = diff(col_start, col_end);
        }
    }

    let row = if pass.split("").nth(7).unwrap() == "F" {
        min(row_start, row_end)
    } else {
        max(row_start, row_end)
    };

    let col = if pass.split("").nth(10).unwrap() == "L" {
        min(col_start, col_end)
    } else {
        max(col_start, col_end)
    };

    row * 8 + col
}

fn diff(start: usize, end: usize) -> usize {
    (end - start + 1) / 2
}

#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn sample_boarding_pass0() {
        assert_eq!(seat_id("FBFBBFFRLR"), 357)
    }

    #[test]
    fn sample_boarding_pass1() {
        assert_eq!(seat_id("BFFFBBFRRR"), 567)
    }

    #[test]
    fn sample_boarding_pass2() {
        assert_eq!(seat_id("FFFBBBFRRR"), 119)
    }

    #[test]
    fn sample_boarding_pass3() {
        assert_eq!(seat_id("BBFFBBFRLL"), 820)
    }

    #[test]
    fn sample_boarding_pass4() {
        assert_eq!(seat_id("FFFFFFFLLL"), 0)
    }

    #[test]
    fn sample_boarding_pass5() {
        assert_eq!(seat_id("BBBBBBBRRR"), 1023)
    }
}
