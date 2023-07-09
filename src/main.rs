#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(noki_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(dead_code)]


use core::panic::PanicInfo;
use noki_os::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    noki_os::test_panic_handler(info);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}
