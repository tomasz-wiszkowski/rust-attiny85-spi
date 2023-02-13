## Digispark ATtiny85 Minimal SPI Master/Slave interface.

Built as a scaffolding for future attiny85 rust prototypes of mine.

### Requirements

To use this, you need to 
* download, or better - *build* - `micronucleus` from [Github repo](https://github.com/micronucleus/micronucleus/tree/master). 
* install Rust nightly 
* install AVR GCC

```sh
rustup install nightly
rustup default nightly
rustup component add rust-src
sudo apt install gcc-avr avr-libc
```

Lastly, your `udev` rules must specify the appropriate access rights to allow
you to program the ATtiny controllers: install rules specified [here](https://github.com/micronucleus/micronucleus/blob/master/commandline/49-micronucleus.rules).

### ATTiny85 notes

Due to limitations of the programmable memory on this microcontroller,
* be sure to build and run `---release` version (`cargo build --release` or `cargo run --release`) - otherwise your code just won't fit.
* the code interfaces registers directly to preserve programmable memory (believe it or not, it's also faster this way)

The compiled project consumes about 540 bytes.

### Other notes
* This has only been used on Linux. Not sure how this will work on other platforms.
* When using Arduino IDE, replace the Digispark-shipped `micronucleus` with the one you built. 
* `micronucleus` repo has bootrom upgrades that can be used to update your `attiny85`
* Updated `attiny85` won't work with obsolete version shipped by Digispark.
