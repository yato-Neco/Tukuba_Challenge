pub struct DisplayMode {
     pub operation: Vec<(f64,f64)>
}

#[test]
fn test() {


    let mut tmp = DisplayMode::new();

    tmp.load_csv();
    tmp.start();

}

impl DisplayMode {

    pub fn new() -> DisplayMode {
        let mut operation:Vec<(f64,f64)> = Vec::new();
        Self { operation: operation }
    }

    pub fn load_csv(&mut self) {
        extern crate csv;
        use std::fs::File;


        let file = File::open("operation.csv").unwrap();
        let mut rdr = csv::Reader::from_reader(file);
        for (i, result) in rdr.records().enumerate() {
            let record = result.expect("a CSV record");

            let sdistance = match record.get(0) {
                Some(e) => e,
                None => panic!("{}行目 の設定", i),
            };
            let sazimuth = match record.get(1) {
                Some(e) => e,
                None => panic!("{}行目 の設定", i),
            };


            let distance: f64 = match sdistance.trim().replace("_", "").parse() {
                Ok(e) => e,
                Err(_) => panic!("{}行目 がf64形式じゃないよ", i),
            };

            let azimuth: f64 = match sazimuth.trim().replace("_", "").parse() {
                Ok(e) => e,
                Err(_) => panic!("{}行目 がf64形式じゃないよ", i),
            };

      
            self.operation.push((distance,azimuth));

        }

    }

    pub fn start(&self) {
        for i in self.operation.iter() {
            println!("{:?}",*i);
        }

    }
}