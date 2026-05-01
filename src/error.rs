/*!
This module contains the [`Error`] enum, which represents the different ways
building one of the predefined wires can fail due to invalid input data. The
[`Error::Other`] variants supports arbitrary errors resulting from user-created
wire types.
*/

use compare_variables::Comparison;
use planar_geo::{error::ShapeConstructorError, prelude::*};
use stem_material::uom::si::f64::Length;

/// An enum representing errors returned by [`Magnet`](crate::magnet::Magnet)
/// constructors.
#[derive(Debug)]
pub enum Error {
    /**
    A given physical [`Length`] is not within its allowed value range (as
    specified inside the [`Comparison`], usually a length needs to be
    positive).
     */
    InvalidLength(Comparison<Length>),
    /// A given [`usize`] is not within its allowed value range.
    InvalidUsize(Comparison<usize>),
    /// A given [`f64`] is not within its allowed value range.
    InvalidF64(Comparison<f64>),
    /// Failed to create a magnet geometry due to the contained error.
    GeometryError(planar_geo::error::Error),
    /// Fallback variant for arbitrary other errors (e.g. from custom
    /// [`Magnet`](crate::magnet::Magnet) implementations).
    Other(Box<dyn std::error::Error>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidLength(comparison_error) => comparison_error.fmt(f),
            Error::InvalidUsize(comparison_error) => comparison_error.fmt(f),
            Error::InvalidF64(comparison_error) => comparison_error.fmt(f),
            Error::GeometryError(err) => err.fmt(f),
            Error::Other(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<Comparison<Length>> for Error {
    fn from(value: Comparison<Length>) -> Self {
        return Error::InvalidLength(value);
    }
}

impl From<Comparison<usize>> for Error {
    fn from(value: Comparison<usize>) -> Self {
        return Error::InvalidUsize(value);
    }
}

impl From<Comparison<f64>> for Error {
    fn from(value: Comparison<f64>) -> Self {
        return Error::InvalidF64(value);
    }
}

impl From<planar_geo::error::Error> for Error {
    fn from(value: planar_geo::error::Error) -> Self {
        return Error::GeometryError(value);
    }
}

impl From<ShapeConstructorError<Vec<Contour>>> for Error {
    fn from(value: ShapeConstructorError<Vec<Contour>>) -> Self {
        return planar_geo::error::Error::from(value).into();
    }
}

impl From<ShapeConstructorError<Contour>> for Error {
    fn from(value: ShapeConstructorError<Contour>) -> Self {
        return planar_geo::error::Error::from(value).into();
    }
}
