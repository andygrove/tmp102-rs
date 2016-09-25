extern crate i2cdev;

use i2cdev::core::*;
use i2cdev::linux::*;

pub enum TempScale {
    Fahrenheit,
    Celcius
}

pub struct TMP102 {
    dev: Box<LinuxI2CDevice>,
    buf: Vec<u8>
}

impl TMP102 {

    pub fn new(filename: &'static str, address: u16) -> Result<Self, Box<LinuxI2CError>> {
        let dev = try!(LinuxI2CDevice::new(filename, address));
        Ok(TMP102 { dev: Box::new(dev), buf: vec![0_u8; 2] })
    }

    pub fn read(&mut self, scale: TempScale) -> Result<f32, Box<LinuxI2CError>> {
        try!(self.dev.smbus_write_byte(0x00));
        try!(self.dev.read(&mut self.buf));
        let mut temp: u16 = ((self.buf[0] as u16) << 8_u16 | (self.buf[1] as u16)) >> 4_u16;
        // move the negative bit
        if temp & 0x0800 > 0 {
            temp |= 0xF800;
        }
        let celcius = temp as f32 / 16_f32;
        match scale {
            TempScale::Celcius => Ok(celcius),
            TempScale::Fahrenheit => Ok(celcius*1.8+32.0),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::TMP102;
    use super::TempScale;

    #[test]
    fn it_works() {
        // assumes ADR0 is connected to GND
        let mut tmp = TMP102::new("/dev/i2c-1", 0x48).unwrap();
        for _ in 0..10 {
            println!("Temperature is {}", tmp.read(TempScale::Fahrenheit).unwrap());
        }
    }
}
