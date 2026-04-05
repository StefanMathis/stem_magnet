use std::{f64::consts::PI, sync::Arc};

use cairo_viewport::*;
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
        BlockMagnet::new(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(20.0),
            Length::new::<millimeter>(10.0),
            Length::new::<millimeter>(-1000.0),
            mat_arc.clone()
        )
        .is_err()
    ); // Fillet too small
    assert!(
        BlockMagnet::new(
            Length::new::<millimeter>(165.0),
            Length::new::<millimeter>(20.0),
            Length::new::<millimeter>(10.0),
            Length::new::<millimeter>(1000.0),
            mat_arc.clone()
        )
        .is_err()
    ); // Fillet too large
}

#[test]
fn test_block_magnet_no_fillet() {
    let mat_arc = Arc::new(ferrite());

    let magnet = BlockMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(20.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(0.0),
        mat_arc,
    )
    .unwrap();

    approx::assert_abs_diff_eq!(magnet.area().get::<square_meter>(), magnet.shape().area());
    assert_eq!(magnet.volume().get::<cubic_meter>(), 0.165 * 10e-3 * 20e-3);
    assert_eq!(magnet.mass().get::<kilogram>(), 0.165);

    approx::assert_abs_diff_eq!(
        magnet
            .magnetomotive_force(&[ThermodynamicTemperature::new::<degree_celsius>(20.0).into()])
            .get::<ampere>(),
        3258.887,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        magnet
            .magnetomotive_force(&[ThermodynamicTemperature::new::<degree_celsius>(120.0).into()])
            .get::<ampere>(),
        2694.122,
        epsilon = 0.001
    );
}

#[test]

fn block_magnet_no_fillet_visualize() {
    let material = Material::default();
    let magnet = BlockMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(20.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(0.0),
        Arc::new(material),
    )
    .unwrap();

    // Image comparison
    let drawables = magnet.drawables(false, false);

    let view = Viewport::from_bounded_entities(drawables.iter(), SideLength::Long(500)).unwrap();
    let path = std::path::Path::new("tests/img/block_single_no_fillet.png");

    // Always compare to the same reference image
    let callback = |path: &std::path::Path| {
        return view.write_to_file(path, move |cr| {
            cr.set_source_rgb(1.0, 1.0, 1.0);
            for drawable in drawables.iter() {
                drawable.draw(&cr)?;
            }
            return Ok(());
        });
    };
    assert!(compare_or_create(path, callback, 0.95).is_ok());

    let drawables = magnet.drawables(true, false);

    let view = Viewport::from_bounded_entities(drawables.iter(), SideLength::Long(500)).unwrap();
    let path = std::path::Path::new("tests/img/block_split_no_fillet.png");

    // Always compare to the same reference image
    let callback = |path: &std::path::Path| {
        return view.write_to_file(path, move |cr| {
            cr.set_source_rgb(1.0, 1.0, 1.0);
            for drawable in drawables.iter() {
                drawable.draw(&cr)?;
            }
            return Ok(());
        });
    };
    assert!(compare_or_create(path, callback, 0.95).is_ok());
}

#[test]
fn test_block_magnet_fillet() {
    let mat_arc = Arc::new(ferrite());

    let magnet = BlockMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(20.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(1.0),
        mat_arc,
    )
    .unwrap();

    approx::assert_abs_diff_eq!(magnet.area().get::<square_meter>(), magnet.shape().area());
    assert_eq!(
        magnet.volume().get::<cubic_meter>(),
        0.165 * (10e-3 * 20e-3 - 4.0e-6 + PI * 1e-3f64.powi(2))
    );
    approx::assert_abs_diff_eq!(magnet.mass().get::<kilogram>(), 0.16429, epsilon = 0.001);

    approx::assert_abs_diff_eq!(
        magnet
            .magnetomotive_force(&[ThermodynamicTemperature::new::<degree_celsius>(20.0).into()])
            .get::<ampere>(),
        3258.887,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        magnet
            .magnetomotive_force(&[ThermodynamicTemperature::new::<degree_celsius>(120.0).into()])
            .get::<ampere>(),
        2694.122,
        epsilon = 0.001
    );
}

#[test]
fn test_block_magnet_fillet_visualize() {
    let material = Material::default();
    let magnet = BlockMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(20.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(1.0),
        Arc::new(material),
    )
    .unwrap();

    // Image comparison
    let drawables = magnet.drawables(false, false);

    let view = Viewport::from_bounded_entities(drawables.iter(), SideLength::Long(500)).unwrap();
    let path = std::path::Path::new("tests/img/block_single_fillet.png");

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
    let path = std::path::Path::new("tests/img/block_split_fillet.png");

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
