extern "C" {
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

use std::slice;

static mut COUNTER: i32 = 3;
fn add_to_conut(inc: i32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe trait Foo {}

unsafe impl Foo for i32 {}

fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    let address = 0x012345usize;
    let r = address as *const i32;
    a;
    add_to_conut(3);
    unsafe {
        dangerous();
        println!("r1 is: {}", *r1);
        println!("r1 is: {}", *r2);
        println!("{}", COUNTER);
        println!("{}", abs(-3));
    }
}

unsafe fn dangerous() {}
fn a() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [1, 2, 3]);
}
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut((ptr.add(mid)), len - mid),
        )
    }
}
