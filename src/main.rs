#![deny(warnings)]

use embedded_graphics::{
    pixelcolor::BinaryColor::Off as White,
    prelude::*
};
use epd_waveshare::{
    graphics::DisplayRotation,
    prelude::*,
};

mod epd;

fn main() {
    
    let mut display = epd::init();

    display.set_rotation(DisplayRotation::Rotate180);
    epd::draw_text(&mut display, "Rotate 180!", 5, 5);

    display.clear(White).ok(); 

    let test = "---------------\n\
        Temperature 25°C\n\
        Pressure 1000hPa\n\
        Humidity 40%\n\
        Gas Resistance 100Ω\n\
        ---------------";
    epd::draw_text(&mut display, test, 5, 5);

}