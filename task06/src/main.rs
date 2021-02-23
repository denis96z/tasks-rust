use crate::sort::bubble_sort;
use argparse::{ArgumentParser, Store};
use std::str::FromStr;

mod sort;

fn main() {
    let mut arr = String::new();
    let mut sort = "bubble".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut arr)
            .required()
            .add_argument("array", Store, "array of elements");
        ap.refer(&mut sort)
            .add_option(&["--sort"], Store, "sorting algorithm");
        ap.parse_args_or_exit();
    }

    let mut arr = arr
        .split(' ')
        .map(|x| i32::from_str(x).unwrap())
        .collect::<Vec<_>>();
    if arr.is_empty() {
        panic!("array is empty");
    }

    (match sort.as_str() {
        "bubble" => bubble_sort,
        _ => panic!("unknown sorting algorithm"),
    })(&mut arr);

    for x in arr {
        print!("{} ", x);
    }
    println!();
}
