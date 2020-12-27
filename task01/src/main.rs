fn main() {
    let mut a = String::new();
    let mut b = String::new();

    println!("A:");
    std::io::stdin().read_line(&mut a).unwrap();

    println!("B:");
    std::io::stdin().read_line(&mut b).unwrap();

    let a: i32 = a.trim().parse().unwrap();
    let b: i32 = b.trim().parse().unwrap();

    let c = a * a;
    let d = b * b * b;

    println!("A={0}, B={1}, C={2}, D={3}", a, b, c, d);
}
