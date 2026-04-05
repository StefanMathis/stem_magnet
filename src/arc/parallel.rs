/*!
This module defines an [`ArcParallelMagnet`] - a curved magnet where the magnet
sides are parallel. See the struct documentation for more.
 */

use std::{borrow::Cow, sync::Arc};
use stem_material::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
use serde_mosaic::serialize_arc_link;

use compare_variables::compare_variables;

use planar_geo::prelude::*;

use super::*;
use crate::magnet::Magnet;

/**
A helper enum for creating an [`ArcParallelMagnet`] representing either its
angle or width value.

The x-extents of an [`ArcParallelMagnet`] can be defined either by its `width`
or the `angle` covered by the arc segment which defines the contact surface
to the rotor. This enum is used as input to the constructor functions available
to [`ArcParallelMagnet`] in order to allow using either. They are related via
the following equation:

`width = 2 * abs(core_radius) * sin(angle / 2)`

The two enum methods [`AngleOrWidth::angle`] and [`AngleOrWidth::width`]
automate this conversion within the magnet constructors.
*/
#[doc = ""]
#[cfg_attr(
    feature = "doc-images",
    doc = "![Options for defining an arc magnet][drawing_arc_parallel_enums]"
)]
#[cfg_attr(
    feature = "doc-images",
    embed_doc_image::embed_doc_image(
        "drawing_arc_parallel_enums",
        "docs/img/drawing_arc_parallel_enums.svg"
    )
)]
#[cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile docs with
    `cargo doc --features 'doc-images'` and Rust version >= 1.54."
)]
/**

This enum also provides [`From`] converters for [`f64`] and [`Length`]:
- A [`f64`] is wrapped in [`AngleOrWidth::Angle`].
- A [`Length`] is wrapped in [`AngleOrWidth::Width`].
 */
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum AngleOrWidth {
    /// Angle covered by the core contact surface in radians.
    Angle(f64),
    /// Width of the core contact surface.
    Width(Length),
}

impl AngleOrWidth {
    /**
    Returns the `width` represented by the enum:
    - If `self` is [`AngleOrWidth::Angle`], the value is calculated as
    `width = 2 * abs(core_radius) * sin(angle / 2)`
    - If `self` is [`AngleOrWidth::Width`], the underlying value is simply
    returned.

    # Examples

    ```
    use std::f64::consts::PI;
    use approx;
    use stem_magnet::prelude::*;

    let w = AngleOrWidth::Angle(PI).width(Length::new::<meter>(1.0));
    approx::assert_abs_diff_eq!(w.get::<meter>(), 2.0);

    let w = AngleOrWidth::Width(Length::new::<meter>(2.0)).width(Length::new::<meter>(1.0));
    approx::assert_abs_diff_eq!(w.get::<meter>(), 2.0);
    ```
     */
    pub fn width(&self, radius: Length) -> Length {
        match self {
            AngleOrWidth::Angle(angle) => 2.0 * radius.abs() * (0.5 * *angle).sin(),
            AngleOrWidth::Width(width) => *width,
        }
    }

    /**
    Returns the `angle` represented by the enum:
    - If `self` is [`AngleOrWidth::Angle`], the underlying value is simply
    returned.
    - If `self` is [`AngleOrWidth::Width`], the value is calculated as
    `angle = 2 * arcsin(width/(2*abs(core_radius)))`

    # Examples

    ```
    use std::f64::consts::PI;
    use approx;
    use stem_magnet::prelude::*;

    let a = AngleOrWidth::Angle(PI).angle(Length::new::<meter>(1.0));
    approx::assert_abs_diff_eq!(a, PI);

    let a = AngleOrWidth::Width(Length::new::<meter>(2.0)).angle(Length::new::<meter>(1.0));
    approx::assert_abs_diff_eq!(a, PI);
    ```
     */
    pub fn angle(&self, radius: Length) -> f64 {
        match self {
            AngleOrWidth::Angle(angle) => *angle,
            AngleOrWidth::Width(width) => {
                2.0 * (0.5 * (*width / radius.abs()).get::<ratio>()).asin()
            }
        }
    }
}

impl From<f64> for AngleOrWidth {
    fn from(value: f64) -> Self {
        return AngleOrWidth::Angle(value);
    }
}

