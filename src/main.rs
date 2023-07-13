use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    prelude::*,
    text::{Baseline, Text, TextStyleBuilder},
};
use embedded_hal::prelude::*;
use epd_waveshare::{
    color::*,
    epd2in13_v2::{Display2in13, Epd2in13},
    graphics::DisplayRotation,
    prelude::*,
};
use linux_embedded_hal::{
    spidev::{self, SpidevOptions},
    sysfs_gpio::Direction,
    Delay, Pin, Spidev,
};
use chrono;

fn main() -> Result<(), std::io::Error> {
    // Configure SPI
    // Settings are taken from
    let mut spi = Spidev::open("/dev/spidev0.0").expect("spidev directory");
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(4_000_000)
        .mode(spidev::SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&options).expect("spi configuration");

    // Configure Digital I/O Pin to be used as Chip Select for SPI
    let cs = Pin::new(26);
    cs.export().expect("cs export");
    while !cs.is_exported() {}
    cs.set_direction(Direction::Out).expect("CS Direction");
    cs.set_value(1).expect("CS Value set to 1");

    let busy = Pin::new(24);
    busy.export().expect("busy export");
    while !busy.is_exported() {}
    busy.set_direction(Direction::In).expect("busy Direction");

    let dc = Pin::new(25);
    dc.export().expect("dc export");
    while !dc.is_exported() {}
    dc.set_direction(Direction::Out).expect("dc Direction");
    dc.set_value(1).expect("dc Value set to 1");

    let rst = Pin::new(17);
    rst.export().expect("rst export");
    while !rst.is_exported() {}
    rst.set_direction(Direction::Out).expect("rst Direction");
    rst.set_value(1).expect("rst Value set to 1");

    let mut delay = Delay {};

    let mut epd2in13 =
        Epd2in13::new(&mut spi, cs, busy, dc, rst, &mut delay).expect("e-ink initialize error");

    let mut display = Display2in13::default();

    epd2in13
        .wake_up(&mut spi, &mut delay)
        .expect("wake-up error");
    epd2in13
        .clear_frame(&mut spi, &mut delay)
        .expect("clear frame error");

    display.clear_buffer(Color::Black);
    display.set_rotation(DisplayRotation::Rotate270);
    
    let dt = chrono::offset::Local::now();
    let test_text = format!("---------------\n\
    Current dt: {:?} \n\
    Temperature 25C\n\
    Pressure 1000hPa\n\
    Humidity 40%\n\
    Gas Resistance 100Ohm\n\
    ---------------", dt);
    
    draw_text(&mut display, &test_text, 5, 5);
    epd2in13
        .update_and_display_frame(&mut spi, display.buffer(), &mut delay)
        .unwrap();
    delay.delay_ms(5000u16);

    epd2in13.sleep(&mut spi, &mut delay)
}

fn draw_text(display: &mut Display2in13, text: &str, x: i32, y: i32) {
    let style = MonoTextStyleBuilder::new()
        .font(&embedded_graphics::mono_font::ascii::FONT_6X10)
        .text_color(White)
        .background_color(Black)
        .build();

    let text_style = TextStyleBuilder::new().baseline(Baseline::Top).build();
    let _ = Text::with_text_style(text, Point::new(x, y), style, text_style).draw(display);
}