use crate::errors::MapError;
use petgraph::prelude::NodeIndex;
use petgraph::Undirected;
use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Sub};
use voronator::delaunator::{Coord, Vector};
use voronator::VoronoiDiagram;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub(crate) struct Vertex {
    x: f64,
    y: f64,
}

impl Vertex {
    pub(crate) const ZERO: Self = Self { x: 0.0, y: 0.0 };
}
impl Add for Vertex {
    type Output = Vertex;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Sub for Vertex {
    type Output = Vertex;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Mul for Vertex {
    type Output = Vertex;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
impl Div for Vertex {
    type Output = Vertex;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
impl Coord for Vertex {
    fn from_xy(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }
}
impl Vector<Self> for Vertex {}

/// The type of Node, used to differentiate the Voronoi cell center from the corners
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum NodeType {
    #[default]
    VoronoiCenter,
    VoronoiCorner,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Polygon {
    pub center: Vertex,
    pub node_type: NodeType,
}

pub(crate) type PolygonGraph = petgraph::Graph<Polygon, (), Undirected>;

/// Generate the grid of points and apply the voronoi algorithm
///
/// TODO May require to use the Lloyd algorithm or use Barycentric Dual mesh rather than Voronoi Diagram, but it's enough for a start
pub(crate) fn generate_grid(width: u64, height: u64) -> Result<PolygonGraph, MapError> {
    let mut grid_graph = PolygonGraph::default();

    // Size of a single cell
    let size = 1.0;

    // generate the grid
    for y in 0..height {
        for x in 0..width {
            let offset = if y % 2 == 0 { 0.0 } else { size / 2.0 };
            let center = Vertex {
                x: x as f64 * size + offset,
                y: y as f64 * size * (3.0_f64).sqrt() / 2.0,
            };
            grid_graph.add_node(Polygon {
                center,
                node_type: NodeType::VoronoiCenter,
            });
        }
    }

    let points = grid_graph
        .raw_nodes()
        .iter()
        .map(|n| n.weight.center)
        .collect::<Vec<Vertex>>();

    // Use the voronoi algorithm
    let voronoi_diagram = VoronoiDiagram::new(
        &Vertex::ZERO,
        &Vertex {
            x: width as f64,
            y: height as f64,
        },
        points.as_slice(),
    );

    if voronoi_diagram.is_none() {
        return Err(MapError::VoronoiDiagramError);
    };
    let voronoi_diagram = voronoi_diagram.unwrap();

    Ok(create_combined_graph(&voronoi_diagram))
}

/// Transform the Voronoi diagram into a `PolygonGraph`
///
/// FIXME this algorithm uses an insane amount of memory with large maps (ex. (2000, 1000) takes 20s...)
fn create_combined_graph(diagram: &VoronoiDiagram<Vertex>) -> PolygonGraph {
    let mut graph = PolygonGraph::default();

    let diagram_centers: Vec<NodeIndex> = diagram
        .sites
        .iter()
        .map(|site| {
            graph.add_node(Polygon {
                center: *site,
                node_type: NodeType::VoronoiCenter,
            })
        })
        .collect();

    println!(
        "diagram_centers size: {} Mb",
        (size_of::<NodeIndex>() * diagram_centers.len()) / 1024 / 1024
    );

    let mut cell_corners: Vec<Vec<NodeIndex>> = Vec::new();
    for cell in diagram.cells() {
        let mut corners_for_cell: Vec<NodeIndex> = Vec::new();

        for vertex in cell.points() {
            let corner_index = graph.add_node(Polygon {
                center: *vertex,
                node_type: NodeType::VoronoiCorner,
            });
            corners_for_cell.push(corner_index);
        }

        cell_corners.push(corners_for_cell);
    }

    println!(
        "cell_corners size: {} Mb",
        (size_of::<NodeIndex>() * cell_corners.iter().map(|c| c.len()).sum::<usize>())
            / 1024
            / 1024
    );

    // Add Delaunay edges
    for triangle in diagram.delaunay.triangles.chunks(3) {
        let a = diagram_centers[triangle[0]];
        let b = diagram_centers[triangle[1]];
        let c = diagram_centers[triangle[2]];

        graph.add_edge(a, b, ());
        graph.add_edge(b, c, ());
        graph.add_edge(c, a, ());
    }

    // Connect all corner vertex's to their center
    for (cell_index, corners) in cell_corners.iter().enumerate() {
        let center_index = diagram_centers[cell_index];
        for corner_index in corners {
            graph.add_edge(center_index, *corner_index, ());
        }
    }

    let graph_capacity = graph.capacity();
    println!(
        "Graph size: {} nodes, {} edges",
        graph_capacity.0, graph_capacity.1
    );
    println!(
        "graph memory usage: {} Mb",
        ((graph_capacity.0 * size_of::<Polygon>()) + (graph_capacity.1 * size_of_val(&())))
            / 1024
            / 1024
    );

    graph
}
