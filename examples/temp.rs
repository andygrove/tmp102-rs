extern crate tmp102;

use tmp102::{TMP102, TempScale};

fn main() {
    // address 0x48 assumes ADR0 is connected to GND
    let mut tmp = TMP102::new("/dev/i2c-1", 0x48).unwrap();
    println!("Temperature is {}", tmp.read(TempScale::Fahrenheit).unwrap());
}
