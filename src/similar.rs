use image::Rgb;

use crate::pallette::Pallette;

// enum SimilarType {
//     Distance,
//     // Complementary,
// }

pub(crate) struct Similar {
    pub color: Rgb<u8>,
    pub similar_colors: Vec<(Rgb<u8>, usize, f32)>,
    // pub sim_type: SimilarType,
}

impl Similar {
    pub fn new_similar(
        c: Rgb<u8>,
        all_colors: &Vec<(Rgb<u8>, usize)>,
        count: usize,
        similar_threshold: f32,
    ) -> Self {
        let mut similar_colors: Vec<(Rgb<u8>, usize, f32)> = Vec::new();
        for color in all_colors {
            let d = Pallette::color_distance(c, color.0);
            if d < similar_threshold {
                similar_colors.push((color.0, color.1, d));
            }
        }
        similar_colors.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        Self {
            // sim_type: SimilarType::Distance,
            color: c,
            similar_colors: similar_colors.into_iter().take(count).collect(),
        }
    }

    // pub fn new_complementary(c: Rgb<u8>) -> Self {
    //     let complementary_colors: Vec<(Rgb<u8>, usize, f32)> = Vec::new();

    //     Self {
    //         sim_type: SimilarType::Complementary,
    //         color: c,
    //         similar_colors: complementary_colors,
    //     }
    // }
}
