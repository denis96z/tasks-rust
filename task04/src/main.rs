extern crate argparse;

use argparse::{ArgumentParser, Store, StoreTrue};
use std::fmt::Write;

fn main() {
    let mut n = 0u8;
    let mut verbose = false;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("S = 1^4 + 2^4 + 3^4 + ... + n^4");
        ap.refer(&mut n)
            .required()
            .add_argument("number", Store, "number");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "verbose output");
        ap.parse_args_or_exit();
    }
    if n == 0 {
        eprintln!("error: invalid number");
        std::process::exit(1);
    }

    if verbose {
        let v = make_num_vec(n);
        println!("numbers: {}", make_str_from_num_vec(&v));
    }

    let s = calc_sum(n);
    println!("sum: {}", s);
}

fn calc_sum(n: u8) -> f64 {
    let x = n as f64;

    let s1 = 6f64 * x.powf(5f64);
    let s2 = 15f64 * x.powf(4f64);
    let s3 = 10f64 * x.powf(3f64);

    (s1 + s2 + s3 - x) / 30f64
}

fn make_num_vec(n: u8) -> Vec<f64> {
    let n = n as usize;
    debug_assert!(n > 0);

    let mut v = Vec::with_capacity(n);
    for i in 1..=n {
        v.push((i as f64).powf(4f64));
    }

    v
}

fn make_str_from_num_vec(v: &[f64]) -> String {
    debug_assert!(!v.is_empty());

    let mut s = String::new();
    {
        for item in v.iter().take(v.len() - 1) {
            write!(&mut s, "{}, ", item).unwrap();
        }
        write!(&mut s, "{}", v[v.len() - 1]).unwrap();
    }

    s
}
