
use io::{data_write_high, dir_write, addr_0, lcd_wr_assert, data_write_low, lcd_wr_deassert, addr_1};
use nop;

pub fn command(value: u8) {
	data_write_high(0);
	dir_write();					/* Turn around data bus, MCU->CPLD */
	addr_0();						/* Indicate command */
	nop();
	nop();
	nop();
	lcd_wr_assert();				/* Latch high byte */

	data_write_low(value as u32);	/* Drive low byte (pass-through) */
	nop();
	nop();
	nop();
	lcd_wr_deassert();				/* Complete write operation */

	addr_1();						/* Set up for data phase (most likely after a command) */
}

pub fn data_write(value: u16) {
	// NOTE: Assumes and DIR=0 and ADDR=1 from command phase.
	data_write_high(value as u32);	/* Drive high byte */
	nop();
	lcd_wr_assert();				/* Latch high byte */

	data_write_low(value as u32);	/* Drive low byte (pass-through) */
	nop();
	nop();
	nop();
	lcd_wr_deassert();				/* Complete write operation */
}
