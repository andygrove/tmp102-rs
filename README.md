# tmp102-rs

Rust library for the Texas Instruments TMP102 digital temperature sensor.

This library will only compile on Linux and has only been tested on the Raspberry Pi and with this TMP102 breakout: https://www.sparkfun.com/products/11931

```rust
extern crate tmp102;

use tmp102::{TMP102, TempScale};

fn main() {
    // address 0x48 assumes ADR0 is connected to GND
    let mut tmp = TMP102::new("/dev/i2c-1", 0x48).unwrap();
    println!("Temperature is {}", tmp.read(TempScale::Fahrenheit).unwrap());
}

```

 
