use fj_interop::debug::DebugInfo;
use fj_kernel::{
    algorithms::Tolerance,
    geometry::Surface,
    shape::{LocalForm, Shape, ValidationError},
    topology::{Cycle, Edge, Face},
};
use fj_math::{Aabb, Point, Scalar};

use super::ToShape;

impl ToShape for fj::Circle {
    fn to_shape(
        &self,
        _: Tolerance,
        _: &mut DebugInfo,
    ) -> Result<Shape, ValidationError> {
        let mut shape = Shape::new();

        // Circles have just a single round edge with no vertices. So none need
        // to be added here.

        let edge = Edge::builder(&mut shape)
            .build_circle(Scalar::from_f64(self.radius()))?;

        let cycle_local = Cycle {
            edges: vec![edge.clone()],
        };
        let cycle_canonical =
            shape.insert(Cycle::new(vec![edge.canonical()]))?;

        let surface = shape.insert(Surface::xy_plane())?;
        shape.insert(Face::new(
            surface,
            vec![LocalForm::new(cycle_local, cycle_canonical)],
            Vec::new(),
            self.color(),
        ))?;

        Ok(shape)
    }

    fn bounding_volume(&self) -> Aabb<3> {
        Aabb {
            min: Point::from([-self.radius(), -self.radius(), 0.0]),
            max: Point::from([self.radius(), self.radius(), 0.0]),
        }
    }
}
