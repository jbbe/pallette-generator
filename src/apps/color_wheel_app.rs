use crate::core::color::Rgb;
use eframe::egui;
use egui::{
    Color32, ColorImage, Mesh, Pos2, Rect, Response, Sense, Shape, Stroke, TextureHandle,
    TextureId, UserData, Vec2, ViewportCommand, emath,
};
use image::{DynamicImage, RgbaImage};

pub struct ColorWheelApp {
    color: Rgb<u8>,
    wheel_texture_id: Option<TextureHandle>,
    control_points: [Pos2; 2],
}

impl Default for ColorWheelApp {
    fn default() -> Self {
        Self {
            color: Rgb([1, 1, 1]),
            wheel_texture_id: None,
            control_points: [Pos2 { x: 300., y: 300. }, Pos2 { x: 420., y: 190. }],
        }
    }
}

impl eframe::App for ColorWheelApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Wheel");
            self.ui_wheel(ui, ctx);
            self.color_info(ui, ctx);
            ui.ctx()
                .send_viewport_cmd(ViewportCommand::Screenshot(UserData::default()));
        });
    }
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

        let control_point_radius = 8.0;

        let control_point_shapes: Vec<Shape> = self
            .control_points
            .iter_mut()
            .enumerate()
            // .take(self.degree)
            .map(|(i, point)| {
                let size = Vec2::splat(2.0 * control_point_radius);

                let point_in_screen = to_screen.transform_pos(*point);
                let point_rect = Rect::from_center_size(point_in_screen, size);
                let point_id = response.id.with(i);
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

                // let stroke = ui.style().interact(&point_response).fg_stroke;

                Shape::circle_stroke(point_in_screen, control_point_radius, stroke)
            })
            .collect();
        painter.extend(control_point_shapes);

        let point = self.control_points[0];
        let point_in_screen = to_screen.transform_pos(point);

        let c = self.get_pixel_at(ui, point_in_screen.x, point_in_screen.y);
        if let Some(c1) = c {
            self.color = c1;
        }
        response
    }
    fn color_info(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        for p in self.control_points.iter() {
            let x = p.x;
            let y = p.y;
            ui.label(format!("({x}, {y})"));
        }
        let r = self.color[0];
        let g = self.color[1];
        let b = self.color[2];
        ui.label(format!("Color: r: {r} g: {g} b: {b}"));
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
}
