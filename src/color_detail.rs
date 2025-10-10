use image::Rgb;

use crate::color::ColorUtil;



pub (crate)struct ColorDetail {
    pub egui_color: egui::Color32,
    pub color: Rgb<u8>,
    pub hex: String,
    pub complement: Rgb<u8>,
    pub complement_hex: String
}

impl ColorDetail {
    pub fn new(c: Rgb<u8>) -> Self {
        let complement = ColorUtil::get_compliment(c);
        Self {
            color: c,
            hex: ColorUtil::rgb_to_hex(c),
            complement,
            complement_hex: ColorUtil::rgb_to_hex(complement),
            egui_color: egui::Color32::from_rgb(c[0], c[1], c[2])
        }
    }

    pub fn default() -> Self {
        let c = ColorUtil::rand_color();
        Self::new(c)
    }
}