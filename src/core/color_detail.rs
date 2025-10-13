use image::Rgb;

use crate::core::color::{ColorUtil, HSV};

pub (crate)struct ColorDetail {
    pub egui_color: egui::Color32,
    pub color: Rgb<u8>,
    pub color_hsv: HSV,
    pub hex: String,
    pub compliment_egui: egui::Color32,
    pub complement: Rgb<u8>,
    pub complement_hex: String
}

impl ColorDetail {
    pub fn new(c: Rgb<u8>) -> Self {
        let complement = ColorUtil::get_compliment(c);
        Self {
            color: c,
            hex: ColorUtil::rgb_to_hex(c),
            color_hsv: HSV::from_rgb(c),
            complement,
            complement_hex: ColorUtil::rgb_to_hex(complement),
            egui_color: egui::Color32::from_rgb(c[0], c[1], c[2]),
            compliment_egui: egui::Color32::from_rgb(complement[0], complement[1], complement[2])
        }
    }

    pub fn default() -> Self {
        let c = ColorUtil::rand_color();
        Self::new(c)
    }

    pub fn update_from_egui_color(&mut self, update_complement: bool) {
        if self.egui_color.r() != self.color[0] ||
            self.egui_color.g() != self.color[1] ||
            self.egui_color.b() != self.color[2]
         {
            self.color = Rgb([self.egui_color.r(), self.egui_color.g(), self.egui_color.b()]);
            self.hex = ColorUtil::rgb_to_hex(self.color);
            if update_complement {
                let complement = ColorUtil::get_compliment(self.color);
                self.complement_hex = ColorUtil::rgb_to_hex(complement);
                self.compliment_egui = egui::Color32::from_rgb(complement[0], complement[1], complement[2]);
                self.complement = complement;

                self.color_hsv = HSV::from_rgb(self.color);

            }
        }
    }
}