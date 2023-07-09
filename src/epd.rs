#![deny(warnings)]

use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    pixelcolor::BinaryColor::On as Black,
    pixelcolor::BinaryColor::Off as White,
    prelude::*,
    text::{Baseline, Text, TextStyleBuilder},
};
use embedded_hal::prelude::*;
use epd_waveshare::{
    epd2in13_v2::{Display2in13}
};
use linux_embedded_hal::{
    spidev::{self, SpidevOptions},
    sysfs_gpio::Direction,
    Delay, Pin, Spidev,
};


pub fn init() -> Display2in13 {

    let mut spi = Spidev::open("/dev/spidev0.0").expect("spidev directory");
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(4_000_000)
        .mode(spidev::SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&options).expect("spi configuration");

    // Configure Digital I/O Pin to be used as Chip Select for SPI
    let cs = Pin::new(26); //BCM7 CE0
    cs.export().expect("cs export");
    while !cs.is_exported() {}
    cs.set_direction(Direction::Out).expect("CS Direction");
    cs.set_value(1).expect("CS Value set to 1");

    let busy = Pin::new(24); // GPIO 24, board J-18
    busy.export().expect("busy export");
    while !busy.is_exported() {}
    busy.set_direction(Direction::In).expect("busy Direction");

    let dc = Pin::new(25); // GPIO 25, board J-22
    dc.export().expect("dc export");
    while !dc.is_exported() {}
    dc.set_direction(Direction::Out).expect("dc Direction");
    dc.set_value(1).expect("dc Value set to 1");

    let rst = Pin::new(17); // GPIO 17, board J-11
    rst.export().expect("rst export");
    while !rst.is_exported() {}
    rst.set_direction(Direction::Out).expect("rst Direction");
    rst.set_value(1).expect("rst Value set to 1");

    let mut delay = Delay {};
    delay.delay_ms(5000u16);
    
    // let epd2in13 =
    //     Epd2in13::new(&mut spi, cs, busy, dc, rst, &mut delay).expect("e-ink initialization error");

    let mut display = Display2in13::default();
    
    display.clear(White).ok();

    return display
}

pub fn draw_text(display: &mut Display2in13, text: &str, x: i32, y: i32) {
    let style = MonoTextStyleBuilder::new()
        .font(&embedded_graphics::mono_font::ascii::FONT_10X20)
        .text_color(White)
        .background_color(Black)
        .build();

    let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();

    let _ = Text::with_text_style(text, Point::new(x, y), style, text_style).draw(display);
}