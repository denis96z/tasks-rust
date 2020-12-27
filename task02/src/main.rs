use std::str::FromStr;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("invalid arguments");
        std::process::exit(1);
    }

    let x = i32::from_str(&args[1]).expect("argument [1] is not a number");
    let s = xsum(x);

    println!("SUM: {}", s);
}

fn xsum(x: i32) -> i32 {
    let mut n = 10;
    let mut s = x % n;
    while x / n != 0 {
        s += (x / n) % 10;
        n *= 10;
    }
    s
}

#[cfg(test)]
mod tests {
    use super::xsum;

    #[test]
    fn test_xsum() {
        assert_eq!(xsum(0), 0);
        assert_eq!(xsum(1), 1);
        assert_eq!(xsum(103), 4);
    }
}
