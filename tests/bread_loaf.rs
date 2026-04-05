use std::{f64::consts::PI, sync::Arc};

use cairo_viewport::*;
use planar_geo::prelude::*;
use stem_magnet::prelude::*;

fn ferrite() -> Material {
    let str = indoc::indoc! {"
    name: NMF-12J 430mT
    relative_permeability: 1.05
    remanence: 
        FirstOrderTaylor:
            base_value: 0.43 T
            expansion_point: 20.0 °C
            slope: -0.1733 % / K # 17,33 % remanence loss per 100 K
    iron_losses: 0
    mass_density: 5000.0 kg / m^3 # https://www.bomatec.com/wp-content/uploads/2021/12/BMHFa-3227.pdf
    electrical_resistivity: 1e6 Ohm*m
    intrinsic_coercivity:
        FirstOrderTaylor:
            base_value: 430 kA/m
            expansion_point: 20.0 °C
            slope: 0.11 % / K # 11 % intrinsic coercivity gain per 100 K
    heat_capacity: 700.0 J / kg / K # https://www.bomatec.com/wp-content/uploads/2021/12/BMHFa-3227.pdf
    thermal_conductivity: 4.0 W / m / K # https://www.bomatec.com/wp-content/uploads/2021/12/BMHFa-3227.pdf
    "};

    return serde_yaml::from_str(&str).expect("valid material");
}

#[test]
fn test_failed_creation() {
    let mat_arc = Arc::new(ferrite());

    assert!(
        BreadLoafMagnet::new(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(20.0),
            Length::new::<millimeter>(10.0),
            Length::new::<millimeter>(9.0),
            mat_arc.clone()
        )
        .is_err()
    ); // Radius too small

    // Negative parameters
    assert!(
        BreadLoafMagnet::new(
            Length::new::<millimeter>(-165.0),
            Length::new::<millimeter>(20.0),
            Length::new::<millimeter>(10.0),
            Length::new::<millimeter>(50.0),
            mat_arc.clone()
        )
        .is_err()
    );
    assert!(
        BreadLoafMagnet::new(
            Length::new::<millimeter>(-165.0),
            Length::new::<millimeter>(20.0),
            Length::new::<millimeter>(10.0),
            Length::new::<millimeter>(50.0),
            mat_arc.clone()
        )
        .is_err()
    );
    assert!(
        BreadLoafMagnet::new(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(-20.0),
            Length::new::<millimeter>(10.0),
            Length::new::<millimeter>(50.0),
            mat_arc.clone()
        )
        .is_err()
    );
    assert!(
        BreadLoafMagnet::new(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(20.0),
            Length::new::<millimeter>(-10.0),
            Length::new::<millimeter>(50.0),
            mat_arc.clone()
        )
        .is_err()
    );
}

#[test]
fn test_properties_convex() {
    let mat_arc = Arc::new(ferrite());

    let magnet = BreadLoafMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(20.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(50.0),
        mat_arc,
    )
    .unwrap();

    approx::assert_abs_diff_eq!(
        magnet.arc_segment_height().get::<millimeter>(),
        1.0102,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        magnet.center_thickness().get::<millimeter>(),
        11.010,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        magnet.thickness().get::<millimeter>(),
        10.505,
        epsilon = 0.001
    );

    approx::assert_abs_diff_eq!(magnet.area().get::<square_meter>(), magnet.shape().area());
    approx::assert_abs_diff_eq!(
        magnet.volume().get::<cubic_meter>(),
        3.522698e-5,
        epsilon = 1e-9
    );
    approx::assert_abs_diff_eq!(magnet.mass().get::<kilogram>(), 0.1761, epsilon = 0.001);

    approx::assert_abs_diff_eq!(
        magnet
            .magnetomotive_force(&[ThermodynamicTemperature::new::<degree_celsius>(20.0).into()])
            .get::<ampere>(),
        3423.494,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        magnet
            .magnetomotive_force(&[ThermodynamicTemperature::new::<degree_celsius>(120.0).into()])
            .get::<ampere>(),
        2830.203,
        epsilon = 0.001
    );
}

#[test]
fn test_properties_concave() {
    let mat_arc = Arc::new(ferrite());

    let magnet = BreadLoafMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(20.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(-50.0),
        mat_arc,
    )
    .unwrap();

    approx::assert_abs_diff_eq!(
        magnet.arc_segment_height().get::<millimeter>(),
        -1.0102,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        magnet.center_thickness().get::<millimeter>(),
        8.989,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        magnet.thickness().get::<millimeter>(),
        9.4948,
        epsilon = 0.001
    );

    approx::assert_abs_diff_eq!(magnet.area().get::<square_meter>(), magnet.shape().area());
    approx::assert_abs_diff_eq!(
        magnet.volume().get::<cubic_meter>(),
        3.0773019e-5,
        epsilon = 1e-9
    );
    approx::assert_abs_diff_eq!(magnet.mass().get::<kilogram>(), 0.1538, epsilon = 0.001);

    approx::assert_abs_diff_eq!(
        magnet
            .magnetomotive_force(&[ThermodynamicTemperature::new::<degree_celsius>(20.0).into()])
            .get::<ampere>(),
        3094.279,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        magnet
            .magnetomotive_force(&[ThermodynamicTemperature::new::<degree_celsius>(120.0).into()])
            .get::<ampere>(),
        2558.041,
        epsilon = 0.001
    );
}

#[test]
fn test_draw_convex() {
    let magnet = BreadLoafMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(20.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(50.0),
        Arc::new(Material::default()),
    )
    .unwrap();

    // Image comparison
    let drawables = magnet.drawables(false, false);

    let view = Viewport::from_bounded_entities(drawables.iter(), SideLength::Long(500)).unwrap();
    let path = std::path::Path::new("tests/img/bread_loaf_single_convex.png");

    // Always compare to the same reference image
    let callback = |path: &std::path::Path| {
        return view.write_to_file(path, move |cr| {
            cr.set_source_rgb(1.0, 1.0, 1.0);
            cr.paint()?;
            for drawable in drawables.iter() {
                drawable.draw(&cr)?;
            }
            return Ok(());
        });
    };
    assert!(compare_or_create(path, callback, 0.95).is_ok());

    let drawables = magnet.drawables(true, false);

    let view = Viewport::from_bounded_entities(drawables.iter(), SideLength::Long(500)).unwrap();
    let path = std::path::Path::new("tests/img/bread_loaf_split_convex.png");

    // Always compare to the same reference image
    let callback = |path: &std::path::Path| {
        return view.write_to_file(path, move |cr| {
            cr.set_source_rgb(1.0, 1.0, 1.0);
            cr.paint()?;
            for drawable in drawables.iter() {
                drawable.draw(&cr)?;
            }
            return Ok(());
        });
    };
    assert!(compare_or_create(path, callback, 0.95).is_ok());
}

#[test]
fn test_draw_concave() {
    let magnet = BreadLoafMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(20.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(-50.0),
        Arc::new(Material::default()),
    )
    .unwrap();

    // Image comparison
    let drawables = magnet.drawables(false, false);

    let view = Viewport::from_bounded_entities(drawables.iter(), SideLength::Long(500)).unwrap();
    let path = std::path::Path::new("tests/img/bread_loaf_single_concave.png");

    // Always compare to the same reference image
    let callback = |path: &std::path::Path| {
        return view.write_to_file(path, move |cr| {
            cr.set_source_rgb(1.0, 1.0, 1.0);
            cr.paint()?;
            for drawable in drawables.iter() {
                drawable.draw(&cr)?;
            }
            return Ok(());
        });
    };
    assert!(compare_or_create(path, callback, 0.95).is_ok());

    let drawables = magnet.drawables(true, false);

    let view = Viewport::from_bounded_entities(drawables.iter(), SideLength::Long(500)).unwrap();
    let path = std::path::Path::new("tests/img/bread_loaf_split_concave.png");

    // Always compare to the same reference image
    let callback = |path: &std::path::Path| {
        return view.write_to_file(path, move |cr| {
            cr.set_source_rgb(1.0, 1.0, 1.0);
            cr.paint()?;
            for drawable in drawables.iter() {
                drawable.draw(&cr)?;
            }
            return Ok(());
        });
    };
    assert!(compare_or_create(path, callback, 0.95).is_ok());
}

#[test]
fn test_inner_and_outer_rotor() {
    let inner_radius = 0.05;
    let outer_radius = 0.08;

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

    let polysegment =
        Polysegment::from(ArcSegment::circle([0.0, 0.0], inner_radius).expect("valid"));
    let inner = Contour::new(polysegment);

    let polysegment =
        Polysegment::from(ArcSegment::circle([0.0, 0.0], outer_radius).expect("valid"));
    let hole = Contour::new(polysegment);

    let polysegment =
        Polysegment::from(ArcSegment::circle([0.0, 0.0], 1.1 * outer_radius).expect("valid"));
    let contour = Contour::new(polysegment);

    let outer = Shape::new(vec![contour, hole]).expect("valid");

    let magnet = BreadLoafMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(20.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(50.0),
        Arc::new(Material::default()),
    )
    .unwrap();

    let drawables = magnet.drawables(true, false);
    let view = Viewport::from_bounding_box(&outer.bounding_box(), SideLength::Long(500));

    // Always compare to the same reference image
    let callback = |path: &std::path::Path| {
        return view.write_to_file(path, move |cr| {
            cr.set_source_rgb(1.0, 1.0, 1.0);
            cr.paint()?;

            // Draw the cores
            inner.draw(&core_style, cr)?;
            outer.draw(&core_style, cr)?;

            // Inner magnet
            for drawable in drawables.iter() {
                // Inner magnet
                let mut inner = Drawable {
                    geometry: drawable.geometry.clone().into(),
                    style: drawable.style.clone(),
                };
                inner.translate([0.0, inner_radius]);
                inner.draw(&cr)?;

                // Outer magnet
                let mut outer = Drawable {
                    geometry: drawable.geometry.clone().into(),
                    style: drawable.style.clone(),
                };
                outer.rotate([0.0, 0.0], PI);
                outer.translate([0.0, outer_radius]);
                outer.draw(&cr)?;
            }
            return Ok(());
        });
    };
    assert!(compare_or_create("tests/img/bread_loaf_inner_and_outer.png", callback, 0.99).is_ok());
}
