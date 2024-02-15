pub mod exported;

use petgraph::Graph;
use petgraph::prelude::NodeIndex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use exported::{ExportedEdge, ExportedMap, ExportedNode};

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Copy, Serialize, Deserialize)]
pub struct Coordinates(i64, i64);

impl Coordinates {
    pub fn new(x: i64, y: i64) -> Self {
        Self(x, y)
    }
}

impl From<(i64, i64)> for Coordinates {
    fn from(v: (i64, i64)) -> Self {
        Self(v.0, v.1)
    }
}
impl From<(i32, i32)> for Coordinates {
    fn from(v: (i32, i32)) -> Self {
        Self(v.0 as i64, v.1 as i64)
    }
}
impl From<(usize, usize)> for Coordinates {
    fn from(v: (usize, usize)) -> Self {
        Self(v.0 as i64, v.1 as i64)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NodeType {
    Land(NodeLandType),
    Water
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NodeLandType {
    Plain = 0,
    Forest = 1,
    Mountain = 2
}

impl From<NodeLandType> for NodeType {
    fn from(v: NodeLandType) -> Self {
        Self::Land(v)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Node {
    id: Uuid,
    node_type: NodeType,
    coordinates: Coordinates
}

impl Node {
    pub fn new(t: impl Into<NodeType>, coords: impl Into<Coordinates>, id: Uuid) -> Self {
        Self {
            node_type: t.into(),
            coordinates: coords.into(),
            id
        }
    }
}

#[derive(Debug, Clone)]
pub struct Edge(Uuid, Uuid);

impl From<(Uuid, Uuid)> for Edge {
    fn from(v: (Uuid, Uuid)) -> Self {
        Self(v.0, v.1)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Map {
    graph: Graph<Node, Edge>
}

impl Map {
    pub fn add_node(&mut self, node: impl Into<Node>) -> NodeIndex {
        self.graph.add_node(node.into())
    }

    pub fn add_node_mut(mut self, node: impl Into<Node>) -> Self {
        self.graph.add_node(node.into());
        self
    }

    pub fn delete_node(&mut self, node_index: NodeIndex) -> Option<Node> {
        self.graph.remove_node(node_index)
    }

    pub fn delete_node_mut(mut self, node_index: NodeIndex) -> Self {
        self.graph.remove_node(node_index);
        self
    }

    pub fn add_edge(&mut self, source: Uuid, target: Uuid) {
        let source_index = self.node_index_from_id(source).unwrap();
        let target_index = self.node_index_from_id(target).unwrap();
        self.graph.add_edge(source_index, target_index, Edge(source, target));
    }

    pub fn delete_edge(&mut self, source: Uuid, target: Uuid) -> Option<Edge> {
        let source_index = self.node_index_from_id(source)?;
        let target_index = self.node_index_from_id(target)?;

        if let Some(edge_index) = self.graph.find_edge(source_index, target_index) {
            return self.graph.remove_edge(edge_index);
        }

        None
    }

    pub fn get_edge(&self, source: Uuid, target: Uuid) -> Option<&Edge> {
        let source_index = self.node_index_from_id(source)?;
        let target_index = self.node_index_from_id(target)?;

        if let Some(edge_index) = self.graph.find_edge(source_index, target_index) {
            return self.graph.edge_weight(edge_index);
        }

        None
    }

    pub fn get_node(&self, id: Uuid) -> Option<&Node> {
        self.graph.node_weight(self.node_index_from_id(id)?)
    }

    fn node_index_from_id(&self, id: Uuid) -> Option<NodeIndex> {
        self.graph.node_indices().find(|&index| self.graph[index].id == id)
    }

    pub fn export(&self) -> ExportedMap {
        let nodes = self
            .graph
            .node_indices()
            .map(|index| {
                let node = &self.graph[index];
                ExportedNode::new(node.id, node.node_type, node.coordinates)
            })
            .collect();

        let edges = self
            .graph
            .edge_indices()
            .map(|index| {
                let edge = &self.graph[index];
                ExportedEdge::new(edge.0, edge.1)
            })
            .collect();

        ExportedMap::new(nodes, edges)
    }
}


#[cfg(test)]
mod map_tests {
    use uuid::Uuid;
    use super::{Edge, Map, Node, NodeLandType, NodeType};

    #[test]
    fn one_node_graph(){
        let mut m = Map::default();

        let id = Uuid::new_v4();
        let n = Node::new(NodeType::Water, (6, 5), id);
        m.add_node(n.clone());

        assert_eq!(m.get_node(id), Some(&n));
    }

    #[test]
    fn simple_graph(){
        let mut m = Map::default();

        let ids = [Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4()];

        let mut expected_nodes = Vec::new();
        let mut expected_edges = Vec::new();

        for i in 0..ids.len() {
            let n = Node::new(NodeType::Land(NodeLandType::Forest), (2 * i, 3 * i + 1), ids[i]);
            m.add_node(n.clone());
            expected_nodes.push(n);

            if i > 0 {
                m.add_edge(ids[i - 1], ids[i]);
                expected_edges.push(Edge(ids[i - 1], ids[i]))
            }
        }

        let exported = m.export();

        assert_eq!(exported.get_nodes().len(), ids.len());
        assert_eq!(exported.get_edges().len(), ids.len() - 1);
    }
}
