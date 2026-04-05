/*!
This module provides the [`MagnetAssembly`], which forms a magnetic rotor pole
out of one or more magnets which are packed next to each other. See the struct
documentation for more.
 */

use std::num::NonZeroUsize;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use stem_material::prelude::*;

use crate::magnet::Magnet;

/**
An assembly of one or more magnets which forms a magnetic rotor pole.

Oftentimes, a pole of a rotor with surface-mounted permanent magnets does not
consist of a single large magnet, but of multiple small ones. This is done for
several reasons: Reduction of eddy currents due smaller current paths, easier
handling in manufcaturing, cost reduction etc.

In the stem framework, the following assumptions for such an assembly are made:
- Only one instance of a [`Magnet`] (the "base magnet") is used.
- The base magnet is repeated [`MagnetAssembly::number_axial`] times along the
axial air gap direction. The individual magnets are placed next to each other
without a gap inbetween. This number must not be zero, which is enforced by the
type system in the constructors.
- The base magnet is repeated [`MagnetAssembly::number_tangential`] times along
the tangential air gap direction. The individual magnets are placed next to each
other without a gap inbetween. The image below shows an example for an assembly
consisting of a [`ArcSegmentMagnet`](crate::arc::ArcSegmentMagnet) repeated 3
times. This number must not be zero, which is enforced by the type system in the
constructors.
 */
#[doc = ""]
#[cfg_attr(
    feature = "doc-images",
    doc = "![Example of a magnet assembly with three tangentially repeated magnets][drawing_magnet_assembly]"
)]
#[cfg_attr(
    feature = "doc-images",
    embed_doc_image::embed_doc_image(
        "drawing_magnet_assembly",
        "docs/img/drawing_magnet_assembly.svg"
    )
)]
#[cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile docs with
    `cargo doc --features 'doc-images'` and Rust version >= 1.54."
)]
/**
- The individual magnets produce the same resulting magnetic field as a single,
large magnet of the same dimensions would, see
[`MagnetAssembly::magnetomotive_force`].

To deserialize a [`MagnetAssembly`], the fields `magnet`, `number_axial` and
`number_tangential` need to be specified (similar to the
[`new`](MagnetAssembly::new)) constructor. The latter two fields must not be
zero.
```
use approx;
use stem_magnet::prelude::*;
use serde_yaml;

let str = indoc::indoc! {"
magnet:
    BlockMagnet:
        length: 100 mm
        width: 20 mm
        thickness: 10 mm
        fillet: 0 mm
        material:
            name: NMF-12J 430mT
            relative_permeability: 1.05
number_axial: 2
number_tangential: 3
"};

let assembly: MagnetAssembly = serde_yaml::from_str(&str).expect("valid dimensions");
assert_eq!(assembly.length().get::<millimeter>(), 200.0);

// Number of axial magnets must not be zero!
let str = indoc::indoc! {"
magnet:
    BlockMagnet:
        length: 100 mm
        width: 20 mm
        thickness: 10 mm
        fillet: 0 mm
        material:
            name: NMF-12J 430mT
            relative_permeability: 1.05
number_axial: 0
number_tangential: 3
"};
assert!(serde_yaml::from_str::<MagnetAssembly>(&str).is_err());
```
 */
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct MagnetAssembly {
    magnet: Box<dyn Magnet>,
    number_axial: usize,
    number_tangential: usize,
}

impl MagnetAssembly {
    /**
    Creates a [`MagnetAssembly`] from the base `magnet` and the number of axial
    and tangential repetitions.

    This is a convenience wrapper around [`MagnetAssembly::from_boxed`], which
    wraps the given magnet in a [`Box`].
     */
    pub fn new<M: Magnet>(
        magnet: M,
        number_axial: NonZeroUsize,
        number_tangential: NonZeroUsize,
    ) -> MagnetAssembly {
        return MagnetAssembly {
            magnet: Box::new(magnet),
            number_axial: number_axial.into(),
            number_tangential: number_tangential.into(),
        };
    }

    /**
    Creates a [`MagnetAssembly`] from the base `magnet` and the number of axial
    and tangential repetitions.
    */
    pub fn from_boxed(
        magnet: Box<dyn Magnet>,
        number_axial: NonZeroUsize,
        number_tangential: NonZeroUsize,
    ) -> MagnetAssembly {
        return MagnetAssembly {
            magnet,
            number_axial: number_axial.into(),
            number_tangential: number_tangential.into(),
        };
    }

    /// Returns a reference to the base magnet.
    pub fn magnet(&self) -> &dyn Magnet {
        return &*self.magnet;
    }

    /// Returns the number of times the base magnet is repeated axially.
    pub fn number_axial(&self) -> usize {
        return self.number_axial;
    }

    /// Returns the number of times the base magnet is repeated tangentially.
    pub fn number_tangential(&self) -> usize {
        return self.number_tangential;
    }

    /**
    Returns how many individual magnets are used in `self`.

    This is the product of [`MagnetAssembly::number_axial()`] and
    [`MagnetAssembly::number_tangential()`].

    # Examples

    ```
    use stem_magnet::prelude::*;
    use std::sync::Arc;

    let magnet = BlockMagnet::new(
        Length::new::<millimeter>(100.0),
        Length::new::<millimeter>(20.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(2.0),
        Arc::new(Default::default()),
    ).expect("valid inputs");
    let assembly = MagnetAssembly::new(magnet, 2.try_into().unwrap(), 3.try_into().unwrap());
    assert_eq!(assembly.number_magnets(), 6);
    ```
     */
    pub fn number_magnets(&self) -> usize {
        return self.number_axial * self.number_tangential;
    }

