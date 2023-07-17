#![no_std]
#![no_main]

use cortex_m_rt::entry;
use heapless::consts::*;
use heapless::Vec;

#[entry]
fn main() -> ! {
    // Static vector of size 8
    let mut xs: Vec<_, U8> = Vec::new();

    xs.push(42).unwrap();
    assert_eq!(xs.pop(), Some(42));
    loop {}
}
