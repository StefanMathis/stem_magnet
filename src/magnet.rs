/*!
The [`Magnet`] trait for defining permanent magnets in stem.

Permanent magnets are used within electric motors to provide a constant magnetic
field. This constant field interacts with the changing field created by a
winding to create a useful force. In stem, any type can be used as a
surface-mounted permanent magnet if it implements the [`Magnet`] trait. See its
docstring for more.
 */

use dyn_clone::DynClone;
use planar_geo::prelude::*;
use std::borrow::Cow;
use std::{any::Any, sync::Arc};
use stem_material::prelude::*;

#[cfg(feature = "cairo")]
use planar_geo::draw::Style;

/**
A trait for defining permanent magnets for radial motors.

This trait provides a simple interface for defining a permanent magnet for a
radial flux motor which provides a source of constant magnetic excitation.
Multiple individual magnets which are mounted in parallel (magnetization axes
are parallel) on a rotor surface form a
[`MagnetAssembly`](crate::assembly::MagnetAssembly). Such an assembly represents
a magnetic pole of the rotor.

The [`Magnet`] trait serves two different functions in stem:
1) Providing information for electromagnetic calculations (e.g. magnetic
excitation, eddy current losses etc.)
2) Representing the cross-section shape of a magnet for collision checks and
visualization.

These two purposes are discussed in the following sections.

# Electromagnetic calculation

Various analytical formulae in electromagnetic models assume a "block"
magnet, where the axial cross section is a rectangle across the entire axial
length of the magnet (in fact, oftentimes models even assume an infinitely long
magnet). Any [`Magnet`] therefore needs to implement the three dimension methods
[`Magnet::length`], [`Magnet::thickness`] and [`Magnet::width`]. The
magnetization is assumed to be parallel and to be perpendicular to the surfaces
formed by the length and width dimensions.

While the geometry methods are trivial to implement for a
[`BlockMagnet`](crate::block::BlockMagnet), some other magnets require
approximations to fit within this interface (what exactly is the "thickness" of
a [`BreadLoafMagnet`](crate::bread_loaf::BreadLoafMagnet), see image below?).
As a general rule of thumb, the area [`Magnet::thickness`] times
[`Magnet::width`] should roughly match that of the actual magnet cross section.
The docstrings of the magnet types defined in this crate provide some examples.

# Cross-section shape

The cross section shape of the magnet is provided by the [`Magnet::shape`]
method. Its origin needs to be located in the middle of the "contact" line to
the rotor, with the y-axis pointing into the air gap. This definition is used
for both rotary and linear motors, as shown below:

*/
#[doc = ""]
#[cfg_attr(
    feature = "doc-images",
    doc = "![Magnet coordinate system in rotary motors][drawing_rotary_motor]"
)]
#[cfg_attr(
    feature = "doc-images",
    embed_doc_image::embed_doc_image("drawing_rotary_motor", "docs/img/drawing_rotary_motor.svg")
)]
#[cfg_attr(
    feature = "doc-images",
    doc = "![Magnet coordinate system in linear motors][drawing_linear_motor]"
)]
#[cfg_attr(
    feature = "doc-images",
    embed_doc_image::embed_doc_image("drawing_linear_motor", "docs/img/drawing_linear_motor.svg")
)]
#[cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile docs with
    `cargo doc --features 'doc-images'` and Rust version >= 1.54."
)]
/**

The second image also provides an example where the magnet thickness is not
constant: While the [`BlockMagnet`](crate::block::BlockMagnet) on the left fits
exactly into the approximation parameters discussed in the previous section, the
mean [`Magnet::thickness`] of the
[`BreadLoafMagnet`](crate::bread_loaf::BreadLoafMagnet) on the right is between
the minimum thickness at the magnet edges and the maximum thickness in the
magnet center.

# Implementation of the trait

When implementing [`Magnet`], the conventions defined above need to be
respected:

- [`Magnet::length`], [`Magnet::thickness`] and [`Magnet::width`] must represent
a block magnet which roughly approximates the magnet.
- [`Magnet::shape`] must return a shape representing the cross section with the
origin at the center of the inner contact surface. This cross section is assumed
to be constant along the axial [`Magnet::length`].
*/
#[cfg_attr(feature = "serde", typetag::serde)]
pub trait Magnet: Sync + Send + DynClone + std::fmt::Debug + Any {
    /**
    Returns the mean width of the magnet.

    This value is used to approximate `self` as a block magnet (together with
    [`Magnet::length`] and [`Magnet::thickness`]). See the [`Magnet`] docstring
    for details.
     */
    fn width(&self) -> Length;

