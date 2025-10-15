use crate::core::color::{ColorUtil, HSV, Rgb};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum RelationType {
    Complement,
    SplitComplement,
    // Analagous,
}

pub(crate) struct ColorRelation {
    pub color: Rgb<u8>,
    pub egui_color: egui::Color32,
    pub relation_type: RelationType,
    pub related_colors: Vec<Rgb<u8>>,
    pub related_egui_colors: Vec<egui::Color32>,
}

impl ColorRelation {
    pub fn default() -> Self {
        let color = ColorUtil::rand_color();
        let related_colors = Self::get_related_colors(color, RelationType::Complement);
        Self {
            color,
            egui_color: ColorUtil::egui_from_rgb(color),
            relation_type: RelationType::Complement,
            related_colors: related_colors.0,
            related_egui_colors: related_colors.1,
        }
    }

    pub fn set_color(&mut self, c: Rgb<u8>) {
        if self.color != c {
            self.color = c;
            self.egui_color = egui::Color32::from_rgb(c[0], c[1], c[2]);
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
        color: Rgb<u8>,
        relation_type: RelationType,
    ) -> (Vec<Rgb<u8>>, Vec<egui::Color32>) {
        match relation_type {
            RelationType::Complement => {
                let c = ColorUtil::get_compliment(color);
                (vec![c], vec![ColorUtil::egui_from_rgb(c)])
            }
            RelationType::SplitComplement => {
                let color_hsv = HSV::from_rgb(color);
                let sp = ColorUtil::split_complement(&color_hsv);
                (
                    vec![sp.0, sp.1],
                    vec![
                        ColorUtil::egui_from_rgb(sp.0),
                        ColorUtil::egui_from_rgb(sp.1),
                    ],
                )
            }
        }
    }
}
