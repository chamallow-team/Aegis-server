//! This module contains methods to transform a graph into a beautiful Mesh
//!
//! Please note that it'll not give a width to the edges and nodes. It'll simply transform the map graph
//! in a 3d space.
//!
//! For outlining the edges and nodes, please see [bevy_mod_outline](https://docs.rs/bevy_mod_outline/latest/bevy_mod_outline/)

use bevy::math::Vec3;
use bevy::prelude::Mesh;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::utils::HashMap;
use delaunator::Point;
use petgraph::graph::UnGraph;
use petgraph::visit::{IntoNodeReferences, NodeRef};
use uuid::Uuid;

use crate::regions::WorldGraph;

/// This trait must be implemented for all nodes that can be passed to the mesh builder
pub trait MeshNode: Clone {
    /// Get the coordinates of the current node
    fn get_coordinates(&self) -> Vec3;
}

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
        positions.push(Vec3::new(coordinates.x, coordinates.y, coordinates.z));
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

/// Contains the datas about a meshed region
#[derive(Clone, Debug)]
pub struct MeshedRegion {
    /// The top-right corner of the region in the 3d space
    pub top_right_corner: Vec3,
    /// The converted region in a mesh.
    /// This is only a plane.
    ///
    /// The mesh use the following topology:
    /// [`PrimitiveTopology::TRIANGLES_LIST`](https://docs.rs/bevy/latest/bevy/render/mesh/enum.PrimitiveTopology.html)
    pub mesh: Mesh,
}

/// Takes a world's graph and convert every region to mesh with their coordinates in a 2d space
///
/// FIXME This method does not take into consideration the edges
pub fn build_regions_meshes(world: &WorldGraph) -> HashMap<Uuid, MeshedRegion> {
    let mut meshes = HashMap::default();
    let graph = world.get_graph();

    for (id, region_nodes) in world.regions().iter() {
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        );

        // get all nodes positions
        let mut positions: Vec<Vec3> = Vec::new();
        for node_index in region_nodes.iter() {
            let node_data = graph.node_references().find(|n| n.id() == node_index.id());

            if let Some(node) = node_data {
                positions.push(node.weight().get_coordinates());
            }
        }

        let triangles: Vec<u32> = triangulate_region(&positions);

        let min_x = positions
            .iter()
            .map(|p| p.x)
            .reduce(f32::min)
            .expect("Cannot find the minimal X in the nodes");

        let min_z = positions
            .iter()
            .map(|p| p.z)
            .reduce(f32::min)
            .expect("Cannot find the minimal Z in the nodes");

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_indices(Indices::U32(triangles));

        meshes.insert(
            *id,
            MeshedRegion {
                mesh,
                top_right_corner: Vec3::new(min_x, 0.0, min_z),
            },
        );
    }

    meshes
}

/// Triangulate a list of positions in a mesh and returns the triangles
///
/// Will use the logic of [`PrimitiveTopology::TRIANGLES_LIST`](https://docs.rs/bevy/latest/bevy/render/mesh/enum.PrimitiveTopology.html)
fn triangulate_region(positions: &[Vec3]) -> Vec<u32> {
    if positions.len() < 3 {
        return vec![];
    }

    let triangulation_result = delaunator::triangulate(
        positions
            .iter()
            .map(|v| Point {
                x: v.x as f64,
                y: v.z as f64,
            })
            .collect::<Vec<Point>>()
            .as_slice(),
    );

    let mut triangles = triangulation_result
        .hull
        .iter()
        .map(|t| *t as u32)
        .collect::<Vec<u32>>();

    let mut other_triangles = triangulation_result
        .triangles
        .iter()
        .map(|t| *t as u32)
        .collect::<Vec<u32>>();

    triangles.append(&mut other_triangles);

    triangles
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use bevy::math::{Vec2, Vec3};
    use bevy::prelude::Mesh;
    use bevy::render::mesh::PrimitiveTopology;
    use bevy::render::render_asset::RenderAssetUsages;
    use uuid::Uuid;

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

        world.add_region(Uuid::new_v4(), vec![n1, n2, n3]);

        world
    }

    #[test]
    fn mesh_simple_world() {
        let world = create_world();

        let n = Instant::now();
        let meshed = super::build_regions_meshes(&world);
        println!(
            "Simple world region generated in {}ms",
            n.elapsed().as_micros() as f64 / 1000.0
        );

        assert_eq!(meshed.len(), 1);

        let region = meshed
            .get(meshed.keys().collect::<Vec<&Uuid>>()[0])
            .unwrap();

        assert_eq!(region.top_right_corner, Vec3::new(-1.0, 0.0, -1.0));

        // check mesh
        let mesh = &region.mesh;

        assert_eq!(
            mesh.attribute(Mesh::ATTRIBUTE_POSITION)
                .map(|positions| positions.as_float3().map(|slice| slice.to_vec())),
            Some(Some(vec![
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 1.0],
                [-1.0, 0.0, -1.0],
            ]))
        );

        // check indices
        assert_eq!(
            // get the indices and transform it to `Option<Vec<usize>>`
            mesh.indices()
                .map(|indices| indices.iter().collect::<Vec<_>>()),
            Some(vec![2, 0, 1])
        );
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
                [1.0, 0.0, 1.0],
                [-1.0, 0.0, -1.0],
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
