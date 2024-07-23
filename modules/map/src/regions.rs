use bevy::math::Vec2;
use bevy::utils::HashMap;
use petgraph::{Graph, Undirected};
use petgraph::stable_graph::NodeIndex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::mesh_builder::MeshNode;

type WorldGraphType = Graph<RegionGraphNode, (), Undirected>;

/// An undirected graph for the regions
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct WorldGraph {
    graph: WorldGraphType,
    regions: HashMap<Uuid, Region>,
}

/// A list of node indexes that delimit the region. All the nodes must be connected by edges, otherwise you'll have some problems
pub type Region = Vec<NodeIndex>;

impl WorldGraph {
    pub fn get_graph(&self) -> &WorldGraphType {
        &self.graph
    }

    pub fn get_graph_mut(&mut self) -> &mut WorldGraphType {
        &mut self.graph
    }

    pub fn add_region(&mut self, id: Uuid, region: Region) {
        self.regions.insert(id, region);
    }

    pub fn get_region(&self, id: &Uuid) -> Option<&Region> {
        self.regions.get(id)
    }

    pub fn get_region_mut(&mut self, id: &Uuid) -> Option<&mut Region> {
        self.regions.get_mut(id)
    }

    pub fn regions(&self) -> &HashMap<Uuid, Region> {
        &self.regions
    }

    pub fn regions_mut(&mut self) -> &mut HashMap<Uuid, Region> {
        &mut self.regions
    }
}

/// Create a new node for a region
///
/// # Example
///
/// ```
/// use map::regions::RegionGraphNode;
/// use bevy::math::Vec2;
///
/// let node = RegionGraphNode::new(Vec2::new(5.6, 9.2));
/// ```
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RegionGraphNode {
    pub coords: Vec2,
}

impl RegionGraphNode {
    pub fn new(coords: Vec2) -> Self {
        Self { coords }
    }
}

impl MeshNode for RegionGraphNode {
    fn get_coordinates(&self) -> Vec2 {
        self.coords
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::Vec2;
    use uuid::Uuid;

    use super::{RegionGraphNode, WorldGraph};

    #[test]
    fn create_world_graph() {
        let mut world = WorldGraph::default();
        let graph = world.get_graph_mut();

        assert_eq!(graph.node_count(), 0);
        graph.add_node(RegionGraphNode::new(Vec2::ZERO));
        graph.add_node(RegionGraphNode::new(Vec2::new(1., 1.)));
        assert_eq!(graph.node_count(), 2);
        assert_eq!(world.regions().len(), 0);
    }

    #[test]
    fn create_world_with_simple_region() {
        let mut world = WorldGraph::default();

        let region_id = Uuid::new_v4();

        {
            let graph = world.get_graph_mut();

            assert_eq!(graph.node_count(), 0);
            assert_eq!(graph.edge_count(), 0);

            let n1 = graph.add_node(RegionGraphNode::new(Vec2::ZERO));
            let n2 = graph.add_node(RegionGraphNode::new(Vec2::new(1., 1.)));
            let n3 = graph.add_node(RegionGraphNode::new(Vec2::new(-1., -1.)));

            graph.add_edge(n1, n2, ());
            graph.add_edge(n2, n3, ());
            graph.add_edge(n3, n1, ());

            world.add_region(region_id, vec![n1, n2, n3]);
        }

        assert_eq!(world.regions().len(), 1);
        assert_eq!(world.get_region(&region_id).map(|r| r.len()), Some(3));

        let graph = world.get_graph();
        assert_eq!(graph.node_count(), 3);
        assert_eq!(graph.edge_count(), 3);
    }
}
