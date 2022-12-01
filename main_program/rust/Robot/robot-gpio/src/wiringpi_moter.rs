use wiringpi::pin::{SoftPwmPin, WiringPi};
//sudo apt-get install wiringpi

#[derive(PartialEq)]
pub enum Mode {
    Front,
    Back,
}

extern crate wiringpi;

struct WiMoter {
    r: [SoftPwmPin<WiringPi>; 2],
    l: [SoftPwmPin<WiringPi>; 2],
}

impl WiMoter {
    pub fn new() -> Self {
        let r0 = wiringpi::setup();
        let r1 = wiringpi::setup();
        let l0 = wiringpi::setup();
        let l1 = wiringpi::setup();

        Self {
            r: [r0.soft_pwm_pin(22), r1.soft_pwm_pin(23)],

            l: [l0.soft_pwm_pin(24), l1.soft_pwm_pin(25)],
        }
    }

    pub fn right(&self, duty: f64, mode: Mode) {
        if mode == Mode::Front {
            self.r[1].pwm_write((duty * 100.0) as i32);
            self.r[0].pwm_write((duty * 100.0) as i32);
        } else {
            self.r[0].pwm_write((duty * 100.0) as i32);
            self.r[1].pwm_write((duty * 100.0) as i32);
        }
    }

    pub fn left(&self, duty: f64, mode: Mode) {
        if mode == Mode::Front {
            self.l[1].pwm_write((duty * 100.0) as i32);
            self.l[0].pwm_write((duty * 100.0) as i32);
        } else {
            self.l[0].pwm_write((duty * 100.0) as i32);
            self.l[1].pwm_write((duty * 100.0) as i32);
        }
    }


    pub fn _front(&self, r_duty: f64, l_duty: f64) {
        self.r[0].pwm_write((r_duty * 100.0) as i32);
        self.r[1].pwm_write(0);
        self.l[0].pwm_write((l_duty * 100.0) as i32);
        self.l[1].pwm_write(0);
    }


    pub fn _back(&self, r_duty: f64, l_duty: f64) {
        self.r[0].pwm_write(0);
        self.r[1].pwm_write((r_duty * 100.0) as i32);
        self.l[0].pwm_write(0);
        self.l[1].pwm_write((l_duty * 100.0) as i32);
    }
    
    pub fn _left(&self, r_duty: f64, l_duty: f64) {
        self.r[0].pwm_write(0);
        self.r[1].pwm_write((r_duty * 100.0) as i32);
        self.l[0].pwm_write((l_duty * 100.0) as i32);
        self.l[1].pwm_write(0);
    }

    pub fn _right(&self, r_duty: f64, l_duty: f64) {
        self.r[0].pwm_write((r_duty * 100.0) as i32);
        self.r[1].pwm_write(0);
        self.l[0].pwm_write(0);
        self.l[1].pwm_write((l_duty * 100.0) as i32);
    }
    pub fn _stop(&self) {
        self.r[0].pwm_write(0);
        self.r[1].pwm_write(0);
        self.l[0].pwm_write(0);
        self.l[1].pwm_write(0);
    }

    #[inline]
    pub fn moter_control(&mut self, order: u32) {
        let rM: i8 = ((order & 0x00F00000) >> 20) as i8;
        let lM: i8 = ((order & 0x000F0000) >> 16) as i8;

        match (rM, lM) {
            (1..=7, 1..=7) => {
                self._front((rM / 7) as f64, (lM / 7) as f64);
                //self.right(rM as f64 / 7.0, Mode::Back);
                //self.left(lM as f64 / 7.0, Mode::Back);
            }
            (8..=14, 8..=14) => {
                self._back(((rM - 7) / 7) as f64, ((lM - 7) / 7) as f64);

                //self.right((rM - 7) as f64 / 7.0, Mode::Front);
                //self.left((lM - 7) as f64 / 7.0, Mode::Front);
            }
            (1..=7, 8..=14) => {
                self._right((rM / 7) as f64, ((lM - 7) / 7) as f64);
                //self.right(rM as f64 / 7.0, Mode::Back);
                //self.left((lM - 7) as f64 / 7.0, Mode::Front);
            }
            (8..=14, 1..=7) => {
                self._left(((rM - 7) / 7) as f64, (lM / 7) as f64);
                //self.right((rM - 7) as f64 / 7.0, Mode::Front);
                //self.left(lM as f64 / 7.0, Mode::Back);
            }
            _ => {
                self._stop()
                //self.pwm_all_clean();
            }
        }
    }
}
