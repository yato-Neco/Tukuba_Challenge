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

/*

if flag.0 {
            msg.send(STOP).unwrap();
            break;
        } else {
            let mut azimuth = (flag.1 .0 * c2).round();
            //println!("{}", now_azimuth);
            //azimuth = azimuth + now_azimuth;

            println!("azimuth {}", azimuth / c2);
            let razi = azimuth;

            //println!("{} {} {}", azi, azi >= 0.0, azi <= 0.0);

            // 回転系
            let mut count = 0;
            loop {
                let r: bool = azimuth >= 0.0;
                let l: bool = azimuth <= 0.0;

                // シュミ系 ->

                //azi = azi + (-1.0 * azi);
                //println!("{}", razi);

                if r != l {
                    //now_azimuth = razi;

                    if count == 0 {
                        match msg.send(0x1F18FFFF) {
                            Ok(_) => {}
                            Err(_) => {}
                        };
                    }
                    if r {
                        azimuth -= 1000.0;
                    } else {
                        azimuth += 1000.0;
                    }
                }
                // <-
                //time_sleep(1);
                else if r == l {
                    //println!("回転");
                    now_azimuth = razi;

                    if count == 0 {
                        break;
                    } else {
                        match msg.send(STOP) {
                            Ok(_) => {}
                            Err(_) => {}
                        };

                        break;
                    }
                }

                count += 1;
            }
            //

            let index: usize = (flag.2 .0 + flag.2 .1).abs() as usize;

            let order: u32 = distance_con(index);

            match msg.send(order) {
                Ok(_) => {}
                Err(_) => {}
            };

            //println!("{:?}", r);

            //rintln!("distance {} {}", flag.2 .0, flag.2 .1);

            // シュミ系 ->
            println!("{:?} {:?}", nlatlot, flag.2);

            if (flag.2 .0 * t1) > 0.0 {
                nlatlot.0 += 0.1;
            } else if (flag.2 .0 * t1) < 0.0 {
                nlatlot.0 -= 0.1;
            }

            if (flag.2 .1 * t1) > 0.0 {
                nlatlot.1 += 0.1;
            } else if (flag.2 .1 * t1) < 0.0 {
                nlatlot.1 -= 0.1;
            }

            //nlatlot.0 += (flag.2 .0) * t1;
            //nlatlot.1 += (flag.2 .1) * t1;
            // <-
        }







*/