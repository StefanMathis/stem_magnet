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
fn test_arc_segment_width() {
    {
        let magnet = ArcSegmentMagnet::with_const_thickness(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(50.0),
            Length::new::<millimeter>(10.0),
            PI,
            Arc::new(Material::default()),
        )
        .unwrap();
        approx::assert_abs_diff_eq!(magnet.width().get::<millimeter>(), 110.0, epsilon = 0.00001);
    }
    {
        let magnet = ArcSegmentMagnet::with_const_thickness(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(-60.0),
            Length::new::<millimeter>(10.0),
            PI,
            Arc::new(Material::default()),
        )
        .unwrap();
        approx::assert_abs_diff_eq!(magnet.width().get::<millimeter>(), 110.0, epsilon = 0.00001);
    }
}

/// All three constructors result in the same magnet
#[test]
fn test_compare_constructors_const_thickness_inner() {
    let mag_new = ArcSegmentMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(50.0),
        Length::new::<millimeter>(60.0),
        Length::new::<millimeter>(10.0),
        PI / 6.0,
        Arc::new(Material::default()),
    )
    .unwrap();

    let mag_with_const_thickness = ArcSegmentMagnet::with_const_thickness(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(50.0),
        Length::new::<millimeter>(10.0),
        PI / 6.0,
        Arc::new(Material::default()),
    )
    .unwrap();

    let mag_with_center_thickness = ArcSegmentMagnet::with_center_thickness(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(50.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(10.0),
        PI / 6.0,
        Arc::new(Material::default()),
    )
    .unwrap();

    approx::assert_abs_diff_eq!(mag_new.air_gap_radius().get::<meter>(), 0.06);
    approx::assert_abs_diff_eq!(
        mag_new.air_gap_radius().get::<meter>(),
        mag_with_const_thickness.air_gap_radius().get::<meter>(),
    );
    approx::assert_abs_diff_eq!(
        mag_new.air_gap_radius().get::<meter>(),
        mag_with_center_thickness.air_gap_radius().get::<meter>(),
    );

    approx::assert_abs_diff_eq!(
        mag_new.center_thickness().get::<meter>(),
        0.01,
        epsilon = 1e-3
    );
    approx::assert_abs_diff_eq!(
        mag_new.center_thickness().get::<meter>(),
        mag_with_const_thickness.center_thickness().get::<meter>(),
        epsilon = 1e-3,
    );
    approx::assert_abs_diff_eq!(
        mag_new.center_thickness().get::<meter>(),
        mag_with_center_thickness.center_thickness().get::<meter>(),
        epsilon = 1e-3,
    );

    approx::assert_abs_diff_eq!(mag_new.thickness().get::<meter>(), 0.01);
    approx::assert_abs_diff_eq!(
        mag_new.thickness().get::<meter>(),
        mag_with_const_thickness.thickness().get::<meter>(),
    );
    approx::assert_abs_diff_eq!(
        mag_new.thickness().get::<meter>(),
        mag_with_center_thickness.thickness().get::<meter>(),
    );

    approx::assert_abs_diff_eq!(
        mag_new.area().get::<square_meter>(),
        mag_with_const_thickness.area().get::<square_meter>(),
        epsilon = 1e-3,
    );
    approx::assert_abs_diff_eq!(
        mag_new.area().get::<square_meter>(),
        mag_with_center_thickness.area().get::<square_meter>(),
        epsilon = 1e-3,
    );
}

