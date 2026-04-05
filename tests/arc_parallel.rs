use std::{
    f64::consts::{PI, SQRT_2},
    sync::Arc,
};

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
fn test_angle_and_width() {
    approx::assert_abs_diff_eq!(
        AngleOrWidth::Width(Length::new::<millimeter>(50.0) * SQRT_2)
            .width(Length::new::<millimeter>(50.0))
            .get::<millimeter>(),
        50.0 * SQRT_2,
        epsilon = 0.00001
    );
    approx::assert_abs_diff_eq!(
        AngleOrWidth::Angle(0.5 * PI)
            .width(Length::new::<millimeter>(50.0))
            .get::<millimeter>(),
        50.0 * SQRT_2,
        epsilon = 0.00001
    );
    approx::assert_abs_diff_eq!(
        AngleOrWidth::Angle(0.5 * PI).angle(Length::new::<millimeter>(50.0)),
        0.5 * PI,
        epsilon = 0.00001
    );
    approx::assert_abs_diff_eq!(
        AngleOrWidth::Width(Length::new::<millimeter>(50.0) * SQRT_2)
            .angle(Length::new::<millimeter>(50.0)),
        0.5 * PI,
        epsilon = 0.00001
    );

    // Convert forth and back
    let angle = PI / 6.0;
    let radius = Length::new::<millimeter>(50.0);
    let width = AngleOrWidth::Angle(angle).width(radius);
    approx::assert_abs_diff_eq!(
        AngleOrWidth::Width(width).angle(radius),
        angle,
        epsilon = 0.00001
    );
}

#[test]
fn test_side_and_thickness() {
    approx::assert_abs_diff_eq!(
        SideHeightOrThickness::Height(Length::new::<millimeter>(10.0))
            .height(
                Length::new::<millimeter>(50.0),
                AngleOrWidth::Angle(PI / 2.0)
            )
            .get::<millimeter>(),
        10.0,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        SideHeightOrThickness::Thickness(Length::new::<millimeter>(7.50744))
            .height(
                Length::new::<millimeter>(50.0),
                AngleOrWidth::Angle(PI / 2.0)
            )
            .get::<millimeter>(),
        10.0,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        SideHeightOrThickness::Height(Length::new::<millimeter>(10.0))
            .thickness(
                Length::new::<millimeter>(50.0),
                AngleOrWidth::Angle(PI / 2.0)
            )
            .get::<millimeter>(),
        7.50744,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        SideHeightOrThickness::Thickness(Length::new::<millimeter>(7.50744))
            .thickness(
                Length::new::<millimeter>(50.0),
                AngleOrWidth::Angle(PI / 2.0)
            )
            .get::<millimeter>(),
        7.50744,
        epsilon = 0.001
    );

    // Convert forth and back
    let side_height = Length::new::<millimeter>(10.0);
    let radius = Length::new::<millimeter>(50.0);
    let thickness = SideHeightOrThickness::Height(side_height)
        .thickness(radius, Length::new::<millimeter>(30.0).into());
    approx::assert_abs_diff_eq!(
        SideHeightOrThickness::Thickness(thickness)
            .height(radius, Length::new::<millimeter>(30.0).into())
            .get::<millimeter>(),
        side_height.get::<millimeter>(),
        epsilon = 0.00001
    );

    // Positive and negative radius
    let h_pos = SideHeightOrThickness::Thickness(Length::new::<millimeter>(10.0))
        .height(
            Length::new::<millimeter>(50.0),
            Length::new::<millimeter>(30.0).into(),
        )
        .get::<millimeter>();
    let h_neg = SideHeightOrThickness::Thickness(Length::new::<millimeter>(10.0))
        .height(
            Length::new::<millimeter>(-60.0),
            Length::new::<millimeter>(30.0).into(),
        )
        .get::<millimeter>();
    approx::assert_abs_diff_eq!(h_pos, h_neg, epsilon = 1e-12);

    let t_pos = SideHeightOrThickness::Height(Length::new::<millimeter>(h_pos))
        .thickness(
            Length::new::<millimeter>(-60.0),
            Length::new::<millimeter>(30.0).into(),
        )
        .get::<millimeter>();
    let t_neg = SideHeightOrThickness::Height(Length::new::<millimeter>(h_pos))
        .thickness(
            Length::new::<millimeter>(-60.0),
            Length::new::<millimeter>(30.0).into(),
        )
        .get::<millimeter>();
    approx::assert_abs_diff_eq!(t_pos, t_neg, epsilon = 1e-12);
}

