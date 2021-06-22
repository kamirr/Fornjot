use nalgebra::Point;

use crate::geometry::aabb::Aabb;

/// Provides a signed distance function
pub trait Surface<const D: usize> {
    fn sample(&self, point: impl Into<Point<f32, D>>) -> SurfaceSample;
}

#[derive(Debug)]
pub struct SurfaceSample {
    pub distance: f32,
    // TASK: Add normal.
}

pub trait BoundingVolume<const D: usize> {
    /// Axis-aligned bounding box
    fn aabb(&self) -> Aabb<D>;
}