    /**
    Returns the length of the magnet.

    This value is used to approximate `self` as a block magnet (together with
    [`Magnet::thickness`] and [`Magnet::width`]). See the [`Magnet`] docstring
    for details.
     */
    fn length(&self) -> Length;

    /**
    Returns the mean thickness of the magnet along its magnetization axis.

    This value is used to approximate `self` as a block magnet (together with
    [`Magnet::length`] and [`Magnet::width`]). See the [`Magnet`] docstring
    for details.
     */
    fn thickness(&self) -> Length;

    /**
    Returns a shared reference to the conductor material of the magnet.
     */
    fn material(&self) -> &Material;

    /**
    Returns the [`Shape`] of the magnet.

    This is the cross-sectional shape of the magnet in the x-y plane as defined
    in the [trait docstring](Magnet). The returned [`Shape`] must use the
    coordinate system specified in the "Coordinate system" section. It is
    assumed that this cross section is constant along the entire length of the
    magnet.

    Some implementors of [`Magnet`] may construct their shape eagerly during
    initialization, while others may construct it on demand. Returning [`Cow`]
    allows implementations to either return a borrowed precomputed shape or an
    owned value created lazily.

    # Examples

    ```
    use stem_magnet::prelude::*;
    use std::sync::Arc;

    let magnet = BlockMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(20.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(0.0),
        Arc::new(Default::default()),
    ).expect("valid inputs");
    let shape = magnet.shape();
    assert_eq!(shape.area(), 0.0002);
    ```
     */
    fn shape(&self) -> Cow<'_, Shape>;

    // ==============================================================================

    /**
    Returns the conductor material as a reference-counted [`Arc`].

    The default implementation clones the underlying [`Material`]
    and wraps it in a new `Arc`.

    Implementors that internally store their material in an
    [`Arc<Material>`] may override this method to return a clone
    of that `Arc` instead, avoiding an additional allocation
    and material clone.
     */
    fn material_arc(&self) -> Arc<Material> {
        Arc::new(self.material().clone())
    }

    /**
    Returns the cross-section area of the magnet.

    This value equals the [`Shape::area`] of the [`Magnet::shape`].
     */
    fn area(&self) -> Area {
        let shape = self.shape();
        return Area::new::<square_meter>(shape.area());
    }

    /**
    Returns the volume of the magnet.

    This value equals the [`Magnet::area`] times the [`Magnet::length`].
     */
    fn volume(&self) -> Volume {
        return self.area() * self.length();
    }

    /**
    Returns the mass of the magnet.

    This value equals the [`Magnet::volume`] times the
    [`Material::mass_density`] of the [`Magnet::material`].
     */
    fn mass(&self) -> Mass {
        return self.volume() * self.material().mass_density().get(&[]);
    }

