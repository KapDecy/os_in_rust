#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![allow(static_mut_refs)]

mod booblick;
mod gdt;
mod interrupts;
mod vga_buffer;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    init();

    // x86_64::instructions::interrupts::int3();

    main();
    hlt_loop()
}

fn main() {
    println!("hello world");
    booblick::booblick();
    // for i in 1..500 {
    //     println!("aboba {}", i);
    //     for _ in 0..10000000 {
    //         continue;
    //     }
    // }
    // panic!("NOOOOOOO!")
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        // for _ in 0..10000000 {
        //     continue;
        // }
    }
}

fn init() {
    gdt::init();
    interrupts::init_idt();

    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
