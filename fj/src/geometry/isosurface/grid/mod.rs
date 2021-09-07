mod cell;
mod descriptor;
mod edge;
mod index;
mod surface_vertices;
mod vertex;

pub use self::{
    cell::Cell, descriptor::Descriptor, edge::Edge, index::Index,
    vertex::Vertex,
};

use std::collections::BTreeMap;

use nalgebra::Point;

use crate::geometry::traits::{Geometry, Normal};

use self::surface_vertices::SurfaceVertices;

use super::place_surface_vertex::place_surface_vertex;

// TASK: Don't use uniform grid when sampling geometry. Use an adaptive octree
//       instead (as described in the paper), to increase performance and reduce
//       memory use.

/// A uniform grid for isosurface extraction
#[derive(Debug)]
pub struct Grid {
    edges: BTreeMap<(Index, Index), Edge>,
    surface_vertices: SurfaceVertices,
}

impl Grid {
    /// Create the grid from the descriptor and populate it with distance values
    pub fn from_descriptor(
        descriptor: Descriptor,
        geometry: &(impl Geometry<3> + Normal<3>),
    ) -> Self {
        let mut grid_vertex_samples = BTreeMap::new();
        let mut edges = BTreeMap::new();

        let surface_vertices = descriptor
            .cells()
            .filter_map(|cell| {
                for index in cell.vertices() {
                    let vertex = index.to_position(
                        descriptor.aabb.min,
                        descriptor.resolution,
                    );
                    grid_vertex_samples
                        .entry(index)
                        .or_insert_with(|| geometry.sample(vertex));
                }

                let mut points_and_normals = Vec::new();

                for (a, b) in cell.edges() {
                    let sample_a = grid_vertex_samples[&a];
                    let sample_b = grid_vertex_samples[&b];

                    let edge = Edge {
                        a: Vertex {
                            index: a,
                            point: sample_a.point,
                            distance: sample_a.distance,
                        },
                        b: Vertex {
                            index: b,
                            point: sample_b.point,
                            distance: sample_b.distance,
                        },
                    };

                    if edge.at_surface() {
                        edges.insert((a, b), edge);

                        let f = edge.a.distance.abs()
                            / (edge.a.distance.abs() + edge.b.distance.abs());

                        assert!(f.is_finite());
                        assert!(!f.is_nan());

                        let point =
                            edge.a.point + (edge.b.point - edge.a.point) * f;
                        let normal = geometry.normal(point);

                        points_and_normals.push((point, normal));
                    }
                }

                if points_and_normals.len() == 0 {
                    return None;
                }

                let surface_vertex = place_surface_vertex(
                    cell,
                    descriptor.resolution,
                    &points_and_normals,
                );

                Some((cell.min_index, surface_vertex))
            })
            .collect();

        Self {
            edges,
            surface_vertices: SurfaceVertices(surface_vertices),
        }
    }

    /// Iterate over all grid edges that are near a surface
    pub fn edges_at_surface(&self) -> impl Iterator<Item = Edge> + '_ {
        self.edges.values().copied()
    }

    /// Get the 4 neighboring surface vertices of a grid edge
    pub fn neighbors_of_edge(&self, edge: Edge) -> [Point<f32, 3>; 4] {
        self.surface_vertices.neighbors_of_edge(edge)
    }
}
