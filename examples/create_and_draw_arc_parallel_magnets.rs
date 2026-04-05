use std::{path::PathBuf, sync::Arc};

use cairo_viewport::*;
use planar_geo::prelude::*;
use stem_magnet::prelude::*;

/**
This binary draws the ./docs/img/arc_segment_vary_air_gap_radius.svg and
./docs/img/arc_segment_vary_center_thickness.svg images used in
the documentation of the [`ArcParallelMagnet`] struct.
 */
fn main() {
    air_gap_radius();
    center_thickness();
}

fn air_gap_radius() {
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

    texts.push(Text::new(
        "Shared parameters: core_radius = 5 m, side_height = 1 m, width = 2.6 m".into(),
        Anchor::Center,
        [0.0, 0.0],
        [1.5 * dx, -2.0],
        Color::new(0.0, 0.0, 0.0, 1.0),
        16.0,
        0.0,
    ));
    texts.push(Text::new(
        "Shared parameters: core_radius = -6 m, side_height = 1 m, width = 2.6 m".into(),
        Anchor::Center,
        [0.0, 0.0],
        [1.5 * dx, 1.5],
        Color::new(0.0, 0.0, 0.0, 1.0),
        16.0,
        0.0,
    ));

    // Inner magnets
    {
        let magnet = ArcParallelMagnet::new(
            Length::new::<meter>(1.0),
            Length::new::<meter>(5.0),
            Length::new::<meter>(6.0),
            Length::new::<meter>(1.0),
            AngleOrWidth::Width(Length::new::<meter>(2.6)),
            Arc::new(Material::default()),
        )
        .unwrap();
        drawables.extend(magnet.drawables(true, false).into_iter().map(|d| {
            let mut d = Drawable::from(d);
            d.line_reflection([0.0, 0.0], [1.0, 0.0]);
            d
        }));

        texts.push(Text::new(
            "air_gap_radius = 6 m".into(),
            Anchor::Center,
            [0.0, 0.0],
            [0.0, -1.4],
            Color::new(0.0, 0.0, 0.0, 1.0),
            12.0,
            0.0,
        ));
    }
    {
        let magnet = ArcParallelMagnet::new(
            Length::new::<meter>(1.0),
            Length::new::<meter>(5.0),
            Length::new::<meter>(3.0),
            Length::new::<meter>(1.0),
            AngleOrWidth::Width(Length::new::<meter>(2.6)),
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
            "air_gap_radius = 3 m".into(),
            Anchor::Center,
            [0.0, 0.0],
            [dx, -1.4],
            Color::new(0.0, 0.0, 0.0, 1.0),
            12.0,
            0.0,
        ));
    }
    {
        let magnet = ArcParallelMagnet::new(
            Length::new::<meter>(1.0),
            Length::new::<meter>(5.0),
            Length::new::<meter>(100.0),
            Length::new::<meter>(1.0),
            AngleOrWidth::Width(Length::new::<meter>(2.6)),
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
            "air_gap_radius = 100 m".into(),
            Anchor::Center,
            [0.0, 0.0],
            [2.0 * dx, -1.4],
            Color::new(0.0, 0.0, 0.0, 1.0),
            12.0,
            0.0,
        ));
    };
    {
        let magnet = ArcParallelMagnet::new(
            Length::new::<meter>(1.0),
            Length::new::<meter>(5.0),
            Length::new::<meter>(-4.0),
            Length::new::<meter>(1.0),
            AngleOrWidth::Width(Length::new::<meter>(2.6)),
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
            "air_gap_radius = -4 m".into(),
            Anchor::Center,
            [0.0, 0.0],
            [3.0 * dx, -1.4],
            Color::new(0.0, 0.0, 0.0, 1.0),
            12.0,
            0.0,
        ));
    }

    // Outer magnets
    {
        let magnet = ArcParallelMagnet::new(
            Length::new::<meter>(1.0),
            Length::new::<meter>(-6.0),
            Length::new::<meter>(-5.0),
            Length::new::<meter>(1.0),
            AngleOrWidth::Width(Length::new::<meter>(2.6)),
            Arc::new(Material::default()),
        )
        .unwrap();
        drawables.extend(magnet.drawables(true, false).into_iter().map(|d| {
            let mut d = Drawable::from(d);
            d.translate([0.0, dy]);
            d
        }));
        texts.push(Text::new(
            "air_gap_radius = -5 m".into(),
            Anchor::Center,
            [0.0, 0.0],
            [0.0, 2.1],
            Color::new(0.0, 0.0, 0.0, 1.0),
            12.0,
            0.0,
        ));
    }
    {
        let magnet = ArcParallelMagnet::new(
            Length::new::<meter>(1.0),
            Length::new::<meter>(-6.0),
            Length::new::<meter>(-2.0),
            Length::new::<meter>(1.0),
            AngleOrWidth::Width(Length::new::<meter>(2.6)),
            Arc::new(Material::default()),
        )
        .unwrap();
        drawables.extend(magnet.drawables(true, false).into_iter().map(|d| {
            let mut d = Drawable::from(d);
            d.translate([dx, dy]);
            d
        }));
        texts.push(Text::new(
            "air_gap_radius = -2 m".into(),
            Anchor::Center,
            [0.0, 0.0],
            [dx, 2.1],
            Color::new(0.0, 0.0, 0.0, 1.0),
            12.0,
            0.0,
        ));
    }
    {
        let magnet = ArcParallelMagnet::new(
            Length::new::<meter>(1.0),
            Length::new::<meter>(-6.0),
            Length::new::<meter>(-100.0),
            Length::new::<meter>(1.0),
            AngleOrWidth::Width(Length::new::<meter>(2.6)),
            Arc::new(Material::default()),
        )
        .unwrap();
        drawables.extend(magnet.drawables(true, false).into_iter().map(|d| {
            let mut d = Drawable::from(d);
            d.translate([2.0 * dx, dy]);
            d
        }));
        texts.push(Text::new(
            "air_gap_radius = -100 m".into(),
            Anchor::Center,
            [0.0, 0.0],
            [2.0 * dx, 2.1],
            Color::new(0.0, 0.0, 0.0, 1.0),
            12.0,
            0.0,
        ));
    };
    {
        let magnet = ArcParallelMagnet::new(
            Length::new::<meter>(1.0),
            Length::new::<meter>(-6.0),
            Length::new::<meter>(4.0),
            Length::new::<meter>(1.0),
            AngleOrWidth::Width(Length::new::<meter>(2.6)),
            Arc::new(Material::default()),
        )
        .unwrap();
        drawables.extend(magnet.drawables(true, false).into_iter().map(|d| {
            let mut d = Drawable::from(d);
            d.translate([3.0 * dx, dy]);
            d
        }));
        texts.push(Text::new(
            "air_gap_radius = 4 m".into(),
            Anchor::Center,
            [0.0, 0.0],
            [3.0 * dx, 2.1],
            Color::new(0.0, 0.0, 0.0, 1.0),
            12.0,
            0.0,
        ));
    }

    let mut bb = BoundingBox::from_bounded_entities(drawables.iter()).unwrap();
    bb.scale(1.001);
    let bb = BoundingBox::new(bb.xmin() - 0.5, bb.xmax() + 0.5, bb.ymin() - 1.1, bb.ymax());
    let view = Viewport::from_bounding_box(&bb, SideLength::Long(800));

    let fp = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(&format!("docs/img/arc_parallel_vary_air_gap_radius.svg"));
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

