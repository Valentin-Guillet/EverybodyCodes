use crate::args::RunArgs;

use std::{
    fs::read_to_string,
    ops::{Add, Div, Mul},
};

#[derive(Clone, Copy)]
struct Complex {
    a: i64,
    b: i64,
}

impl Complex {
    fn new(a: i64, b: i64) -> Self {
        Complex { a, b }
    }

    fn from(input: String) -> Self {
        let range = input.find('=').unwrap() + 2..input.len() - 1;
        let (a, b) = input[range].split_once(',').unwrap();
        let a = a.parse().unwrap();
        let b = b.parse().unwrap();
        Complex { a, b }
    }
}

impl ToString for Complex {
    fn to_string(&self) -> String {
        format!("[{},{}]", self.a, self.b)
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self {
        Self {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, rhs: Self) -> Self {
        Self {
            a: self.a * rhs.a - self.b * rhs.b,
            b: self.a * rhs.b + self.b * rhs.a,
        }
    }
}

impl Div for Complex {
    type Output = Complex;

    fn div(self, rhs: Self) -> Self {
        Self {
            a: self.a / rhs.a,
            b: self.b / rhs.b,
        }
    }
}

pub fn run(args: &RunArgs) -> String {
    let data = read_to_string(&args.input_file).expect("Error opening input file");
    let input = Complex::from(data);

    match args.part {
        1 => understand_complex(input).to_string(),
        2 => count_engraved_points(input, 10).to_string(),
        3 => count_engraved_points(input, 1).to_string(),
        _ => unreachable!(),
    }
}

fn understand_complex(n: Complex) -> Complex {
    let mut res = Complex::new(0, 0);
    for _ in 0..3 {
        res = res * res;
        res = res / Complex::new(10, 10);
        res = res + n;
    }
    res
}

fn count_engraved_points(point: Complex, step: usize) -> u32 {
    let mut count = 0;
    for dx in (0..=1000).step_by(step) {
        for dy in (0..=1000).step_by(step) {
            let mut res = Complex::new(0, 0);
            let mut should_engrave = true;
            let coords = point + Complex::new(dx, dy);
            for _ in 0..100 {
                res = res * res;
                res = res / Complex::new(100_000, 100_000);
                res = res + coords;

                if !(-1_000_000..1_000_000).contains(&res.a) || !(-1_000_000..1_000_000).contains(&res.b) {
                    should_engrave = false;
                    break;
                }
            }
            if should_engrave {
                count += 1;
            }
        }
    }
    count
}
