use serde::{Deserialize,Serialize};

use std::collections::HashMap;
use std::fs;
use std::io::BufReader;

#[derive(Deserialize, Serialize, Debug)]
pub struct UOKConfig{
    pub endpoints: Vec<UOKEndpointConfig>
}



#[derive(Deserialize, Serialize, Debug)]
pub struct UOKEndpointConfig{
    pub url: String,
    pub headers: Option<HashMap<String,String>>,
    pub method: Option<String>
}

impl std::hash::Hash for UOKEndpointConfig {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.url.hash(state);
    }
}

impl PartialEq for UOKEndpointConfig {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}


pub fn load_config(config_file: &String) -> UOKConfig {

    let file = fs::File::open(config_file.clone()).expect(format!("Config file {} does not exist", config_file.clone()).as_ref());
    let reader = BufReader::new(file);

    // return parsed config
    serde_json::from_reader(reader).expect(format!("Config file {} could not be parsed.", config_file.clone()).as_ref())

}