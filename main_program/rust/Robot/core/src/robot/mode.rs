use getch;

use super::config;


pub struct  Mode {

}

impl Mode {
    pub fn key() -> u32 {
        let key = getch::Getch::new();
            let key_order_u8 = key.getch().unwrap();
            //println!("{}", b);
    
            let order = match key_order_u8 {
                119 => {
                    // w
                    0x1FBBFFFF
                }
                97 => {
                    // a
                    0x1F48FFFF
                }
                115 => {
                    // s
                    0x1F33FFFF
                }
                100 => {
                    // d
                    0x1F84FFFF
                }
                32 => {
                    // stop
                    config::STOP
                }
                3 => {
                    // break
                    config::BREAK
                }
    
                _ => 0xFFFFFFF,
            };
            order
    }
    
}
