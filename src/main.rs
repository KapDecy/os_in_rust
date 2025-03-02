#![no_std]
#![no_main]
#![allow(static_mut_refs)]

mod vga_buffer;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    main();
    loop {}
}

fn main() {
    for i in 1..5 {
        println!("aboba {}", i);
        for _ in 0..10000000 {
            continue;
        }
    }
    panic!("NOOOOOOO!")
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {
        println!("{}", info);
        for _ in 0..10000000 {
            continue;
        }
    }
}