impl From<Length> for AngleOrWidth {
    fn from(value: Length) -> Self {
        return AngleOrWidth::Width(value);
    }
}

/**
A helper enum for creating an [`ArcParallelMagnet`] with constant thickness
representing either its side height or thickness.

If the magnet is constructed with [`ArcParallelMagnet::with_const_thickness`]
(and therefore has a constant thickness), its side height can alternatively be
derived from the magnet thickness via the following relationship:

`side_height = sqrt((abs(core_radius) + thickness)² - (0.5*width)²) - abs(core_radius) * cos(angle/2)`

The two enum methods [`SideHeightOrThickness::height`] and
[`SideHeightOrThickness::thickness`] automate this conversion within the magnet
constructor. These methods do not sanity-check the input (for example, the
height or thickness must be positive), but the constructor methods (which is the
only place where the enum methods are used in stem) do check the
resulting geometry. However, if the enum methods are called in custom code, it
is recommended to sanity-check the inputs or the output.
*/
#[doc = ""]
#[cfg_attr(
    feature = "doc-images",
    doc = "![Options for defining an arc magnet][drawing_arc_parallel_enums]"
)]
#[cfg_attr(
    feature = "doc-images",
    embed_doc_image::embed_doc_image(
        "drawing_arc_parallel_enums",
        "docs/img/drawing_arc_parallel_enums.svg"
    )
)]
#[cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile docs with
    `cargo doc --features 'doc-images'` and Rust version >= 1.54."
)]
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum SideHeightOrThickness {
    /// Side height of the magnet.
    Height(Length),
    /// Thickness of the magnet.
    Thickness(Length),
}

impl SideHeightOrThickness {
    /**
    Returns the `side_height` represented by the enum:
    - If `self` is [`SideHeightOrThickness::Height`], the underlying value is
    simply returned.
    - If `self` is [`SideHeightOrThickness::Thickness`], the value is calculated
    as `side_height = abs(sqrt((core_radius + thickness)² - (0.5*width)²) - abs(core_radius) * cos(angle/2))`.

    # Examples

    ```
    use std::f64::consts::PI;
    use approx;
    use stem_magnet::prelude::*;

    let r = Length::new::<meter>(1.0);
    let w = AngleOrWidth::Width(Length::new::<meter>(0.1));

    let h = SideHeightOrThickness::Height(Length::new::<meter>(0.1)).height(r, w);
    approx::assert_abs_diff_eq!(h.get::<meter>(), 0.1);

    let h = SideHeightOrThickness::Thickness(Length::new::<meter>(0.1)).height(r, w);
    approx::assert_abs_diff_eq!(h.get::<meter>(), 0.10011, epsilon=1e-3);

    // Negative radius
    let h = SideHeightOrThickness::Thickness(Length::new::<meter>(0.1)).height(-r, w);
    approx::assert_abs_diff_eq!(h.get::<meter>(), 0.10011, epsilon=1e-3);
    ```
     */
    pub fn height(&self, core_radius: Length, angle_or_width: AngleOrWidth) -> Length {
        match self {
            SideHeightOrThickness::Height(height) => *height,
            SideHeightOrThickness::Thickness(thickness) => {
                use uom::typenum::P2;
                let width = angle_or_width.width(core_radius);
                let angle = angle_or_width.angle(core_radius);

                let cos_ag_r = ((core_radius + *thickness).powi(P2::new())
                    - (0.5 * width).powi(P2::new()))
                .sqrt();
                (cos_ag_r - core_radius.abs() * (0.5 * angle).cos()).abs()
            }
        }
    }

