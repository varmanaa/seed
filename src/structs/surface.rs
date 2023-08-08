use std::{env::current_dir, f32, fs};

use skia_safe::{
    surfaces::raster_n32_premul,
    utils::text_utils::Align,
    ClipOp,
    Data,
    EncodedImageFormat,
    Font,
    Image,
    Paint,
    PaintStyle,
    Path,
    PathDirection,
    Point,
    Rect,
};
use twilight_model::id::{marker::GuildMarker, Id};

use crate::types::surface::Surface;

impl Surface {
    pub fn draw_avatar(
        &mut self,
        center: Point,
        radius: f32,
        avatar_image: Image,
    ) {
        self.surface.canvas().draw_circle(
            center,
            radius,
            &Paint::default()
                .set_style(PaintStyle::Fill)
                .set_color(0xF8F8FFFF),
        );

        self.surface
            .canvas()
            .clip_path(
                Path::new().add_circle(center, radius - 5.0, PathDirection::CCW),
                Some(ClipOp::Intersect),
                Some(true),
            )
            .draw_image_rect(
                avatar_image,
                None,
                Rect {
                    bottom: center.y + radius - 5.0,
                    left: center.x - radius + 5.0,
                    right: center.x + radius - 5.0,
                    top: center.y - radius + 5.0,
                },
                Paint::default().set_style(PaintStyle::Fill),
            );
    }

    pub fn draw_progress_bar(
        &mut self,
        rect: Rect,
        progress: f32,
        radius: f32,
        paint: &Paint,
    ) {
        let mut path_one = Path::new();
        let mut path_two = Path::new();
        let displacement = 500.0 - progress;

        path_one
            .move_to((rect.left + radius, rect.top))
            .line_to((rect.right - radius, rect.top))
            .quad_to((rect.right, rect.top), (rect.right, rect.top + radius))
            .line_to((rect.right, rect.bottom - radius))
            .quad_to(
                (rect.right, rect.bottom),
                (rect.right - radius, rect.bottom),
            )
            .line_to((rect.left + radius, rect.bottom))
            .quad_to((rect.left, rect.bottom), (rect.left, rect.bottom - radius))
            .line_to((rect.left, rect.top + radius))
            .quad_to((rect.left, rect.top), (rect.left + radius, rect.top))
            .close();

        path_two
            .move_to((rect.left + radius - displacement, rect.top))
            .line_to((rect.right - radius - displacement, rect.top))
            .quad_to(
                (rect.right - displacement, rect.top),
                (rect.right - displacement, rect.top + radius),
            )
            .line_to((rect.right - displacement, rect.bottom - radius))
            .quad_to(
                (rect.right - displacement, rect.bottom),
                (rect.right - radius - displacement, rect.bottom),
            )
            .line_to((rect.left + radius - displacement, rect.bottom))
            .quad_to(
                (rect.left - displacement, rect.bottom),
                (rect.left - displacement, rect.bottom - radius),
            )
            .line_to((rect.left - displacement, rect.top + radius))
            .quad_to(
                (rect.left - displacement, rect.top),
                (rect.left + radius - displacement, rect.top),
            )
            .close();

        self.surface
            .canvas()
            .clip_path(&path_one, Some(ClipOp::Intersect), Some(true))
            .clip_path(&path_two, Some(ClipOp::Intersect), Some(true))
            .draw_paint(paint);
    }

    pub fn draw_round_rect(
        &mut self,
        rect: Rect,
        radius: f32,
        paint: &Paint,
    ) {
        let mut path = Path::new();

        path.move_to((rect.left + radius, rect.top))
            .line_to((rect.right - radius, rect.top))
            .quad_to((rect.right, rect.top), (rect.right, rect.top + radius))
            .line_to((rect.right, rect.bottom - radius))
            .quad_to(
                (rect.right, rect.bottom),
                (rect.right - radius, rect.bottom),
            )
            .line_to((rect.left + radius, rect.bottom))
            .quad_to((rect.left, rect.bottom), (rect.left, rect.bottom - radius))
            .line_to((rect.left, rect.top + radius))
            .quad_to((rect.left, rect.top), (rect.left + radius, rect.top))
            .close();

        self.surface.canvas().draw_path(&path, paint);
    }

    pub fn draw_text(
        &mut self,
        text: String,
        point: Point,
        font: &Font,
        paint: &Paint,
        align: Align,
    ) {
        self.surface
            .canvas()
            .draw_str_align(text, point, font, paint, align);
    }

    pub fn new() -> Self {
        Self {
            surface: raster_n32_premul((875i32, 250i32)).unwrap(),
        }
    }

    pub fn png_bytes(&mut self) -> Vec<u8> {
        self.surface
            .image_snapshot()
            .encode(None, EncodedImageFormat::PNG, None)
            .unwrap()
            .as_bytes()
            .to_owned()
    }

    pub fn restore(&mut self) {
        self.surface.canvas().restore();
    }

    pub fn save(&mut self) {
        self.surface.canvas().save();
    }

    pub fn set_background(
        &mut self,
        guild_id: Id<GuildMarker>,
    ) {
        let cwd = current_dir().unwrap();
        let background_image_path = format!(
            "{}/assets/images/{}.png",
            cwd.to_string_lossy(),
            guild_id.get()
        );
        let background_image_bytes = fs::read(background_image_path).map_or(
            fs::read(format!(
                "{}/assets/images/default.png",
                cwd.to_string_lossy()
            ))
            .unwrap(),
            |bytes| bytes,
        );
        let background_image_data = Data::new_copy(&background_image_bytes);
        let background_image = Image::from_encoded(background_image_data).unwrap();

        self.surface
            .canvas()
            .draw_image(background_image, (0, 0), None);
    }
}
