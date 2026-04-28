#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(blog_os::test_runner)]

use blog_os::println;
use core::panic::PanicInfo;

// We don’t need any cfg(test) attributes because
// integration test executables are never built in non-test mode.

#[unsafe(no_mangle)]
pub extern "C" fn _start() {
    test_main();

    loop {}
}

fn test_runner(tests: &[&dyn Fn()]) {
    unimplemented!();
}

#[test_case]
fn test_println() {
    println!("test_println output");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}
