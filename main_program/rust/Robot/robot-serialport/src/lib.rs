use std::time::Duration;

use serialport::SerialPort;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

///Simple Robot Management Protocol
/// SRMP
pub struct RasPico {
    port: Box<dyn SerialPort>,
}

impl RasPico {
    pub fn new(port: &str, rate: u32) -> Self {
        let mut port = match serialport::new(port, rate)
            .stop_bits(serialport::StopBits::One)
            .data_bits(serialport::DataBits::Eight)
            .timeout(Duration::from_millis(10))
            .open()
        {
            Ok(p) => p,
            Err(_) => panic!(),
        };

        Self { port }
    }

    pub fn write(&mut self, order: u32) {
        
        self.port.write(&order.to_be_bytes()).expect("Write failed!");
    }

    pub fn check(&mut self) {
        self.port.write(&0x10FFFFFF_u32.to_be_bytes()).expect("Write failed!");

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
