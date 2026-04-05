/*!
This module defines a [`BreadLoafMagnet`] - a block magnet with an arced surface
towards the air gap. See the struct documentation for more.
 */

use std::{borrow::Cow, sync::Arc};
use stem_material::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "serde")]
use serde_mosaic::serialize_arc_link;

use compare_variables::compare_variables;

use planar_geo::prelude::*;

use super::magnet::Magnet;

/**
A block-shaped permanent magnet with an arc at the air gap side.

# Geometry

While a [`BlockMagnet`](crate::block::BlockMagnet) is cheap to manufacture, it
produces a block-shaped magnetic excitation in the air gap, resulting in a
magnetic field containing a lot of unwanted higher-order harmonics (besides the
useful base harmonic which interacts with the stator field to create force /
torque). To mitigate this, the bread loaf magnet has a curved surface on its air
gap side, resulting in a more sinusoidal field at the cost of higher
manufacturing expenses:
 */
#[doc = ""]
#[cfg_attr(
    feature = "doc-images",
    doc = "![Block magnet vs. bread loaf magnet excitation][bread_loaf_magnet_excitation]"
)]
#[cfg_attr(
    feature = "doc-images",
    embed_doc_image::embed_doc_image(
        "bread_loaf_magnet_excitation",
        "docs/img/bread_loaf_magnet_excitation.svg"
    )
)]
#[cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile docs with
    `cargo doc --features 'doc-images'` and Rust version >= 1.54."
)]
/**

A bread loaf magnet is defined by the following parameters:
- `length`: Axial length of the magnet. Must be positive (`length > 0 m`).
- `width`: Width of the magnet (perpendicular to magnetization axis). Must be
positive (`width > 0 m`).
- `side_thickness`: Thickness of the magnet at its sides. Must be
positive (`side_thickness > 0 m`).
- One of `radius` or `center_thickness` to define the arc segment:
    - `radius`: Radius of the curved surface. If positive, the magnet is convex
    (curves towards the stator). If negative, the magnet is concave (curves away
    from the stator). See the image below for examples. Its absolute value must
    be larger than half the magnet `width` (`0.5*width <= radius.abs()`).
    - `center_thickness`: Thickness of the magnet at its center. Must be
    positive (`center_thickness > 0 m`).
- `material`: The material of the magnet.
*/
#[doc = ""]
#[cfg_attr(
    feature = "doc-images",
    doc = "![Breadloaf magnet definitions][drawing_breadloaf_magnet]"
)]
#[cfg_attr(
    feature = "doc-images",
    embed_doc_image::embed_doc_image(
        "drawing_breadloaf_magnet",
        "docs/img/drawing_breadloaf_magnet.svg"
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
- [`new`](BreadLoafMagnet::new): Creates a magnet from `length`, `width`,
`side_thickness`, `radius` and `material`.
- [`with_center_thickness`](BreadLoafMagnet::with_center_thickness): Creates a
magnet from `length`, `width`, side_thickness`, `center_thickness` and
`material`.

# Deserialization

For each of the aforementioned constructors, there exists a serialized
representation:

## `new`

```
use approx;
use stem_magnet::prelude::*;
use serde_yaml;

let str = indoc::indoc! {"
length: 165 mm
width: 20 mm
side_thickness: 10 mm
radius: 50 mm
material:
    name: NMF-12J 430mT
    relative_permeability: 1.05
"}; // All other material fields are default

let magnet: BreadLoafMagnet = serde_yaml::from_str(&str).expect("valid dimensions");
assert_eq!(magnet.length().get::<meter>(), 0.165);
approx::assert_abs_diff_eq!(magnet.radius().get::<meter>(), 0.05, epsilon=1e-3);
approx::assert_abs_diff_eq!(magnet.center_thickness().get::<meter>(), 0.011, epsilon=1e-3);
```

## with_center_thickness

```
use stem_magnet::prelude::*;
use serde_yaml;

let str = indoc::indoc! {"
length: 165 mm
width: 20 mm
side_thickness: 10 mm
center_thickness: 11 mm
material:
    name: NMF-12J 430mT
    relative_permeability: 1.05
"}; // All other material fields are default

let magnet: BreadLoafMagnet = serde_yaml::from_str(&str).expect("valid dimensions");
assert_eq!(magnet.length().get::<meter>(), 0.165);
approx::assert_abs_diff_eq!(magnet.radius().get::<meter>(), 0.05, epsilon=1e-3);
approx::assert_abs_diff_eq!(magnet.center_thickness().get::<meter>(), 0.011, epsilon=1e-3);
```

 */
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct BreadLoafMagnet {
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_quantity"))]
    length: Length,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_quantity"))]
    width: Length,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_quantity"))]
    side_thickness: Length,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_quantity"))]
    radius: Length,
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_arc_link",))]
    material: Arc<Material>,
    #[cfg_attr(feature = "serde", serde(skip))]
    shape: Shape,
    #[cfg_attr(feature = "serde", serde(skip))]
    north_shape: Shape,
    #[cfg_attr(feature = "serde", serde(skip))]
    south_shape: Shape,
}

impl BreadLoafMagnet {
    /**
    Creates a new [`BreadLoafMagnet`] if the given parameters are within the
    valid value ranges defined in the struct docstring.

    # Examples

    ```
    use stem_magnet::prelude::*;
    use std::sync::Arc;

    // Convex magnet
    assert!(BreadLoafMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(20.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(50.0),
        Arc::new(Default::default()),
    ).is_ok());

    // Concave magnet
    assert!(BreadLoafMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(20.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(-50.0),
        Arc::new(Default::default()),
    ).is_ok());

    // Negative length
    assert!(BreadLoafMagnet::new(
        Length::new::<millimeter>(-165.0),
        Length::new::<millimeter>(20.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(50.0),
        Arc::new(Default::default()),
    ).is_err());

    // Absolute value of radius smaller than half the width
    assert!(BreadLoafMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(20.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(6.0),
        Arc::new(Default::default()),
    ).is_err());
    ```
     */
    pub fn new(
        length: Length,
        width: Length,
        side_thickness: Length,
        radius: Length,
        material: Arc<Material>,
    ) -> Result<Self, crate::error::Error> {
        use uom::typenum::P2;

        let halfed_magnet_width = width / 2.0;
        let abs_radius = radius.abs();
        compare_variables!(abs_radius >= halfed_magnet_width)?;

        let offset =
            radius.abs() - 0.5 * (4.0 * radius.powi(P2::new()) - width.powi(P2::new())).sqrt();
        let center_thickness = if radius.is_sign_positive() {
            side_thickness + offset
        } else {
            side_thickness - offset
        };

        return Self::new_priv(
            length,
            width,
            side_thickness,
            center_thickness,
            Some(radius),
            material,
        );
    }

    /**
    Creates a new [`BreadLoafMagnet`] if the given parameters are within the
    valid value ranges defined in the struct docstring.

    # Examples

    ```
    use stem_magnet::prelude::*;
    use std::sync::Arc;

    assert!(BreadLoafMagnet::with_center_thickness(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(20.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(12.0),
        Arc::new(Default::default()),
    ).is_ok());

    // Negative length
    assert!(BreadLoafMagnet::with_center_thickness(
        Length::new::<millimeter>(-165.0),
        Length::new::<millimeter>(20.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(12.0),
        Arc::new(Default::default()),
    ).is_err());

    // Negative center thickness
    assert!(BreadLoafMagnet::with_center_thickness(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(20.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(-12.0),
        Arc::new(Default::default()),
    ).is_err());
    ```
     */
    pub fn with_center_thickness(
        length: Length,
        width: Length,
        side_thickness: Length,
        center_thickness: Length,
        material: Arc<Material>,
    ) -> Result<Self, crate::error::Error> {
        return Self::new_priv(
            length,
            width,
            side_thickness,
            center_thickness,
            None,
            material,
        );
    }

    fn new_priv(
        length: Length,
        width: Length,
        side_thickness: Length,
        center_thickness: Length,
        radius: Option<Length>,
        material: Arc<Material>,
    ) -> Result<Self, crate::error::Error> {
        let zero = Length::new::<meter>(0.0);
        compare_variables!(val zero < length)?;
        compare_variables!(val zero < width)?;
        compare_variables!(val zero <= side_thickness)?;
        compare_variables!(val zero <= center_thickness)?;

        // Create shape
        let e = DEFAULT_EPSILON;
        let m = DEFAULT_MAX_ULPS;

        let mut polysegment = Polysegment::with_capacity(5);
        let line = LineSegment::new([0.0, 0.0], [0.5 * width.get::<meter>(), 0.0], e, m)?;
        polysegment.push_back(line.into());
        polysegment.extend_back([0.5 * width.get::<meter>(), side_thickness.get::<meter>()]);

        let arc = ArcSegment::from_start_middle_stop(
            [0.5 * width.get::<meter>(), side_thickness.get::<meter>()],
            [0.0, center_thickness.get::<meter>()],
            [-0.5 * width.get::<meter>(), side_thickness.get::<meter>()],
            e,
            m,
        )?;

        let radius = radius.unwrap_or_else(|| {
            let r = Length::new::<meter>(arc.radius());
            if center_thickness >= side_thickness {
                r
            } else {
                -r
            }
        });

        polysegment.push_back(arc.into());
        polysegment.extend_back([-0.5 * width.get::<meter>(), side_thickness.get::<meter>()]);
        polysegment.extend_back([-0.5 * width.get::<meter>(), 0.0]);
        let contour = Contour::new(polysegment);
        let shape = Shape::from_outer(contour)?;

        // Separate the shape into two
        let c = shape.centroid()[1];
        let cut =
            Polysegment::from_points(&[[-width.get::<meter>(), c], [width.get::<meter>(), c]]);
        let mut chains = shape.contour().intersection_cut(&cut, e, m);
        let number_north_south_chains = chains.len();

        compare_variables!(number_north_south_chains == 2)?;

        let south_shape = Shape::from_outer(chains.pop().expect("has two elements").into())?;
        let north_shape = Shape::from_outer(chains.pop().expect("has two elements").into())?;

        return Ok(BreadLoafMagnet {
            length,
            width,
            side_thickness,
            radius,
            material,
            shape,
            north_shape,
            south_shape,
        });
    }

    /**
    Returns the thickness of the magnet at its sides.
     */
    pub fn side_thickness(&self) -> Length {
        return self.side_thickness;
    }

    /**
    Returns the thickness at the magnet center.

    This value is equal to [`BreadLoafMagnet::side_thickness`] plus
    [`BreadLoafMagnet::arc_segment_height`].
     */
    pub fn center_thickness(&self) -> Length {
        return self.side_thickness() + self.arc_segment_height();
    }

    /**
    Returns the radius of the arc segment on the air gap side of the magnet.
     */
    pub fn radius(&self) -> Length {
        return self.radius;
    }

    /**
    Returns the height of the arc segment.

    This value is calculated as:
    `radius - 0.5 * sqrt(4*radius² - width²)`,
    where `radius` equals [`BreadLoafMagnet::radius`] and `width` equals
    [`BreadLoafMagnet::width`].
     */
    pub fn arc_segment_height(&self) -> Length {
        use uom::typenum::P2;
        let val = self.radius().abs()
            - 0.5 * (4.0 * self.radius().powi(P2::new()) - self.width().powi(P2::new())).sqrt();
        if self.radius().is_sign_positive() {
            val
        } else {
            -val
        }
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Magnet for BreadLoafMagnet {
    fn width(&self) -> Length {
        return self.width;
    }

    fn length(&self) -> Length {
        return self.length;
    }

    fn thickness(&self) -> Length {
        return self.side_thickness() + 0.5 * self.arc_segment_height();
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

    fn area(&self) -> Area {
        use uom::typenum::P2;
        let rect_area = self.side_thickness() * self.width();

        let r = self.radius().abs();
        let h = self.arc_segment_height().abs();
        let arc_seg_area = r.powi(P2::new()) * (1.0 - (h / r).get::<ratio>()).acos()
            - (r - h) * (h * (2.0 * r - h)).sqrt();

        if self.radius().is_sign_positive() {
            rect_area + arc_seg_area
        } else {
            rect_area - arc_seg_area
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for BreadLoafMagnet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde_mosaic::deserialize_arc_link;
        use stem_material::prelude::deserialize_quantity;

        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct WithRadius {
            #[serde(deserialize_with = "deserialize_quantity")]
            length: Length,
            #[serde(deserialize_with = "deserialize_quantity")]
            width: Length,
            #[serde(deserialize_with = "deserialize_quantity")]
            side_thickness: Length,
            #[serde(deserialize_with = "deserialize_quantity")]
            radius: Length,
            #[serde(deserialize_with = "deserialize_arc_link")]
            material: Arc<Material>,
        }

        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct WithCenterThickness {
            #[serde(deserialize_with = "deserialize_quantity")]
            length: Length,
            #[serde(deserialize_with = "deserialize_quantity")]
            width: Length,
            #[serde(deserialize_with = "deserialize_quantity")]
            side_thickness: Length,
            #[serde(deserialize_with = "deserialize_quantity")]
            center_thickness: Length,
            #[serde(deserialize_with = "deserialize_arc_link")]
            material: Arc<Material>,
        }

        #[derive(deserialize_untagged_verbose_error::DeserializeUntaggedVerboseError)]
        enum MagnetEnum {
            WithRadius(WithRadius),
            WithCenterThickness(WithCenterThickness),
        }

        let m = MagnetEnum::deserialize(deserializer)?;
        match m {
            MagnetEnum::WithRadius(m) => {
                return Self::new(m.length, m.width, m.side_thickness, m.radius, m.material)
                    .map_err(serde::de::Error::custom);
            }
            MagnetEnum::WithCenterThickness(m) => {
                return Self::with_center_thickness(
                    m.length,
                    m.width,
                    m.side_thickness,
                    m.center_thickness,
                    m.material,
                )
                .map_err(serde::de::Error::custom);
            }
        }
    }
}
