use std::{u32, ops::Add};

use plotters::prelude::*;
use plotters::coord::types::RangedCoordf32;
use rand::SeedableRng;
use rand_distr::{Distribution, Normal};
use rand_xorshift::XorShiftRng;

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


    let x = std::f64::consts::PI / 4.0;
    let abs_difference = x.atan();
    

    println!("{}",abs_difference);




    for (i,data) in ld1.iter().enumerate() {
        
        let angcorrect = 0;


        //println!("{}",data);


        


        

    }



    
    

    let root = BitMapBackend::new("0.png", (1024, 768)).into_drawing_area();

    root.fill(&WHITE)?;

    let sd = 0.13;

    let random_points: Vec<(f64, f64)> = {
        let norm_dist = Normal::new(0.5, sd).unwrap();
        let mut x_rand = XorShiftRng::from_seed(*b"MyFragileSeed123");
        let mut y_rand = XorShiftRng::from_seed(*b"MyFragileSeed321");
        let x_iter = norm_dist.sample_iter(&mut x_rand);
        let y_iter = norm_dist.sample_iter(&mut y_rand);
        x_iter.zip(y_iter).take(5000).collect()
    };

    let mut points = Vec::new();

    points.push((0.5_f64,0.5_f64));
    points.push((0.7_f64,0.0_f64));


    let areas = root.split_by_breakpoints([944], [80]);

    let mut x_hist_ctx = ChartBuilder::on(&areas[0])
        .y_label_area_size(40)
        .build_cartesian_2d((0.0..1.0).step(0.01).use_round().into_segmented(), 0..250)?;
    let mut y_hist_ctx = ChartBuilder::on(&areas[3])
        .x_label_area_size(40)
        .build_cartesian_2d(0..250, (0.0..1.0).step(0.01).use_round())?;
    let mut scatter_ctx = ChartBuilder::on(&areas[2])
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..1f64, 0f64..1f64)?;
    scatter_ctx
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;
    scatter_ctx.draw_series(
        points
            .iter()
            .map(|(x, y)| Circle::new((*x, *y), 2, GREEN.filled())),
    )?;
    let x_hist = Histogram::vertical(&x_hist_ctx)
        .style(GREEN.filled())
        .margin(0)
        .data(points.iter().map(|(x, _)| (*x, 1)));
    let y_hist = Histogram::horizontal(&y_hist_ctx)
        .style(GREEN.filled())
        .margin(0)
        .data(points.iter().map(|(_, y)| (*y, 1)));

    x_hist_ctx.draw_series(x_hist)?;
    y_hist_ctx.draw_series(y_hist)?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    
    
    
    


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