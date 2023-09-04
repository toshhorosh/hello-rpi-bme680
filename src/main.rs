use bme680::*;
use core::result;
use core::time::Duration;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::i2c;
use linux_embedded_hal as hal;
use log::info;
use std::env;
use chrono;
use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    prelude::*,
    text::{Baseline, Text, TextStyleBuilder},
};
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

fn main(
) -> result::Result<(), Error<<hal::I2cdev as i2c::Read>::Error, <hal::I2cdev as i2c::Write>::Error>>
{
    // sensor initialization
    env_logger::init();
    let _primary = String::from("76");
    let _secondary = String::from("77");

    let i2c_address = match env::var("BME_I2C_ADDRESS") {
        x if x == Ok(_primary) => I2CAddress::Primary,
        x if x == Ok(_secondary) => I2CAddress::Secondary,
        Ok(_) => panic!("Unknown i2c address was received!"),
        Err(e) => panic!("Set env value 'BME_I2C_ADDRESS' before run the program! Error: {}", e)
    };

    let i2c = hal::I2cdev::new("/dev/i2c-1").unwrap();
    let mut delayer = Delay {};

    let mut dev = Bme680::init(i2c, &mut delayer, i2c_address)?;
    let mut delay = Delay {};

    let settings = SettingsBuilder::new()
        .with_humidity_oversampling(OversamplingSetting::OS2x)
        .with_pressure_oversampling(OversamplingSetting::OS4x)
        .with_temperature_oversampling(OversamplingSetting::OS8x)
        .with_temperature_filter(IIRFilterSize::Size3)
        .with_gas_measurement(Duration::from_millis(1500), 320, 25)
        .with_temperature_offset(-2.2)
        .with_run_gas(true)
        .build();

    let profile_dur = dev.get_profile_dur(&settings.0)?;
    info!("Profile duration {:?}", profile_dur);
    info!("Setting sensor settings");
    dev.set_sensor_settings(&mut delayer, settings)?;
    info!("Setting forced power modes");
    dev.set_sensor_mode(&mut delayer, PowerMode::ForcedMode)?;

    let sensor_settings = dev.get_sensor_settings(settings.1);
    info!("Sensor settings: {:?}", sensor_settings);

    // display initialization
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

    loop {
        let dt = chrono::offset::Local::now();

        dev.set_sensor_mode(&mut delayer, PowerMode::ForcedMode)?;
        let (data, _state) = dev.get_sensor_data(&mut delayer)?;
        info!("---------------");
        info!("Temperature {}°C", data.temperature_celsius());
        info!("Pressure {}hPa", data.pressure_hpa());
        info!("Humidity {}%", data.humidity_percent());
        info!("Gas Resistence {}Ω", data.gas_resistance_ohm());

        let text_to_draw = format!(
            "---------------\n\
        Current dt: {:?} \n\
        Temperature {:?}C\n\
        Pressure {:?}hPa\n\
        Humidity {:?}%\n\
        Gas Resistance {:?}Ohm\n\
        ---------------",
            dt,
            data.temperature_celsius(),
            data.pressure_hpa(),
            data.humidity_percent(),
            data.gas_resistance_ohm()
        );

        draw_text(&mut display, &text_to_draw, 5, 5);
        epd2in13
            .update_and_display_frame(&mut spi, display.buffer(), &mut delay)
            .unwrap();

        delay.delay_ms(60000u32);
    }
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
