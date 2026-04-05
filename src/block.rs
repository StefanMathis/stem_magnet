/*!
This module defines a [`BlockMagnet`] - a basic cuboid magnet shape. See the
struct documentation for more.
 */

use std::{borrow::Cow, f64::consts::PI, sync::Arc};
use stem_material::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
use serde_mosaic::serialize_arc_link;

use compare_variables::compare_variables;

use planar_geo::prelude::*;

use super::magnet::Magnet;

/**
A block (cuboid) permanent magnet with optional fillets and parallel
magnetization.

# Geometry

The vast majority of interior permanent magnet (IPM) rotors and also some
surface permanent magnet (SPM) rotor use block magnets because of its
comparatively low manufacturing costs.

A block magnet is defined by the following parameters:
- `length`: Axial length of the magnet. Must be positive (`length > 0 m`).
- `width`: Width of the magnet (perpendicular to magnetization axis). Must be
positive (`width > 0 m`).
- `thickness`: Thickness of the magnet (parallel to magnetization axis). Must be
positive (`thickness > 0 m`).
- `fillet`: Edge radius of the magnet between the surfaces `length` x `width`
and `length` x `thickness`. Must be positive or zero, but must not be larger
than half the `thickness` or `width` (`0 m <= fillet <= 0.5*thickness` and
`0 m <= fillet <= 0.5*width`).
- `material`: The material of the magnet.
*/
#[doc = ""]
#[cfg_attr(
    feature = "doc-images",
    doc = "![Block magnet definitions][drawing_block_magnet]"
)]
#[cfg_attr(
    feature = "doc-images",
    embed_doc_image::embed_doc_image("drawing_block_magnet", "docs/img/drawing_block_magnet.svg")
)]
#[cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile docs with
    `cargo doc --features 'doc-images'` and Rust version >= 1.54."
)]
/**
# Deserialization

The following example shows how a block magnet can be deserialized from these
five fields:

```
use stem_magnet::prelude::*;
use serde_yaml;

let str = indoc::indoc! {"
length: 165 mm
width: 20 mm
thickness: 10 mm
fillet: 2 mm
material:
    name: NMF-12J 430mT
    relative_permeability: 1.05
"}; // All other material fields are default

let magnet: BlockMagnet = serde_yaml::from_str(&str).expect("valid dimensions");
assert_eq!(magnet.length().get::<meter>(), 0.165);
assert_eq!(magnet.fillet().get::<meter>(), 0.002);
```
 */
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct BlockMagnet {
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_quantity"))]
    length: Length, // Magnet length (axial dimension)
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_quantity"))]
    width: Length, // Magnet width (tangential dimension)
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_quantity"))]
    thickness: Length, // Magnet thickness along the magnetization axis
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_quantity"))]
    fillet: Length, // Edge fillets
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_arc_link"))]
    material: Arc<Material>,
    #[cfg_attr(feature = "serde", serde(skip))]
    shape: Shape,
    #[cfg_attr(feature = "serde", serde(skip))]
    north_shape: Shape,
    #[cfg_attr(feature = "serde", serde(skip))]
    south_shape: Shape,
}

