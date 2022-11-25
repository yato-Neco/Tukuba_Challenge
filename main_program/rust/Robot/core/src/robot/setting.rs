extern crate yaml_rust;
use yaml_rust::{Yaml, YamlLoader};
use std::fs::File;

#[test]
fn test() {
    let tmp = Settings::load_setting("./settings.yaml");
    println!("{:?}", tmp.load_waypoint());
}

pub struct Settings {
    setting_yaml: Yaml,
}

/// 設定ファイル関係
impl Settings {
    pub fn _load_setting(path: &str) -> Yaml {
        let f = std::fs::read_to_string(path);
        let s = f.unwrap().to_string();

        let settings_yaml = YamlLoader::load_from_str(&s).unwrap();

        let setting_yaml = settings_yaml[0].clone();

        setting_yaml
    }

    pub fn _load_moter_pin(settings_yaml: &Yaml) -> ([u8; 2], [u8; 2]) {
        let r0 = settings_yaml["Robot"]["Moter"]["right_gpio"][0]
            .as_i64()
            .unwrap() as u8;
        let r1 = settings_yaml["Robot"]["Moter"]["right_gpio"][1]
            .as_i64()
            .unwrap() as u8;

        let l0 = settings_yaml["Robot"]["Moter"]["left_gpio"][0]
            .as_i64()
            .unwrap() as u8;
        let l1 = settings_yaml["Robot"]["Moter"]["left_gpio"][1]
            .as_i64()
            .unwrap() as u8;

        ([r0, r1], [l0, l1])
    }

    /// Yaml 読み込み
    pub fn load_setting(path: &str) -> Self {
        let f = std::fs::read_to_string(path);
        let s = f.unwrap().to_string();

        let settings_yaml = YamlLoader::load_from_str(&s).unwrap();

        let setting_yaml = settings_yaml[0].clone();

        Self {
            setting_yaml: setting_yaml,
        }
    }

    /// Moter の GPIO の設定ファイルを読み込み
    pub fn load_moter_pins(&self) -> ([u8; 2], [u8; 2]) {
        let r0 = self.setting_yaml["Robot"]["Moter"]["right_gpio"][0]
            .as_i64()
            .unwrap() as u8;
        let r1 = self.setting_yaml["Robot"]["Moter"]["right_gpio"][1]
            .as_i64()
            .unwrap() as u8;

        let l0 = self.setting_yaml["Robot"]["Moter"]["left_gpio"][0]
            .as_i64()
            .unwrap() as u8;
        let l1 = self.setting_yaml["Robot"]["Moter"]["left_gpio"][1]
            .as_i64()
            .unwrap() as u8;

        ([r0, r1], [l0, l1])
    }

    /// GPS のシリアル通信系の設定を読み込み
    pub fn load_gps_serial(&self) -> (String, u32, usize) {
        let port = self.setting_yaml["Robot"]["GPS"]["Serial"]["port"][0]
            .as_str()
            .unwrap_or("COM4")
            .to_string();
        let rate = self.setting_yaml["Robot"]["GPS"]["Serial"]["rate"][0]
            .as_i64()
            .unwrap_or(115200) as u32;
        let buf_size = self.setting_yaml["Robot"]["GPS"]["Serial"]["buf_size"][0]
            .as_i64()
            .unwrap_or(500) as usize;

        return (port, rate, buf_size);
    }

    pub fn load_raspico(&self) -> (String, u32) {
        let port = self.setting_yaml["Robot"]["RasPico"]["Serial"]["port"][0]
            .as_str()
            .unwrap_or("COM4")
            .to_string();
        let rate = self.setting_yaml["Robot"]["RasPico"]["Serial"]["rate"][0]
            .as_i64()
            .unwrap_or(115200) as u32;

        return (port, rate);
    }

    pub fn load_lidar(&self) {}

    pub fn load_move_csv(&self) -> Vec<(u32, u32)> {
        extern crate csv;
        use std::fs::File;

        let mut operation: Vec<(u32, u32)> = Vec::new();

        let file = File::open(
            self.setting_yaml["Robot"]["Display_mode"]["order"][0]
                .as_str()
                .unwrap(),
        )
        .unwrap();
        let mut rdr = csv::Reader::from_reader(file);
        for (i, result) in rdr.records().enumerate() {
            let record = result.expect("a CSV record");

            let sorder = match record.get(0) {
                Some(e) => e,
                None => panic!("{}行目 の設定", i),
            };
            let stime = match record.get(1) {
                Some(e) => e,
                None => panic!("{}行目 の設定", i),
            };

            if sorder.len() <= 2 {
                panic!("len > 2");
            };

            let (front, back) = sorder.split_at(2);
            if front != "0x" {
                panic!("not use 0x");
            };

            let order: u32 = match u32::from_str_radix(&back, 16) {
                Ok(e) => e,
                Err(_) => panic!("{}行目 がu32形式じゃないよ", i),
            };

            let time: u32 = match stime.trim().replace("_", "").parse() {
                Ok(e) => e,
                Err(_) => panic!("{}行目 がu32形式じゃないよ", i),
            };

            operation.push((order, time));
        }

        operation
    }


    pub fn load_waypoint(&self) -> Vec<(f64,f64)> {

        let file = File::open(
            self.setting_yaml["Robot"]["GPS"]["waypoint"][0]
                .as_str()
                .unwrap(),
        ).unwrap();
        let mut latlot = Vec::new();
        let mut rdr = csv::Reader::from_reader(file);
        for (i, result) in rdr.records().enumerate() {
            let record = result.expect("a CSV record");

            let slat = match record.get(0) {
                Some(e) => e,
                None => panic!("{}行目 latの設定", i),
            };
            let slot = match record.get(1) {
                Some(e) => e,
                None => panic!("{}行目 lotの設定", i),
            };

            let lat: f64 = match slat.trim().replace("_", "").parse() {
                Ok(e) => e,
                Err(_) => panic!("{}行目 latがf64形式じゃないよ", i),
            };

            let lot: f64 = match slot.trim().replace("_", "").parse() {
                Ok(e) => e,
                Err(_) => panic!("{}行目 lotがf64形式じゃないよ", i),
            };

            latlot.push((lat, lot));
        };

        latlot
    }
}
