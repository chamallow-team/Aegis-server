//! This module contains methods to transform a graph into a beautiful Mesh
//!
//! Please note that it'll not give a width to the edges and nodes. It'll simply transform the map graph
//! in a 3d space.
//!
//! For outlining the edges and nodes, please see [bevy_mod_outline](https://docs.rs/bevy_mod_outline/latest/bevy_mod_outline/)

use bevy::math::Vec3;
use bevy::prelude::Mesh;
use bevy::render::mesh::Indices;
use petgraph::graph::NodeIndex;
use petgraph::graph::UnGraph;

/// This trait must be implemented for all nodes that can be passed to the mesh builder
pub trait MeshNode: Clone {
    /// Get the coordinates of the current node
    fn get_coordinates(&self) -> bevy::math::Vec2;
}

/// This trait must be implemented for all edges that can be passed to the mesh builder
pub trait MeshEdge: Clone {
    /// Get the two nodes at the extremities
    fn get_nodes(&self) -> (NodeIndex, NodeIndex);
}

// impl MeshEdge for () {
//     fn get_nodes(&self) -> (NodeIndex, NodeIndex) {
//         (self.source(), self.target())
//     }
// }

/// Takes an undirected graph and a mesh at the start to transform the graph into a mesh.
///
/// # Requirements
/// The mesh must use the topology [`PrimitiveTopology::TRIANGLES_LIST`](https://docs.rs/bevy/latest/bevy/render/mesh/enum.PrimitiveTopology.html), otherwise you will run into non so-pretty errors and weird bugs
pub fn build_mesh<N, E>(graph: &UnGraph<N, E>, mesh: &mut Mesh)
where
    N: MeshNode,
{
    let mut positions: Vec<Vec3> = Vec::new();
    for node in graph.raw_nodes() {
        let coordinates = node.weight.get_coordinates();
        positions.push(Vec3::new(coordinates.x, coordinates.y, 0.0));
    }

    let mut triangles: Vec<u32> = Vec::new();
    // TODO: Implement logic for non-triangle shaped regions (fuck)
    for edge in graph.raw_edges() {
        let (node_a_index, _) = (edge.source(), edge.target());

        triangles.push(node_a_index.index() as u32);
        // triangles.push(node_b_index.index() as u32);
        // triangles.push(node_a_index.index() as u32);
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_indices(Indices::U32(triangles));
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use bevy::math::Vec2;
    use bevy::prelude::Mesh;
    use bevy::render::mesh::PrimitiveTopology;
    use bevy::render::render_asset::RenderAssetUsages;

    use crate::regions::{RegionGraphNode, WorldGraph};

    // exact code used in a test, no worries
    fn create_world() -> WorldGraph {
        let mut world = WorldGraph::default();
        let graph = world.get_graph_mut();

        let n1 = graph.add_node(RegionGraphNode::new(Vec2::ZERO));
        let n2 = graph.add_node(RegionGraphNode::new(Vec2::new(1., 1.)));
        let n3 = graph.add_node(RegionGraphNode::new(Vec2::new(-1., -1.)));

        graph.add_edge(n1, n2, ());
        graph.add_edge(n2, n3, ());
        graph.add_edge(n3, n1, ());

        world
    }

    #[test]
    fn generate_mesh_from_empty_world() {
        let world = WorldGraph::default();

        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        );

        let n = Instant::now();
        super::build_mesh(world.get_graph(), &mut mesh);
        println!(
            "Empty mesh generated in {}ms",
            n.elapsed().as_micros() as f64 / 1000.0
        );

        assert_eq!(
            mesh.attribute(Mesh::ATTRIBUTE_POSITION)
                .map(|positions| positions.len()),
            Some(0)
        );

        assert_eq!(mesh.indices().map(|indices| indices.len()), Some(0));
    }

    #[test]
    fn generate_simple_world_mesh() {
        let world = create_world();

        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        );

        let n = Instant::now();
        super::build_mesh(world.get_graph(), &mut mesh);
        println!(
            "Simple world mesh generated in {}ms",
            n.elapsed().as_micros() as f64 / 1000.0
        );

        // check positions :)
        assert_eq!(
            mesh.attribute(Mesh::ATTRIBUTE_POSITION)
                .map(|positions| positions.as_float3().map(|slice| slice.to_vec())),
            Some(Some(vec![
                [0.0, 0.0, 0.0],
                [1.0, 1.0, 0.0],
                [-1.0, -1.0, 0.0],
            ]))
        );

        // check indices
        assert_eq!(
            // get the indices and transform it to `Option<Vec<usize>>`
            mesh.indices()
                .map(|indices| indices.iter().collect::<Vec<_>>()),
            Some(vec![0, 1, 2])
        );
    }
}
