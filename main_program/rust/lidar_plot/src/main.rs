use std::{u32, ops::Add};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 3000Hz


    let mut ld0:[u8;90] = [170, 85, 134, 40, 237, 112, 199, 142, 202, 217, 232, 36, 216, 36, 196, 36, 132, 36, 60, 36, 24, 36, 0, 0, 0, 0, 170, 91, 6, 3, 240, 2, 244, 2, 252, 2, 0, 0, 174, 3, 78, 3, 62, 3, 50, 3, 48, 3, 86, 3, 0, 0, 0, 0, 0, 0, 0, 0, 42, 11, 108, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let mut ld1 = [170, 85, 14, 34, 133, 143, 223, 168, 26, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 150, 8, 156, 8, 168, 8, 184, 8, 196, 8, 212, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 238, 6, 0, 0, 0, 0, 0, 0, 0, 0];

    let mut ld2 = [170, 85, 42, 40, 93, 170, 51, 20, 217, 245, 186, 220, 41, 222, 156, 223, 40, 224, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 234, 2, 0, 0, 0, 0, 74, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 182, 9, 144, 9, 220, 9, 40, 10, 0, 0, 150, 13, 216, 12, 180, 12, 148, 12, 56, 12, 52, 11];    

    //println!("len {}",ld0[3]);
    //println!("FSA angle {}",((u32::from_str_radix(&hex2(&mut [ld0[5],ld0[4]]), 16).unwrap()) >> 1) as f32 / 64.0);
    //println!("LSA angle {}",((u32::from_str_radix(&hex2(&mut [ld0[7],ld0[6]]), 16).unwrap()) >> 1) as f32 / 64.0);

    //println!("TEST angle {}",(28645 >> 1) as f32 / 64.0);
    //println!("{}",ld0[7]);

    change(&mut ld2);


    let (mut f, mut b) = ld2.split_at_mut(10);
    let mut tmp2 = data_shaping(&b);



    println!("{:?}",hex(&mut f));

    
    let mut data = Vec::new();

    for j in tmp2.iter_mut() {

        data.push(u32::from_str_radix(&hex2(j), 16).unwrap() as f32 / 4.0);
        
    }

    println!("{:?}",data);    
    println!("{:?}",data.len());

    /*

    for i in 2..39 {
        println!("{}",intermediate_angle((ld2[3] - 1) as usize, i,223.78,243.47));

    }
    
    */

    let mut x = std::f64::consts::FRAC_PI_4;

    
    println!("{}",x);

    x = 3.14 / 2.0;

    println!("{}",(x.sin()));    


    for (i,data) in ld1.iter().enumerate() {
        
        let angcorrect = 0;


        //println!("{}",data);


        


        

    }



    
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






fn intermediate_angle(LSN:usize,i:usize,angleFSA:f32,angleLSA:f32) -> f32{
    //diff angleFSA - angleLSA


    (angleLSA - angleFSA) / (LSN as f32 - 1.0) * (i - 1) as f32 + angleFSA



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