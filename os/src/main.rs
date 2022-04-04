#![no_std]
#![no_main]
#![feature(panic_info_message)]
mod lang_items;
mod sbi;
#[macro_use]
mod console;

use core::arch::global_asm;
// use sbi::{ shutdown };
global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
	clear_bss();
	println!("hello world");
	// shutdown()
	panic!("Shutting Down")
}

fn clear_bss() {
	extern "C" {
		fn sbss();
		fn ebss();
	}
	(sbss as usize..ebss as usize).for_each(|a| {
		unsafe { (a as *mut u8).write_volatile(0) }
	});
}
