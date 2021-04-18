fn main() {
    let mut v = Vec::new();
    for i in 0..3 {
        v.push(std::thread::spawn(move || {
            do_work(i);
        }));
    }
    for t in v {
        let _ = t.join();
    }
}

fn do_work(i: i32) {
    for j in 0..i {
        println!("{} {}", i, j);
    }
}