    /**
    Returns the `thickness` represented by the enum:
    - If `self` is [`SideHeightOrThickness::Height`], the value is calculated as
    `thickness = abs(((side_height + core_radius * cos(angle/2))² + (0.5*width)²).sqrt() - abs(core_radius))`
    - If `self` is [`SideHeightOrThickness::Thickness`], the underlying value is
    simply returned.

    # Examples

    ```
    use std::f64::consts::PI;
    use approx;
    use stem_magnet::prelude::*;

    let r = Length::new::<meter>(1.0);
    let a = AngleOrWidth::Angle(PI);

    let h = SideHeightOrThickness::Height(Length::new::<meter>(3.0f64.sqrt())).thickness(r, a);
    approx::assert_abs_diff_eq!(h.get::<meter>(), 1.0);

    let h = SideHeightOrThickness::Thickness(Length::new::<meter>(1.0)).thickness(r, a);
    approx::assert_abs_diff_eq!(h.get::<meter>(), 1.0);

    // Negative radius
    let h = SideHeightOrThickness::Height(Length::new::<meter>(3.0f64.sqrt())).thickness(-r, a);
    approx::assert_abs_diff_eq!(h.get::<meter>(), 1.0);
    ```
     */
    pub fn thickness(&self, core_radius: Length, angle_or_width: AngleOrWidth) -> Length {
        match self {
            SideHeightOrThickness::Height(height) => {
                use uom::typenum::P2;
                let width = angle_or_width.width(core_radius);
                let angle = angle_or_width.angle(core_radius);
                let cos_ag_r = core_radius * (0.5 * angle).cos() + *height;
                ((cos_ag_r.powi(P2::new()) + (0.5 * width).powi(P2::new())).sqrt()
                    - core_radius.abs())
                .abs()
            }
            SideHeightOrThickness::Thickness(thickness) => *thickness,
        }
    }
}

