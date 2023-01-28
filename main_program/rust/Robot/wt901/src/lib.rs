use std::{collections::VecDeque, time::{Duration, Instant}};

#[derive(Debug, Clone)]
pub struct WT901 {
    pub acc: Option<(f32, f32, f32)>,
    pub gyro: Option<(f32, f32, f32)>,
    pub ang: Option<(f32, f32, f32)>,
    pub mag: Option<(i16, i16, i16)>,
    pub angvec: Option<(f32, f32, f32)>,
    start_time: Instant,
    pub aziment:(f64,f64,f64),
    pub anglar:(f32, f32, f32),
    pub old_anglar:(f32, f32, f32)

}


impl WT901 {
    #[inline]
    pub fn new() -> Self {
        Self {
            acc: None,
            gyro: None,
            ang: None,
            mag: None,
            angvec: None,
            start_time:Instant::now(),
            aziment:(0.0,0.0,0.0),
            anglar:(0.0,0.0,0.0),
            old_anglar:(0.0,0.0,0.0),
        }
    }

    #[inline]
    pub fn cope_serial_data(&mut self, serial_buf: Vec<u8>) {
        let mut data = VecDeque::from(serial_buf);

        loop {
            if data.len() >= 11 {
                if data[0] != 0x55 {
                    //TODO: 0x55を先頭にずらす。
                    data.pop_front();

                    continue;
                }

                match data[1] {
                    0x50 => {}
                    0x51 => {
                        self.acc = Some((
                            i16::from_le_bytes([data[2], data[3]]) as f32 / 32768.0 * 16.0,
                            i16::from_le_bytes([data[4], data[5]]) as f32 / 32768.0 * 16.0,
                            i16::from_le_bytes([data[6], data[7]]) as f32 / 32768.0 * 16.0,
                        ));
                    }
                    0x52 => {
                        self.gyro = Some((
                            i16::from_le_bytes([data[2], data[3]]) as f32 / 32768.0 * 2000.0,
                            i16::from_le_bytes([data[4], data[5]]) as f32 / 32768.0 * 2000.0,
                            i16::from_le_bytes([data[6], data[7]]) as f32 / 32768.0 * 2000.0,
                        ));
                    }
                    0x54 => {
                        //println!("mag_X: {:?}, mag_Y: {:?}, mag_Z: {:?}",);

                        self.mag = Some((
                            i16::from_le_bytes([data[2], data[3]]),
                            i16::from_le_bytes([data[4], data[5]]),
                            i16::from_le_bytes([data[6], data[7]]),
                        ));
                    }

                    0x53 => {
                        self.ang = Some((
                            i16::from_le_bytes([data[2], data[3]]) as f32 / 32768.0 * 180.0,
                            i16::from_le_bytes([data[4], data[5]]) as f32 / 32768.0 * 180.0,
                            i16::from_le_bytes([data[6], data[7]]) as f32 / 32768.0 * 180.0,
                        ));
                        //println!("ang: {}, {}, {}",);
                    }
                    _ => {}
                }
            }
            break;
        }
    }


    pub fn z_aziment(&mut self) {
        self.anglar.2 = self.gyro.unwrap_or((0.0, 0.0, 0.0)).2;

        if self.anglar.2 != self.old_anglar.2 {
            self.aziment.2 += (self.anglar.2 as f64 + self.old_anglar.2 as f64) / 2.0 * self.end();
            self.old_anglar.2 = self.anglar.2;
        }
    }


    pub fn end(&mut self) -> f64 {
        let end = self.start_time.elapsed();
        self.start_time = Instant::now();
        end.as_secs_f64()
    }

}
