use crate::COLOR_DICT;
use image::Rgb;
use std::{collections::HashMap, error::Error, fs::File};

pub struct ColorNames {}

impl ColorNames {
    pub fn load() -> Result<HashMap<Rgb<u8>, String>, Box<dyn Error>> {
        println!("Loading color names");
        let file_path = "src/data/colors.csv";
        let file = File::open(file_path)?;
        let mut rdr = csv::Reader::from_reader(file);
        let mut map = HashMap::new();
        for result in rdr.records() {
            let record = result?;

            let _color_name = &record[0];
            let color_name_pretty = &record[1];
            let _hex = &record[2];
            let r: u8 = record[3].parse()?;
            let g: u8 = record[4].parse()?;
            let b: u8 = record[5].parse()?;

            let c = Rgb([r, g, b]);
            map.entry(c).or_insert(color_name_pretty.to_owned());
            // println!("{:?}", record);
        }
        Ok(map)
    }

    pub fn get_color_name(color: &Rgb<u8>) -> Option<String> {
        // let r = color[0];
        // let g = color[1];
        // let b = color[2];
        // println!("try get color name {r} {g} {b}");
        let lock = COLOR_DICT.lock().unwrap();
        let binding = String::default();
        let reference = lock.get(&color).unwrap_or(&binding); //.unwrap_or_default();
        // println!("color name {reference}");
        Some(reference.to_string())
    }
}
