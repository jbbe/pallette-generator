use crate::core::color::Rgb;
use eframe::egui;
use egui::{
    Color32, ColorImage, Mesh, Pos2, Rect, Response, Sense, Shape, TextureHandle, TextureId, Vec2,
    emath,
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
            self.color_info(ui, ctx)
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
                let stroke = ui.style().interact(&point_response).fg_stroke;

                Shape::circle_stroke(point_in_screen, control_point_radius, stroke)
            })
            .collect();
        painter.extend(control_point_shapes);

        response
    }
    fn color_info(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        for p in self.control_points.iter() {
            let x = p.x;
            let y = p.y;
            ui.label(format!("({x}, {y})"));
        }
    }
}
