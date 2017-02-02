#![feature(asm)]

#![no_std]

extern crate lpc43xx;
extern crate hackrf_hal;

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
