use image::{Rgb};

use crate::pallette::Pallette;
pub(crate) struct Similar {
    pub color: Rgb<u8>,
    pub similar_colors: Vec<(Rgb<u8>,  usize, f32)>
}

impl Similar {
    pub fn new(c: Rgb<u8>, all_colors: &Vec<(Rgb<u8>, usize)>) -> Self {
        let mut similar_colors = Vec::<(Rgb<u8>, usize, f32)>::new();
        similar_colors.reserve(all_colors.len());
        for color in all_colors {
            let d = Pallette::color_distance(c, color.0);
            if d < 100. {
                similar_colors.push((color.0, color.1, d));
            }
        }

        similar_colors.sort_by(|a, b| {
            b.2.partial_cmp(&a.2).unwrap()
        } );

        Self {
            color: c,
            similar_colors
        }
    }
}