/*
let tmp: [u16; 2] = [0x7fff, 0x9fff];
    let tmp2: [u16; 4] = [0x000F, 0x00F0, 0x0F00, 0xF000];

    let mut r_order: u8 = 255;
    let mut r_speed: u8 = 255;
    let mut r_angle: u8 = 255;

    for (i, d) in tmp.iter().enumerate() {
        for p in (0..=3).rev() {
            //println!("{}: {}", ((p as isize) - 3).abs(), (d & tmp2[p]) >> p * 4);
            if i == 0 {
                match (d & tmp2[p]) >> p * 4 {
                    15 => {
                        //println!("15");
                    }
                    e => {
                        //println!("not 15");
                        //println!("{}: {}", ((p as isize) - 3).abs(), (d & tmp2[p]) >> p * 4);
                        
                        match p {
                            0 => {
                                
                            }
                            1 => {
                                
                            }
                            2 => {

                            }
                            3 => {
                                r_order = e as u8;
                                println!("{}", r_order);

                            }
                            _=>{}
                        }
                        
                    }
                }
            }
        }
    }
*/