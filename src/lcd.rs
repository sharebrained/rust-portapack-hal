
use {nop, spin_wait};
use io::{addr_0, addr_1, dir_read, dir_write, data_read, data_write_high, data_write_low, lcd_wr_assert, lcd_wr_deassert, lcd_rd_assert, lcd_rd_deassert};

/// Write a command to the LCD via the PortaPack CPLD.
/// Reset Cx/D to "data" after the transaction, assuming the next word will be data.
fn command_write(value: u32) {
	data_write_high(value);
	dir_write();			// Turn around data bus, MCU->CPLD
	addr_0();				// Indicate command
	nop();
	nop();
	nop();
	lcd_wr_assert();		// Latch high byte

	data_write_low(value);	// Drive low byte (pass-through)
	nop();
	nop();
	nop();
	lcd_wr_deassert();		// Complete write operation

	addr_1();				// Set up for data phase (most likely after a command)
}

/// Write a data word. This word will be between 8 and 18 bits in size, depending on if it's
/// a parameter or pixel value, and the particular format of the pixels.
/// The assumption is that data_write() comes after a command(), and the data bus will remain
/// in the "write" direction.
/// NOTE: Assumes and DIR=0 and ADDR=1 from command phase.
fn data_write(value: u32) {
	data_write_high(value);	// Drive high byte
	nop();
	lcd_wr_assert();		// Latch high byte

	data_write_low(value);	// Drive low byte (pass-through)
	nop();
	nop();
	nop();
	lcd_wr_deassert();		// Complete write operation
}

/// Reads a parameter value. Timing is different than reading from display memory.
/// NOTE: Assumes ADDR=1 from command phase.
fn data_read_parameter() -> u32 {
	dir_read();
	lcd_rd_assert();		// Start read operation
	nop();					// Wait for passthrough data(15:8) to settle -- ~16ns (3 cycles) typical
	nop();
	nop();
	nop();
	nop();
	nop();
	nop();
	let value_high = data_read();

	lcd_rd_deassert();		// Latch data[7:0]
	nop();					// Wait for latched data[7:0] to settle -- ~26ns (5 cycles) typical
	nop();
	nop();
	nop();
	nop();
	nop();
	nop();
	nop();
	nop();
	let value_low = data_read();

	(value_high << 8) | value_low
}

/// Reads a word from memory. Timing is different from reading a parameter.
/// Reads only 16 bits, due to hardware limitations (16-bit bus).
/// NOTE: Assumes ADDR=1 from command phase.
fn data_read_memory() -> u32 {
	dir_read();

	lcd_rd_assert();		// Start read operation
	
	// Wait for passthrough data(15:8) to settle -- ~16ns (3 cycles) typical
	// Wait for read control L duration (355ns)
	spin_wait(71);			// 355ns @ 204 MHz

	let value_high = data_read();

	lcd_rd_deassert();		// Latch data[7:0]
	
	// Wait for latched data[7:0] to settle -- ~26ns (5 cycles) typical
	// Wait for read control H duration (90ns)
	spin_wait(18);			// 90ns @ 204 MHz

	let value_low = data_read();
	(value_high << 8) | value_low
}
