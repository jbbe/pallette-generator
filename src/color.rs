#![allow(warnings)]

use image::Rgb;
use rand::Rng;

pub(crate) struct ColorUtil {}

impl ColorUtil {
    pub fn color_distance(c1: Rgb<u8>, c2: Rgb<u8>) -> f32 {
        let ap_r = 0.5 * (c1[0] as f32 + c2[0] as f32);
        let dr = Self::component_diff(c1, c2, 0);
        let dg = Self::component_diff(c1, c2, 1);
        let db = Self::component_diff(c1, c2, 2);

        let dc_sq = (2. + (ap_r / 256.)) * (dr * dr)
            + 4. * (dg * dg)
            + (2. + ((256. - ap_r) / 256.)) * (db * db);

        f32::sqrt(dc_sq)
    }

    fn component_diff(c1: Rgb<u8>, c2: Rgb<u8>, component: usize) -> f32 {
        (c1[component] as f32) - (c2[component] as f32)
    }

    pub fn rgb_to_hex(color: Rgb<u8>) -> String {
        format!("#{:02X}{:02X}{:02X}", color[0], color[1], color[2])
    }

    pub fn get_compliment(c: Rgb<u8>) -> Rgb<u8> {
        Rgb([255- c[0], 255 - c[1], 255 - c[2]])
    }

    pub fn rand_color() -> Rgb<u8> {
        let mut rng = rand::thread_rng();
        let r: u8 = rng.gen_range(0..=255);
        let g: u8 = rng.gen_range(0..=255);
        let b: u8 = rng.gen_range(0..=255);
        Rgb([r,g,b])
    }

    pub fn rand_egui_color() -> egui::Color32 {
        let mut rng = rand::thread_rng();
        let r: u8 = rng.gen_range(0..=255);
        let g: u8 = rng.gen_range(0..=255);
        let b: u8 = rng.gen_range(0..=255);
        egui::Color32::from_rgb(r, g, b)
    }
}
