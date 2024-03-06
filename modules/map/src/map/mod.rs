pub mod exported;

use petgraph::Graph;
use petgraph::prelude::NodeIndex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use exported::{ExportedEdge, ExportedMap, ExportedNode};

#[derive(Debug, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Copy, Serialize, Deserialize)]
pub struct Coordinates {
    pub x: i64,
    pub y: i64
}


impl Coordinates {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    
    pub fn x(&self) -> i64 {
        self.x
    }
    
    pub fn y(&self) -> i64 {
        self.y
    }
}

impl From<(i64, i64)> for Coordinates {
    fn from(v: (i64, i64)) -> Self {
        Self::new(v.0, v.1)
    }
}
impl From<(i32, i32)> for Coordinates {
    fn from(v: (i32, i32)) -> Self {
        Self::new(v.0 as i64, v.1 as i64)
    }
}
impl From<(usize, usize)> for Coordinates {
    fn from(v: (usize, usize)) -> Self {
        Self::new(v.0 as i64, v.1 as i64)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NodeType {
    Water,
    Land(NodeLandType),
}

impl TryFrom<u32> for NodeType {
    type Error = ();
    
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Water),
            i => match NodeLandType::try_from(i) {
                Ok(t) => Ok(Self::Land(t)),
                _ => Err(())
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NodeLandType {
    Plain = 0,
    Forest = 1,
    Mountain = 2
}

impl TryFrom<u32> for NodeLandType {
    type Error = ();
    
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Plain),
            2 => Ok(Self::Forest),
            3 => Ok(Self::Mountain),
            _ => Err(())
        }
    }
}


impl From<NodeLandType> for NodeType {
    fn from(v: NodeLandType) -> Self {
        Self::Land(v)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
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

    pub fn get_id(&self) -> Uuid {
        self.id
    }
    
    pub fn get_type(&self) -> NodeType {
        self.node_type
    }
    
    pub fn get_coordinates(&self) -> Coordinates {
        self.coordinates
    }


    pub fn get_coordinates_mut(&mut self) -> &mut Coordinates {
        &mut self.coordinates
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge(Uuid, Uuid);

impl From<(Uuid, Uuid)> for Edge {
    fn from(v: (Uuid, Uuid)) -> Self {
        Self(v.0, v.1)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GameMap {
    graph: Graph<Node, Edge>
}

impl GameMap {
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

    pub fn get_nodes(&self) -> Vec<&Node> {
        self.graph.node_indices().map(|index| &self.graph[index]).collect()
    }

    pub fn get_edges(&self) -> Vec<&Edge> {
        self.graph.edge_indices().map(|index| &self.graph[index]).collect()
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
    use super::{Edge, GameMap, Node, NodeLandType, NodeType};

    #[test]
    fn one_node_graph(){
        let mut m = GameMap::default();

        let id = Uuid::new_v4();
        let n = Node::new(NodeType::Water, (6, 5), id);
        m.add_node(n.clone());

        assert_eq!(m.get_node(id), Some(&n));
    }

    #[test]
    fn simple_graph(){
        let mut m = GameMap::default();

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
