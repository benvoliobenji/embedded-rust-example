#![no_std]
#![no_main]

// pick a panicking behavior
// def profile: easier to debug panics; can put a breakpoint on `rust_begin_unwind`
// #[cfg(debug_assertions)]
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

// release profile: minimize the binary size of the application
// #[cfg(not(debug_assertions))]
// use panic_abort as _;
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use core::fmt::Write;
use core::ptr;

use cortex_m::asm;
use cortex_m_rt::{entry, exception, ExceptionFrame};
use cortex_m_semihosting::hio;

#[entry]
fn main() -> ! {
    // read a nonexistent memory location
    unsafe {
        ptr::read_volatile(0x3FFF_FFFE as *const u32);
    }

    loop {}
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    if let Ok(mut hstdout) = hio::hstdout() {
        writeln!(hstdout, "{:#?}", ef).ok();
    }

    loop {}
}