fn center_thickness() {
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

    texts.push(Text::new(
        "Shared parameters: core_radius = 5 m, side_height = 1 m, width = 2.6 m".into(),
        Anchor::Center,
        [0.0, 0.0],
        [1.5 * dx, -2.0],
        Color::new(0.0, 0.0, 0.0, 1.0),
        16.0,
        0.0,
    ));
    texts.push(Text::new(
        "Shared parameters: core_radius = -6 m, side_height = 1 m, width = 2.6 m".into(),
        Anchor::Center,
        [0.0, 0.0],
        [1.5 * dx, 1.5],
        Color::new(0.0, 0.0, 0.0, 1.0),
        16.0,
        0.0,
    ));

    // Inner magnets
    {
        let magnet = ArcParallelMagnet::with_center_thickness(
            Length::new::<meter>(1.0),
            Length::new::<meter>(5.0),
            Length::new::<meter>(1.0),
            Length::new::<meter>(1.0),
            AngleOrWidth::Width(Length::new::<meter>(2.6)),
            Arc::new(Material::default()),
        )
        .unwrap();
        drawables.extend(magnet.drawables(true, false).into_iter().map(|d| {
            let mut d = Drawable::from(d);
            d.line_reflection([0.0, 0.0], [1.0, 0.0]);
            d
        }));

        texts.push(Text::new(
            "center_thickness = 1.0 m".into(),
            Anchor::Center,
            [0.0, 0.0],
            [0.0, -1.4],
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
            Length::new::<meter>(1.2),
            AngleOrWidth::Width(Length::new::<meter>(2.6)),
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
            "center_thickness = 1.2 m".into(),
            Anchor::Center,
            [0.0, 0.0],
            [dx, -1.4],
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
            Length::new::<meter>(0.8),
            AngleOrWidth::Width(Length::new::<meter>(2.6)),
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
            "center_thickness = 0.8 m".into(),
            Anchor::Center,
            [0.0, 0.0],
            [2.0 * dx, -1.4],
            Color::new(0.0, 0.0, 0.0, 1.0),
            12.0,
            0.0,
        ));
    };
    {
        let magnet = ArcParallelMagnet::with_center_thickness(
            Length::new::<meter>(1.0),
            Length::new::<meter>(5.0),
            Length::new::<meter>(1.0),
            Length::new::<meter>(0.2),
            AngleOrWidth::Width(Length::new::<meter>(2.6)),
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
            "center_thickness = 0.2 m".into(),
            Anchor::Center,
            [0.0, 0.0],
            [3.0 * dx, -1.4],
            Color::new(0.0, 0.0, 0.0, 1.0),
            12.0,
            0.0,
        ));
    }

    // Outer magnets
    {
        let magnet = ArcParallelMagnet::with_center_thickness(
            Length::new::<meter>(1.0),
            Length::new::<meter>(-6.0),
            Length::new::<meter>(1.0),
            Length::new::<meter>(1.0),
            AngleOrWidth::Width(Length::new::<meter>(2.6)),
            Arc::new(Material::default()),
        )
        .unwrap();
        drawables.extend(magnet.drawables(true, false).into_iter().map(|d| {
            let mut d = Drawable::from(d);
            d.translate([0.0, dy]);
            d
        }));
        texts.push(Text::new(
            "center_thickness = 1.0 m".into(),
            Anchor::Center,
            [0.0, 0.0],
            [0.0, 2.1],
            Color::new(0.0, 0.0, 0.0, 1.0),
            12.0,
            0.0,
        ));
    }
    {
        let magnet = ArcParallelMagnet::with_center_thickness(
            Length::new::<meter>(1.0),
            Length::new::<meter>(-6.0),
            Length::new::<meter>(1.0),
            Length::new::<meter>(1.5),
            AngleOrWidth::Width(Length::new::<meter>(2.6)),
            Arc::new(Material::default()),
        )
        .unwrap();
        drawables.extend(magnet.drawables(true, false).into_iter().map(|d| {
            let mut d = Drawable::from(d);
            d.translate([dx, dy]);
            d
        }));
        texts.push(Text::new(
            "center_thickness = 1.5 m".into(),
            Anchor::Center,
            [0.0, 0.0],
            [dx, 2.1],
            Color::new(0.0, 0.0, 0.0, 1.0),
            12.0,
            0.0,
        ));
    }
    {
        let magnet = ArcParallelMagnet::with_center_thickness(
            Length::new::<meter>(1.0),
            Length::new::<meter>(-6.0),
            Length::new::<meter>(1.0),
            Length::new::<meter>(0.8),
            AngleOrWidth::Width(Length::new::<meter>(2.6)),
            Arc::new(Material::default()),
        )
        .unwrap();
        drawables.extend(magnet.drawables(true, false).into_iter().map(|d| {
            let mut d = Drawable::from(d);
            d.translate([2.0 * dx, dy]);
            d
        }));
        texts.push(Text::new(
            "center_thickness = 0.8 m".into(),
            Anchor::Center,
            [0.0, 0.0],
            [2.0 * dx, 2.1],
            Color::new(0.0, 0.0, 0.0, 1.0),
            12.0,
            0.0,
        ));
    };
    {
        let magnet = ArcParallelMagnet::with_center_thickness(
            Length::new::<meter>(1.0),
            Length::new::<meter>(-6.0),
            Length::new::<meter>(1.0),
            Length::new::<meter>(0.2),
            AngleOrWidth::Width(Length::new::<meter>(2.6)),
            Arc::new(Material::default()),
        )
        .unwrap();
        drawables.extend(magnet.drawables(true, false).into_iter().map(|d| {
            let mut d = Drawable::from(d);
            d.translate([3.0 * dx, dy]);
            d
        }));
        texts.push(Text::new(
            "center_thickness = 0.2 m".into(),
            Anchor::Center,
            [0.0, 0.0],
            [3.0 * dx, 2.1],
            Color::new(0.0, 0.0, 0.0, 1.0),
            12.0,
            0.0,
        ));
    }

    let mut bb = BoundingBox::from_bounded_entities(drawables.iter()).unwrap();
    bb.scale(1.001);
    let bb = BoundingBox::new(bb.xmin() - 0.5, bb.xmax() + 0.5, bb.ymin() - 1.0, bb.ymax());
    let view = Viewport::from_bounding_box(&bb, SideLength::Long(800));

    let fp = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(&format!("docs/img/arc_parallel_vary_center_thickness.svg"));
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
