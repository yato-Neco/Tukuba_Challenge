#[derive(PartialEq)]
pub enum Mode {
    Front,
    Back,
}

use mytools::mic_sleep;
#[cfg(target_os = "linux")]
use rppal::gpio::{Gpio, OutputPin};
#[cfg(target_os = "linux")]
use rppal::system::DeviceInfo;

#[cfg(target_os = "linux")]
#[derive(Debug)]
pub struct Moter {
    pub r_pin0: OutputPin,
    pub r_pin1: OutputPin,

    pub l_pin0: OutputPin,
    pub l_pin1: OutputPin,
}

#[cfg(target_os = "linux")]
///
/// ```
/// let mut moter = MoterGPIO::new([25,24], [23,22]);
/// moter.right(1.0,Mode::Front);
/// moter.left(1.0,Mode::Front);
/// moter.pwm_all_clean();
/// ```
impl Moter {
    /// GPIO のピンをセットする関数
    ///
    /// ```
    /// let mut moter = MoterGPIO::new([25,24], [23,22]);
    ///
    /// ```
    ///
    pub fn new(r_pin: [u8; 2], l_pin: [u8; 2]) -> Self {
        let mut r_pin0: OutputPin = Gpio::new().unwrap().get(r_pin[0]).unwrap().into_output();
        let mut r_pin1: OutputPin = Gpio::new().unwrap().get(r_pin[1]).unwrap().into_output();

        let mut l_pin0: OutputPin = Gpio::new().unwrap().get(l_pin[0]).unwrap().into_output();
        let mut l_pin1: OutputPin = Gpio::new().unwrap().get(l_pin[1]).unwrap().into_output();

        return Self {
            r_pin0,
            r_pin1,

            l_pin0,
            l_pin1,
        };
    }

    /// 右モーター制御
    #[inline]
    pub fn right(&mut self, duty: f64, mode: Mode) {
        if mode == Mode::Back {
            self.r_pin1.set_pwm_frequency(50.0, duty).unwrap();
            self.r_pin0.set_pwm_frequency(0.0, 0.0).unwrap();
        } else {
            self.r_pin1.set_pwm_frequency(0.0, 0.0).unwrap();
            self.r_pin0.set_pwm_frequency(50.0, duty).unwrap();
        }
    }
    /// 左モーター制御
    #[inline]
    pub fn left(&mut self, duty: f64, mode: Mode) {
        if mode == Mode::Back {
            self.l_pin1.set_pwm_frequency(50.0, duty).unwrap();
            self.l_pin0.set_pwm_frequency(0.0, 0.0).unwrap();
        } else {
            self.l_pin1.set_pwm_frequency(0.0, 0.0).unwrap();
            self.l_pin0.set_pwm_frequency(50.0, duty).unwrap();
        }
    }

    /// right モーター 前後
    /// duty 0.0 ~ 1.0
    ///  非推奨
    pub fn rfpwm(&mut self, duty: f64) {
        self.r_pin1.set_pwm_frequency(50.0, duty).unwrap();
        self.r_pin0.set_pwm_frequency(0.0, 0.0).unwrap();
    }

    /// right モーター　後進
    /// duty 0.0 ~ 1.0
    /// 非推奨
    pub fn rbpwm(&mut self, duty: f64) {
        self.r_pin1.set_pwm_frequency(0.0, 0.0).unwrap();
        self.r_pin0.set_pwm_frequency(50.0, duty).unwrap();
    }

    /// left モーター 前後
    /// duty 0.0 ~ 1.0
    /// 非推奨
    pub fn lfpwm(&mut self, duty: f64) {
        self.l_pin1.set_pwm_frequency(50.0, duty).unwrap();
        self.l_pin0.set_pwm_frequency(0.0, 0.0).unwrap();
    }

    /// left モーター　後進
    /// duty 0.0 ~ 1.0
    /// 非推奨
    pub fn lbpwm(&mut self, duty: f64) {
        self.l_pin1.set_pwm_frequency(0.0, 0.0).unwrap();
        self.l_pin0.set_pwm_frequency(50.0, duty).unwrap();
    }

    /// PWMのリセット
    #[inline]
    pub fn pwm_all_clean(&mut self) {
        self.r_pin0.clear_pwm().unwrap();
        self.r_pin1.clear_pwm().unwrap();
        self.l_pin0.clear_pwm().unwrap();
        self.l_pin1.clear_pwm().unwrap();
    }
    #[inline]

    #[inline]
    pub fn reset(&mut self) -> bool {

        self.r_pin0.reset_on_drop()
            && self.r_pin1.reset_on_drop()
            && self.l_pin0.reset_on_drop()
            && self.l_pin1.reset_on_drop()
    }

    /// ロボットの命令をモーターに伝える。
    #[inline]
    pub fn moter_control(&mut self, order: u32) {
        let rM: i8 = ((order & 0x00F00000) >> 20) as i8;
        let lM: i8 = ((order & 0x000F0000) >> 16) as i8;

        match (rM, lM) {
            (1..=7, 1..=7) => {
                self.right(rM as f64 / 7.0, Mode::Back);
                self.left(lM as f64 / 7.0, Mode::Back);
            }
            (8..=14, 8..=14) => {
                self.right((rM - 7) as f64 / 7.0, Mode::Front);
                self.left((lM - 7) as f64 / 7.0, Mode::Front);
            }
            (1..=7, 8..=14) => {
                self.right(rM as f64 / 7.0, Mode::Back);
                self.left((lM - 7) as f64 / 7.0, Mode::Front);
            }
            (8..=14, 1..=7) => {
                self.right((rM - 7) as f64 / 7.0, Mode::Front);
                self.left(lM as f64 / 7.0, Mode::Back);
            }
            _ => {
                self.pwm_all_clean();
            }
        }
    }
}

