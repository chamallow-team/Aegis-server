use uuid::Uuid;
use serde::{Deserialize, Serialize};
use super::{Coordinates, NodeType};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct ExportedNode {
    id: Uuid,
    node_type: NodeType,
    coordinates: Coordinates,
}

impl ExportedNode {
    pub fn new(id: Uuid, node_type: NodeType, coordinates: Coordinates) -> Self {
        Self { id, node_type, coordinates }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct ExportedEdge {
    source_id: Uuid,
    target_id: Uuid,
}

impl ExportedEdge {
    pub fn new(source_id: Uuid, target_id: Uuid) -> Self {
        Self { source_id, target_id }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ExportedMap {
    nodes: Vec<ExportedNode>,
    edges: Vec<ExportedEdge>,
}

impl ExportedMap {
    pub fn new(nodes: Vec<ExportedNode>, edges: Vec<ExportedEdge>) -> Self {
        Self { nodes, edges }
    }

    pub fn get_nodes(&self) -> &Vec<ExportedNode> {
        &self.nodes
    }

    pub fn get_edges(&self) -> &Vec<ExportedEdge> {
        &self.edges
    }
}

#[cfg(test)]
mod exported_map_tests {
    use uuid::Uuid;
    use super::{ExportedEdge, ExportedMap, ExportedNode};
    use crate::map::NodeType;

    #[test]
    fn empty_exported_map(){
        let m = ExportedMap::default();

        assert_eq!(m.get_edges().len(), 0);
        assert_eq!(m.get_nodes().len(), 0);
    }

    #[test]
    fn only_nodes(){
        let n1 = ExportedNode::new(Uuid::new_v4(), NodeType::Water, (1, 2).into());
        let n2 = ExportedNode::new(Uuid::new_v4(), NodeType::Water, (4, 5).into());

        let m = ExportedMap::new(
            vec![n1.clone(), n2.clone()],
            Vec::default()
        );

        assert_eq!(m.get_edges(), &vec![]);
        assert_eq!(m.get_nodes(), &vec![n1, n2]);
    }

    #[test]
    fn only_edges(){
        let n1 = Uuid::new_v4();
        let n2 = Uuid::new_v4();
        let n3  = Uuid::new_v4();

        let e1 = ExportedEdge::new(n1, n2);
        let e2 = ExportedEdge::new(n2, n3);

        let m = ExportedMap::new(
            Vec::default(),
            vec![e1.clone(), e2.clone()]
        );

        assert_eq!(m.get_edges(), &vec![e1, e2]);
        assert_eq!(m.get_nodes(), &vec![]);
    }

    #[test]
    fn nodes_and_edges(){
        let n1 = ExportedNode::new(Uuid::new_v4(), NodeType::Water, (1, 2).into());
        let n2 = ExportedNode::new(Uuid::new_v4(), NodeType::Water, (4, 5).into());
        let n3 = ExportedNode::new(Uuid::new_v4(), NodeType::Water, (6, 9).into());


        let e1 = ExportedEdge::new(n1.id(), n2.id());
        let e2 = ExportedEdge::new(n2.id(), n3.id());

        let m = ExportedMap::new(
            vec![n1.clone(), n2.clone(), n3.clone()],
            vec![e1.clone(), e2.clone()]
        );

        assert_eq!(m.get_edges(), &vec![e1, e2]);
        assert_eq!(m.get_nodes(), &vec![n1, n2, n3]);
    }
}