    /**
    Returns the magnetomotive force produced by the magnet for the given
    external `conditions`.

    The magnetomotive force `F` is the property of the magnet which creates a
    magnetic flux `Φ` according to Hopkinsons's law:

    `F = Φ * R`

    with `R` being the magnetic resistance or reluctance of the magnetic circuit
    the flux goes through. It is therefore the equivalent to the voltage in
    Ohm's law.

    For a block magnet, it is calculated as:

    `F = Br * t / μ`,

    with `t` being [`Magnet::thickness`], `Br` being the remanence flux density
    of the magnet material and `μ` being the permeability of the magnet
    material (equals relative permeability times magnetic field constant).
    Since both `Br` and `μ` are material properties and can therefore be
    influenced by external conditions such as temperature, the magnetomotive
    force is a function of these external conditions as well.

    # Examples

    ```
    use std::sync::Arc;
    use approx::assert_abs_diff_eq;
    use stem_magnet::prelude::*;
    use stem_magnet::prelude::unary::FirstOrderTaylor;

    let mut material = Material::default();
    material.remanence = VarQuantity::try_from_quantity_function(
        FirstOrderTaylor::new(
            DynQuantity::new(0.43, PredefUnit::MagneticFluxDensity),
            DynQuantity::new(-0.001733, Unit::from(PredefUnit::Temperature).powi(-1)),
            DynQuantity::new(293.15, PredefUnit::Temperature),
        ).expect("valid inputs")
    ).expect("valid inputs");

    let magnet = BlockMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(20.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(0.0),
        Arc::new(material),
    ).expect("valid inputs");

    // Magnetomotive force at 20 degree celsius
    assert_abs_diff_eq!(
        magnet.magnetomotive_force(&[ThermodynamicTemperature::new::<degree_celsius>(20.0).into()]).get::<ampere>(),
        3421.831, epsilon = 1e-3
    );

    // Magnetomotive force at 120 degree celsius
    assert_abs_diff_eq!(
        magnet.magnetomotive_force(&[ThermodynamicTemperature::new::<degree_celsius>(120.0).into()]).get::<ampere>(),
        2828.828, epsilon = 1e-3
    );
    ```
     */
    fn magnetomotive_force(&self, conditions: &[DynQuantity<f64>]) -> ElectricCurrent {
        let remanence = self.material().remanence().get(conditions);
        if remanence > MagneticFluxDensity::new::<tesla>(0.0) {
            let rel_permeability = self.material().relative_permeability().get(conditions);
            return remanence * self.thickness() / (rel_permeability * *VACUUM_PERMEABILITY);
        } else {
            return ElectricCurrent::new::<ampere>(0.0);
        }
    }

    /**
    Returns the north- and south pole [`Shape`]s of the magnet.

    The default implementation of this function splits the [`Shape`] returned by
    [`Magnet::shape`] at its centroid the magnetization axis into two shapes
    representing the north and south pole of the magnet. These shapes are purely
    used for visualization of the magnet, hence this method is hidden behind the
    `visualize` feature flag.

    When defining a custom magnet shape, it is recommended to overwrite this
    method, as the default implementation may produce unasthetic result for non-
    block magnets and also assumes that the shape only has an outer contour
    (i.e. no holes).

    # Examples

    ```
    use std::sync::Arc;
    use approx::assert_abs_diff_eq;
    use stem_magnet::prelude::*;

    let magnet = BlockMagnet::new(
        Length::new::<millimeter>(165.0),
        Length::new::<millimeter>(20.0),
        Length::new::<millimeter>(10.0),
        Length::new::<millimeter>(2.0),
        Arc::new(Material::default()),
    ).expect("valid inputs");

    let [north, south] = magnet.north_south_shapes();
    assert_abs_diff_eq!(magnet.shape().area(), north.area() + south.area());
    ```
     */
    fn north_south_shapes(&self) -> [Cow<'_, Shape>; 2] {
        let shape: Cow<'_, Shape> = self.shape();
        let mut bb = shape.bounding_box();
        bb.scale(2.0); // To account for float rounding errors;
        let y = shape.centroid()[1];
        let cut_line = Polysegment::from_points(&[[bb.xmin(), y], [bb.xmax(), y]]);

        // If "cutted" does not contain two results, give up and just return the
        // initial shape twice. The algorithm clearly does not work for this
        // magnet geometry ...
        let cutted: Vec<Polysegment> =
            shape
                .contour()
                .intersection_cut(&cut_line, DEFAULT_EPSILON, DEFAULT_MAX_ULPS);
        if cutted.len() == 2 {
            let mut it = cutted.into_iter();
            let north_chain = it.next().expect("has two elements");
            let south_chain = it.next().expect("has two elements");
            if let Ok(north) = Shape::new(vec![Contour::new(north_chain)]) {
                if let Ok(south) = Shape::new(vec![Contour::new(south_chain)]) {
                    return [Cow::Owned(north), Cow::Owned(south)];
                }
            }
        }
        match shape {
            Cow::Borrowed(shape) => {
                return [Cow::Borrowed(shape), Cow::Borrowed(shape)];
            }
            Cow::Owned(shape) => {
                return [Cow::Owned(shape.clone()), Cow::Owned(shape)];
            }
        }
    }

