use std::{time::Instant, fmt::Display};


pub struct Solution<T> where T: Display + Eq {
    function: fn(&String) -> T,
    expected: Option<T>,
    timing: Option<u128>,
}

impl<T> Solution<T> where T: Display + Eq + Clone {
    pub fn new(function: fn(&String) -> T, expected: Option<T>, timing: Option<u128>) -> Self {
        Self { function, expected, timing }
    }

    pub fn solve(&self, input: &String) {
        let output = (self.function)(input);
        if let Some(answer) = &self.expected {
            if *answer == output {
                println!("\x1b[32m{output}\x1b[0m")
            } else {
                println!("\x1b[31mThat is not the right answer.\n\x1b[33m{}\n\x1b[31m{}\x1b[0m", answer, output)
            }
        } else {
            println!("\x1b[32m{output}\x1b[0m")
        }
    }
    
    pub fn time(&self, input: &String) {
        if let Some(precision) = self.timing {
            println!("\x1b[90mSolved in \x1b[34m{}ms\x1b[90m average\x1b[0m",
                delta(|| { (self.function)(input); }, precision)
            )
        } else {
            println!("\x1b[90mSolve time not measured\x1b[0m")
        }
    }
    
}

fn delta<F: Fn()>(f: F, precision: u128) -> f32 {
    let now = Instant::now();
    for _i in 0..precision { f(); }
    let time = now.elapsed();
    ((time.as_micros()/precision) as f32)/1000.0
}
