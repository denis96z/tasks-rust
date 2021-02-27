#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
mod bindings;

use bindings::{error_t_ERR_NONE, ralloc};

pub struct RandArr<T: Sized> {
    cap: usize,
    items: *mut T,
}

impl<T> RandArr<T> {
    pub unsafe fn with_capacity(n: usize) -> RandArr<T> {
        let mut v = RandArr {
            cap: n,
            items: std::ptr::null_mut::<T>(),
        };

        let mut err = error_t_ERR_NONE;
        let p = ralloc(n * std::mem::size_of::<T>(), &mut err);
        if err != error_t_ERR_NONE {
            panic!(format!("ralloc() fail: {}", err))
        };

        v.items = p.cast();
        v
    }
}

impl<T: std::fmt::Display> RandArr<T> {
    pub unsafe fn print(&self) {
        for x in std::slice::from_raw_parts(self.items, self.cap) {
            print!("{} ", *x);
        }
        println!();
    }
}
