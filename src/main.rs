#![no_std]
#![no_main]

use panic_halt as _; // (1)
use wio_terminal as wio; // (1)
use wio::hal::clock::GenericClockController; // (1)
use wio::pac::Peripherals; // (1)
use wio::prelude::*; // (1)
use wio::{entry, Pins, Sets}; // (1)

use wio_lib::uart; // (2)

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap(); // (3)
    // クロックを初期化する
    let mut clocks = GenericClockController::with_external_32kosc( // (4)
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    // UARTドライバオブジェクトを初期化する
    let mut sets: Sets = Pins::new(peripherals.PORT).split(); // (5)
    let mut serial = sets.uart.init( // (6)
        &mut clocks,
        115200.hz(),
        peripherals.SERCOM2,
        &mut peripherals.MCLK,
        &mut sets.port,
    );

    // 「hello world」と出力する
    for c in b"hello world\r\n".iter() { // (7)
        uart::putc(&mut serial, c); // (8)
    }
    // 「this is UART example!」と出力する
    uart::puts(&mut serial, "this is UART example!\r\n"); // (9)

    loop {
        uart::read_write(&mut serial); // (10)
    }
}