/**
A curved permanent magnet where the sides are parallel.

It is closely related to the [`ArcSegmentMagnet`], which features radially
oriented sides instead.

# Geometry

This magnet type is meant to be used with rotary motors and can be seen as the
arced counterpart to the [`BlockMagnet`](crate::block::BlockMagnet). It is
comparably expensive to manufacture, but delivers a smooth and constant
excitation along the circumference of the magnetic core and can be easily
mounted.
*/
#[doc = ""]
#[cfg_attr(
    feature = "doc-images",
    doc = "![Arc parallel magnet definitions][drawing_arc_parallel_magnet]"
)]
#[cfg_attr(
    feature = "doc-images",
    embed_doc_image::embed_doc_image(
        "drawing_arc_parallel_magnet",
        "docs/img/drawing_arc_parallel_magnet.svg"
    )
)]
#[cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile docs with
    `cargo doc --features 'doc-images'` and Rust version >= 1.54."
)]
/**
An arc parallel magnet is defined by the following parameters:
- `length`: Axial length of the magnet. Must be positive (`length > 0 m`).
- `side_height`: Height of the magnet at its sides. Must be positive or
zero (`side_height >= 0 m`). If the magnet has constant thickness,
alternatively the `thickness` can be specified (see [`SideHeightOrThickness`]).
The thickness must be positive (`thickness >= 0 m`)
- `core_radius`: Radius at the surface facing the core. If this value is
positive, the magnet is convex and suited for an inner rotor, otherwise it is
concave and meant for an outer rotor (as shown in the image above). Must not be
zero (`air_gap_radius != 0 m`)
- One of `air_gap_radius` or `center_thickness` to define the air gap arc
segment and hence the shape of the magnetic field:
    - `air_gap_radius`: Radius at the surface facing the air gap. If this value
    is equal to `core_radius + side_thickness`, the resulting magnet has a
    constant thickness (as shown on the right side of the image above). Like
    with `core_radius`, the arc is convex if `air_gap_radius` is positive and
    concave if `air_gap_radius` is negative. Must not be zero
    (`air_gap_radius != 0 m`)
    - `center_thickness`: Thickness of the magnet at its center. Must be
    positive (`center_thickness > 0 m`)
- One of `angle` or `width` to specify the horizontal extents of the magnet (see
[`AngleOrWidth`]):
    - `angle`: The angle covered by this magnet in radians. Must be positive and
    smaller than or equal to π (`0 < angle <= π`).
    - `width`: Distance of the two parallel magnet sides. Must be positive
    (`width > 0 m`)
- `material`: The material of the magnet.

The influence of the radii signs on the magnet form is shown in the following
image:
*/
#[doc = ""]
#[cfg_attr(
    feature = "doc-images",
    doc = "![Magnet form dependence on radius][drawing_arc_segment_magnet_pos_neg_radii]"
)]
#[cfg_attr(
    feature = "doc-images",
    embed_doc_image::embed_doc_image(
        "drawing_arc_segment_magnet_pos_neg_radii",
        "docs/img/drawing_arc_segment_magnet_pos_neg_radii.svg"
    )
)]
#[cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile docs with
    `cargo doc --features 'doc-images'` and Rust version >= 1.54."
)]
/**

By varying `air_gap_radius`, a lot of different magnet shapes can be realized:
*/
#[doc = ""]
#[cfg_attr(
    feature = "doc-images",
    doc = "![Different arc parallel shapes][arc_parallel_vary_air_gap_radius]"
)]
#[cfg_attr(
    feature = "doc-images",
    embed_doc_image::embed_doc_image(
        "arc_parallel_vary_air_gap_radius",
        "docs/img/arc_parallel_vary_air_gap_radius.svg"
    )
)]
#[cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile docs with
    `cargo doc --features 'doc-images'` and Rust version >= 1.54."
)]
/**

# Constructors

The following constructors are available for this struct:
- [`new`](ArcParallelMagnet::new): Creates a magnet from its `length`,
`side_height`, `core_radius`, `air_gap_radius`, `angle` / `width` and `material`.
- [`with_const_thickness`](ArcParallelMagnet::with_const_thickness): Like `new`,
but the `air_gap_radius` is calculated from the `side_height` or alternatively
from the `thickness` (see [`SideHeightOrThickness`]), resulting in a magnet with
constant thickness.
- [`with_center_thickness`](ArcParallelMagnet::with_center_thickness): Instead
of specifying the `air_gap_radius`, it is also possible to provide the
`center_thickness` instead.

# Deserialization

For each of the aforementioned constructors, there exists a serialized
representation:

## `new`

```
use stem_magnet::prelude::*;
use serde_yaml;
use approx;

let str = indoc::indoc! {"
length: 165 mm
core_radius: 80 mm
air_gap_radius: 90 mm
side_height: 10 mm
width: 50 mm # Alternatively, the `angle` could be given here
material:
    name: NMF-12J 430mT
    relative_permeability: 1.05
"};
let magnet: ArcParallelMagnet = serde_yaml::from_str(str).expect("valid magnet");
approx::assert_abs_diff_eq!(magnet.air_gap_radius().get::<meter>(), 0.09, epsilon = 1e-3);
approx::assert_abs_diff_eq!(magnet.area().get::<square_millimeter>(), 484.300, epsilon = 1e-3);
```

## `with_const_thickness`

```
use stem_magnet::prelude::*;
use serde_yaml;
use approx;

let str = indoc::indoc! {"
length: 165 mm
core_radius: 80 mm
thickness: 10 mm # Alternatively, the `side_height` could be given here
width: 50 mm # Alternatively, the `angle` could be given here
material:
    name: NMF-12J 430mT
    relative_permeability: 1.05
"};
let magnet: ArcParallelMagnet = serde_yaml::from_str(str).expect("valid magnet");
approx::assert_abs_diff_eq!(magnet.air_gap_radius().get::<meter>(), 0.09, epsilon = 1e-3);
approx::assert_abs_diff_eq!(magnet.area().get::<square_millimeter>(), 507.533, epsilon = 1e-3);
```

## `with_center_thickness`

```
use stem_magnet::prelude::*;
use serde_yaml;
use approx;

let str = indoc::indoc! {"
length: 165 mm
core_radius: 80 mm
side_height: 10 mm
center_thickness: 10 mm
width: 50 mm # Alternatively, the `angle` could be given here
material:
    name: NMF-12J 430mT
    relative_permeability: 1.05
"};
let magnet: ArcParallelMagnet = serde_yaml::from_str(str).expect("valid magnet");
approx::assert_abs_diff_eq!(magnet.air_gap_radius().get::<meter>(), 0.08, epsilon = 1e-3);
approx::assert_abs_diff_eq!(magnet.area().get::<square_millimeter>(), 499.999, epsilon = 1e-3);
```
*/
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct ArcParallelMagnet {
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_quantity"))]
    length: Length,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_quantity"))]
    side_height: Length,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_quantity"))]
    core_radius: Length,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_quantity"))]
    air_gap_radius: Length,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_quantity"))]
    width: Length,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_arc_link",))]
    material: Arc<Material>,
    #[cfg_attr(feature = "serde", serde(skip))]
    center_thickness: Length,
    #[cfg_attr(feature = "serde", serde(skip))]
    shape: Shape,
    #[cfg_attr(feature = "serde", serde(skip))]
    north_shape: Shape,
    #[cfg_attr(feature = "serde", serde(skip))]
    south_shape: Shape,
}

