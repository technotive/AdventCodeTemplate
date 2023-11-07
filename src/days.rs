use std::fmt::Display;

use crate::solutions::Solution;

mod d00;
mod d01; mod d02; mod d03; mod d04; mod d05; mod d06; mod d07; mod d08; mod d09; mod d10;
mod d11; mod d12; mod d13; mod d14; mod d15; mod d16; mod d17; mod d18; mod d19; mod d20;
mod d21; mod d22; mod d23; mod d24; mod d25;

pub fn solve(i: usize, input: &String) {
    match i {
        0  => Day::new(
            Solution::new(d00::one, Some(d00::ONE.into()), Some(10)),
            Solution::new(d00::two, Some(d00::TWO.into()), Some(10))
        ).solve(input),
        1  => Day::new(Solution::new(d01::one, None, Some(10)),Solution::new(d01::two, None, Some(10))).solve(input),
        2  => Day::new(Solution::new(d02::one, None, Some(10)),Solution::new(d02::two, None, Some(10))).solve(input),
        3  => Day::new(Solution::new(d03::one, None, Some(10)),Solution::new(d03::two, None, Some(10))).solve(input),
        4  => Day::new(Solution::new(d04::one, None, Some(10)),Solution::new(d04::two, None, Some(10))).solve(input),
        5  => Day::new(Solution::new(d05::one, None, Some(10)),Solution::new(d05::two, None, Some(10))).solve(input),
        6  => Day::new(Solution::new(d06::one, None, Some(10)),Solution::new(d06::two, None, Some(10))).solve(input),
        7  => Day::new(Solution::new(d07::one, None, Some(10)),Solution::new(d07::two, None, Some(10))).solve(input),
        8  => Day::new(Solution::new(d08::one, None, Some(10)),Solution::new(d08::two, None, Some(10))).solve(input),
        9  => Day::new(Solution::new(d09::one, None, Some(10)),Solution::new(d09::two, None, Some(10))).solve(input),
        10 => Day::new(Solution::new(d10::one, None, Some(10)),Solution::new(d10::two, None, Some(10))).solve(input),
        11 => Day::new(Solution::new(d11::one, None, Some(10)),Solution::new(d11::two, None, Some(10))).solve(input),
        12 => Day::new(Solution::new(d12::one, None, Some(10)),Solution::new(d12::two, None, Some(10))).solve(input),
        13 => Day::new(Solution::new(d13::one, None, Some(10)),Solution::new(d13::two, None, Some(10))).solve(input),
        14 => Day::new(Solution::new(d14::one, None, Some(10)),Solution::new(d14::two, None, Some(10))).solve(input),
        15 => Day::new(Solution::new(d15::one, None, Some(10)),Solution::new(d15::two, None, Some(10))).solve(input),
        16 => Day::new(Solution::new(d16::one, None, Some(10)),Solution::new(d16::two, None, Some(10))).solve(input),
        17 => Day::new(Solution::new(d17::one, None, Some(10)),Solution::new(d17::two, None, Some(10))).solve(input),
        18 => Day::new(Solution::new(d18::one, None, Some(10)),Solution::new(d18::two, None, Some(10))).solve(input),
        19 => Day::new(Solution::new(d19::one, None, Some(10)),Solution::new(d19::two, None, Some(10))).solve(input),
        20 => Day::new(Solution::new(d20::one, None, Some(10)),Solution::new(d20::two, None, Some(10))).solve(input),
        21 => Day::new(Solution::new(d21::one, None, Some(10)),Solution::new(d21::two, None, Some(10))).solve(input),
        22 => Day::new(Solution::new(d22::one, None, Some(10)),Solution::new(d22::two, None, Some(10))).solve(input),
        23 => Day::new(Solution::new(d23::one, None, Some(10)),Solution::new(d23::two, None, Some(10))).solve(input),
        24 => Day::new(Solution::new(d24::one, None, Some(10)),Solution::new(d24::two, None, Some(10))).solve(input),
        25 => Day::new(Solution::new(d25::one, None, Some(10)),Solution::new(d25::two, None, Some(10))).solve(input),
        _ => eprintln!("No solution for {i}")
    }
}

pub struct Day<T, U> where T: Eq + Display, U: Eq + Display {
    one: Solution<T>,
    two: Solution<U>
}
impl<'a, T, U> Day<T, U> where T: Eq + Display + Clone, U: Eq + Display + Clone {
    pub fn new(one: Solution<T>, two: Solution<U>) -> Self {
        Self { one, two }
    }

    pub fn solve(&self, input: &String) {
        self.one.solve(input);
        self.two.solve(input);
        
        self.one.time(input);
        self.two.time(input);
    }
}