/// All three constructors result in the same magnet
#[test]
fn test_compare_constructors_const_thickness_outer() {
    let mag_new = ArcSegmentMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(-60.0),
        Length::new::<millimeter>(-50.0),
        Length::new::<millimeter>(10.0),
        PI / 6.0,
        Arc::new(Material::default()),
    )
    .unwrap();

    let mag_with_const_thickness = ArcSegmentMagnet::with_const_thickness(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(-60.0),
        Length::new::<millimeter>(10.0),
        PI / 6.0,
        Arc::new(Material::default()),
    )
    .unwrap();

    let mag_with_center_thickness = ArcSegmentMagnet::with_center_thickness(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(-60.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(10.0),
        PI / 6.0,
        Arc::new(Material::default()),
    )
    .unwrap();

    approx::assert_abs_diff_eq!(mag_new.air_gap_radius().get::<meter>(), -0.05);
    approx::assert_abs_diff_eq!(
        mag_new.air_gap_radius().get::<meter>(),
        mag_with_const_thickness.air_gap_radius().get::<meter>(),
    );
    approx::assert_abs_diff_eq!(
        mag_new.air_gap_radius().get::<meter>(),
        mag_with_center_thickness.air_gap_radius().get::<meter>(),
    );

    approx::assert_abs_diff_eq!(
        mag_new.center_thickness().get::<meter>(),
        0.01,
        epsilon = 1e-3
    );
    approx::assert_abs_diff_eq!(
        mag_new.center_thickness().get::<meter>(),
        mag_with_const_thickness.center_thickness().get::<meter>(),
        epsilon = 1e-3,
    );
    approx::assert_abs_diff_eq!(
        mag_new.center_thickness().get::<meter>(),
        mag_with_center_thickness.center_thickness().get::<meter>(),
        epsilon = 1e-3,
    );

    approx::assert_abs_diff_eq!(mag_new.thickness().get::<meter>(), 0.01);
    approx::assert_abs_diff_eq!(
        mag_new.thickness().get::<meter>(),
        mag_with_const_thickness.thickness().get::<meter>(),
    );
    approx::assert_abs_diff_eq!(
        mag_new.thickness().get::<meter>(),
        mag_with_center_thickness.thickness().get::<meter>(),
    );

    approx::assert_abs_diff_eq!(
        mag_new.area().get::<square_meter>(),
        mag_with_const_thickness.area().get::<square_meter>(),
        epsilon = 1e-3,
    );
    approx::assert_abs_diff_eq!(
        mag_new.area().get::<square_meter>(),
        mag_with_center_thickness.area().get::<square_meter>(),
        epsilon = 1e-3,
    );
}

#[test]
fn test_convex_outer_magnet() {
    let mag = ArcSegmentMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(-50.0),
        Length::new::<millimeter>(15.0),
        Length::new::<millimeter>(10.0),
        PI / 6.0,
        Arc::new(Material::default()),
    )
    .unwrap();
    approx::assert_abs_diff_eq!(mag.thickness().get::<meter>(), 0.0127542, epsilon = 1e-6);

    let drawables = mag.drawables(true, false);
    let view = Viewport::from_bounded_entities(drawables.iter(), SideLength::Long(500)).unwrap();

    let callback = |path: &std::path::Path| {
        return view.write_to_file(path, move |cr| {
            cr.set_source_rgb(1.0, 1.0, 1.0);
            cr.paint()?;
            for drawable in drawables.iter() {
                drawable.draw(cr)?;
            }
            return Ok(());
        });
    };
    assert!(compare_or_create("tests/img/convex_outer_magnet.png", callback, 0.95).is_ok());
}

#[test]
fn test_concave_outer_magnet() {
    let mag = ArcSegmentMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(-50.0),
        Length::new::<millimeter>(-15.0),
        Length::new::<millimeter>(10.0),
        PI / 6.0,
        Arc::new(Material::default()),
    )
    .unwrap();
    approx::assert_abs_diff_eq!(mag.thickness().get::<meter>(), 0.008608, epsilon = 1e-6);

    let drawables = mag.drawables(true, false);
    let view = Viewport::from_bounded_entities(drawables.iter(), SideLength::Long(500)).unwrap();

    let callback = |path: &std::path::Path| {
        return view.write_to_file(path, move |cr| {
            cr.set_source_rgb(1.0, 1.0, 1.0);
            cr.paint()?;
            for drawable in drawables.iter() {
                drawable.draw(cr)?;
            }
            return Ok(());
        });
    };
    assert!(compare_or_create("tests/img/concave_outer_magnet.png", callback, 0.95).is_ok());
}

