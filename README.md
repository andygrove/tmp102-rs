# tmp102-rs

[![Version](https://img.shields.io/crates/v/tmp102.svg)](https://crates.io/crates/tmp102)
[![Docs](https://docs.rs/tmp102/badge.svg)](https://docs.rs/tmp102)

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

If you are not sure what address the device is using, just use i2cdetect.

```
$ i2cdetect  -y 1
     0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
00:          -- -- -- -- -- -- -- -- -- -- -- -- -- 
10: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- 
20: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- 
30: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- 
40: -- -- -- -- -- -- -- -- 48 -- -- -- -- -- -- -- 
50: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- 
60: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- 
70: -- -- -- -- -- -- -- --                         
```

# Wiring
  
| Raspberry Pi | TMP102 |
| ------------ | ------ |
| 3.3V         | VCC    |
| GND          | GND    |
| SDA          | SDA    |
| SCL          | SCL    |
| GND          | ADR0   |

  