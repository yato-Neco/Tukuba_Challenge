///
/// ```
/// let mut port = match serialport::new("COM4", 115200)
///         .stop_bits(serialport::StopBits::One)
///         .data_bits(serialport::DataBits::Eight)
///         .timeout(Duration::from_millis(10))
///         .open()
///      {
///         Ok(p) => (p),
///         Err(_) => (panic!()),
///       };
///
/// let mut serial_buf: Vec<u8> = vec![0; 500];
///
/// loop {
///     match port.read(serial_buf.as_mut_slice()) {
///         Ok(t) => {
/// 
///             let mut data = serial_buf[..t].to_vec();
///             let points =  ydlidarx2_rs::ydlidarx2(&mut data);
/// 
///                 }
///             }
/// 
///             Err(_) => {}
///         }
/// }
///
/// ```
///
/// Vec<(azimuth, distance)>
///
///
///
pub fn ydlidarx2(data: &mut [u8]) -> Vec<(f64, f64)> {
    let rounf_num = 10_f64.powf(6.0);

    let mut points = Vec::with_capacity(300);

    let (f, l) = data.split_at_mut(10);

    let mut distance_i = 0.0_f64;

    let angel_lsn = f[3] as f64 - 1.0;
    let f_len = l.len() - 1;

    let mut angel_fsa = (as_u32_be(&[f[5], f[4]]) >> 1) as f64 / 64.0;

    let mut angel_lsa = (as_u32_be(&[f[7], f[6]]) >> 1) as f64 / 64.0;

    let distance_1 = as_u32_be(&[l[1], l[0]]) as f64;

    let distance_lsa = as_u32_be(&[l[f_len], l[f_len - 1]]) as f64;

    angel_fsa += ang_correct(distance_1);
    angel_lsa += ang_correct(distance_lsa);

    let pre_angle = ((angel_lsa - angel_fsa) * rounf_num).round() / rounf_num;

    let mut count = 0;
    let mut angle_i = 0.0;

    for i in 2..(angel_lsn as usize) {
        let t1 = match l.get(count + 1) {
            Some(e) => e,
            None => &0_u8,
        };

        let t2 = match l.get(count) {
            Some(e) => e,
            None => &0_u8,
        };

        distance_i = as_u32_be(&[*t1, *t2]) as f64 / 4.0;

        angle_i = ((((pre_angle / (angel_lsn)) * (i as f64)) + angel_fsa) * 1.0).round() / 1.0;

        if distance_i == 0.0 {
            angle_i = 0.0;
        }

        distance_i = distance_i / 10.0; //mm -> cm => m
                                        //mm 10 100

        if angle_i != 0.0 && distance_i != 0.0 {
            points.push((angle_i, distance_i));
        }

        count += 2;
    }

    return points;
}

#[inline]
fn ang_correct(distance: f64) -> f64 {
    let rounf_num = 10_f64.powf(4.0);
    let mut ang_correct_i = 0.0;
    if distance != 0.0 {
        ang_correct_i = (((21.8 * (155.3 - distance) / (155.3 * distance)).atan())
            * (180.0 / std::f64::consts::PI)
            * rounf_num)
            .round()
            / rounf_num;
    }

    ang_correct_i
}

#[inline]
fn as_u32_be(array: &[u8; 2]) -> u32 {
    ((array[0] as u32) << 8) | ((array[1] as u32) << 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", as_u32_be(&[170, 85]));

        let mut test_data: [u8; 90] = [
            170, 85, 134, 40, 237, 112, 199, 142, 202, 217, 232, 36, 216, 36, 196, 36, 132, 36, 60,
            36, 24, 36, 0, 0, 0, 0, 170, 91, 6, 3, 240, 2, 244, 2, 252, 2, 0, 0, 174, 3, 78, 3, 62,
            3, 50, 3, 48, 3, 86, 3, 0, 0, 0, 0, 0, 0, 0, 0, 42, 11, 108, 11, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let result = ydlidarx2(&mut test_data);

        println!("{:?}", result);
    }
}