#[test]
fn test_convex_inner_magnet() {
    let mag = ArcSegmentMagnet::with_center_thickness(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(50.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(12.0),
        PI / 6.0,
        Arc::new(Material::default()),
    )
    .unwrap();
    approx::assert_abs_diff_eq!(mag.thickness().get::<meter>(), 0.011);
}

#[test]
fn test_invalid_inputs_new() {
    assert!(
        ArcSegmentMagnet::new(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(50.0),
            Length::new::<millimeter>(60.0),
            Length::new::<millimeter>(10.0),
            0.0,
            Arc::new(Material::default()),
        )
        .is_err()
    );
    assert!(
        ArcSegmentMagnet::new(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(50.0),
            Length::new::<millimeter>(60.0),
            Length::new::<millimeter>(10.0),
            -1.0,
            Arc::new(Material::default()),
        )
        .is_err()
    );
    assert!(
        ArcSegmentMagnet::new(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(50.0),
            Length::new::<millimeter>(60.0),
            Length::new::<millimeter>(-10.0),
            3.5,
            Arc::new(Material::default()),
        )
        .is_err()
    );
}

#[test]
fn test_invalid_inputs_with_center_thickness() {
    assert!(
        ArcSegmentMagnet::with_center_thickness(
            Length::new::<meter>(1.0),
            Length::new::<meter>(6.0),
            Length::new::<meter>(1.0),
            Length::new::<meter>(-0.2),
            PI / 6.0,
            Arc::new(Material::default()),
        )
        .is_err()
    );
    assert!(
        ArcSegmentMagnet::with_center_thickness(
            Length::new::<meter>(1.0),
            Length::new::<meter>(6.0),
            Length::new::<meter>(-1.0),
            Length::new::<meter>(0.2),
            PI / 6.0,
            Arc::new(Material::default()),
        )
        .is_err()
    );
}

#[test]
fn test_arc_segment_inner() {
    let mat_arc = Arc::new(ferrite());

    let magnet = ArcSegmentMagnet::with_const_thickness(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(50.0),
        Length::new::<millimeter>(10.0),
        PI / 6.0,
        mat_arc,
    )
    .unwrap();

    approx::assert_abs_diff_eq!(magnet.thickness().get::<millimeter>(), 10.0, epsilon = 0.01);
    approx::assert_abs_diff_eq!(magnet.area().get::<square_meter>(), magnet.shape().area());
    approx::assert_abs_diff_eq!(magnet.width().get::<millimeter>(), 28.47, epsilon = 0.01);
    approx::assert_abs_diff_eq!(
        magnet.volume().get::<cubic_meter>(),
        4.751658e-5,
        epsilon = 1e-9
    );
    approx::assert_abs_diff_eq!(magnet.mass().get::<kilogram>(), 0.2375, epsilon = 0.001);

    approx::assert_abs_diff_eq!(
        magnet
            .magnetomotive_force(&[ThermodynamicTemperature::new::<degree_celsius>(20.0).into()])
            .get::<ampere>(),
        3258.887,
        epsilon = 1e-3
    );
    approx::assert_abs_diff_eq!(
        magnet
            .magnetomotive_force(&[ThermodynamicTemperature::new::<degree_celsius>(120.0).into()])
            .get::<ampere>(),
        2694.122,
        epsilon = 1e-3
    );
}

#[test]
fn test_arc_segment_outer() {
    let mat_arc = Arc::new(ferrite());

    let magnet = ArcSegmentMagnet::with_const_thickness(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(-60.0),
        Length::new::<millimeter>(10.0),
        PI / 6.0,
        mat_arc,
    )
    .unwrap();

    approx::assert_abs_diff_eq!(magnet.thickness().get::<millimeter>(), 10.0, epsilon = 0.01);
    approx::assert_abs_diff_eq!(magnet.area().get::<square_meter>(), magnet.shape().area());
    approx::assert_abs_diff_eq!(magnet.width().get::<millimeter>(), 28.47, epsilon = 0.01);
    approx::assert_abs_diff_eq!(
        magnet.volume().get::<cubic_meter>(),
        4.751658e-5,
        epsilon = 1e-9
    );
    approx::assert_abs_diff_eq!(magnet.mass().get::<kilogram>(), 0.2375, epsilon = 0.001);

    approx::assert_abs_diff_eq!(
        magnet
            .magnetomotive_force(&[ThermodynamicTemperature::new::<degree_celsius>(20.0).into()])
            .get::<ampere>(),
        3258.887,
        epsilon = 1e-3
    );
    approx::assert_abs_diff_eq!(
        magnet
            .magnetomotive_force(&[ThermodynamicTemperature::new::<degree_celsius>(120.0).into()])
            .get::<ampere>(),
        2694.122,
        epsilon = 1e-3
    );
}

#[test]
fn test_comparable_block_magnet() {
    let mat_arc = Arc::new(ferrite());
    let arc_magnet = ArcSegmentMagnet::with_const_thickness(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(-60.0),
        Length::new::<millimeter>(10.0),
        PI / 6.0,
        mat_arc.clone(),
    )
    .unwrap();
    let block_magnet = BlockMagnet::new(
        arc_magnet.length(),
        arc_magnet.width(),
        arc_magnet.thickness(),
        Length::new::<meter>(0.0),
        mat_arc,
    )
    .unwrap();

    approx::assert_abs_diff_eq!(
        arc_magnet
            .magnetomotive_force(&[ThermodynamicTemperature::new::<degree_celsius>(120.0).into()])
            .get::<ampere>(),
        block_magnet
            .magnetomotive_force(&[ThermodynamicTemperature::new::<degree_celsius>(120.0).into()])
            .get::<ampere>(),
        epsilon = 1e-3
    );
}

#[test]
fn test_draw_arc_segment_half_circle() {
    let inner_magnet = ArcSegmentMagnet::with_const_thickness(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(50.0),
        Length::new::<millimeter>(10.0),
        PI,
        Arc::new(Material::default()),
    )
    .unwrap();

    let outer_magnet = ArcSegmentMagnet::with_const_thickness(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(-80.0),
        Length::new::<millimeter>(10.0),
        PI,
        Arc::new(Material::default()),
    )
    .unwrap();

    let drawables = inner_magnet.drawables(true, false);
    let view = Viewport::from_bounded_entities(drawables.iter(), SideLength::Long(500)).unwrap();

    let callback = |path: &std::path::Path| {
        return view.write_to_file(path, move |cr| {
            cr.set_source_rgb(1.0, 1.0, 1.0);
            cr.paint()?;
            for drawable in drawables.iter() {
                drawable.draw(cr)?;
            }
            return Ok(());
        });
    };
    assert!(
        compare_or_create(
            "tests/img/half_circle_arc_segment_split_inner.png",
            callback,
            0.95
        )
        .is_ok()
    );

    let drawables = outer_magnet.drawables(true, false);
    let view = Viewport::from_bounded_entities(drawables.iter(), SideLength::Long(500)).unwrap();

    let callback = |path: &std::path::Path| {
        return view.write_to_file(path, move |cr| {
            cr.set_source_rgb(1.0, 1.0, 1.0);
            cr.paint()?;
            for drawable in drawables.iter() {
                drawable.draw(cr)?;
            }
            return Ok(());
        });
    };
    assert!(
        compare_or_create(
            "tests/img/half_circle_arc_segment_split_outer.png",
            callback,
            0.95
        )
        .is_ok()
    );
}

#[test]
fn test_draw_arc_segment() {
    {
        let magnet = ArcSegmentMagnet::with_const_thickness(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(50.0),
            Length::new::<millimeter>(10.0),
            PI / 6.0,
            Arc::new(Default::default()),
        )
        .unwrap();

        let drawables = magnet.drawables(true, false);
        let view =
            Viewport::from_bounded_entities(drawables.iter(), SideLength::Long(500)).unwrap();

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
        assert!(
            compare_or_create(
                "tests/img/arc_segment_inner_split_const_thickness.png",
                callback,
                0.99
            )
            .is_ok()
        );
    }
    {
        let magnet = ArcSegmentMagnet::with_const_thickness(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(-60.0),
            Length::new::<millimeter>(10.0),
            PI / 6.0,
            Arc::new(Default::default()),
        )
        .unwrap();

        let drawables = magnet.drawables(true, false);
        let view =
            Viewport::from_bounded_entities(drawables.iter(), SideLength::Long(500)).unwrap();

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
        assert!(
            compare_or_create(
                "tests/img/arc_segment_outer_split_const_thickness.png",
                callback,
                0.99
            )
            .is_ok()
        );
    }
    {
        let magnet = ArcSegmentMagnet::new(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(50.0),
            Length::new::<millimeter>(40.0),
            Length::new::<millimeter>(10.0),
            PI / 6.0,
            Arc::new(Default::default()),
        )
        .unwrap();

        let drawables = magnet.drawables(true, false);
        let view =
            Viewport::from_bounded_entities(drawables.iter(), SideLength::Long(500)).unwrap();

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
        assert!(
            compare_or_create(
                "tests/img/arc_segment_inner_split_convex.png",
                callback,
                0.99
            )
            .is_ok()
        );
    }
    {
        let magnet = ArcSegmentMagnet::new(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(-60.0),
            Length::new::<millimeter>(-70.0),
            Length::new::<millimeter>(10.0),
            PI / 6.0,
            Arc::new(Default::default()),
        )
        .unwrap();

        let drawables = magnet.drawables(true, false);
        let view =
            Viewport::from_bounded_entities(drawables.iter(), SideLength::Long(500)).unwrap();

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
        assert!(
            compare_or_create(
                "tests/img/arc_segment_outer_split_convex.png",
                callback,
                0.99
            )
            .is_ok()
        );
    }
    {
        let magnet = ArcSegmentMagnet::new(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(60.0),
            Length::new::<millimeter>(30.0),
            Length::new::<millimeter>(0.0),
            PI / 6.0,
            Arc::new(Default::default()),
        )
        .unwrap();

        let drawables = magnet.drawables(true, false);
        let view =
            Viewport::from_bounded_entities(drawables.iter(), SideLength::Long(500)).unwrap();

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
        assert!(
            compare_or_create(
                "tests/img/arc_segment_inner_no_side_thickness.png",
                callback,
                0.99
            )
            .is_ok()
        );
    }
    {
        let magnet = ArcSegmentMagnet::new(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(-60.0),
            Length::new::<millimeter>(-1000000000.0),
            Length::new::<millimeter>(10.0),
            PI / 6.0,
            Arc::new(Default::default()),
        )
        .unwrap();

        let drawables = magnet.drawables(true, false);
        let view =
            Viewport::from_bounded_entities(drawables.iter(), SideLength::Long(500)).unwrap();

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
        assert!(compare_or_create("tests/img/arc_segment_outer_flat.png", callback, 0.99).is_ok());
    }
    {
        let magnet = ArcSegmentMagnet::with_center_thickness(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(50.0),
            Length::new::<millimeter>(10.0),
            Length::new::<millimeter>(10.0),
            PI / 6.0,
            Arc::new(Material::default()),
        )
        .unwrap();

        let drawables = magnet.drawables(true, false);
        let view =
            Viewport::from_bounded_entities(drawables.iter(), SideLength::Long(500)).unwrap();

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
        assert!(
            compare_or_create(
                "tests/img/arc_segment_inner_center_thickness.png",
                callback,
                0.99
            )
            .is_ok()
        );
    }
    {
        let magnet = ArcSegmentMagnet::with_center_thickness(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(-60.0),
            Length::new::<millimeter>(10.0),
            Length::new::<millimeter>(10.0),
            PI / 6.0,
            Arc::new(Material::default()),
        )
        .unwrap();

        let drawables = magnet.drawables(true, false);
        let view =
            Viewport::from_bounded_entities(drawables.iter(), SideLength::Long(500)).unwrap();

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
        assert!(
            compare_or_create(
                "tests/img/arc_segment_outer_center_thickness.png",
                callback,
                0.99
            )
            .is_ok()
        );
    }
    {
        let magnet = ArcSegmentMagnet::with_center_thickness(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(-60.0),
            Length::new::<millimeter>(10.0),
            Length::new::<millimeter>(1.0),
            PI / 6.0,
            Arc::new(Material::default()),
        )
        .unwrap();

        let drawables = magnet.drawables(true, false);
        let view =
            Viewport::from_bounded_entities(drawables.iter(), SideLength::Long(500)).unwrap();

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
        assert!(
            compare_or_create(
                "tests/img/arc_segment_outer_center_thickness_concave.png",
                callback,
                0.99
            )
            .is_ok()
        );
    }
    {
        let magnet = ArcSegmentMagnet::with_center_thickness(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(-60.0),
            Length::new::<millimeter>(10.0),
            Length::new::<millimeter>(15.0),
            PI / 6.0,
            Arc::new(Material::default()),
        )
        .unwrap();

        let drawables = magnet.drawables(true, false);
        let view =
            Viewport::from_bounded_entities(drawables.iter(), SideLength::Long(500)).unwrap();

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
        assert!(
            compare_or_create(
                "tests/img/arc_segment_outer_center_thickness_convex.png",
                callback,
                0.99
            )
            .is_ok()
        );
    }
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

    let inner_magnet = ArcSegmentMagnet::with_const_thickness(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(50.0),
        Length::new::<millimeter>(10.0),
        PI / 6.0,
        Arc::new(Default::default()),
    )
    .unwrap();

    let outer_magnet = ArcSegmentMagnet::with_const_thickness(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(-80.0),
        Length::new::<millimeter>(10.0),
        PI / 6.0,
        Arc::new(Default::default()),
    )
    .unwrap();

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
            for drawable in inner_magnet.drawables(true, false).iter() {
                // Inner magnet
                let mut inner = Drawable {
                    geometry: drawable.geometry.clone().into(),
                    style: drawable.style.clone(),
                };
                inner.translate([0.0, inner_radius]);
                inner.draw(&cr)?;
            }

            // Outer magnet
            for drawable in outer_magnet.drawables(true, false).iter() {
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
    assert!(compare_or_create("tests/img/arc_segment_inner_and_outer.png", callback, 0.99).is_ok());
}

#[test]
fn serialize_and_deserialize() {
    let magnet = ArcSegmentMagnet::with_const_thickness(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(80.0),
        Length::new::<millimeter>(10.0),
        PI / 6.0,
        Arc::new(Default::default()),
    )
    .unwrap();

    let string = serde_yaml::to_string(&magnet).unwrap();
    let de_magnet: ArcSegmentMagnet = serde_yaml::from_str(&string).unwrap();

    approx::assert_abs_diff_eq!(
        magnet.air_gap_radius().get::<meter>(),
        de_magnet.air_gap_radius().get::<meter>(),
    );
    approx::assert_abs_diff_eq!(
        magnet.center_thickness().get::<meter>(),
        de_magnet.center_thickness().get::<meter>(),
        epsilon = 1e-3,
    );
    approx::assert_abs_diff_eq!(
        magnet.thickness().get::<meter>(),
        de_magnet.thickness().get::<meter>(),
    );
    approx::assert_abs_diff_eq!(
        magnet.area().get::<square_meter>(),
        de_magnet.area().get::<square_meter>(),
        epsilon = 1e-3,
    );
}

#[test]
fn deserialize() {
    {
        let str = indoc::indoc! {"
        length: 165 mm
        core_radius: 80 mm
        air_gap_radius: 60 mm
        side_thickness: 10 mm
        angle: PI/6 rad
        material:
            name: NMF-12J 430mT
            relative_permeability: 1.05
        "};
        let magnet: ArcSegmentMagnet = serde_yaml::from_str(str).unwrap();

        approx::assert_abs_diff_eq!(magnet.air_gap_radius().get::<meter>(), 0.06,);
        approx::assert_abs_diff_eq!(
            magnet.center_thickness().get::<meter>(),
            0.0116,
            epsilon = 1e-3,
        );
        approx::assert_abs_diff_eq!(magnet.side_thickness().get::<meter>(), 0.01,);
        approx::assert_abs_diff_eq!(
            magnet.area().get::<square_millimeter>(),
            496.837,
            epsilon = 1e-3
        );
    }
    {
        let str = indoc::indoc! {"
        length: 165 mm
        core_radius: 80 mm
        side_thickness: 10 mm
        angle: PI/6 rad
        material:
            name: NMF-12J 430mT
            relative_permeability: 1.05
        "};
        let magnet: ArcSegmentMagnet = serde_yaml::from_str(str).unwrap();

        approx::assert_abs_diff_eq!(magnet.air_gap_radius().get::<meter>(), 0.09,);
        approx::assert_abs_diff_eq!(
            magnet.center_thickness().get::<meter>(),
            0.01,
            epsilon = 1e-3,
        );
        approx::assert_abs_diff_eq!(magnet.side_thickness().get::<meter>(), 0.01,);
    }
    {
        let str = indoc::indoc! {"
        length: 165 mm
        core_radius: 80 mm
        side_thickness: 10 mm
        center_thickness: 15 mm
        angle: PI/6 rad
        material:
            name: NMF-12J 430mT
            relative_permeability: 1.05
        "};
        let magnet: ArcSegmentMagnet = serde_yaml::from_str(str).unwrap();

        approx::assert_abs_diff_eq!(
            magnet.air_gap_radius().get::<meter>(),
            0.03766,
            epsilon = 1e-3
        );
        approx::assert_abs_diff_eq!(
            magnet.center_thickness().get::<meter>(),
            0.015,
            epsilon = 1e-3,
        );
        approx::assert_abs_diff_eq!(magnet.thickness().get::<meter>(), 0.0125, epsilon = 1e-3,);
        approx::assert_abs_diff_eq!(magnet.side_thickness().get::<meter>(), 0.01,);
    }
}