impl ArcParallelMagnet {
    /**
    Creates a new [`ArcParallelMagnet`] if the given parameters are within the
    valid value ranges defined in the struct docstring.

    # Examples

    ```
    use stem_magnet::prelude::*;
    use std::sync::Arc;

    // Inner rotor magnet
    assert!(ArcParallelMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(50.0),
        Length::new::<millimeter>(60.0),
        Length::new::<millimeter>(10.0),
        1.0.into(), // Is converted into an `AngleOrWidth::Angle`
        Arc::new(Material::default()),
    ).is_ok());

    // Outer rotor magnet
    assert!(ArcParallelMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(-60.0),
        Length::new::<millimeter>(-50.0),
        Length::new::<millimeter>(10.0),
        1.0.into(),
        Arc::new(Material::default()),
    ).is_ok());

    // Angle is negative
    assert!(ArcParallelMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(-60.0),
        Length::new::<millimeter>(50.0),
        Length::new::<millimeter>(10.0),
        (-1.0).into(),
        Arc::new(Material::default()),
    ).is_err());

    // Radius is zero
    assert!(ArcParallelMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(-60.0),
        Length::new::<millimeter>(0.0),
        Length::new::<millimeter>(10.0),
        1.0.into(),
        Arc::new(Material::default()),
    ).is_err());
    ```
     */
    pub fn new(
        length: Length,
        core_radius: Length,
        air_gap_radius: Length,
        side_height: Length,
        angle_or_width: AngleOrWidth,
        material: Arc<Material>,
    ) -> Result<Self, crate::error::Error> {
        let zero = Length::new::<meter>(0.0);
        compare_variables!(val zero < length)?;
        compare_variables!(val zero <= side_height)?;

        let width = angle_or_width.width(core_radius.abs());
        let angle = angle_or_width.angle(core_radius.abs());

        let offset = [0.0, side_height.get::<meter>()];

        let [core_arc, air_gap_arc] =
            core_and_air_gap_from_air_gap_arc_radius(core_radius, air_gap_radius, offset, angle)?;
        let center_thickness =
            Length::new::<meter>(air_gap_arc.segment_point(0.5)[1]) - core_radius;
        let [shape, north_shape, south_shape] = shapes(
            core_arc,
            air_gap_arc,
            offset,
            core_radius.is_sign_positive(),
        )?;

        return Ok(Self {
            length,
            core_radius,
            air_gap_radius,
            width,
            side_height,
            material,
            shape,
            center_thickness,
            north_shape,
            south_shape,
        });
    }

    /**
    Like [`ArcParallelMagnet::new`], but the `air_gap_radius` is calculated as
    `core_radius + side_thickness` (if `core_radius > 0 m` and as
    `core_radius - side_thickness` otherwise. This results in a magnet with
    constant thickness.

    # Examples

    ```
    use stem_magnet::prelude::*;
    use std::sync::Arc;

    // Inner rotor magnet
    let magnet = ArcParallelMagnet::with_const_thickness(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(50.0),
        SideHeightOrThickness::Thickness(Length::new::<millimeter>(10.0)),
        1.0.into(),
        Arc::new(Material::default()),
    ).expect("valid inputs");
    approx::assert_abs_diff_eq!(magnet.air_gap_radius().get::<meter>(), 0.06, epsilon = 1e-3);

    // Outer rotor magnet
    let magnet = ArcParallelMagnet::with_const_thickness(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(-60.0),
        SideHeightOrThickness::Thickness(Length::new::<millimeter>(10.0)),
        1.0.into(),
        Arc::new(Material::default()),
    ).expect("valid inputs");
    approx::assert_abs_diff_eq!(magnet.air_gap_radius().get::<meter>(), -0.05, epsilon = 1e-3);
    ```
    */
    pub fn with_const_thickness(
        length: Length,
        core_radius: Length,
        side_height_or_thickness: SideHeightOrThickness,
        angle_or_width: AngleOrWidth,
        material: Arc<Material>,
    ) -> Result<Self, crate::error::Error> {
        let side_height = side_height_or_thickness.height(core_radius, angle_or_width);
        let thickness = side_height_or_thickness.thickness(core_radius, angle_or_width);
        let air_gap_radius = core_radius + thickness;
        return Self::new(
            length,
            core_radius,
            air_gap_radius,
            side_height,
            angle_or_width,
            material,
        );
    }

