use bme680::*;
use core::result;
use core::time::Duration;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::blocking::i2c;
use linux_embedded_hal as hal;
use linux_embedded_hal::Delay;
use log::info;
use std::env;

fn main(
) -> result::Result<(), Error<<hal::I2cdev as i2c::Read>::Error, <hal::I2cdev as i2c::Write>::Error>>
{
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

    loop {
        delay.delay_ms(60000u32);
        dev.set_sensor_mode(&mut delayer, PowerMode::ForcedMode)?;
        let (data, _state) = dev.get_sensor_data(&mut delayer)?;
        info!("---------------");
        info!("Temperature {}°C", data.temperature_celsius());
        info!("Pressure {}hPa", data.pressure_hpa());
        info!("Humidity {}%", data.humidity_percent());
        info!("Gas Resistence {}Ω", data.gas_resistance_ohm());
    }
}