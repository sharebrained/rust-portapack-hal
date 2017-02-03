#![feature(asm)]

#![no_std]

extern crate lpc43xx;
extern crate hackrf_hal;
extern crate lcd_ili9341;

pub mod io;
pub mod lcd;

fn nop() {
	match () {
		#[cfg(target_arch="arm")]
		() => unsafe { asm!("nop" :::: "volatile"); },
		#[cfg(not(target_arch="arm"))]
		() => {}
	}
}

fn spin_wait(count: u32) {
	for _ in 0..count {
		nop();
	}
}
