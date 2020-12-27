use std::str::FromStr;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() != 1 {
        eprintln!("invalid arguments");
        std::process::exit(1);
    }

    let age = match u8::from_str(&args[0]) {
        Ok(v) => v,
        _ => {
            eprintln!("argument is not a valid number");
            std::process::exit(1);
        }
    };
    if age == 0 || age > 99 {
        eprintln!("age is out of supported range");
        std::process::exit(1);
    }

    if age > 20 {
        println!("access granted");
    } else {
        let s = age_to_str(age);
        println!("access denied [{}]", s);
    }
}

fn age_to_str(age: u8) -> String {
    static SUFFIX: &str = " years old";
    static NUMBERS: &[&str] = &[
        "",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
        "twenty",
    ];

    let mut s = String::with_capacity("seventeen".len() + SUFFIX.len());
    s.push_str(NUMBERS[age as usize]);
    s.push_str(SUFFIX);

    s
}