#[test]
fn test_arc_parallel_width() {
    {
        let magnet = ArcParallelMagnet::with_const_thickness(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(50.0),
            SideHeightOrThickness::Height(Length::new::<millimeter>(10.0)),
            AngleOrWidth::Angle(PI),
            Arc::new(Material::default()),
        )
        .unwrap();
        approx::assert_abs_diff_eq!(magnet.width().get::<millimeter>(), 100.0, epsilon = 0.00001);
    }
    {
        let magnet = ArcParallelMagnet::with_const_thickness(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(50.0),
            SideHeightOrThickness::Height(Length::new::<millimeter>(10.0)),
            AngleOrWidth::Width(Length::new::<millimeter>(100.0)),
            Arc::new(Material::default()),
        )
        .unwrap();
        approx::assert_abs_diff_eq!(magnet.width().get::<millimeter>(), 100.0, epsilon = 0.00001);
    }
    {
        let magnet = ArcParallelMagnet::with_const_thickness(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(-50.0),
            SideHeightOrThickness::Height(Length::new::<millimeter>(10.0)),
            AngleOrWidth::Angle(PI),
            Arc::new(Material::default()),
        )
        .unwrap();
        approx::assert_abs_diff_eq!(magnet.width().get::<millimeter>(), 100.0, epsilon = 0.00001);
    }
    {
        let magnet = ArcParallelMagnet::with_const_thickness(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(-50.0),
            SideHeightOrThickness::Height(Length::new::<millimeter>(10.0)),
            AngleOrWidth::Width(Length::new::<millimeter>(100.0)),
            Arc::new(Material::default()),
        )
        .unwrap();
        approx::assert_abs_diff_eq!(magnet.width().get::<millimeter>(), 100.0, epsilon = 0.00001);
    }
}

