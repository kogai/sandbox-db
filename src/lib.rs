#![feature(test)]

extern crate test;

use std::io::Read;
use std::fs::{File};

pub fn read_file(path: String) -> String {
    let mut file = File::open(&path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_of_small(b: &mut Bencher) {
        b.iter(|| read_file("./fixture/256".to_owned()));
    }

    #[bench]
    fn bench_of_limit(b: &mut Bencher) {
        b.iter(|| read_file("./fixture/512".to_owned()));
    }

    #[bench]
    fn bench_of_large(b: &mut Bencher) {
        b.iter(|| read_file("./fixture/1024".to_owned()));
    }
}