    /// Creates a new [`ArcParallelMagnet`] if the given parameters are within
    /// the valid value ranges defined in the struct docstring.
    ///
    /// This constructor provides the same flexibility as the
    /// [`ArcParallelMagnet::new`] constructor:
    #[doc = ""]
    #[cfg_attr(
        feature = "doc-images",
        doc = "![Different magnet shapes from center thickness][arc_parallel_vary_center_thickness]"
    )]
    #[cfg_attr(
        feature = "doc-images",
        embed_doc_image::embed_doc_image(
            "arc_parallel_vary_center_thickness",
            "docs/img/arc_parallel_vary_center_thickness.svg"
        )
    )]
    #[cfg_attr(
        not(feature = "doc-images"),
        doc = "**Doc images not enabled**. Compile docs with
        `cargo doc --features 'doc-images'` and Rust version >= 1.54."
    )]
    /// # Examples
    ///
    /// ```
    /// use stem_magnet::prelude::*;
    /// use std::sync::Arc;
    /// use std::f64::consts::PI;
    ///
    /// let magnet = ArcParallelMagnet::with_center_thickness(
    ///     Length::new::<meter>(1.0),
    ///     Length::new::<meter>(6.0),
    ///     Length::new::<meter>(1.0),
    ///     Length::new::<meter>(0.8),
    ///     (PI / 6.0).into(),
    ///     Arc::new(Material::default()),
    /// ).expect("valid inputs");
    /// approx::assert_abs_diff_eq!(magnet.side_thickness().get::<meter>(), 0.97073, epsilon = 1e-3);
    /// approx::assert_abs_diff_eq!(magnet.thickness().get::<meter>(), 0.8853, epsilon = 1e-3);
    /// approx::assert_abs_diff_eq!(magnet.center_thickness().get::<meter>(), 0.8, epsilon = 1e-3);
    ///
    /// // The arcs of this magnet would curve into each other
    /// assert!(ArcParallelMagnet::with_center_thickness(
    ///     Length::new::<meter>(1.0),
    ///     Length::new::<meter>(6.0),
    ///     Length::new::<meter>(1.0),
    ///     Length::new::<meter>(-0.2),
    ///     (PI / 6.0).into(),
    ///     Arc::new(Material::default()),
    /// ).is_err());
    /// ```
    pub fn with_center_thickness(
        length: Length,
        core_radius: Length,
        side_height: Length,
        center_thickness: Length,
        angle_or_width: AngleOrWidth,
        material: Arc<Material>,
    ) -> Result<Self, crate::error::Error> {
        let zero = Length::new::<meter>(0.0);
        compare_variables!(val zero < center_thickness)?;

        let offset = [0.0, side_height.get::<meter>()];

        let [core_arc, air_gap_arc] = core_and_air_gap_arc_from_center_thickness(
            core_radius,
            center_thickness,
            offset,
            angle_or_width.angle(core_radius.abs()),
        )?;

        let sign = if air_gap_arc.is_positive() {
            if core_radius.is_sign_positive() {
                -1.0
            } else {
                1.0
            }
        } else {
            if core_radius.is_sign_positive() {
                1.0
            } else {
                -1.0
            }
        };
        let air_gap_radius = Length::new::<meter>(air_gap_arc.radius() * sign);

        let [shape, north_shape, south_shape] = shapes(
            core_arc,
            air_gap_arc,
            offset,
            core_radius.is_sign_positive(),
        )?;

        return Ok(Self {
            length,
            core_radius,
            air_gap_radius,
            width: angle_or_width.width(core_radius.abs()),
            side_height,
            material,
            shape,
            center_thickness,
            north_shape,
            south_shape,
        });
    }

    /**
    Returns the magnet radius at the surface facing the core.
     */
    pub fn core_radius(&self) -> Length {
        return self.core_radius;
    }

    /**
    Returns the magnet radius at the surface facing the air gap.
     */
    pub fn air_gap_radius(&self) -> Length {
        return self.air_gap_radius;
    }

    /**
    Returns the angle covered by the magnet.
     */
    pub fn angle(&self) -> f64 {
        return 2.0 * ((self.width / (2.0 * self.core_radius.abs())).get::<ratio>()).asin();
    }

    /**
    Returns the thickness in the middle / center of the magnet.
     */
    pub fn center_thickness(&self) -> Length {
        return self.center_thickness;
    }

    /**
    Returns the (calculated) thickness at the sides of the magnet.
     */
    pub fn side_thickness(&self) -> Length {
        return SideHeightOrThickness::Height(self.side_height)
            .thickness(self.core_radius, self.angle().into());
    }

    /**
    Returns the side height of the magnet.
     */
    pub fn side_height(&self) -> Length {
        return self.side_height;
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Magnet for ArcParallelMagnet {
    fn width(&self) -> Length {
        return self.width;
    }

    fn length(&self) -> Length {
        return self.length;
    }

    fn thickness(&self) -> Length {
        return 0.5 * (self.center_thickness + self.side_thickness());
    }

    fn material(&self) -> &Material {
        return &*self.material;
    }

    fn material_arc(&self) -> Arc<Material> {
        return self.material.clone();
    }

    fn shape(&self) -> Cow<'_, Shape> {
        return Cow::Borrowed(&self.shape);
    }

    fn north_south_shapes(&self) -> [Cow<'_, Shape>; 2] {
        return [
            Cow::Borrowed(&self.north_shape),
            Cow::Borrowed(&self.south_shape),
        ];
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;
    use serde_mosaic::deserialize_arc_link;
    use stem_material::prelude::{deserialize_opt_quantity, deserialize_quantity};

    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct WithAirGapRadiusSideHeightAndWidth {
        #[serde(deserialize_with = "deserialize_quantity")]
        length: Length,
        #[serde(deserialize_with = "deserialize_quantity")]
        side_height: Length,
        #[serde(deserialize_with = "deserialize_quantity")]
        core_radius: Length,
        #[serde(default, deserialize_with = "deserialize_opt_quantity")]
        air_gap_radius: Option<Length>,
        #[serde(deserialize_with = "deserialize_quantity")]
        width: Length,
        #[serde(deserialize_with = "deserialize_arc_link")]
        material: Arc<Material>,
    }

    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct WithAirGapRadiusSideHeightAndAngle {
        #[serde(deserialize_with = "deserialize_quantity")]
        length: Length,
        #[serde(deserialize_with = "deserialize_quantity")]
        side_height: Length,
        #[serde(deserialize_with = "deserialize_quantity")]
        core_radius: Length,
        #[serde(default, deserialize_with = "deserialize_opt_quantity")]
        air_gap_radius: Option<Length>,
        #[serde(deserialize_with = "deserialize_angle")]
        angle: f64,
        #[serde(deserialize_with = "deserialize_arc_link")]
        material: Arc<Material>,
    }

    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct WithConstThicknessAndAngle {
        #[serde(deserialize_with = "deserialize_quantity")]
        length: Length,
        #[serde(deserialize_with = "deserialize_quantity")]
        thickness: Length,
        #[serde(deserialize_with = "deserialize_quantity")]
        core_radius: Length,
        #[serde(deserialize_with = "deserialize_angle")]
        angle: f64,
        #[serde(deserialize_with = "deserialize_arc_link")]
        material: Arc<Material>,
    }

    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct WithConstThicknessAndWidth {
        #[serde(deserialize_with = "deserialize_quantity")]
        length: Length,
        #[serde(deserialize_with = "deserialize_quantity")]
        thickness: Length,
        #[serde(deserialize_with = "deserialize_quantity")]
        core_radius: Length,
        #[serde(deserialize_with = "deserialize_quantity")]
        width: Length,
        #[serde(deserialize_with = "deserialize_arc_link")]
        material: Arc<Material>,
    }

    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct WithCenterThicknessAndWidth {
        #[serde(deserialize_with = "deserialize_quantity")]
        length: Length,
        #[serde(deserialize_with = "deserialize_quantity")]
        side_height: Length,
        #[serde(deserialize_with = "deserialize_quantity")]
        center_thickness: Length,
        #[serde(deserialize_with = "deserialize_quantity")]
        core_radius: Length,
        #[serde(deserialize_with = "deserialize_quantity")]
        width: Length,
        #[serde(deserialize_with = "deserialize_arc_link")]
        material: Arc<Material>,
    }

    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct WithCenterThicknessAndAngle {
        #[serde(deserialize_with = "deserialize_quantity")]
        length: Length,
        #[serde(deserialize_with = "deserialize_quantity")]
        side_height: Length,
        #[serde(deserialize_with = "deserialize_quantity")]
        center_thickness: Length,
        #[serde(deserialize_with = "deserialize_quantity")]
        core_radius: Length,
        #[serde(deserialize_with = "deserialize_angle")]
        angle: f64,
        #[serde(deserialize_with = "deserialize_arc_link")]
        material: Arc<Material>,
    }

    #[derive(deserialize_untagged_verbose_error::DeserializeUntaggedVerboseError)]
    enum MagnetEnum {
        WithAirGapRadiusSideHeightAndWidth(WithAirGapRadiusSideHeightAndWidth),
        WithAirGapRadiusSideHeightAndAngle(WithAirGapRadiusSideHeightAndAngle),
        WithConstThicknessAndWidth(WithConstThicknessAndWidth),
        WithConstThicknessAndAngle(WithConstThicknessAndAngle),
        WithCenterThicknessAndAngle(WithCenterThicknessAndAngle),
        WithCenterThicknessAndWidth(WithCenterThicknessAndWidth),
    }

    impl<'de> Deserialize<'de> for ArcParallelMagnet {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let m = MagnetEnum::deserialize(deserializer)?;
            match m {
                MagnetEnum::WithAirGapRadiusSideHeightAndWidth(m) => match m.air_gap_radius {
                    Some(r) => {
                        return Self::new(
                            m.length,
                            m.core_radius,
                            r,
                            m.side_height,
                            AngleOrWidth::Width(m.width),
                            m.material,
                        )
                        .map_err(serde::de::Error::custom);
                    }
                    None => {
                        return Self::with_const_thickness(
                            m.length,
                            m.core_radius,
                            SideHeightOrThickness::Height(m.side_height),
                            AngleOrWidth::Width(m.width),
                            m.material,
                        )
                        .map_err(serde::de::Error::custom);
                    }
                },
                MagnetEnum::WithAirGapRadiusSideHeightAndAngle(m) => match m.air_gap_radius {
                    Some(r) => {
                        return Self::new(
                            m.length,
                            m.core_radius,
                            r,
                            m.side_height,
                            AngleOrWidth::Angle(m.angle),
                            m.material,
                        )
                        .map_err(serde::de::Error::custom);
                    }
                    None => {
                        return Self::with_const_thickness(
                            m.length,
                            m.core_radius,
                            SideHeightOrThickness::Height(m.side_height),
                            AngleOrWidth::Angle(m.angle),
                            m.material,
                        )
                        .map_err(serde::de::Error::custom);
                    }
                },
                MagnetEnum::WithConstThicknessAndWidth(m) => {
                    return Self::with_const_thickness(
                        m.length,
                        m.core_radius,
                        SideHeightOrThickness::Thickness(m.thickness),
                        AngleOrWidth::Width(m.width),
                        m.material,
                    )
                    .map_err(serde::de::Error::custom);
                }
                MagnetEnum::WithConstThicknessAndAngle(m) => {
                    return Self::with_const_thickness(
                        m.length,
                        m.core_radius,
                        SideHeightOrThickness::Thickness(m.thickness),
                        AngleOrWidth::Angle(m.angle),
                        m.material,
                    )
                    .map_err(serde::de::Error::custom);
                }
                MagnetEnum::WithCenterThicknessAndAngle(m) => {
                    return Self::with_center_thickness(
                        m.length,
                        m.core_radius,
                        m.side_height,
                        m.center_thickness,
                        AngleOrWidth::Angle(m.angle),
                        m.material,
                    )
                    .map_err(serde::de::Error::custom);
                }
                MagnetEnum::WithCenterThicknessAndWidth(m) => {
                    return Self::with_center_thickness(
                        m.length,
                        m.core_radius,
                        m.side_height,
                        m.center_thickness,
                        AngleOrWidth::Width(m.width),
                        m.material,
                    )
                    .map_err(serde::de::Error::custom);
                }
            }
        }
    }
}
