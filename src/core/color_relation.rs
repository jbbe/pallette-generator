use serde::{Deserialize, Serialize};

use crate::core::color::{ColorUtil, HSV, Rgb};

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub enum RelationType {
    Complement,
    SplitComplement,
    // Analagous,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct ColorRelation {
    pub color: Rgb<f32>,
    pub selector_color: [f32; 3],
    pub egui_color: egui::Color32,
    pub relation_type: RelationType,
    pub related_colors: Vec<Rgb<f32>>,
    pub related_egui_colors: Vec<egui::Color32>,
}

impl ColorRelation {
    pub fn default() -> Self {
        let color = ColorUtil::rand_color_f();
        let related_colors = Self::get_related_colors(color, RelationType::Complement);
        Self {
            color,
            selector_color: [color[0], color[1], color[2]],
            egui_color: ColorUtil::egui_from_rgb_f(color),
            relation_type: RelationType::Complement,
            related_colors: related_colors.0,
            related_egui_colors: related_colors.1,
        }
    }

    pub fn set_color(&mut self, c: Rgb<f32>) {
        if self.color != c {
            self.color = c;
            self.egui_color = egui::Color32::from_rgb(c[0] as u8, c[1] as u8, c[2] as u8);
            let res = Self::get_related_colors(c, self.relation_type.clone());
            self.related_colors = res.0;
            self.related_egui_colors = res.1;
        }
    }

    pub fn set_relation_type(&mut self, relation_type: RelationType) {
        self.relation_type = relation_type;
        let res = Self::get_related_colors(self.color, self.relation_type.clone());
        self.related_colors = res.0;
        self.related_egui_colors = res.1;
    }

    pub fn get_related_colors(
        color: Rgb<f32>,
        relation_type: RelationType,
    ) -> (Vec<Rgb<f32>>, Vec<egui::Color32>) {
        match relation_type {
            RelationType::Complement => {
                let c = ColorUtil::get_compliment_F(color);
                (vec![c], vec![ColorUtil::egui_from_rgb_f(c)])
            }
            RelationType::SplitComplement => {
                let color_hsv = HSV::from_rgb_f(color);
                let sp = ColorUtil::split_complement_f(&color_hsv);
                (
                    vec![sp.0, sp.1],
                    vec![
                        ColorUtil::egui_from_rgb_f(sp.0),
                        ColorUtil::egui_from_rgb_f(sp.1),
                    ],
                )
            }
        }
    }

    pub fn get_related_colors_f(
        color: Rgb<f32>,
        relation_type: RelationType,
    ) -> (Vec<Rgb<f32>>, Vec<egui::Color32>) {
        match relation_type {
            RelationType::Complement => {
                let c = ColorUtil::get_compliment_F(color);
                (vec![c], vec![ColorUtil::egui_from_rgb_f(c)])
            }
            RelationType::SplitComplement => {
                let color_hsv = HSV::from_rgb_f(color);
                let sp = ColorUtil::split_complement_f(&color_hsv);
                (
                    vec![sp.0, sp.1],
                    vec![
                        ColorUtil::egui_from_rgb_f(sp.0),
                        ColorUtil::egui_from_rgb_f(sp.1),
                    ],
                )
            }
        }
    }
}
