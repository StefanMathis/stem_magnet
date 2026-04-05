> **Feedback welcome!**  
> Found a bug, missing docs, or have a feature request?  
> Please open an issue on [GitHub](https://github.com/StefanMathis/stem_magnet.git).

This crate provides the [`Magnet`] trait for permanent magnets in the stem
(Simulation Toolbox for Electric Motors) framework (see the
[stem book](https://stefanmathis.github.io/stem_book/)). The trait itself has
little logic and mainly specifies an interface for using permanent magnets in
simulation models.

The following predefined implementors of [`Magnet`] are available:
- [`BlockMagnet`]: A cuboid magnet, possibly with fillets.
- [`BreadLoafMagnet`]: A cuboid magnet for surface mounting where the surface
facing the air gap is curved to improve the air gap field.
- [`ArcParallelMagnet`]: A curved magnet for surface mounting in rotary machines
where the sides are parallel to each other.
- [`ArcSegmentMagnet`]: A curved magnet for surface mounting in rotary machines
where the sides are radially oriented.

![Magnet types overview][magnet_types_overview.svg]

If those magnet types do not suffice, it is very easy to define your own magnet
type by implementing [`Magnet`]. See the trait documentation for details.

# Serialization and deserialization

If the `serde` feature is enabled, all magnet types from this crate can be
serialized and deserialized. During deserialization, the invariants are
validated (to e.g. prevent negative length for a [`BlockMagnet`]).

Units and quantities can be deserialized from strings representing SI units via
the [dyn_quantity](https://crates.io/crates/dyn_quantity) crate. Similarily,
it is possible to serialize the quantities of a wire as value-unit strings using
the [serialize_with_units](https://docs.rs/dyn_quantity/latest/dyn_quantity/quantity/serde_impl/fn.serialize_with_units.html) function.

See the chapter [serialization and deserialization](https://stefanmathis.github.io/stem_book/serialization_and_deserialization.html) of the [stem book](https://stefanmathis.github.io/stem_book/)
for details.

# Acknowledgments

The technical drawings used in the docstrings have been created using 
LibreCAD (<https://librecad.org/>).
