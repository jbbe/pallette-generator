use core::f32;

use crate::core::{
    color::Rgb,
    color_relation::{ColorRelation, RelationType},
};
use eframe::egui;
use egui::{
    Color32, ColorImage, Mesh, Pos2, Rect, Response, Sense, Shape, Stroke, TextureHandle, UserData,
    Vec2, ViewportCommand, emath,
};

const PALETTE_BUTTON_SIZE: egui::Vec2 = egui::vec2(100., 100.);
pub struct ColorWheelApp {
    color: ColorRelation,
    wheel_texture_id: Option<TextureHandle>,
    control_points: [Pos2; 2],
    relation_type: RelationType,
    // center_point: Pos2,
}

impl Default for ColorWheelApp {
    fn default() -> Self {
        Self {
            color: ColorRelation::default(),
            wheel_texture_id: None,
            control_points: [Pos2 { x: 300., y: 300. }, Pos2 { x: 420., y: 190. }],
            relation_type: RelationType::Complement,
            // center_point: Pos2::default(),
        }
    }
}

impl eframe::App for ColorWheelApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.color.relation_type != self.relation_type {
            self.color.set_relation_type(self.relation_type.clone());
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Wheel");
            self.ui_wheel(ui, ctx);
            self.color_info(ui, ctx);
            paint_it(ui, ctx);
            ui.ctx()
                .send_viewport_cmd(ViewportCommand::Screenshot(UserData::default()));
        });
    }
}

fn get_color_from_angle_and_radius(angle: f32, radius: f32, distance: f32) -> Color32 {
    // Normalize distance to [0, 1]
    let t = distance / radius;

    // Wrap angle to [0, 360) degrees
    let hue = angle.to_degrees() % 360.0;

    // Convert hue to RGB using HSL for a smooth color transition
    let (r, g, b) = hsv_to_rgb(hue, 1.0, 1.0);
    let r = (r * (1.0 - t) + r * t * 0.5) as u8; // Accent color towards center
    let g = (g * (1.0 - t) + g * t * 0.5) as u8;
    let b = (b * (1.0 - t) + b * t * 0.5) as u8;

    Color32::from_rgb(r, g, b)
}

fn hsv_to_rgb(hue: f32, saturation: f32, value: f32) -> (f32, f32, f32) {
    let h = hue / 60.0;
    let c = value * saturation;
    let x = c * (1.0 - ((h % 2.0) - 1.0).abs());
    let m = value - c;

    let (r, g, b) = match h as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        5 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };

    ((r + m) * 255.0, (g + m) * 255.0, (b + m) * 255.0)
}

fn draw_radial_gradient_circle(ui: &mut egui::Ui, ctx: &egui::Context, center: Pos2, radius: f32) {
    // let painter = ctx.painter();
    let (response, painter) = ui.allocate_painter(Vec2::new(400., 400.), Sense::hover());
    let num_points = 360; // Number of points to create the gradient effect

    for y in 0..=(2. * radius) as i32 {
        for x in 0..=(2. * radius) as i32 {
            let pos = Pos2::new(center.x + x as f32 - radius, center.y + y as f32 - radius);
            let distance = ((pos.x - center.x).powi(2) + (pos.y - center.y).powi(2)).sqrt();

            if distance <= radius {
                let angle = (pos.y - center.y).atan2(pos.x - center.x);
                let color = get_color_from_angle_and_radius(angle, radius, distance);
                painter.rect_filled(
                    egui::Rect::from_center_size(pos, egui::vec2(1.0, 1.0)),
                    0.0,
                    color,
                );
            }
        }
    }
}

// In your main paint function
fn paint_it(ui: &mut egui::Ui, ctx: &egui::Context) {
    let center = Pos2::new(300.0, 300.0); // Circle center
    let radius = 200.0; // Circle radius

    draw_radial_gradient_circle(ui, ctx, center, radius);
}

