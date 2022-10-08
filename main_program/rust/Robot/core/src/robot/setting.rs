extern crate yaml_rust;
use yaml_rust::{Yaml, YamlLoader};


#[test]
fn test() {
    let tmp = Settings::_load_setting("./settings.yaml")["Robot"]["Lidar"]["threshold"][0]
    .as_f64()
    .unwrap();
    println!("{}",tmp);

}

pub struct Settings {
    setting_yaml:Yaml
}


impl Settings {

    pub fn _load_setting(path: &str) -> Yaml {
        let f = std::fs::read_to_string(path);
        let s = f.unwrap().to_string();

        let settings_yaml = YamlLoader::load_from_str(&s).unwrap();

        let setting_yaml = settings_yaml[0].clone();

        setting_yaml
    }

    

    pub fn _load_moter_pin(settings_yaml:&Yaml)-> ([u8; 2], [u8; 2]) {
        
        let r0 = settings_yaml["Robot"]["Moter"]["right_gpio"][0].as_i64().unwrap() as u8;
        let r1 = settings_yaml["Robot"]["Moter"]["right_gpio"][1].as_i64().unwrap() as u8;

        let l0 = settings_yaml["Robot"]["Moter"]["left_gpio"][0].as_i64().unwrap() as u8;
        let l1 = settings_yaml["Robot"]["Moter"]["left_gpio"][1].as_i64().unwrap() as u8;


        ([r0,r1],[l0,l1])

    }

    pub fn load_setting(path: &str) -> Self {
        let f = std::fs::read_to_string(path);
        let s = f.unwrap().to_string();

        let settings_yaml = YamlLoader::load_from_str(&s).unwrap();

        let setting_yaml = settings_yaml[0].clone();

        Self { setting_yaml: setting_yaml }
        
    }

    

    pub fn load_moter_pins(self)-> ([u8; 2], [u8; 2]) {
        

        let r0 = self.setting_yaml["Robot"]["Moter"]["right_gpio"][0].as_i64().unwrap() as u8;
        let r1 = self.setting_yaml["Robot"]["Moter"]["right_gpio"][1].as_i64().unwrap() as u8;

        let l0 = self.setting_yaml["Robot"]["Moter"]["left_gpio"][0].as_i64().unwrap() as u8;
        let l1 = self.setting_yaml["Robot"]["Moter"]["left_gpio"][1].as_i64().unwrap() as u8;


        ([r0,r1],[l0,l1])

    }

    
}
