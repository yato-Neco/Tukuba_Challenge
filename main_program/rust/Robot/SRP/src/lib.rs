#[derive(Debug)]
#[repr(C)]

struct SRP<'a> {
    identifier: u32,
    order: u32,
    bytes: &'a [u8],
}

fn u32_to_le(v: &mut Vec<u32>) -> &[u8] {
    for b in v.iter_mut() {
        *b = b.to_le()
    }

    unsafe { v.align_to().1 }
}

fn f64_to_le(v: &mut Vec<f64>) -> &[u8] {
    unsafe { v.align_to().1 }
}

fn to_le<T>(v: &mut Vec<T>) -> &[u8] {
    unsafe { v.align_to().1 }
}

fn to_vecf64(v: &[u8]) -> &[f64] {
    unsafe { v.align_to().1 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let test: &str = "Test";
        let mut tmp_u32 = Vec::<u32>::new();
        let mut tmp_f64 = Vec::<f64>::new();
        tmp_u32.push(0xfffff);

        tmp_f64.push(1.0);
        tmp_f64.push(2.0);
        tmp_f64.push(3.0);
        tmp_f64.push(4.0);
        tmp_f64.push(5.0);
        tmp_f64.push(6.0);

        println!("{:?}", "a");
        let t = SRP {
            identifier: 0,
            order: 1,
            bytes: to_le(&mut tmp_f64),
        };
        println!("{:?}", to_vecf64(t.bytes));
    }
}