    /// Returns a vector of [`Shape`]-[`Style`] pairs for drawing the magnet.
    ///
    /// Using [`Shape::draw`], the elements of the returned vector can be used
    /// to draw the magnet on a
    /// [`cairo::Context`](https://docs.rs/cairo-rs/latest/cairo/struct.Context.html).
    /// If `split` is `true`, the returned vector has two elements
    /// (from [`Magnet::north_south_shapes`]) representing the north and the
    /// south pole. If `invert` is `true`, north and south pole are
    /// inverted.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use cairo_viewport::*;
    /// use stem_magnet::prelude::*;
    ///
    /// let magnet = BlockMagnet::new(
    ///     Length::new::<millimeter>(165.0),
    ///     Length::new::<millimeter>(20.0),
    ///     Length::new::<millimeter>(10.0),
    ///     Length::new::<millimeter>(2.0),
    ///     Arc::new(Material::default()),
    /// ).expect("valid inputs");
    ///
    /// let drawables = magnet.drawables(true, false);
    /// let viewport = Viewport::from_bounded_entities(drawables.iter(), SideLength::Long(500)).unwrap();
    /// let path = std::path::Path::new("docs/img/block_magnet_north_south.svg");
    ///
    /// viewport.write_to_file(path, move |cr| {
    ///     cr.set_source_rgb(1.0, 1.0, 1.0);
    ///     for drawable in drawables.iter() {
    ///         drawable.draw(cr)?;
    ///     }
    ///     return Ok(());
    /// });
    ///
    /// let drawables = magnet.drawables(true, true);
    /// let viewport = Viewport::from_bounded_entities(drawables.iter(), SideLength::Long(500)).unwrap();
    /// let path = std::path::Path::new("docs/img/block_magnet_south_north.svg");
    ///
    /// viewport.write_to_file(path, move |cr| {
    ///     cr.set_source_rgb(1.0, 1.0, 1.0);
    ///     for drawable in drawables.iter() {
    ///         drawable.draw(cr)?;
    ///     }
    ///     return Ok(());
    /// });
    /// ```
    #[doc = ""]
    #[cfg_attr(
        feature = "doc-images",
        doc = "![Magnet with normal north-south orientation][block_magnet_north_south]"
    )]
    #[cfg_attr(
        feature = "doc-images",
        embed_doc_image::embed_doc_image(
            "block_magnet_north_south",
            "docs/img/block_magnet_north_south.svg"
        )
    )]
    #[cfg_attr(
        feature = "doc-images",
        doc = "![Magnet with inverted north-south orientation][block_magnet_south_north]"
    )]
    #[cfg_attr(
        feature = "doc-images",
        embed_doc_image::embed_doc_image(
            "block_magnet_south_north",
            "docs/img/block_magnet_south_north.svg"
        )
    )]
    #[cfg_attr(
        not(feature = "doc-images"),
        doc = "**Doc images not enabled**. Compile docs with
        `cargo doc --features 'doc-images'` and Rust version >= 1.54."
    )]
    #[cfg(feature = "cairo")]
    fn drawables(&self, split: bool, invert: bool) -> Vec<DrawableCow<'_>> {
        if split {
            let shapes = self.north_south_shapes();
            let mut dark_green = Style::default();
            dark_green.background_color = crate::DARK_GREEN;

            let mut red = Style::default();
            red.background_color = crate::RED;

            let styles = if invert {
                [red, dark_green]
            } else {
                [dark_green, red]
            };

            return shapes
                .into_iter()
                .zip(styles.into_iter())
                .map(DrawableCow::from)
                .collect();
        } else {
            let shape = self.shape();
            let mut style = Style::default();
            style.background_color = crate::RED;
            return vec![DrawableCow::from((shape, style))];
        }
    }
}
