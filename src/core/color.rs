#![allow(warnings)]

use std::ops::Index;

use egui::epaint::Hsva;
use rand::Rng;
use serde::{Deserialize, Serialize};

// pub struct PRgb {}
#[derive(Deserialize, Serialize, Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub struct Rgb<T>(pub [T; 3]);

impl Index<usize> for Rgb<u8> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

pub struct HSV {
    h: f32,
    s: f32,
    v: f32,
}

impl HSV {
    pub fn from_rgb(c: Rgb<u8>) -> Self {
        let r_n = (c[0] as f32) / 255.;
        let g_n = (c[1] as f32) / 255.;
        let b_n = (c[2] as f32) / 255.;
        let r = c[0];
        let g = c[1];
        let b = c[2];
        // println!("rgb {r_n}, {g_n}, {b_n}, {r}, {g}, {b} ");
        let c_max = f32::max(f32::max(r_n, g_n), b_n);
        let c_min = f32::min(f32::min(r_n, g_n), b_n);
        let delta = c_max - c_min;
        let h = if delta == 0. {
            0.
        } else if r_n == c_max {
            60. * (((g_n - b_n) / delta) % 6.)
        } else if g_n == c_max {
            60. * (((b_n - r_n) / delta) + 2.)
        } else {
            60. * (((r_n - g_n) / delta) + 4.)
        };

        let s = if c_max == 0.0 { 0. } else { delta / c_max };
        Self { h, s, v: c_max }
    }

    pub fn to_rgb(h: f32, s: f32, v: f32) -> Rgb<u8> {
        if s == 0. {
            let color_f = 255. * v;
            let color_u = color_f as u8;
            return Rgb([color_u, color_u, color_u]);
        }

        let c = v * s;
        let x = c * (1. - f32::abs((h / 60.) % 2. - 1.));
        let m = v - c;

        // println!("h c x {h} {c}  {x} ");
        let c_prime = if h >= 0. && h < 60. {
            [c, x, 0.]
        } else if h <= 120. {
            [x, c, 0.]
        } else if h <= 180. {
            [0., c, x]
        } else if h <= 240. {
            [0., x, c]
        } else if h <= 300. {
            [x, 0., c]
        } else if h <= 360. {
            [c, 0., x]
        } else {
            // h should always be less than eq to 360
            [0., 0., 0.]
        };
        let r = (c_prime[0] + m) * 255.;
        let g = (c_prime[1] + m) * 255.;
        let b = (c_prime[2] + m) * 255.;
        Rgb([r as u8, g as u8, b as u8])
    }

    pub fn hsv_to_rgb(hsv: HSV) -> Rgb<u8> {
        Self::to_rgb(hsv.h, hsv.s, hsv.v)
    }

    // fn rgb_prime(h: f32, c: u)
}

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

    pub fn split_complement(c: &HSV) -> (Rgb<u8>, Rgb<u8>) {
        let h = c.h + 180.;
        let h0 = h + 30.;
        let h1 = h - 30.;
        (HSV::to_rgb(h0, c.s, c.v), HSV::to_rgb(h1, c.s, c.v))
    }

    pub fn rgb_to_egui(c: &Rgb<u8>) -> egui::Color32 {
        egui::Color32::from_rgb(c[0], c[1], c[2])
    }

    fn component_diff(c1: Rgb<u8>, c2: Rgb<u8>, component: usize) -> f32 {
        (c1[component] as f32) - (c2[component] as f32)
    }

    pub fn rgb_to_hex(color: Rgb<u8>) -> String {
        format!("#{:02X}{:02X}{:02X}", color[0], color[1], color[2])
    }

    pub fn get_compliment(c: Rgb<u8>) -> Rgb<u8> {
        Rgb([255 - c[0], 255 - c[1], 255 - c[2]])
    }

    pub fn rand_color() -> Rgb<u8> {
        let mut rng = rand::thread_rng();
        let r: u8 = rng.gen_range(0..=255);
        let g: u8 = rng.gen_range(0..=255);
        let b: u8 = rng.gen_range(0..=255);
        Rgb([r, g, b])
    }

    pub fn rand_egui_color() -> egui::Color32 {
        let mut rng = rand::thread_rng();
        let r: u8 = rng.gen_range(0..=255);
        let g: u8 = rng.gen_range(0..=255);
        let b: u8 = rng.gen_range(0..=255);
        egui::Color32::from_rgb(r, g, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn print_hsv(hsv: &HSV) {
        let h = hsv.h;
        let s = hsv.s;
        let v = hsv.v;
        println!("HSV: {h} {s} {v}")
    }

    #[test]
    fn rgb_hsv_teal() {
        // let teal_str = "#64C09A";
        let rgb = Rgb([100, 192, 154]);
        let hsv = HSV::from_rgb(rgb);
        valid_hsv(&hsv);
        let rgb_out = HSV::hsv_to_rgb(hsv);
        assert_eq!(rgb, rgb_out);
    }
    #[test]
    fn rgb_hsv_grey() {
        let rgb = Rgb([136, 136, 136]);
        let h = HSV::from_rgb(rgb);
        valid_hsv(&h);
        let rgb_out = HSV::hsv_to_rgb(h);
        assert_eq!(rgb, rgb_out);
    }

    // #[test]
    // fn rgb_hsv_1() {
    //     rgb_test(Rgb([3, 229, 91]));
    // }
    // #[test]
    // fn rgb_hsv_2() {
    //     rgb_test(Rgb([3, 229, 91]));
    // }
    // #[test]
    // fn rgb_hsv_3() {
    //     rgb_test(Rgb([33, 29, 191]));
    // }
    // #[test]
    // fn rgb_hsv_4() {
    //     rgb_test(Rgb([255, 29, 191]));
    // }
    #[test]
    fn rgb_hsv_5() {
        rgb_test(Rgb([255, 129, 0]));
    }
    #[test]
    fn rgb_hsv_6() {
        rgb_test(Rgb([55, 0, 255]));
    }

    fn rgb_test(rgb: Rgb<u8>) {
        let h = HSV::from_rgb(rgb);
        valid_hsv(&h);
        let rgb_out = HSV::hsv_to_rgb(h);
        assert_eq!(rgb, rgb_out);
    }

    fn valid_hsv(hsv: &HSV) {
        print_hsv(hsv);
        assert!(hsv.h < 360.);
        assert!(hsv.s <= 1.);
        assert!(hsv.v <= 1.);
    }
}
