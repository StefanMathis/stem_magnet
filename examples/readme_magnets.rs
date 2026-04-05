use std::{path::PathBuf, sync::Arc};

use cairo_viewport::*;
use planar_geo::prelude::*;
use stem_magnet::prelude::*;

/**
This binary draws the ./docs/img/magnet_types_overview.svg image used in
README.md which provides an overview over all magnet types provided by this
struct.
 */
fn main() {
    let mut core_style = Style::default();
    core_style.background_color = Color {
        r: 0.5,
        g: 0.5,
        b: 0.5,
        a: 1.0,
    };
    core_style.line_color = Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    core_style.line_width = 1.0;

    let dx = 5.0;
    let dy = 2.5;

    let mut drawables: Vec<Drawable> = Vec::new();
    let mut texts: Vec<Text> = Vec::new();

    // Upper row
    {
        let magnet = BlockMagnet::new(
            Length::new::<meter>(1.0),
            Length::new::<meter>(2.6),
            Length::new::<meter>(1.0),
            Length::new::<meter>(0.0),
            Arc::new(Material::default()),
        )
        .unwrap();
        drawables.extend(magnet.drawables(true, false).into_iter().map(|d| {
            let mut d = Drawable::from(d);
            d.line_reflection([0.0, 0.0], [1.0, 0.0]);
            d
        }));

        texts.push(Text::new(
            "Block magnet".into(),
            Anchor::Center,
            [0.0, 0.0],
            [0.0, -1.4],
            Color::new(0.0, 0.0, 0.0, 1.0),
            12.0,
            0.0,
        ));
    }
    {
        let magnet = BlockMagnet::new(
            Length::new::<meter>(1.0),
            Length::new::<meter>(2.6),
            Length::new::<meter>(1.0),
            Length::new::<meter>(0.2),
            Arc::new(Material::default()),
        )
        .unwrap();
        drawables.extend(magnet.drawables(true, false).into_iter().map(|d| {
            let mut d = Drawable::from(d);
            d.translate([dx, 0.0]);
            d.line_reflection([0.0, 0.0], [1.0, 0.0]);
            d
        }));
        texts.push(Text::new(
            "Block magnet with fillets".into(),
            Anchor::Center,
            [0.0, 0.0],
            [dx, -1.4],
            Color::new(0.0, 0.0, 0.0, 1.0),
            12.0,
            0.0,
        ));
    }
    {
        let magnet = BreadLoafMagnet::new(
            Length::new::<meter>(1.0),
            Length::new::<meter>(2.6),
            Length::new::<meter>(1.0),
            Length::new::<meter>(5.0),
            Arc::new(Material::default()),
        )
        .unwrap();
        drawables.extend(magnet.drawables(true, false).into_iter().map(|d| {
            let mut d = Drawable::from(d);
            d.translate([2.0 * dx, 0.0]);
            d.line_reflection([0.0, 0.0], [1.0, 0.0]);
            d
        }));
        texts.push(Text::new(
            "Convex bread loaf magnet".into(),
            Anchor::Center,
            [0.0, 0.0],
            [2.0 * dx, -1.4],
            Color::new(0.0, 0.0, 0.0, 1.0),
            12.0,
            0.0,
        ));
    };
    {
        let magnet = BreadLoafMagnet::new(
            Length::new::<meter>(1.0),
            Length::new::<meter>(2.6),
            Length::new::<meter>(1.0),
            Length::new::<meter>(-5.0),
            Arc::new(Material::default()),
        )
        .unwrap();
        drawables.extend(magnet.drawables(true, false).into_iter().map(|d| {
            let mut d = Drawable::from(d);
            d.translate([3.0 * dx, 0.0]);
            d.line_reflection([0.0, 0.0], [1.0, 0.0]);
            d
        }));
        texts.push(Text::new(
            "Concave bread loaf magnet".into(),
            Anchor::Center,
            [0.0, 0.0],
            [3.0 * dx, -1.4],
            Color::new(0.0, 0.0, 0.0, 1.0),
            12.0,
            0.0,
        ));
    }

    // Lower row
    {
        let magnet = ArcParallelMagnet::with_const_thickness(
            Length::new::<meter>(1.0),
            Length::new::<meter>(5.0),
            SideHeightOrThickness::Thickness(Length::new::<meter>(1.0)),
            0.5.into(),
            Arc::new(Material::default()),
        )
        .unwrap();
        drawables.extend(magnet.drawables(true, false).into_iter().map(|d| {
            let mut d = Drawable::from(d);
            d.line_reflection([0.0, 0.0], [1.0, 0.0]);
            d.translate([0.0, dy]);
            d
        }));
        texts.push(Text::new(
            "Arc parallel magnet".into(),
            Anchor::Center,
            [0.0, 0.0],
            [0.0, 0.7],
            Color::new(0.0, 0.0, 0.0, 1.0),
            12.0,
            0.0,
        ));
        texts.push(Text::new(
            "(constant thickness)".into(),
            Anchor::Center,
            [0.0, 0.0],
            [0.0, 1.0],
            Color::new(0.0, 0.0, 0.0, 1.0),
            12.0,
            0.0,
        ));
    }
    {
        let magnet = ArcSegmentMagnet::new(
            Length::new::<meter>(1.0),
            Length::new::<meter>(5.0),
            Length::new::<meter>(6.0),
            Length::new::<meter>(1.0),
            0.5,
            Arc::new(Material::default()),
        )
        .unwrap();
        drawables.extend(magnet.drawables(true, false).into_iter().map(|d| {
            let mut d = Drawable::from(d);
            d.line_reflection([0.0, 0.0], [1.0, 0.0]);
            d.translate([dx, dy]);
            d
        }));
        texts.push(Text::new(
            "Arc segment magnet".into(),
            Anchor::Center,
            [0.0, 0.0],
            [dx, 0.7],
            Color::new(0.0, 0.0, 0.0, 1.0),
            12.0,
            0.0,
        ));
        texts.push(Text::new(
            "(constant thickness)".into(),
            Anchor::Center,
            [0.0, 0.0],
            [dx, 1.0],
            Color::new(0.0, 0.0, 0.0, 1.0),
            12.0,
            0.0,
        ));
    }
    {
        let magnet = ArcParallelMagnet::with_center_thickness(
            Length::new::<meter>(1.0),
            Length::new::<meter>(5.0),
            Length::new::<meter>(1.0),
            Length::new::<meter>(1.3),
            0.5.into(),
            Arc::new(Material::default()),
        )
        .unwrap();
        drawables.extend(magnet.drawables(true, false).into_iter().map(|d| {
            let mut d = Drawable::from(d);
            d.line_reflection([0.0, 0.0], [1.0, 0.0]);
            d.translate([2.0 * dx, dy]);
            d
        }));
        texts.push(Text::new(
            "Arc parallel magnet".into(),
            Anchor::Center,
            [0.0, 0.0],
            [2.0 * dx, 0.7],
            Color::new(0.0, 0.0, 0.0, 1.0),
            12.0,
            0.0,
        ));
        texts.push(Text::new(
            "(convex)".into(),
            Anchor::Center,
            [0.0, 0.0],
            [2.0 * dx, 1.0],
            Color::new(0.0, 0.0, 0.0, 1.0),
            12.0,
            0.0,
        ));
    };
    {
        let magnet = ArcSegmentMagnet::with_center_thickness(
            Length::new::<meter>(1.0),
            Length::new::<meter>(5.0),
            Length::new::<meter>(1.0),
            Length::new::<meter>(0.5),
            0.5,
            Arc::new(Material::default()),
        )
        .unwrap();
        drawables.extend(magnet.drawables(true, false).into_iter().map(|d| {
            let mut d = Drawable::from(d);
            d.line_reflection([0.0, 0.0], [1.0, 0.0]);
            d.translate([3.0 * dx, dy]);
            d
        }));
        texts.push(Text::new(
            "Arc parallel magnet".into(),
            Anchor::Center,
            [0.0, 0.0],
            [3.0 * dx, 0.7],
            Color::new(0.0, 0.0, 0.0, 1.0),
            12.0,
            0.0,
        ));
        texts.push(Text::new(
            "(concave)".into(),
            Anchor::Center,
            [0.0, 0.0],
            [3.0 * dx, 1.0],
            Color::new(0.0, 0.0, 0.0, 1.0),
            12.0,
            0.0,
        ));
    }

    let mut bb = BoundingBox::from_bounded_entities(drawables.iter()).unwrap();
    bb.scale(1.001);
    let bb = BoundingBox::new(
        bb.xmin() - 0.5,
        bb.xmax() + 0.5,
        bb.ymin() - 0.6,
        bb.ymax() + 0.2,
    );
    let view = Viewport::from_bounding_box(&bb, SideLength::Long(800));

    let fp = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(&format!("docs/img/magnet_types_overview.svg"));
    view.write_to_file(&fp, |cr| {
        cr.set_source_rgb(1.0, 1.0, 1.0);
        cr.paint()?;

        for drawable in drawables {
            drawable.draw(cr)?;
        }
        for text in texts {
            text.draw(cr)?;
        }

        return Ok(());
    })
    .expect("image creation failed");
}
