/*!
This module offers arc magnet types (via submodules which are reexported).
- [`ArcParallelMagnet`]: An arc magnet where both sides are parallel (module
[parallel]).
- [`ArcSegmentMagnet`]: An arc magnet where both sides are radially oriented
(module [segment]).

Some of the constructor methods of [`ArcParallelMagnet`] use the enums
[`SideHeightOrThickness`] and [`AngleOrWidth`]. These enums can represent two
different physical dimensions which equivalently define the magnet. See their
docstrings for more.
 */

use std::f64::consts::{FRAC_PI_2, PI};

use compare_variables::compare_variables;
use planar_geo::prelude::*;
use stem_material::prelude::*;

pub mod parallel;
pub use parallel::*;

pub mod segment;
pub use segment::*;

/// Helper function for creating arc magnets
fn core_and_air_gap_from_air_gap_arc_radius(
    core_radius: Length,
    air_gap_radius: Length,
    offset: [f64; 2],
    angle: f64,
) -> Result<[ArcSegment; 2], crate::error::Error> {
    compare_variables!(0.0 < angle <= PI)?;

    let zero = Length::new::<meter>(0.0);
    compare_variables!(val zero != core_radius)?;
    compare_variables!(val zero != air_gap_radius)?;

    let e = DEFAULT_EPSILON;
    let m = DEFAULT_MAX_ULPS;

    let start_angle = if core_radius.is_sign_positive() {
        FRAC_PI_2 - 0.5 * angle
    } else {
        3.0 * FRAC_PI_2 - 0.5 * angle
    };

    let core_arc = ArcSegment::from_center_radius_start_offset_angle(
        [0.0, 0.0],
        core_radius.get::<meter>().abs(),
        start_angle,
        angle,
        e,
        m,
    )?;

    let air_gap_arc_start = [
        core_arc.stop()[0] - offset[0],
        core_arc.stop()[1] + offset[1],
    ];
    let air_gap_arc_stop = [
        core_arc.start()[0] + offset[0],
        core_arc.start()[1] + offset[1],
    ];

    let positive = if core_radius.is_sign_positive() {
        air_gap_radius.is_sign_negative()
    } else {
        air_gap_radius.is_sign_positive()
    };

    let air_gap_arc = ArcSegment::from_start_stop_radius(
        air_gap_arc_start,
        air_gap_arc_stop,
        air_gap_radius.get::<meter>().abs(),
        positive,
        false,
        e,
        m,
    )?;

    return Ok([core_arc, air_gap_arc]);
}

/// Helper function for creating arc magnets
fn core_and_air_gap_arc_from_center_thickness(
    core_radius: Length,
    center_thickness: Length,
    offset: [f64; 2],
    angle: f64,
) -> Result<[ArcSegment; 2], crate::error::Error> {
    let zero = Length::new::<meter>(0.0);
    compare_variables!(val zero < center_thickness)?;

    let e = DEFAULT_EPSILON;
    let m = DEFAULT_MAX_ULPS;

    let start_angle = if core_radius.is_sign_positive() {
        FRAC_PI_2 - 0.5 * angle
    } else {
        3.0 * FRAC_PI_2 - 0.5 * angle
    };

    let core_arc = ArcSegment::from_center_radius_start_offset_angle(
        [0.0, 0.0],
        core_radius.get::<meter>().abs(),
        start_angle,
        angle,
        e,
        m,
    )?;

    let ag_end_pt_height = core_arc.stop()[1] + offset[1];

    let air_gap_arc_start = [core_arc.stop()[0] - offset[0], ag_end_pt_height];
    let air_gap_arc_stop = [core_arc.start()[0] + offset[0], ag_end_pt_height];

    let air_gap_arc = ArcSegment::from_start_middle_stop(
        air_gap_arc_start,
        [0.0, (core_radius + center_thickness).get::<meter>()],
        air_gap_arc_stop,
        e,
        m,
    )?;
    return Ok([core_arc, air_gap_arc]);
}

/// Returns `[shape, north_shape, south_shape]`.
fn shapes(
    mut core_arc: ArcSegment,
    mut air_gap_arc: ArcSegment,
    offset: [f64; 2],
    inner: bool,
) -> Result<[Shape; 3], crate::error::Error> {
    let core_radius = core_arc.radius();
    let shift = if inner {
        [0.0, -core_radius]
    } else {
        [0.0, core_radius]
    };
    core_arc.translate(shift);
    air_gap_arc.translate(shift);

    let polysegment =
        Polysegment::from_iter([core_arc.clone().into(), air_gap_arc.clone().into()].into_iter());
    let shape = Shape::from_outer(polysegment.into())?;

    let half_offset = [offset[0] * 0.5, offset[1] * 0.5];

    let arc_mean_start = [
        core_arc.stop()[0] - half_offset[0],
        core_arc.stop()[1] + half_offset[1],
    ];
    let arc_mean_stop = [
        core_arc.start()[0] + half_offset[0],
        core_arc.start()[1] + half_offset[1],
    ];

    let halfway_height = air_gap_arc.segment_point(0.5)[1];
    let middle = [0.0, halfway_height * 0.5];
    let mut mean_gap_arc = ArcSegment::from_start_middle_stop(
        arc_mean_start,
        middle,
        arc_mean_stop,
        DEFAULT_EPSILON,
        DEFAULT_MAX_ULPS,
    )?;
    let polysegment =
        Polysegment::from_iter([core_arc.into(), mean_gap_arc.clone().into()].into_iter());
    let north_shape = Shape::from_outer(polysegment.into())?;

    mean_gap_arc.reverse();

    let polysegment = Polysegment::from_iter([mean_gap_arc.into(), air_gap_arc.into()].into_iter());
    let south_shape = Shape::from_outer(polysegment.into())?;

    return Ok([shape, north_shape, south_shape]);
}
