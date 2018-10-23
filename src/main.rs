#![no_std] //don't link the Rust standard library because we can't call a library that doesn't exist on the system
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points because the system won't have a default
//allow a main if running a test
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]//if running a test allow for unused macros and code

use core::panic::PanicInfo;

#[macro_use]
extern crate TuxenOS;

/// This function is called on panic.
#[cfg(not(test))] // only compiled when the test flag is not set
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


// don't mangle the name of this function
// this function is the entry point, since the linker looks for a function
// named `_start` by default on linux [ONLY COMPILE ON LINUX SYSTEMS]
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    /*use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(vga_buffer:WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();*/

    //println!("Hello World{}", "!");
    serial_println!("Hello Host{}", "!");
    println!("___ _  _ _  _ ____ _  _    ____ ____ ");
    println!(" |  |  |  \\/  |___ |\\ |    |  | [__  ");
    println!(" |  |__| _/\\_ |___ | \\|    |__| ___] ");
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();
    println!();

    TuxenOS::interrupts::init_idt();
    
    // call a breakpoint exception
    x86_64::instructions::int3();
    
    println!("It didn't crash!!!!!!! :)");
    
    //unsafe { exit_qemu(); }
    
    
    loop {

    }
}
