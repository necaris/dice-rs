#![feature(test)]

extern crate test;


#[cfg(test)]
mod tests {
    use super::die::*;
    use test::{Bencher, black_box};

    #[bench]
    fn bench_roll_die(b: &mut Bencher) {
        let sides = black_box(6);
        let mut d = super::die::Die::new(sides);
        b.iter(|| d.roll());
    }
}
