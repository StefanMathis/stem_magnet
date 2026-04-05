/*!
[`Magnet`]: crate::magnet::Magnet
[`BlockMagnet`]: crate::block::BlockMagnet
[`BreadLoafMagnet`]: crate::bread_loaf::BreadLoafMagnet
[`ArcParallelMagnet`]: crate::arc::ArcParallelMagnet
[`ArcSegmentMagnet`]: crate::arc::ArcSegmentMagnet

Permanent magnet definition for stem - a Simulation Toolbox for Electric Motors.

 */
#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("magnet_types_overview.svg", "docs/img/magnet_types_overview.svg"),
))]
#![cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile docs with `cargo doc --features 'doc-images'` and Rust version >= 1.54."
)]
#![doc = include_str!("../docs/main.md")]
#![deny(missing_docs)]

pub mod arc;
pub mod assembly;
pub mod block;
pub mod bread_loaf;
pub mod error;
pub mod magnet;

/// Color used for the south side when visualizing a magnet.
#[cfg(feature = "cairo")]
pub const DARK_GREEN: planar_geo::draw::Color = planar_geo::draw::Color {
    r: 0.0,
    g: 0.5,
    b: 0.0,
    a: 1.0,
};

/// Color used for the north side when visualizing a magnet.
#[cfg(feature = "cairo")]
pub const RED: planar_geo::draw::Color = planar_geo::draw::Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

pub mod prelude {
    /*!
    This module reexports all wire types defined in stem_material, the
    [`Magnet`] trait as well as the [`stem_material::prelude`]
    module to simplify the usage of this crate.
     */

    pub use crate::arc::*;
    pub use crate::assembly::MagnetAssembly;
    pub use crate::block::BlockMagnet;
    pub use crate::bread_loaf::BreadLoafMagnet;
    pub use crate::magnet::Magnet;
    pub use stem_material;

    pub use stem_material::prelude::*;
}
