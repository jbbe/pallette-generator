#![allow(warnings)]


#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct PColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

// fn color_dist(a: Rgb<u8>, b: Rgb<u8>) -> u32 {

// }

impl PColor {
    pub fn from_string(color: String) -> Self {
        let r_in = u8::from_str_radix(&color[1..3], 16).unwrap();
        let g_in = u8::from_str_radix(&color[3..5], 16).unwrap();
        let b_in = u8::from_str_radix(&color[5..7], 16).unwrap();
        // println!("color {r_in}, {g_in}, {b_in}");
        return Self {
            r: r_in,
            g: g_in,
            b: b_in,
            // frequency: 0,
        };
    }
    pub fn new(r_in: u8, g_in: u8, b_in: u8) -> Self {
        Self {
            r: r_in,
            g: g_in,
            b: b_in,
            // frequency: 0,
        }
    }
}

// impl Eq for PColor {
//     fn eq(&self, other: &Self) -> bool {
//         self.r == other.r && self.g == other.g && self.b == other.b
//     }
// }

// fn rgb_eq(p_color: &PColor, rgb: &Rgb<u8>) -> bool {
//     p_color.r == rgb[0] && p_color.g == rgb[1] && p_color.b == rgb[2]
// }

// fn rgb_from_str(color: &str) -> Rgb<u8> {
//     let r_in = u8::from_str_radix(&color[1..3], 16).unwrap();
//     let g_in = u8::from_str_radix(&color[3..5], 16).unwrap();
//     let b_in = u8::from_str_radix(&color[5..7], 16).unwrap();
//     Rgb([r_in, g_in, b_in])
// }
