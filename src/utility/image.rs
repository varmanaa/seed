use std::{cmp, env::current_dir, fs};

use skia_safe::{
    utils::text_utils::Align,
    Data,
    Font,
    Image,
    Paint,
    PaintStyle,
    Point,
    Rect,
    Typeface,
};
use twilight_model::{
    http::attachment::Attachment,
    id::{marker::GuildMarker, Id},
};

use crate::{
    types::surface::Surface,
    utility::{constants::FLUCTUATING_XP, decimal::abbreviate},
};

pub fn get_profile(
    guild_id: Id<GuildMarker>,
    avatar_image: Image,
    username: String,
    rank: u64,
    xp: i64,
) -> Attachment {
    let mut surface = Surface::new();

    surface.save();

    surface.set_background(guild_id);

    surface.restore();

    surface.draw_round_rect(
        Rect {
            bottom: 230.0,
            left: 20.0,
            right: 855.0,
            top: 20.0,
        },
        50.0,
        Paint::default().set_style(PaintStyle::Fill).set_alpha(128),
    );

    surface.save();

    surface.draw_avatar(Point::new(125.0, 125.0), 75.0, avatar_image);

    surface.restore();

    let cwd = current_dir().unwrap();
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

    surface.draw_text(
        username,
        Point::new(520.0, 87.5),
        &source_sans_3.set_size(40.0),
        &Paint::default()
            .set_style(PaintStyle::StrokeAndFill)
            .set_argb(255, 248, 248, 255),
        Align::Center,
    );

    surface.draw_text(
        format!("Rank #{rank} ({level_text})"),
        Point::new(270.0, 140.0),
        &source_sans_3.set_size(20.0),
        &Paint::default()
            .set_style(PaintStyle::StrokeAndFill)
            .set_argb(255, 248, 248, 255),
        Align::Left,
    );

    surface.draw_text(
        progress_text,
        Point::new(770.0, 140.0),
        &source_sans_3.set_size(20.0),
        &Paint::default()
            .set_style(PaintStyle::StrokeAndFill)
            .set_argb(255, 248, 248, 255),
        Align::Right,
    );

    surface.save();

    surface.draw_round_rect(
        Rect {
            bottom: 180.0,
            left: 265.0,
            right: 775.0,
            top: 150.0,
        },
        15.0,
        Paint::default()
            .set_style(PaintStyle::Fill)
            .set_argb(255, 248, 248, 255),
    );

    surface.restore();

    let progress: f32 = progress_percentage * 5.0;

    let mut paint = Paint::default();

    paint
        .set_style(PaintStyle::StrokeAndFill)
        .set_argb(255, 201, 173, 127);

    surface.draw_progress_bar(
        Rect {
            bottom: 175.0,
            left: 270.0,
            right: 770.0,
            top: 155.0,
        },
        progress,
        10.0,
        &paint,
    );

    Attachment::from_bytes("profile.png".to_owned(), surface.png_bytes(), 1)
}
