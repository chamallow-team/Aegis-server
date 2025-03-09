use thiserror::Error;

#[derive(Debug, Error)]
pub enum MapError {
    #[error("Cannot generate the voronoi diagram")]
    VoronoiDiagramError,
}
