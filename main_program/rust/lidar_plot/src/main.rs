use std::{u32, ops::Add};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 3000Hz


    let mut ld0:[u8;90] = [170, 85, 134, 40, 237, 112, 199, 142, 202, 217, 232, 36, 216, 36, 196, 36, 132, 36, 60, 36, 24, 36, 0, 0, 0, 0, 170, 91, 6, 3, 240, 2, 244, 2, 252, 2, 0, 0, 174, 3, 78, 3, 62, 3, 50, 3, 48, 3, 86, 3, 0, 0, 0, 0, 0, 0, 0, 0, 42, 11, 108, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let mut ld1 = [170, 85, 14, 34, 133, 143, 223, 168, 26, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 150, 8, 156, 8, 168, 8, 184, 8, 196, 8, 212, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 238, 6, 0, 0, 0, 0, 0, 0, 0, 0];

    

    println!("len {}",ld0[3]);
    println!("FSA angle {} {}",ld0[4], ld0[5]);
    println!("LSA angle {} {}",ld0[6], ld0[7]);

    println!("angle {}",(28645 >> 1) as f32 / 64.0);
    //println!("{}",ld0[7]);

    change(&mut ld0);


    let (mut f, mut b) = ld0.split_at_mut(10);
    let mut tmp2 = data_shaping(&b);



    println!("{:?}",hex(&mut f));

    
    let mut data = Vec::new();

    for j in tmp2.iter_mut() {

        data.push(u32::from_str_radix(&hex2(j), 16).unwrap() / 4);
        
    }

    println!("{:?}",data);    
    println!("{:?}",data.len());


    
    /*

    let root = BitMapBackend::new("0.png", (640, 480)).into_drawing_area();

    root.fill(&RGBColor(240, 200, 200))?;

    let root = root.apply_coord_spec(Cartesian2d::<RangedCoordf32, RangedCoordf32>::new(
        0f32..1f32,
        0f32..1f32,
        (0..640, 0..480),
    ));

    let dot_and_label = |x: f32, y: f32| {
        return EmptyElement::at((x, y))
            + Circle::new((0, 0), 3, ShapeStyle::from(&BLACK).filled())
            + Text::new(
                format!("({:.2},{:.2})", x, y),
                (10, 0),
                ("sans-serif", 15.0).into_font(),
            );
    };

    root.draw(&dot_and_label(0.5, 0.6))?;
    root.draw(&dot_and_label(0.25, 0.33))?;
    root.draw(&dot_and_label(0.8, 0.8))?;
    root.draw(&dot_and_label(0.8, 0.6))?;
    root.present()?;
    
    
    
    */


    Ok(())
}




#[inline]
fn change(bytes: &mut [u8]) {
    let mut count = 0;

    for i in 0..(bytes.len() / 2){
        
        let a0 = bytes[count];
        //println!("{:?}",count);

        let a1 = bytes[count + 1];
        //println!("{:?}",a1);

        bytes[count] = a1;

        bytes[count + 1] = a0;

        count+=2;
        
     }

     
}




fn dec(bytes: &mut [u8])  {
    //println!("{}",bytes.len());
}




#[inline]
fn data_shaping(bytes: &[u8])-> Vec<[u8; 2]>  {

    let mut count = 0;


    let mut tmp = [0u8;2];

    let mut tmp2 = Vec::new();
    

    for i in 0..(bytes.len() / 2){

        
        let a0 = bytes[count];
        //println!("{:?}",count);

        let a1 = bytes[count + 1];
        //println!("{:?}",a1);

        tmp[0] = a1;

        tmp[1] = a0;


        tmp2.push(tmp);

        count+=2;
        
     }


     tmp2



}



#[inline]
fn hex(bytes: &mut [u8]) -> String {

    bytes.iter().fold("".to_owned(), |s, b| format!("{}{:x}", s.to_uppercase(), b) )

}


fn hex2(bytes: &mut [u8;2]) -> String {

    bytes.iter().fold("".to_owned(), |s, b| format!("{}{:x}", s.to_uppercase(), b) )

}