    /**
    Returns the total width of the assembly.

    This is the product of `self.magnet().width()` and
    [`MagnetAssembly::number_tangential`].

    # Examples

    ```
    use stem_magnet::prelude::*;
    use std::sync::Arc;

    let magnet = BlockMagnet::new(
        Length::new::<millimeter>(100.0),
        Length::new::<millimeter>(20.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(0.0),
        Arc::new(Default::default()),
    ).expect("valid inputs");
    let assembly = MagnetAssembly::new(magnet, 2.try_into().unwrap(), 3.try_into().unwrap());
    assert_eq!(assembly.width().get::<millimeter>(), 60.0);
    ```
     */
    pub fn width(&self) -> Length {
        return self.magnet().width() * self.number_tangential as f64;
    }

    /**
    Returns the total length of the assembly.

    This is the product of `self.magnet().length()` and
    [`MagnetAssembly::number_axial`].

    # Examples

    ```
    use stem_magnet::prelude::*;
    use std::sync::Arc;

    let magnet = BlockMagnet::new(
        Length::new::<millimeter>(100.0),
        Length::new::<millimeter>(20.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(0.0),
        Arc::new(Default::default()),
    ).expect("valid inputs");
    let assembly = MagnetAssembly::new(magnet, 2.try_into().unwrap(), 3.try_into().unwrap());
    assert_eq!(assembly.length().get::<millimeter>(), 200.0);
    ```
     */
    pub fn length(&self) -> Length {
        return self.magnet().length() * self.number_axial as f64;
    }

    /**
    Returns the total volume of the assembly.

    This is the product of `self.magnet().volume()` and
    [`MagnetAssembly::number_magnets`].

    # Examples

    ```
    use stem_magnet::prelude::*;
    use std::sync::Arc;

    let magnet = BlockMagnet::new(
        Length::new::<millimeter>(100.0),
        Length::new::<millimeter>(20.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(0.0),
        Arc::new(Default::default()),
    ).expect("valid inputs");
    let assembly = MagnetAssembly::new(magnet, 2.try_into().unwrap(), 3.try_into().unwrap());
    assert_eq!(assembly.volume().get::<cubic_millimeter>(), 120000.0);
    ```
     */
    pub fn volume(&self) -> Volume {
        return self.magnet().volume() * self.number_magnets() as f64;
    }

    /**
    Returns the total mass of the assembly.

    This is the product of `self.magnet().mass()` and
    [`MagnetAssembly::number_magnets`].

    # Examples

    ```
    use stem_magnet::prelude::*;
    use std::sync::Arc;

    let magnet = BlockMagnet::new(
        Length::new::<millimeter>(100.0),
        Length::new::<millimeter>(20.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(0.0),
        Arc::new(Default::default()),
    ).expect("valid inputs");
    let assembly = MagnetAssembly::new(magnet, 2.try_into().unwrap(), 3.try_into().unwrap());
    assert_eq!(assembly.mass().get::<kilogram>(), 0.12);
    ```
     */
    pub fn mass(&self) -> Mass {
        return self.magnet().mass() * self.number_magnets() as f64;
    }

    /**
    Returns the total magnetomotive force created by the assembly.

    This is the product of `self.magnet().magnetomotive()` and
    [`MagnetAssembly::number_magnets`].

    # Examples

    ```
    use approx::assert_abs_diff_eq;
    use stem_magnet::prelude::*;
    use std::sync::Arc;

    let mut material = Material::default();
    material.remanence = MagneticFluxDensity::new::<tesla>(1.0).into();

    let magnet = BlockMagnet::new(
        Length::new::<millimeter>(100.0),
        Length::new::<millimeter>(20.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(0.0),
        Arc::new(material),
    ).expect("valid inputs");
    let assembly = MagnetAssembly::new(magnet, 2.try_into().unwrap(), 3.try_into().unwrap());

    // Magnetomotive force of a single magnet
    assert_abs_diff_eq!(assembly.magnet().magnetomotive_force(&[]).get::<ampere>(), 7957.747, epsilon=1e-3);

    // Magnetomotive force of all six magnets
    assert_abs_diff_eq!(assembly.magnetomotive_force(&[]).get::<ampere>(), 47746.482, epsilon=1e-3);
    ```
     */
    pub fn magnetomotive_force(&self, conditions: &[DynQuantity<f64>]) -> ElectricCurrent {
        return self.magnet().magnetomotive_force(conditions) * self.number_magnets() as f64;
    }
}

impl Clone for MagnetAssembly {
    fn clone(&self) -> Self {
        Self {
            magnet: dyn_clone::clone_box(&*self.magnet),
            number_axial: self.number_axial.clone(),
            number_tangential: self.number_tangential.clone(),
        }
    }
}
#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for MagnetAssembly {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct MagnetAssemblyNonZero {
            magnet: Box<dyn Magnet>,
            number_axial: NonZeroUsize,
            number_tangential: NonZeroUsize,
        }
        let a = MagnetAssemblyNonZero::deserialize(deserializer)?;
        return Ok(MagnetAssembly {
            magnet: a.magnet,
            number_axial: a.number_axial.into(),
            number_tangential: a.number_tangential.into(),
        });
    }
}
