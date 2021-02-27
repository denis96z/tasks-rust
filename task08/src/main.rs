mod ralloc;
use ralloc::RandArr;

fn main() {
    unsafe {
        let x = RandArr::<i32>::with_capacity(100);
        x.print();
    }
}
