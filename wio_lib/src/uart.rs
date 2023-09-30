use atsamd_hal::gpio::{Pb26, Pb27, PfC}; // (1)
use atsamd_hal::sercom::{Sercom2Pad0, Sercom2Pad1, UART2}; // (1)
use embedded_hal::serial::{Read, Write}; // (1)

pub fn putc(uart: &mut UART2<Sercom2Pad1<Pb27<PfC>>, Sercom2Pad0<Pb26<PfC>>, (), ()>, c: &u8) { // (2)
    nb::block!(uart.write(*c)).unwrap(); // (3)
}

pub fn puts(uart: &mut UART2<Sercom2Pad1<Pb27<PfC>>, Sercom2Pad0<Pb26<PfC>>, (), ()>, s: &str) { // (4)

    // 文字列をバイトスライスに変換し、イテレータを作成
    let byte_iter = s.as_bytes().iter(); // (5)

    // イテレータを使ってバイト（u8）を1つずつ取り出す
    for &byte in byte_iter { // (6)
        nb::block!(uart.write(byte)).unwrap(); // (7)
    }
}

pub fn read_write(uart: &mut UART2<Sercom2Pad1<Pb27<PfC>>, Sercom2Pad0<Pb26<PfC>>, (), ()>) { // (8)
    if let Ok(c) = nb::block!(uart.read()) { // (9)
        // 受信したデータをそのまま送信する
        nb::block!(uart.write(c)).unwrap(); // (10)
    }
}
