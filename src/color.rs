#![allow(warnings)]

use image::Rgb;


pub (crate)struct ColorUtil {}

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

}