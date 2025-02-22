use std::fmt;

use fj_math::{Circle, Line, Point, Transform, Vector};

use crate::geometry;

/// A one-dimensional shape
///
/// The word "curve" is used as an umbrella term for all one-dimensional shapes,
/// and doesn't imply that those shapes need to be curved. Straight lines are
/// included.
///
/// The nomenclature is inspired by Boundary Representation Modelling Techniques
/// by Ian Stroud. "Curve" refers to unbounded one-dimensional geometry, while
/// while edges are bounded portions of curves.
///
/// The `D` parameter defines the dimensions in which the curve is defined.
/// Typically, only `2` or `3` make sense, which means the curve is defined on
/// a surface or in a space, respectively.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Curve<const D: usize> {
    /// A circle
    Circle(Circle<D>),

    /// A line
    Line(Line<D>),
}

impl<const D: usize> Curve<D> {
    /// Access the origin of the curve's coordinate system
    pub fn origin(&self) -> Point<D> {
        match self {
            Self::Circle(curve) => curve.center,
            Self::Line(curve) => curve.origin,
        }
    }

    /// Create a new instance that is reversed
    #[must_use]
    pub fn reverse(self) -> Self {
        match self {
            Self::Circle(curve) => Self::Circle(curve.reverse()),
            Self::Line(curve) => Self::Line(curve.reverse()),
        }
    }

    /// Convert a point in model coordinates to curve coordinates
    ///
    /// Projects the point onto the curve before computing curve coordinate.
    /// This is done to make this method robust against floating point accuracy
    /// issues.
    ///
    /// Callers are advised to be careful about the points they pass, as the
    /// point not being on the curve, intentional or not, will never result in
    /// an error.
    pub fn point_to_curve_coords(
        &self,
        point: impl Into<Point<D>>,
    ) -> geometry::Point<1, D> {
        let point_canonical = point.into();

        let point_local = match self {
            Self::Circle(curve) => {
                curve.point_to_circle_coords(point_canonical)
            }
            Self::Line(curve) => curve.point_to_line_coords(point_canonical),
        };

        geometry::Point::new(point_local, point_canonical)
    }

    /// Convert a point on the curve into model coordinates
    pub fn point_from_curve_coords(
        &self,
        point: impl Into<Point<1>>,
    ) -> Point<D> {
        match self {
            Self::Circle(curve) => curve.point_from_circle_coords(point),
            Self::Line(curve) => curve.point_from_line_coords(point),
        }
    }

    /// Convert a vector on the curve into model coordinates
    pub fn vector_from_curve_coords(
        &self,
        point: impl Into<Vector<1>>,
    ) -> Vector<D> {
        match self {
            Self::Circle(curve) => curve.vector_from_circle_coords(point),
            Self::Line(curve) => curve.vector_from_line_coords(point),
        }
    }
}

impl Curve<2> {
    /// Construct a `Curve` that represents the u-axis
    pub fn u_axis() -> Self {
        Self::Line(Line {
            origin: Point::origin(),
            direction: Vector::unit_u(),
        })
    }

    /// Construct a `Curve` that represents the v-axis
    pub fn v_axis() -> Self {
        Self::Line(Line {
            origin: Point::origin(),
            direction: Vector::unit_v(),
        })
    }
}

impl Curve<3> {
    /// Construct a `Curve` that represents the x-axis
    pub fn x_axis() -> Self {
        Self::Line(Line {
            origin: Point::origin(),
            direction: Vector::unit_x(),
        })
    }

    /// Construct a `Curve` that represents the y-axis
    pub fn y_axis() -> Self {
        Self::Line(Line {
            origin: Point::origin(),
            direction: Vector::unit_y(),
        })
    }

    /// Construct a `Curve` that represents the z-axis
    pub fn z_axis() -> Self {
        Self::Line(Line {
            origin: Point::origin(),
            direction: Vector::unit_z(),
        })
    }

    /// Create a new instance that is transformed by `transform`
    #[must_use]
    pub fn transform(self, transform: &Transform) -> Self {
        match self {
            Self::Circle(curve) => {
                Self::Circle(transform.transform_circle(&curve))
            }
            Self::Line(curve) => Self::Line(transform.transform_line(&curve)),
        }
    }
}

impl<const D: usize> fmt::Display for Curve<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Circle(curve) => write!(f, "{:?}", curve),
            Self::Line(curve) => write!(f, "{:?}", curve),
        }
    }
}