impl ColorWheelApp {
    fn ui_wheel(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) -> Response {
        let (response, painter) = ui.allocate_painter(Vec2::new(400., 400.), Sense::hover());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2 { x: 100., y: 100. }, response.rect.size()),
            response.rect,
        );
        let rect = Rect::from_min_size(Pos2::ZERO, response.rect.size());
        let uv = Rect::from_min_size(Pos2 { x: 0., y: 0. }, Vec2::new(1., 1.));
        let center_x = response.rect.min.x + (response.rect.max.x - response.rect.min.x) / 2.0;
        let center_y = response.rect.min.y + (response.rect.max.y - response.rect.min.y) / 2.0;
        let center_point = Pos2::new(center_x, center_y);
        if let Some(texture_id) = &self.wheel_texture_id {
            let mut mesh = Mesh::with_texture(texture_id.into());
            mesh.add_rect_with_uv(rect, uv, Color32::from_rgb(255, 255, 255));
            ui.painter().add(Shape::mesh(mesh));
        } else {
            let img_path = "src/assets/color_wheel.png";
            let img = image::open(img_path).unwrap(); // Load the image
            let rgba_image = img.to_rgba8(); // Convert to RGBA format
            let (width, height) = rgba_image.dimensions();
            let pixel_data = rgba_image.as_raw(); // Get raw pixel data
            let color_image =
                ColorImage::from_rgba_unmultiplied([width as usize, height as usize], pixel_data);
            self.wheel_texture_id =
                Some(ctx.load_texture("color_wheel", color_image, Default::default()));
        }
        // let center = Pos2::new(200.0, 200.0); // Circle center
        // let radius = 100.0; // Circle radius

        // let num_points = 360; // Define the number of points to sample
        // let step = 2.0 * f32::consts::PI / num_points as f32;

        // // for i in 0..num_points {
        // //     let angle = step * i as f32;
        // //     let x = center.x + radius * angle.cos();
        // //     let y = center.y + radius * angle.sin();
        // //     let pos = Pos2::new(x, y);

        // //     let color = Self::get_color_from_coords(pos, center, radius);

        // //     painter.rect_filled(
        // //         Rect::from_center_size(pos, egui::vec2(2.0, 2.0)),
        // //         0.0,
        // //         Color32::from_rgb(255, 255, 255),
        // //     );
        // //     painter.circle_filled(pos, 2.0, Color32::from_rgb(color[0], color[1], color[2]));
        // // }
        let control_point_radius = 8.0;

        let control_point_shapes: Vec<Shape> = self
            .control_points
            .iter_mut()
            .enumerate()
            // .take(self.degree)
            .map(|(i, point)| {
                let size = if i == 0 {
                    Vec2::splat(3.0 * control_point_radius)
                } else {
                    Vec2::splat(1.0 * control_point_radius)
                };
                let point_in_screen = to_screen.transform_pos(*point);
                let point_rect = Rect::from_center_size(point_in_screen, size);
                let point_id = response.id.with(i);
                if i == 0 {
                    let point_response = ui.interact(point_rect, point_id, Sense::drag());

                    *point += point_response.drag_delta();
                    *point = to_screen.from().clamp(*point);
                    let point_in_screen = to_screen.transform_pos(*point);
                    let stroke_with_interaction = if point_response.hovered() {
                        // Change color on hover
                        Color32::from_rgb(200, 100, 100) // Lighter red on hover
                    } else {
                        Color32::from_rgb(100, 0, 0) // Darker red when not hovered
                    };

                    let stroke = Stroke::new(control_point_radius, stroke_with_interaction);

                    Shape::circle_stroke(point_in_screen, control_point_radius, stroke)
                } else {
                    // Lighter red on hover
                    let stroke =
                        Stroke::new(control_point_radius, Color32::from_rgb(200, 100, 100));

                    Shape::circle_stroke(point_in_screen, control_point_radius, stroke)
                }
            })
            .collect();
        painter.extend(control_point_shapes);

        let point = self.control_points[0];
        let point_in_screen = to_screen.transform_pos(point);

        let picked_color = self.get_pixel_at(ui, point_in_screen.x, point_in_screen.y);
        // let c = Self::get_color_from_coords(point_in_screen, center, response.rect.size().x / 2.);
        if let Some(c) = picked_color {
            self.color.set_color(c);
        }
        response
    }
    fn color_info(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        let rt = &self.relation_type;
        egui::ComboBox::from_label("Take your pick")
            .selected_text(format!("{rt:?}"))
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut self.relation_type,
                    RelationType::Complement,
                    "Complement",
                );
                ui.selectable_value(
                    &mut self.relation_type,
                    RelationType::SplitComplement,
                    "Split Complement",
                );
            });

        for p in self.control_points.iter() {
            let x = p.x;
            let y = p.y;
            ui.label(format!("({x}, {y})"));
        }
        let r = self.color.color[0];
        let g = self.color.color[1];
        let b = self.color.color[2];
        ui.label(format!("Color: r: {r} g: {g} b: {b}"));
        ui.add_sized(
            PALETTE_BUTTON_SIZE,
            egui::Button::new(egui::RichText::new("Color")).fill(self.color.egui_color), // .sense(egui::Sense::click()),
        );
        ui.label("Relation");
        for c in self.color.related_colors.iter() {
            let r = c[0];
            let g = c[1];
            let b = c[2];
            ui.label(format!("Related Color: r: {r} g: {g} b: {b}"));
        }
        for c in self.color.related_egui_colors.iter() {
            ui.add_sized(
                PALETTE_BUTTON_SIZE,
                egui::Button::new(egui::RichText::new("Related")).fill(*c), // .sense(egui::Sense::click()),
            );
        }
    }

    fn get_pixel_at(&mut self, ui: &mut egui::Ui, x: f32, y: f32) -> Option<Rgb<u8>> {
        let image = ui.ctx().input(|i| {
            i.events
                .iter()
                .filter_map(|e| {
                    if let egui::Event::Screenshot { image, .. } = e {
                        Some(image.clone())
                    } else {
                        None
                    }
                })
                .next_back()
        });

        match image {
            Some(img) => {
                let x_u = x as usize;
                let y_u = y as usize;
                // img.pixels()
                // let [width, height] = img.();
                let idx = img.width() * y_u + x_u;

                if x_u < img.width() && y_u < img.height() {
                    let pixel = img.pixels[idx].clone();
                    // return (pixel[0], pixel[1], pixel[2], pixel[3]); // RGBA
                    Some(Rgb([pixel[0], pixel[1], pixel[2]]))
                } else {
                    None
                }
            }
            None => None,
        }
    }
    fn draw_circle(ui: &mut egui::Ui, ctx: &egui::Context, center: Pos2, radius: f32) {
        let (response, painter) = ui.allocate_painter(Vec2::new(400., 400.), Sense::hover());
        let num_points = 360; // Define the number of points to sample
        let step = 2.0 * f32::consts::PI / num_points as f32;

        for i in 0..num_points {
            let angle = step * i as f32;
            let x = center.x + radius * angle.cos();
            let y = center.y + radius * angle.sin();
            let pos = Pos2::new(x, y);

            let color = Self::get_color_from_coords(pos, center, radius);

            painter.rect_filled(
                Rect::from_center_size(pos, egui::vec2(2.0, 2.0)),
                0.0,
                Color32::from_rgb(255, 255, 255),
            );
            painter.circle_filled(pos, 2.0, Color32::from_rgb(color[0], color[1], color[2]));
        }
    }

    fn get_color_from_coords(pos: Pos2, center: Pos2, r: f32) -> Rgb<u8> {
        // let c1 = Rgb([])
        let theta = f32::atan2(pos.y - center.y, pos.x - center.x);
        let theta_deg = f32::to_degrees(theta) % 360.;
        let (c1, c2) = if theta_deg <= 60. {
            (Rgb([255., 0., 0.]), Rgb([255., 255., 0.])) // Red Yellow
        } else if theta_deg <= 120. {
            (Rgb([255., 255., 0.]), Rgb([0., 255., 0.])) // Yellow Green
        } else if theta_deg <= 180. {
            (Rgb([0., 255., 0.]), Rgb([0., 255., 255.])) // Green Cyan
        } else if theta_deg <= 240. {
            (Rgb([0., 255., 255.]), Rgb([255., 0., 0.])) // Cyan Blue
        } else if theta_deg <= 300. {
            (Rgb([255., 0., 0.]), Rgb([255., 0., 255.])) // Blue Magenta
        } else {
            (Rgb([255., 0., 255.]), Rgb([255., 0., 0.])) // Magenta Red
        };

        let d = f32::sqrt((pos.x - center.x).powf(2.) + (pos.y - center.y).powf(2.));
        let t = d / r;
        let r = c1[0] * r + t * (c2[0] - c1[0]);
        let g = c1[1] * r + t * (c2[1] - c1[1]);
        let b = c1[2] * r + t * (c2[2] - c1[2]);

        Rgb([r as u8, g as u8, b as u8])
    }
}
