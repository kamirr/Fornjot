pub mod circle;
pub mod difference_2d;
pub mod difference_3d;
pub mod sketch;
pub mod sweep;
pub mod transform;
pub mod union;

use crate::{
    debug::DebugInfo,
    math::{Aabb, Scalar},
};

use super::topology::{edges::Edges, faces::Faces, vertices::Vertices};

/// Implemented by all shapes
pub trait ToShape {
    /// Access the axis-aligned bounding box of a shape
    ///
    /// If a shape is empty, its [`Aabb`]'s `min` and `max` points must be equal
    /// (but are otherwise not specified).
    fn bounding_volume(&self) -> Aabb<3>;

    /// Compute triangles to approximate the shape's faces
    ///
    /// The shape defined by the approximated triangles must be fully contained
    /// within the actual shape.
    ///
    /// `tolerance` defines by how far this triangulation is allowed to deviate
    /// from the faces' actual dimensions.
    fn faces(&self, tolerance: Scalar, debug: &mut DebugInfo) -> Faces;

    /// Access the edges of the shape
    fn edges(&self) -> Edges;

    /// Return the shape's vertices
    fn vertices(&self) -> Vertices;
}

macro_rules! dispatch {
    ($($method:ident($($arg_name:ident: $arg_ty:ty,)*) -> $ret:ty;)*) => {
        impl ToShape for fj::Shape {
            $(
                fn $method(&self, $($arg_name: $arg_ty,)*) -> $ret {
                    match self {
                        Self::Shape2d(shape) => shape.$method($($arg_name,)*),
                        Self::Shape3d(shape) => shape.$method($($arg_name,)*),
                    }
                }
            )*
        }

        impl ToShape for fj::Shape2d {
            $(
                fn $method(&self, $($arg_name: $arg_ty,)*) -> $ret {
                    match self {
                        Self::Circle(shape) => shape.$method($($arg_name,)*),
                        Self::Difference(shape) => shape.$method($($arg_name,)*),
                        Self::Sketch(shape) => shape.$method($($arg_name,)*),
                    }
                }
            )*
        }

        impl ToShape for fj::Shape3d {
            $(
                fn $method(&self, $($arg_name: $arg_ty,)*) -> $ret {
                    match self {
                        Self::Difference(shape) => shape.$method($($arg_name,)*),
                        Self::Sweep(shape) => shape.$method($($arg_name,)*),
                        Self::Transform(shape) => shape.$method($($arg_name,)*),
                        Self::Union(shape) => shape.$method($($arg_name,)*),
                    }
                }
            )*
        }
    };
}

dispatch! {
    bounding_volume() -> Aabb<3>;
    faces(
        tolerance: Scalar,
        debug: &mut DebugInfo,
    ) -> Faces;
    edges() -> Edges;
    vertices() -> Vertices;
}