impl BlockMagnet {
    /**
    Creates a new [`BlockMagnet`] if the given parameters are within the valid
    value ranges defined in the struct docstring.

    # Examples

    ```
    use stem_magnet::prelude::*;
    use std::sync::Arc;

    assert!(BlockMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(20.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(2.0),
        Arc::new(Default::default()),
    ).is_ok());

    // Negative dimension
    assert!(BlockMagnet::new(
        Length::new::<millimeter>(-165.0),
        Length::new::<millimeter>(20.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(2.0),
        Arc::new(Default::default()),
    ).is_err());

    // Fillet radius larger than half the width
    assert!(BlockMagnet::new(
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
        thickness: Length,
        fillet: Length,
        material: Arc<Material>,
    ) -> Result<Self, crate::error::Error> {
        let zero = Length::new::<meter>(0.0);
        compare_variables!(val zero <= fillet)?;
        compare_variables!(val zero < length)?;
        compare_variables!(val zero < width)?;
        compare_variables!(val zero < thickness)?;
        let halfed_magnet_thickness = thickness / 2.0;
        compare_variables!(halfed_magnet_thickness >= fillet)?;
        let halfed_magnet_width = width / 2.0;
        compare_variables!(halfed_magnet_width >= fillet)?;

        let xmin = -0.5 * width.get::<meter>();
        let xmax = -xmin;
        let ymin = 0.0;
        let ymax = thickness.get::<meter>();

        let e = DEFAULT_EPSILON;
        let m = DEFAULT_MAX_ULPS;
        let f = fillet.get::<meter>();

        // Create shape
        let (shape, north_shape, south_shape) = if fillet == zero {
            let polysegment =
                Polysegment::from_points(&[[xmin, ymin], [xmax, ymin], [xmax, ymax], [xmin, ymax]]);
            let shape = Shape::new(vec![polysegment.into()])?;
            let polysegment = Polysegment::from_points(&[
                [xmin, ymin],
                [xmax, ymin],
                [xmax, 0.5 * ymax],
                [xmin, 0.5 * ymax],
            ]);
            let north_shape = Shape::new(vec![polysegment.into()])?;
            let polysegment = Polysegment::from_points(&[
                [xmin, 0.5 * ymax],
                [xmax, 0.5 * ymax],
                [xmax, ymax],
                [xmin, ymax],
            ]);
            let south_shape = Shape::new(vec![polysegment.into()])?;
            (shape, north_shape, south_shape)
        } else {
            let mut polysegment = Polysegment::new();
            let a_lr = ArcSegment::fillet([xmax, ymax], [xmax, ymin], [xmin, ymin], f, e, m)?;
            polysegment.push_back(a_lr.clone().into());
            let a_ll = ArcSegment::fillet([xmax, ymin], [xmin, ymin], [xmin, ymax], f, e, m)?;
            polysegment.push_back(a_ll.clone().into());
            let a_ul = ArcSegment::fillet([xmin, ymin], [xmin, ymax], [xmax, ymax], f, e, m)?;
            polysegment.push_back(a_ul.clone().into());
            let a_ur = ArcSegment::fillet([xmin, ymax], [xmax, ymax], [xmax, ymin], f, e, m)?;
            polysegment.push_back(a_ur.clone().into());
            let shape = Shape::new(vec![polysegment.into()])?;

            // Create north-south shapes
            let mut s = Polysegment::new();
            s.push_back(a_lr.into());
            s.push_back(a_ll.into());
            s.extend_back([xmin, 0.5 * ymax]);
            s.extend_back([xmax, 0.5 * ymax]);
            let north_shape = Shape::new(vec![s.into()])?;

            let mut n = Polysegment::new();
            n.push_back(a_ul.into());
            n.push_back(a_ur.into());
            n.extend_back([xmax, 0.5 * ymax]);
            n.extend_back([xmin, 0.5 * ymax]);
            let south_shape = Shape::new(vec![n.into()])?;
            (shape, north_shape, south_shape)
        };

        return Ok(BlockMagnet {
            length,
            width,
            thickness,
            fillet,
            material,
            shape,
            north_shape,
            south_shape,
        });
    }

    /**
    Returns the fillet / edge radius of `self`.
     */
    pub fn fillet(&self) -> Length {
        return self.fillet;
    }
}

impl Default for BlockMagnet {
    fn default() -> Self {
        Self::new(
            Length::new::<millimeter>(10.0),
            Length::new::<millimeter>(10.0),
            Length::new::<millimeter>(10.0),
            Length::new::<millimeter>(0.0),
            Arc::new(Material::default()),
        )
        .expect("valid inputs")
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl Magnet for BlockMagnet {
    fn width(&self) -> Length {
        return self.width;
    }

    fn length(&self) -> Length {
        return self.length;
    }

    fn thickness(&self) -> Length {
        return self.thickness;
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
        return self.thickness * self.width + (PI - 4.0) * self.fillet.powi(P2::new());
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for BlockMagnet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde_mosaic::deserialize_arc_link;
        use stem_material::prelude::deserialize_quantity;

        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct BlockMagnetSerde {
            #[serde(deserialize_with = "deserialize_quantity")]
            length: Length,
            #[serde(deserialize_with = "deserialize_quantity")]
            width: Length,
            #[serde(deserialize_with = "deserialize_quantity")]
            thickness: Length,
            #[serde(deserialize_with = "deserialize_quantity")]
            fillet: Length,
            #[serde(deserialize_with = "deserialize_arc_link")]
            material: Arc<Material>,
        }

        let m = BlockMagnetSerde::deserialize(deserializer)?;
        return BlockMagnet::new(m.length, m.width, m.thickness, m.fillet, m.material)
            .map_err(serde::de::Error::custom);
    }
}
