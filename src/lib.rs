extern crate i2cdev;
use i2cdev::core::*;
use i2cdev::linux::*;

pub struct TMP102 {
    dev: Box<LinuxI2CDevice>,
    buf: Vec<u8>
}

impl TMP102 {

  pub fn new(filename: &'static str, address: u16) -> Result<Self, Box<LinuxI2CError>> {
    let dev = try!(LinuxI2CDevice::new(filename, address));
    Ok(TMP102 { dev: Box::new(dev), buf: vec![0_u8; 2] })
  }

  pub fn read(&mut self) -> Result<f32, Box<LinuxI2CError>> {
    try!(self.dev.smbus_write_byte(0x00));
    try!(self.dev.read(&mut self.buf));
    println!("Temperature: {:?}", &self.buf);
    let mut temp : u16 = ((self.buf[0] as u16) << 8_u16 | (self.buf[1] as u16)) >> 4_u16;
    //if temp & (1_u16<11_u16) == 1 {
    //  temp |= 0xF800;
   // }
    let foo = temp as f32 / 16_f32;
    println!("Temperature: {}, {}", temp, foo);
    Ok(foo)
  }

  

}


#[cfg(test)]
mod tests {

    use super::TMP102;

    #[test]
    fn it_works() {
        // assumes ADR0 is connected to GND
        let mut tmp = TMP102::new("/dev/i2c-1", 0x48).unwrap();
        for _ in 0 .. 10 {
            tmp.read().unwrap();
        }
    
    }
}