#[cfg(target_os = "windows")]
#[derive(Debug, Clone)]
pub struct Moter {
    r_pin0: u8,
    r_pin1: u8,

    l_pin0: u8,
    l_pin1: u8,
}

#[cfg(target_os = "windows")]
///　windows実行環境はgpioないから以下プログラムはダミー
/// ```
/// let mut moter = MoterGPIO::new([25,24], [23,22]);
///
/// moter.rfpwm();
///
/// moter.lfpwm();
///
/// moter.pwm_all_clean();
/// ```
impl Moter {
    /// GPIO のピンをセットする関数
    ///
    /// ```
    /// let mut moter = MoterGPIO::new([25,24], [23,22]);
    /// ```
    ///
    pub fn new(r_pin: [u8; 2], l_pin: [u8; 2]) -> Self {
        let r_pin0 = r_pin[0];
        let r_pin1 = r_pin[1];

        let l_pin0 = l_pin[0];
        let l_pin1 = l_pin[1];

        return Self {
            r_pin0,
            r_pin1,

            l_pin0,
            l_pin1,
        };
    }

    /// right モーター 前後
    /// duty 0.0 ~ 1.0
    pub fn _right(&mut self, duty: f64) {}
    pub fn right(&mut self, duty: f64, mode: Mode) {
        if mode == Mode::Front {
        } else {
        }
    }

    /// right モーター　後進
    /// duty 0.0 ~ 1.0
    //pub fn rbpwm(&mut self, duty: f64) {}

    /// left モーター 前後
    /// duty 0.0 ~ 1.0
    pub fn _left(&mut self, duty: f64) {}
    pub fn left(&mut self, duty: f64, mode: Mode) {
        if mode == Mode::Front {
        } else {
        }
    }

    /// left モーター　後進
    /// duty 0.0 ~ 1.0
    //pub fn lbpwm(&mut self, duty: f64) {}

    /// PWMのリセット
    pub fn pwm_all_clean(&mut self) {}

    pub fn reset(&mut self) -> bool {
        true
    }

    pub fn order_analysis(order: u32) -> (f64, f64) {
        let rM: i8 = ((order & 0x00F00000) >> 20) as i8;
        let lM: i8 = ((order & 0x000F0000) >> 16) as i8;

        match (rM, lM) {
            (1..=7, 1..=7) => (rM as f64 / 7.0, lM as f64 / 7.0),
            (8..=14, 8..=14) => ((rM - 7) as f64 / 7.0, (lM - 7) as f64 / 7.0),
            (1..=7, 8..=14) => (rM as f64 / 7.0, (lM - 7) as f64 / 7.0),
            (8..=14, 1..=7) => ((rM - 7) as f64 / 7.0, lM as f64 / 7.0),
            _ => {
                //self.pwm_all_clean();
                (0.0, 0.0)
            }
        }
    }
    pub fn moter_control(&mut self, order: u32) {
        let rM: i8 = ((order & 0x00F00000) >> 20) as i8;
        let lM: i8 = ((order & 0x000F0000) >> 16) as i8;

        match (rM, lM) {
            (1..=7, 1..=7) => {
                self.right(rM as f64 / 7.0, Mode::Front);
                mic_sleep(1);
                self.left(lM as f64 / 7.0, Mode::Front);
            }
            (8..=14, 8..=14) => {
                self.right((rM - 7) as f64 / 7.0, Mode::Back);
                mic_sleep(1);
                self.left((lM - 7) as f64 / 7.0, Mode::Back);
            }
            (1..=7, 8..=14) => {
                self.right(rM as f64 / 7.0, Mode::Front);
                mic_sleep(1);
                self.left((lM - 7) as f64 / 7.0, Mode::Back);
            }
            (8..=14, 1..=7) => {
                self.right((rM - 7) as f64 / 7.0, Mode::Back);
                mic_sleep(1);
                self.left(lM as f64 / 7.0, Mode::Front);
            }
            _ => {
                self.pwm_all_clean();
            }
        }
    }

    /*


    pub fn moter_control(order: u32, moter:&mut MoterGPIO)  {
        let rM: i8 = ((order & 0x00F00000) >> 20) as i8;
        let lM: i8 = ((order & 0x000F0000) >> 16) as i8;
         match (rM, lM) {
            (1..=7, 1..=7) => {
                println!("後進 {} {}", (rM - 8).abs(), (lM - 8).abs());
                moter.rbpwm(roundf((rM - 8).abs() as f64 * 0.1,10));
                moter.lbpwm(roundf((lM - 8).abs() as f64 * 0.1, 10));
            }
            (8..=14, 8..=14) => {
                println!("前進 {} {}", rM - 4, lM - 4);
                moter.rfpwm(roundf((rM - 4) as f64 * 0.1, 10));
                moter.lfpwm(roundf((lM - 4) as f64 * 0.1, 10));

            }
            (0, 0) => {
                println!("ストップ");
                moter.pwm_all_clean();
            }
            (1..=7, 8..=14) => {
                println!("回転 {} {}", (rM - 8).abs(), lM - 4);
                moter.rbpwm(roundf((rM - 8).abs() as f64 * 0.1, 10));
                moter.lfpwm(roundf((lM - 4) as f64 * 0.1, 10));

            }
            (8..=14, 1..=7) => {
                println!("回転 {} {}", rM - 4, (lM - 8).abs());
                moter.rfpwm(roundf((rM - 4) as f64 * 0.1, 10));
                moter.lbpwm(roundf((lM - 8).abs() as f64 * 0.1,10));

            }
            _ => {
                println!("その他 {} {}", rM, lM);
                //moter.pwm_all_clean();
            }
        };

    }
    */
}
