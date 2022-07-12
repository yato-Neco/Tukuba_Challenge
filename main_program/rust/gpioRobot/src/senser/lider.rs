use std::time::Duration;
use std::io;
use std::io::Write;


#[test]
fn lider(){
    let mut port = serialport::new("/dev/ttyUSB0", 115200)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Eight)
        .timeout(Duration::from_millis(10))
        .open()
        .unwrap_or_else(|e| {
            eprintln!("Failed to open \"{}\". Error: {}", "/dev/ttyUSB", e);
            ::std::process::exit(1);
    });



    let mut serial_buf: Vec<u8> = vec![0; 15000];
    //1 2 3 4 5 6


    //let mut tmp = [];
    
    
    loop {
        
         match port.read(serial_buf.as_mut_slice()) {
            Ok(t) => println!("{:?}",&serial_buf[..t]),
            Err(_e) => {},
        }
        
        
       
    }


}


/*


*/