/// All three constructors result in the same magnet
#[test]
fn test_compare_constructors_const_thickness_inner() {
    let side_height = SideHeightOrThickness::Thickness(Length::new::<millimeter>(10.0)).height(
        Length::new::<millimeter>(50.0),
        AngleOrWidth::Width(Length::new::<millimeter>(30.0)),
    );

    let mag_new = ArcParallelMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(50.0),
        Length::new::<millimeter>(60.0),
        side_height,
        AngleOrWidth::Width(Length::new::<millimeter>(30.0)),
        Arc::new(Material::default()),
    )
    .unwrap();

    let mag_with_const_thickness = ArcParallelMagnet::with_const_thickness(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(50.0),
        SideHeightOrThickness::Height(side_height),
        AngleOrWidth::Width(Length::new::<millimeter>(30.0)),
        Arc::new(Material::default()),
    )
    .unwrap();

    let mag_with_center_thickness = ArcParallelMagnet::with_center_thickness(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(50.0),
        side_height,
        Length::new::<millimeter>(10.0),
        AngleOrWidth::Width(Length::new::<millimeter>(30.0)),
        Arc::new(Material::default()),
    )
    .unwrap();

    approx::assert_abs_diff_eq!(
        side_height.get::<meter>(),
        mag_new.side_height().get::<meter>(),
    );
    approx::assert_abs_diff_eq!(
        mag_new.side_height().get::<meter>(),
        mag_with_const_thickness.side_height().get::<meter>(),
    );
    approx::assert_abs_diff_eq!(
        mag_new.side_height().get::<meter>(),
        mag_with_center_thickness.side_height().get::<meter>(),
    );

    approx::assert_abs_diff_eq!(
        mag_new.air_gap_radius().get::<meter>(),
        0.06,
        epsilon = 1e-12
    );
    approx::assert_abs_diff_eq!(
        mag_new.air_gap_radius().get::<meter>(),
        mag_with_const_thickness.air_gap_radius().get::<meter>(),
        epsilon = 1e-12,
    );
    approx::assert_abs_diff_eq!(
        mag_new.air_gap_radius().get::<meter>(),
        mag_with_center_thickness.air_gap_radius().get::<meter>(),
        epsilon = 1e-12,
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

    approx::assert_abs_diff_eq!(
        mag_new.side_thickness().get::<meter>(),
        0.01,
        epsilon = 1e-3
    );
    approx::assert_abs_diff_eq!(
        mag_new.side_thickness().get::<meter>(),
        mag_with_const_thickness.side_thickness().get::<meter>(),
    );
    approx::assert_abs_diff_eq!(
        mag_new.side_thickness().get::<meter>(),
        mag_with_center_thickness.side_thickness().get::<meter>(),
    );

    approx::assert_abs_diff_eq!(mag_new.thickness().get::<meter>(), 0.01, epsilon = 1e-3);
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
    let side_height = SideHeightOrThickness::Thickness(Length::new::<millimeter>(10.0)).height(
        Length::new::<millimeter>(-60.0),
        Length::new::<millimeter>(30.0).into(),
    );

    let mag_new = ArcParallelMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(-60.0),
        Length::new::<millimeter>(-50.0),
        side_height,
        Length::new::<millimeter>(30.0).into(),
        Arc::new(Material::default()),
    )
    .unwrap();

    let mag_with_const_thickness = ArcParallelMagnet::with_const_thickness(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(-60.0),
        SideHeightOrThickness::Thickness(Length::new::<millimeter>(10.0)),
        Length::new::<millimeter>(30.0).into(),
        Arc::new(Material::default()),
    )
    .unwrap();

    let mag_with_center_thickness = ArcParallelMagnet::with_center_thickness(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(-60.0),
        side_height,
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(30.0).into(),
        Arc::new(Material::default()),
    )
    .unwrap();

    approx::assert_abs_diff_eq!(
        side_height.get::<meter>(),
        mag_new.side_height().get::<meter>(),
    );
    approx::assert_abs_diff_eq!(
        mag_new.side_height().get::<meter>(),
        mag_with_const_thickness.side_height().get::<meter>(),
    );
    approx::assert_abs_diff_eq!(
        mag_new.side_height().get::<meter>(),
        mag_with_center_thickness.side_height().get::<meter>(),
    );

    approx::assert_abs_diff_eq!(
        mag_new.air_gap_radius().get::<meter>(),
        -0.05,
        epsilon = 1e-12
    );
    approx::assert_abs_diff_eq!(
        mag_new.air_gap_radius().get::<meter>(),
        mag_with_const_thickness.air_gap_radius().get::<meter>(),
        epsilon = 1e-12,
    );
    approx::assert_abs_diff_eq!(
        mag_new.air_gap_radius().get::<meter>(),
        mag_with_center_thickness.air_gap_radius().get::<meter>(),
        epsilon = 1e-12,
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

    approx::assert_abs_diff_eq!(mag_new.side_thickness().get::<meter>(), 0.01);
    approx::assert_abs_diff_eq!(
        mag_new.side_thickness().get::<meter>(),
        mag_with_const_thickness.side_thickness().get::<meter>(),
    );
    approx::assert_abs_diff_eq!(
        mag_new.side_thickness().get::<meter>(),
        mag_with_center_thickness.side_thickness().get::<meter>(),
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
fn test_draw_arc_parallel() {
    {
        let magnet = ArcParallelMagnet::with_const_thickness(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(50.0),
            SideHeightOrThickness::Height(Length::new::<millimeter>(10.0)),
            AngleOrWidth::Width(Length::new::<millimeter>(40.0)),
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
                "tests/img/arc_parallel_inner_split_const_thickness.png",
                callback,
                0.99
            )
            .is_ok()
        );
    }
    {
        let magnet = ArcParallelMagnet::with_const_thickness(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(-60.0),
            SideHeightOrThickness::Height(Length::new::<millimeter>(10.0)),
            AngleOrWidth::Angle(PI / 6.0),
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
                "tests/img/arc_parallel_outer_split_const_thickness.png",
                callback,
                0.99
            )
            .is_ok()
        );
    }
    {
        let magnet = ArcParallelMagnet::with_const_thickness(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(50.0),
            SideHeightOrThickness::Thickness(Length::new::<millimeter>(10.0)),
            AngleOrWidth::Angle(PI / 6.0),
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
                "tests/img/arc_parallel_inner_split_side_thickness.png",
                callback,
                0.99
            )
            .is_ok()
        );
    }
    {
        let magnet = ArcParallelMagnet::with_const_thickness(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(-60.0),
            SideHeightOrThickness::Thickness(Length::new::<millimeter>(10.0)),
            AngleOrWidth::Angle(PI / 6.0),
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
                "tests/img/arc_parallel_outer_split_side_thickness.png",
                callback,
                0.99
            )
            .is_ok()
        );
    }
    {
        let magnet = ArcParallelMagnet::new(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(50.0),
            Length::new::<millimeter>(40.0),
            Length::new::<millimeter>(10.0),
            AngleOrWidth::Angle(PI / 6.0),
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
                "tests/img/arc_parallel_inner_split_convex.png",
                callback,
                0.99
            )
            .is_ok()
        );
    }
    {
        let magnet = ArcParallelMagnet::new(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(-60.0),
            Length::new::<millimeter>(-70.0),
            Length::new::<millimeter>(10.0),
            AngleOrWidth::Angle(PI / 6.0),
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
                "tests/img/arc_parallel_outer_split_convex.png",
                callback,
                0.99
            )
            .is_ok()
        );
    }
    {
        let magnet = ArcParallelMagnet::new(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(60.0),
            Length::new::<millimeter>(30.0),
            Length::new::<millimeter>(0.0),
            AngleOrWidth::Angle(PI / 6.0),
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
                "tests/img/arc_parallel_inner_no_side_thickness.png",
                callback,
                0.99
            )
            .is_ok()
        );
    }
    {
        let magnet = ArcParallelMagnet::new(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(-60.0),
            Length::new::<millimeter>(-1000000000.0),
            Length::new::<millimeter>(10.0),
            AngleOrWidth::Angle(PI / 6.0),
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
        assert!(compare_or_create("tests/img/arc_parallel_outer_flat.png", callback, 0.99).is_ok());
    }
    {
        let magnet = ArcParallelMagnet::with_center_thickness(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(50.0),
            Length::new::<millimeter>(10.0),
            Length::new::<millimeter>(10.0),
            AngleOrWidth::Angle(PI / 6.0),
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
                "tests/img/arc_parallel_inner_center_thickness.png",
                callback,
                0.99
            )
            .is_ok()
        );
    }
    {
        let magnet = ArcParallelMagnet::with_center_thickness(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(-60.0),
            Length::new::<millimeter>(10.0),
            Length::new::<millimeter>(10.0),
            AngleOrWidth::Angle(PI / 6.0),
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
                "tests/img/arc_parallel_outer_center_thickness.png",
                callback,
                0.99
            )
            .is_ok()
        );
    }
    {
        let magnet = ArcParallelMagnet::with_center_thickness(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(-60.0),
            Length::new::<millimeter>(10.0),
            Length::new::<millimeter>(10.0),
            AngleOrWidth::Angle(PI / 6.0),
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
                "tests/img/arc_parallel_outer_center_thickness_concave.png",
                callback,
                0.99
            )
            .is_ok()
        );
    }
    {
        let magnet = ArcParallelMagnet::with_center_thickness(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(-60.0),
            Length::new::<millimeter>(10.0),
            Length::new::<millimeter>(15.0),
            AngleOrWidth::Angle(PI / 6.0),
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
                "tests/img/arc_parallel_outer_center_thickness_convex.png",
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

    let inner_magnet = ArcParallelMagnet::with_const_thickness(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(50.0),
        SideHeightOrThickness::Thickness(Length::new::<millimeter>(10.0)),
        AngleOrWidth::Width(Length::new::<millimeter>(50.0)),
        Arc::new(Default::default()),
    )
    .unwrap();

    let outer_magnet = ArcParallelMagnet::with_const_thickness(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(-80.0),
        SideHeightOrThickness::Thickness(Length::new::<millimeter>(10.0)),
        AngleOrWidth::Width(Length::new::<millimeter>(50.0)),
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
    assert!(
        compare_or_create("tests/img/arc_parallel_inner_and_outer.png", callback, 0.99).is_ok()
    );
}

#[test]
fn serialize_and_deserialize() {
    let magnet = ArcParallelMagnet::with_const_thickness(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(80.0),
        SideHeightOrThickness::Thickness(Length::new::<millimeter>(10.0)),
        AngleOrWidth::Angle(PI / 6.0),
        Arc::new(Default::default()),
    )
    .unwrap();

    let string = serde_yaml::to_string(&magnet).unwrap();
    let de_magnet: ArcParallelMagnet = serde_yaml::from_str(&string).unwrap();

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
        side_height: 10 mm
        angle: PI/6 rad
        material:
            name: NMF-12J 430mT
            relative_permeability: 1.05
        "};
        let magnet: ArcParallelMagnet = serde_yaml::from_str(str).unwrap();

        approx::assert_abs_diff_eq!(
            magnet.area().get::<square_millimeter>(),
            440.993,
            epsilon = 1e-3
        );
    }
    {
        let str = indoc::indoc! {"
        length: 165 mm
        core_radius: 80 mm
        air_gap_radius: 60 mm
        side_height: 10 mm
        width: 40 mm
        material:
            name: NMF-12J 430mT
            relative_permeability: 1.05
        "};
        let magnet: ArcParallelMagnet = serde_yaml::from_str(str).unwrap();

        approx::assert_abs_diff_eq!(
            magnet.area().get::<square_millimeter>(),
            424.081,
            epsilon = 1e-3
        );
    }
    {
        let str = indoc::indoc! {"
        length: 165 mm
        core_radius: 80 mm
        side_height: 10 mm
        angle: PI/6 rad
        material:
            name: NMF-12J 430mT
            relative_permeability: 1.05
        "};
        let magnet: ArcParallelMagnet = serde_yaml::from_str(str).unwrap();

        approx::assert_abs_diff_eq!(
            magnet.area().get::<square_millimeter>(),
            405.657,
            epsilon = 1e-3
        );
    }
    {
        let str = indoc::indoc! {"
        length: 165 mm
        core_radius: 80 mm
        side_height: 10 mm
        width: 40 mm
        material:
            name: NMF-12J 430mT
            relative_permeability: 1.05
        "};
        let magnet: ArcParallelMagnet = serde_yaml::from_str(str).unwrap();

        approx::assert_abs_diff_eq!(
            magnet.area().get::<square_millimeter>(),
            392.396,
            epsilon = 1e-3
        );
    }
    {
        let str = indoc::indoc! {"
        length: 165 mm
        core_radius: 80 mm
        thickness: 10 mm
        angle: PI/6 rad
        material:
            name: NMF-12J 430mT
            relative_permeability: 1.05
        "};
        let magnet: ArcParallelMagnet = serde_yaml::from_str(str).unwrap();

        approx::assert_abs_diff_eq!(
            magnet.area().get::<square_millimeter>(),
            418.334,
            epsilon = 1e-3
        );
    }
    {
        let str = indoc::indoc! {"
        length: 165 mm
        core_radius: 80 mm
        thickness: 10 mm
        width: 40 mm
        material:
            name: NMF-12J 430mT
            relative_permeability: 1.05
        "};
        let magnet: ArcParallelMagnet = serde_yaml::from_str(str).unwrap();

        approx::assert_abs_diff_eq!(
            magnet.area().get::<square_millimeter>(),
            403.799,
            epsilon = 1e-3
        );
    }
    {
        let str = indoc::indoc! {"
        length: 165 mm
        core_radius: 80 mm
        center_thickness: 15 mm
        side_height: 10 mm
        angle: PI/6 rad
        material:
            name: NMF-12J 430mT
            relative_permeability: 1.05
        "};
        let magnet: ArcParallelMagnet = serde_yaml::from_str(str).unwrap();

        approx::assert_abs_diff_eq!(
            magnet.area().get::<square_millimeter>(),
            557.713,
            epsilon = 1e-3
        );
    }
    {
        let str = indoc::indoc! {"
        length: 165 mm
        core_radius: 80 mm
        center_thickness: 15 mm
        side_height: 10 mm
        width: 40 mm
        material:
            name: NMF-12J 430mT
            relative_permeability: 1.05
        "};
        let magnet: ArcParallelMagnet = serde_yaml::from_str(str).unwrap();

        approx::assert_abs_diff_eq!(
            magnet.area().get::<square_millimeter>(),
            538.720,
            epsilon = 1e-3
        );
    }
}
