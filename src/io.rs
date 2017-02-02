
use hackrf_hal::gpio;
use lpc43xx::peripheral::gpio_port;

pub const GPIO_IO_STBX: gpio::GPIO = gpio::P2_0;
pub const GPIO_ADDR:    gpio::GPIO = gpio::P2_1;
pub const GPIO_LCD_TE:  gpio::GPIO = gpio::P2_3;
pub const GPIO_UNUSED:  gpio::GPIO = gpio::P2_8;
pub const GPIO_LCD_RD:  gpio::GPIO = gpio::P2_4;
pub const GPIO_LCD_WR:  gpio::GPIO = gpio::P2_9;
pub const GPIO_DIR:     gpio::GPIO = gpio::P2_13;

pub const GPIO_CPLD_TMS: gpio::GPIO = gpio::P1_8;
pub const GPIO_CPLD_TDO: gpio::GPIO = gpio::P1_5;
pub const GPIO_CPLD_TCK: gpio::GPIO = gpio::P6_1;
pub const GPIO_CPLD_TDI: gpio::GPIO = gpio::P6_2;

const GPIO_LCD_DATA_PORT:      usize = 3;
const GPIO_LCD_DATA_PIN_BASE:  usize = 8;
const GPIO_LCD_DATA_PIN_COUNT: usize = 8;

pub fn lcd_rd_assert() {
	GPIO_LCD_RD.set();
}

pub fn lcd_rd_deassert() {
	GPIO_LCD_RD.clear();
}

pub fn lcd_wr_assert() {
	GPIO_LCD_WR.set();
}

pub fn lcd_wr_deassert() {
	GPIO_LCD_WR.clear();
}

pub fn io_stb_assert() {
	GPIO_IO_STBX.clear();
}

pub fn io_stb_deassert() {
	GPIO_IO_STBX.set();
}

// fn addr(value: u32) {
// 	GPIO_ADDR.write(value);
// }

pub fn addr_1() {
	GPIO_ADDR.set();
}

pub fn addr_0() {
	GPIO_ADDR.clear();
}

fn dir(write: bool) {
	assert!(GPIO_LCD_DATA_PIN_BASE == 8);
	assert!(GPIO_LCD_DATA_PIN_COUNT == 8);
	gpio_port().dir[GPIO_LCD_DATA_PORT].write(|x| x.dirp8(write).dirp9(write).dirp10(write).dirp11(write).dirp12(write).dirp13(write).dirp14(write).dirp15(write));
	/* TODO: Manipulating DIR[3] makes me queasy. The RFFC5072 DATA pin
	 * is also on port 3, and switches direction periodically...
	 * Time to resort to bit-banding to enforce atomicity? But then, how
	 * to change direction on eight bits efficiently? Or do I care, since
	 * the PortaPack data bus shouldn't change direction too frequently?
	 */
}

pub fn dir_write() {
	GPIO_DIR.clear();
	dir(true);
}

pub fn dir_read() {
	dir(false);
	GPIO_DIR.set();
}

pub fn data_write_low(value: u32) {
	assert!(GPIO_LCD_DATA_PIN_BASE == 8);
	gpio_port().mpin[GPIO_LCD_DATA_PORT].write_word(value << GPIO_LCD_DATA_PIN_BASE);
}

pub fn data_write_high(value: u32) {
	// Lazy code assumes that writing MPIN with 16 bits will put the high word in register[15:8]
	assert!(GPIO_LCD_DATA_PIN_BASE == 8);
	gpio_port().mpin[GPIO_LCD_DATA_PORT].write_word(value);
}
