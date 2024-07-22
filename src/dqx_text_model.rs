use std::{fs::File, io::Read};

use iced::widget::shader::wgpu::naga::FastHashMap;
use indexmap::IndexMap;

type TranslationJsonRoot = IndexMap<String, FastHashMap<String, String>>;

#[derive(Debug, Default)]
pub struct DqxTranslationsModel {
    inner_data: TranslationJsonRoot,
}

impl DqxTranslationsModel {
    pub fn from_path(path: &str) -> Option<Self> {
        // Try to open the path and reat the data...
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(err) => {
                println!("Error opening file: {path}; {:?}", err);
                return None;
            }
        };

        let mut json_data = String::from("");
        match file.read_to_string(&mut json_data) {
            Ok(_) => {}
            Err(err) => {
                println!("Error reading JSON from file: {path}; {:?}", err);
                return None;
            }
        }

        let inner_data: TranslationJsonRoot = match serde_json::from_str(&json_data) {
            Ok(data) => data,
            Err(err) => {
                println!(
                    "Error reading translation data from JSON: {path}; {:?}",
                    err
                );
                return None;
            }
        };

        Some(DqxTranslationsModel { inner_data })
    }
}
