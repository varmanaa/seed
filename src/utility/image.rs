use std::{cmp, env::current_dir, fs};

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
    Typeface,
};
use twilight_model::{
    http::attachment::Attachment,
    id::{marker::GuildMarker, Id},
};

use crate::utility::{constants::FLUCTUATING_XP, decimal::abbreviate};

pub fn get_profile(
    guild_id: Id<GuildMarker>,
    avatar_image: Image,
    username: String,
    rank: usize,
    xp: i64,
) -> Attachment {
    let mut surface = raster_n32_premul((875i32, 250i32)).unwrap();

    surface.canvas().save();

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

    surface.canvas().draw_image(background_image, (0, 0), None);

    surface.canvas().restore();

    let mut translucent_rect = Path::new();

    translucent_rect
        .move_to((70, 20))
        .line_to((805, 20))
        .quad_to((855, 20), (855, 70))
        .line_to((855, 180))
        .quad_to((855, 230), (805, 230))
        .line_to((70, 230))
        .quad_to((20, 230), (20, 180))
        .line_to((20, 70))
        .quad_to((20, 20), (70, 20))
        .close();

    surface.canvas().draw_path(
        &translucent_rect,
        Paint::default().set_style(PaintStyle::Fill).set_alpha(128),
    );

    surface.canvas().save();

    surface.canvas().draw_circle(
        (125, 125),
        75.0,
        &Paint::default()
            .set_style(PaintStyle::Fill)
            .set_color(0xF8F8FFFF),
    );

    surface
        .canvas()
        .clip_path(
            Path::new().add_circle((125, 125), 70.0, PathDirection::CCW),
            Some(ClipOp::Intersect),
            Some(true),
        )
        .draw_image_rect(
            avatar_image,
            None,
            Rect {
                bottom: 195.0,
                left: 55.0,
                right: 195.0,
                top: 55.0,
            },
            Paint::default().set_style(PaintStyle::Fill),
        );

    surface.canvas().restore();

    let typeface_bytes = fs::read(format!(
        "{}/assets/fonts/SourceSans3-SemiBold.ttf",
        cwd.to_string_lossy()
    ))
    .unwrap();
    let typeface_data = Data::new_copy(&typeface_bytes);
    let mut source_sans_3 = Font::new(Typeface::from_data(typeface_data, None).unwrap(), None);
    let (level_text, progress_text, progress_percentage) =
        match FLUCTUATING_XP.iter().position(|&level| xp.lt(&level.1)) {
            None => ("Lv. 100".to_owned(), "MAX LEVEL".to_owned(), 100.0),
            Some(position) => {
                let index = cmp::max(position - 1, 0);
                let (level, total_xp, xp_to_next_level) = FLUCTUATING_XP[index];

                (
                    format!("Lv. {level}"),
                    format!(
                        "{} / {}",
                        abbreviate(xp - total_xp),
                        abbreviate(xp_to_next_level)
                    ),
                    (100.0 * ((xp - total_xp) as f32)) / (xp_to_next_level as f32),
                )
            }
        };

    surface.canvas().draw_str_align(
        username,
        Point::new(520.0, 87.5),
        &source_sans_3.set_size(40.0),
        &Paint::default()
            .set_style(PaintStyle::StrokeAndFill)
            .set_argb(255, 248, 248, 255),
        Align::Center,
    );

    surface.canvas().draw_str_align(
        format!("Rank #{rank} ({level_text})"),
        Point::new(270.0, 140.0),
        &source_sans_3.set_size(20.0),
        &Paint::default()
            .set_style(PaintStyle::StrokeAndFill)
            .set_argb(255, 248, 248, 255),
        Align::Left,
    );

    surface.canvas().draw_str_align(
        progress_text,
        Point::new(770.0, 140.0),
        &source_sans_3.set_size(20.0),
        &Paint::default()
            .set_style(PaintStyle::StrokeAndFill)
            .set_argb(255, 248, 248, 255),
        Align::Right,
    );

    surface.canvas().save();

    let mut outer_progress_bar = Path::new();

    outer_progress_bar
        .move_to((280, 150))
        .line_to((760, 150))
        .quad_to((775, 150), (775, 165))
        .quad_to((775, 180), (760, 180))
        .line_to((280, 180))
        .quad_to((265, 180), (265, 165))
        .quad_to((265, 150), (280, 150))
        .close();

    surface.canvas().draw_path(
        &outer_progress_bar,
        Paint::default()
            .set_style(PaintStyle::Fill)
            .set_argb(255, 248, 248, 255),
    );

    surface.canvas().restore();

    let mut left_inner_progress_bar = Path::new();
    let mut right_inner_progress_bar = Path::new();
    let displacement = 500.0 - (progress_percentage * 5.0);

    left_inner_progress_bar
        .move_to((280, 155))
        .line_to((760, 155))
        .quad_to((770, 155), (770, 165))
        .quad_to((770, 175), (760, 175))
        .line_to((280, 175))
        .quad_to((270, 175), (270, 165))
        .quad_to((270, 155), (280, 155))
        .close();

    right_inner_progress_bar
        .move_to((280.0 - displacement, 155.0))
        .line_to((760.0 - displacement, 155.0))
        .quad_to((770.0 - displacement, 155.0), (770.0 - displacement, 165.0))
        .quad_to((770.0 - displacement, 175.0), (760.0 - displacement, 175.0))
        .line_to((280.0 - displacement, 175.0))
        .quad_to((270.0 - displacement, 175.0), (270.0 - displacement, 165.0))
        .quad_to((270.0 - displacement, 155.0), (280.0 - displacement, 155.0))
        .close();

    surface
        .canvas()
        .clip_path(
            &left_inner_progress_bar,
            Some(ClipOp::Intersect),
            Some(true),
        )
        .clip_path(
            &right_inner_progress_bar,
            Some(ClipOp::Intersect),
            Some(true),
        )
        .draw_paint(
            Paint::default()
                .set_style(PaintStyle::StrokeAndFill)
                .set_argb(255, 201, 173, 127),
        );

    let bytes = surface
        .image_snapshot()
        .encode(None, EncodedImageFormat::PNG, None)
        .unwrap()
        .as_bytes()
        .to_owned();

    Attachment::from_bytes("profile.png".to_owned(), bytes, 1)
}
