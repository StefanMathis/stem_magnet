/*!
This module defines an [`ArcSegmentMagnet`] - a curved magnet where the magnet
sides are radially oriented. See the struct documentation for more.
 */

use std::{borrow::Cow, f64::consts::FRAC_PI_2, sync::Arc};
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
A curved permanent magnet where the sides are radially oriented.

It is closely related to the [`ArcParallelMagnet`], which features parallel
sides instead.

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
    doc = "![Arc segment magnet definitions][drawing_arc_segment_magnet]"
)]
#[cfg_attr(
    feature = "doc-images",
    embed_doc_image::embed_doc_image(
        "drawing_arc_segment_magnet",
        "docs/img/drawing_arc_segment_magnet.svg"
    )
)]
#[cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile docs with
    `cargo doc --features 'doc-images'` and Rust version >= 1.54."
)]
/**
An arc segment magnet is defined by the following parameters:
- `length`: Axial length of the magnet. Must be positive (`length > 0 m`).
- `side_thickness`: Thickness of the magnet at its sides. Must be positive or
zero (`side_thickness >= 0 m`).
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
- `angle`: The angle covered by this magnet in radians. Must be positive and
smaller than or equal to π (`0 < angle <= π`).
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
    doc = "![Different arc segment shapes][arc_segment_vary_air_gap_radius]"
)]
#[cfg_attr(
    feature = "doc-images",
    embed_doc_image::embed_doc_image(
        "arc_segment_vary_air_gap_radius",
        "docs/img/arc_segment_vary_air_gap_radius.svg"
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
- [`new`](ArcSegmentMagnet::new): Creates a magnet from its `length`,
`side_thickness`, `core_radius`, `air_gap_radius`, `angle` and `material`.
- [`with_const_thickness`](ArcSegmentMagnet::with_const_thickness): Like `new`,
but the `air_gap_radius` is calculated as `core_radius +/- side_thickness`
(depending on whether `inner` is `true` or `false`), resulting in a magnet with
constant thickness.
- [`with_center_thickness`](ArcSegmentMagnet::with_center_thickness): Instead
of specifying the `air_gap_radius`, it is also possible to provide the
`center_thickness` instead.

# Deserialization

For each of the aforementioned constructors, there exists a serialized
representation. The following examples all result in the same magnet:

## `new`

```
use stem_magnet::prelude::*;
use serde_yaml;
use approx;

let str = indoc::indoc! {"
length: 165 mm
core_radius: 80 mm
air_gap_radius: 90 mm
side_thickness: 10 mm
angle: PI/6 rad
material:
    name: NMF-12J 430mT
    relative_permeability: 1.05
"};
let magnet: ArcSegmentMagnet = serde_yaml::from_str(str).expect("valid magnet");
approx::assert_abs_diff_eq!(magnet.air_gap_radius().get::<meter>(), 0.09, epsilon = 1e-3);
approx::assert_abs_diff_eq!(magnet.area().get::<square_millimeter>(), 445.058, epsilon = 1e-3);
```

## `with_const_thickness`

```
use stem_magnet::prelude::*;
use serde_yaml;
use approx;

let str = indoc::indoc! {"
length: 165 mm
core_radius: 80 mm
side_thickness: 10 mm # air_gap_radius is derived as core_radius + side_thickness
angle: PI/6 rad
material:
    name: NMF-12J 430mT
    relative_permeability: 1.05
"};
let magnet: ArcSegmentMagnet = serde_yaml::from_str(str).expect("valid magnet");
approx::assert_abs_diff_eq!(magnet.air_gap_radius().get::<meter>(), 0.09, epsilon = 1e-3);
approx::assert_abs_diff_eq!(magnet.area().get::<square_millimeter>(), 445.058, epsilon = 1e-3);
```

## `with_center_thickness`

```
use stem_magnet::prelude::*;
use serde_yaml;
use approx;

let str = indoc::indoc! {"
length: 165 mm
core_radius: 80 mm
side_thickness: 10 mm
center_thickness: 10 mm
angle: PI/6 rad
material:
    name: NMF-12J 430mT
    relative_permeability: 1.05
"};
let magnet: ArcSegmentMagnet = serde_yaml::from_str(str).expect("valid magnet");
approx::assert_abs_diff_eq!(magnet.air_gap_radius().get::<meter>(), 0.09, epsilon = 1e-3);
approx::assert_abs_diff_eq!(magnet.area().get::<square_millimeter>(), 445.058, epsilon = 1e-3);
```
*/
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct ArcSegmentMagnet {
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_quantity"))]
    length: Length,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_quantity"))]
    side_thickness: Length,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_quantity"))]
    core_radius: Length,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_quantity"))]
    air_gap_radius: Length,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_angle"))]
    angle: f64,
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

impl ArcSegmentMagnet {
    /**
    Creates a new [`ArcSegmentMagnet`] if the given parameters are within the
    valid value ranges defined in the struct docstring.

    # Examples

    ```
    use stem_magnet::prelude::*;
    use std::sync::Arc;

    // Inner rotor magnet
    assert!(ArcSegmentMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(50.0),
        Length::new::<millimeter>(60.0),
        Length::new::<millimeter>(10.0),
        1.0,
        Arc::new(Material::default()),
    ).is_ok());

    // Outer rotor magnet
    assert!(ArcSegmentMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(-60.0),
        Length::new::<millimeter>(-50.0),
        Length::new::<millimeter>(10.0),
        1.0,
        Arc::new(Material::default()),
    ).is_ok());

    // Angle is negative
    assert!(ArcSegmentMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(-60.0),
        Length::new::<millimeter>(50.0),
        Length::new::<millimeter>(10.0),
        - 1.0,
        Arc::new(Material::default()),
    ).is_err());

    // Radius is zero
    assert!(ArcSegmentMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(-60.0),
        Length::new::<millimeter>(0.0),
        Length::new::<millimeter>(10.0),
        1.0,
        Arc::new(Material::default()),
    ).is_err());
    ```
     */
    pub fn new(
        length: Length,
        core_radius: Length,
        air_gap_radius: Length,
        side_thickness: Length,
        angle: f64,
        material: Arc<Material>,
    ) -> Result<Self, crate::error::Error> {
        let zero = Length::new::<meter>(0.0);
        compare_variables!(val zero < length)?;

        let mut offset = [side_thickness.get::<meter>(), 0.0];
        offset.rotate([0.0, 0.0], FRAC_PI_2 - 0.5 * angle);

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
            angle,
            side_thickness,
            material,
            shape,
            center_thickness,
            north_shape,
            south_shape,
        });
    }

    /**
    Like [`ArcSegmentMagnet::new`], but the `air_gap_radius` is calculated as
    `core_radius + side_thickness` (if `core_radius > 0 m` and as
    `core_radius - side_thickness` otherwise. This results in a magnet with
    constant thickness.

    # Examples

    ```
    use stem_magnet::prelude::*;
    use std::sync::Arc;

    // Inner rotor magnet
    let magnet = ArcSegmentMagnet::with_const_thickness(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(50.0),
        Length::new::<millimeter>(10.0),
        1.0,
        Arc::new(Material::default()),
    ).expect("valid inputs");
    approx::assert_abs_diff_eq!(magnet.air_gap_radius().get::<meter>(), 0.06, epsilon = 1e-3);

    // Outer rotor magnet
    let magnet = ArcSegmentMagnet::with_const_thickness(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(-60.0),
        Length::new::<millimeter>(10.0),
        1.0,
        Arc::new(Material::default()),
    ).expect("valid inputs");
    approx::assert_abs_diff_eq!(magnet.air_gap_radius().get::<meter>(), -0.05, epsilon = 1e-3);
    ```
    */
    pub fn with_const_thickness(
        length: Length,
        core_radius: Length,
        side_thickness: Length,
        angle: f64,
        material: Arc<Material>,
    ) -> Result<Self, crate::error::Error> {
        let air_gap_radius = core_radius + side_thickness;
        return Self::new(
            length,
            core_radius,
            air_gap_radius,
            side_thickness,
            angle,
            material,
        );
    }

    /// Creates a new [`ArcSegmentMagnet`] if the given parameters are within
    /// the valid value ranges defined in the struct docstring.
    ///
    /// This constructor provides the same flexibility as the
    /// [`ArcSegmentMagnet::new`] constructor:
    #[doc = ""]
    #[cfg_attr(
        feature = "doc-images",
        doc = "![Different magnet shapes from center thickness][arc_segment_vary_center_thickness]"
    )]
    #[cfg_attr(
        feature = "doc-images",
        embed_doc_image::embed_doc_image(
            "arc_segment_vary_center_thickness",
            "docs/img/arc_segment_vary_center_thickness.svg"
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
    /// let magnet = ArcSegmentMagnet::with_center_thickness(
    ///     Length::new::<meter>(1.0),
    ///     Length::new::<meter>(6.0),
    ///     Length::new::<meter>(1.0),
    ///     Length::new::<meter>(0.8),
    ///     PI / 6.0,
    ///     Arc::new(Material::default()),
    /// ).expect("valid inputs");
    /// approx::assert_abs_diff_eq!(magnet.side_thickness().get::<meter>(), 1.0, epsilon = 1e-3);
    /// approx::assert_abs_diff_eq!(magnet.thickness().get::<meter>(), 0.9, epsilon = 1e-3);
    /// approx::assert_abs_diff_eq!(magnet.center_thickness().get::<meter>(), 0.8, epsilon = 1e-3);
    ///
    /// // The arcs of this magnet would curve into each other
    /// assert!(ArcSegmentMagnet::with_center_thickness(
    ///     Length::new::<meter>(1.0),
    ///     Length::new::<meter>(6.0),
    ///     Length::new::<meter>(1.0),
    ///     Length::new::<meter>(-0.2),
    ///     PI / 6.0,
    ///     Arc::new(Material::default()),
    /// ).is_err());
    /// ```
    pub fn with_center_thickness(
        length: Length,
        core_radius: Length,
        side_thickness: Length,
        center_thickness: Length,
        angle: f64,
        material: Arc<Material>,
    ) -> Result<Self, crate::error::Error> {
        let zero = Length::new::<meter>(0.0);
        compare_variables!(val zero < center_thickness)?;

        let mut offset = [side_thickness.get::<meter>(), 0.0];
        offset.rotate([0.0, 0.0], FRAC_PI_2 - 0.5 * angle);

        let [core_arc, air_gap_arc] = core_and_air_gap_arc_from_center_thickness(
            core_radius,
            center_thickness,
            offset,
            angle,
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
            angle,
            side_thickness,
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
        return self.angle;
    }

    /**
    Returns the thickness in the middle / center of the magnet.
     */
    pub fn center_thickness(&self) -> Length {
        return self.center_thickness;
    }

    /**
    Returns the thickness at the sides of the magnet.
     */
    pub fn side_thickness(&self) -> Length {
        return self.side_thickness;
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Magnet for ArcSegmentMagnet {
    fn width(&self) -> Length {
        return 2.0
            * (self.core_radius() + 0.5 * (self.air_gap_radius() - self.core_radius())).abs()
            * (self.angle() / 2.0).sin();
    }

    fn length(&self) -> Length {
        return self.length;
    }

    fn thickness(&self) -> Length {
        return 0.5 * (self.center_thickness + self.side_thickness);
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
    struct WithOrWithoutAirGapRadius {
        #[serde(deserialize_with = "deserialize_quantity")]
        length: Length,
        #[serde(deserialize_with = "deserialize_quantity")]
        side_thickness: Length,
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
    struct WithCenterThickness {
        #[serde(deserialize_with = "deserialize_quantity")]
        length: Length,
        #[serde(deserialize_with = "deserialize_quantity")]
        core_radius: Length,
        #[serde(deserialize_with = "deserialize_quantity")]
        center_thickness: Length,
        #[serde(deserialize_with = "deserialize_quantity")]
        side_thickness: Length,
        #[serde(deserialize_with = "deserialize_angle")]
        angle: f64,
        #[serde(deserialize_with = "deserialize_arc_link")]
        material: Arc<Material>,
    }

    #[derive(deserialize_untagged_verbose_error::DeserializeUntaggedVerboseError)]
    enum MagnetEnum {
        WithOrWithoutAirGapRadius(WithOrWithoutAirGapRadius),
        WithCenterThickness(WithCenterThickness),
    }

    impl<'de> Deserialize<'de> for ArcSegmentMagnet {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let m = MagnetEnum::deserialize(deserializer)?;
            match m {
                MagnetEnum::WithOrWithoutAirGapRadius(m) => match m.air_gap_radius {
                    Some(air_gap_radius) => {
                        return ArcSegmentMagnet::new(
                            m.length,
                            m.core_radius,
                            air_gap_radius,
                            m.side_thickness,
                            m.angle,
                            m.material,
                        )
                        .map_err(serde::de::Error::custom);
                    }
                    None => {
                        return ArcSegmentMagnet::with_const_thickness(
                            m.length,
                            m.core_radius,
                            m.side_thickness,
                            m.angle,
                            m.material,
                        )
                        .map_err(serde::de::Error::custom);
                    }
                },
                MagnetEnum::WithCenterThickness(m) => {
                    return ArcSegmentMagnet::with_center_thickness(
                        m.length,
                        m.core_radius,
                        m.side_thickness,
                        m.center_thickness,
                        m.angle,
                        m.material,
                    )
                    .map_err(serde::de::Error::custom);
                }
            }
        }
    }
}
