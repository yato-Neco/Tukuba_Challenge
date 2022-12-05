/// 定数の置き場所
pub const STOP: u32 = 0x1F00FFFF;
pub const EMERGENCY_STOP: u32 = 0x0F00FFF1;
pub const BREAK: u32 = 0x0FFFFFFF;
pub const NONE: u32 = 0xffffffff;
pub const FRONT:u32 = 0x1FEEFFFF;
pub const TRUN:u32 = 0x1FAEFFFF;

pub type SenderOrders = std::sync::mpsc::Sender<u32>;

/*
0x1F00FFFF

0x0F00FFF1
*/

#[test]
fn test() {
    //let lM = ((0x0F80FFF1 & 0x00F00000) >> 20) ;
    //println!("{lM}");
    moter_control(0x1F6DFFFF);
}

pub fn moter_control(order: u32) {
    let rM = ((order & 0x00F00000) >> 20) as f64;
    let lM = ((order & 0x000F0000) >> 16) as f64;
    //println!("{:?}",(rM,lM));
    match (rM as u8,lM as u8) {
        (1..=7, 1..=7) => {
            println!("{:?} {:?}",(rM / 7.0),(lM / 7.0));
            //self._front((rM / 7) as f64,(lM / 7) as f64);
            //self.right(rM as f64 / 7.0, Mode::Back);
            //self.left(lM as f64 / 7.0, Mode::Back);
        }
        (8..=14, 8..=14) => {
            println!("{:?} {:?}",((rM-7.0) / 7.0) ,((lM-7.0) / 7.0));
            //self._back(((rM-7) / 7) as f64,((lM-7) / 7) as f64);
            //self.right((rM - 7) as f64 / 7.0, Mode::Front);
            //self.left((lM - 7) as f64 / 7.0, Mode::Front);
        }
        (1..=7, 8..=14) => {
            println!("{:?} {:?}",(rM / 7.0),((lM-7.0) / 7.0));
            //self._right((rM / 7) as f64,((lM-7) / 7) as f64);
            //self.right(rM as f64 / 7.0, Mode::Back);
            //self.left((lM - 7) as f64 / 7.0, Mode::Front);
        }
        (8..=14, 1..=7) => {
            println!("{:?} {:?}",((rM-7.0) / 7.0),(lM / 7.0));
            //self._left(((rM-7) / 7) as f64,(lM / 7) as f64);
            //self.right((rM - 7) as f64 / 7.0, Mode::Front);
            //self.left(lM as f64 / 7.0, Mode::Back);
        }
        _ => {
            //self._stop();
            //self.pwm_all_clean();
        }
    }
}
