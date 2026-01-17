use glam::Vec3;
use parry3d::math::Point;
use parry3d::shape::TriMesh;

const TRIANGLE_VERTEX_COUNT: usize = 3;

#[derive(Debug)]
pub enum TriMeshFromPositionsError {
    EmptyVertexSet,
    VertexCountNotMultipleOf3,
}

pub trait TriMeshFromPositionsExt {
    fn from_positions(
        positions: impl Iterator<Item = Vec3>,
    ) -> Result<TriMesh, TriMeshFromPositionsError>;
}

impl TriMeshFromPositionsExt for TriMesh {
    fn from_positions(
        positions: impl Iterator<Item = Vec3>,
    ) -> Result<TriMesh, TriMeshFromPositionsError> {
        let mut vertices: Vec<Point<f32>> = Vec::new();

        for position in positions {
            vertices.push(Point::new(position.x, position.y, position.z));
        }

        if vertices.is_empty() {
            return Err(TriMeshFromPositionsError::EmptyVertexSet);
        }
        if !vertices.len().is_multiple_of(TRIANGLE_VERTEX_COUNT) {
            return Err(TriMeshFromPositionsError::VertexCountNotMultipleOf3);
        }

        let mut indices = Vec::new();
        let tri_count = vertices.len() / TRIANGLE_VERTEX_COUNT;
        for tri in 0..tri_count {
            let base = (tri * TRIANGLE_VERTEX_COUNT) as u32;
            indices.push([base, base + 1, base + 2]);
        }

        return Ok(TriMesh::new(vertices, indices));
    }
}
