# hello-rpi-bme680
Simple Rust application for home atmosphere monitoring system based on Rpi Zero, Bme680 sensor and WaveShare 2.13" V2 e-paper display.

## Local run on rpi 
The main idea of getting data from sensor is taken from examples in [the repo of bme680 crate](https://github.com/marcelbuesing/bme680).

Before running the code you should determine i2c address of sensor connected to rpi with command:

`sudo i2cdetect -y 1`

And add last two-digit number from result print as environment variable *BME_I2C_ADDRESS* during run program in console below.
If you can build the code directly on Raspberry Pi, compile and build it with cargo and run in console:

` sudo RUST_LOG=info BME_I2C_ADDRESS=76 target/debug/hello-rpi-bme680 `

To avoid a using ` sudo` we have to allow access for our user to ` /dev/i2c-1` with adding into ` i2c` group:

` sudo usermod -a -G i2c ${USERNAME}`

However in my case I decided build and compile the code on my own linux workstation and after deploy it on rpi and run.
To 'speak in the same language' with raspberry pi zero we add the build target:

` rustup target add arm-unknown-linux-gnueabihf`

Download and install [GNU Toolchain](https://developer.arm.com/downloads/-/gnu-a) linker for rpi zero on a workstation - The __gcc-arm-8.3-2019.03-x86_64-arm-linux-gnueabihf.tar.xz__ works fine for me. Don't miss link it into __$PATH__ after extracting.

After adding a reference to linker into [.cargo/config.toml](.cargo/config.toml) file, we can run cross build:

` cargo build --target arm-unknown-linux-gnueabihf`

And now we have received the file ` target/arm-unknown-linux-gnueabihf/debug/hello-rpi-bme680` ready for deployment.

## Docker run
[WIP]

## Display sensor data
[WIP]