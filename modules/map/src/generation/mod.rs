mod terrain;

pub struct WorldGenerator {
    width: u64,
    height: u64,
    terrain: terrain::PolygonGraph,
}

impl WorldGenerator {
    pub fn new(width: u64, height: u64) -> Result<Self, crate::errors::MapError> {
        Ok(Self {
            terrain: terrain::generate_grid(width, height)?,
            width,
            height,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Rock,
    Sand,
    Grass,
    Dirt,
}

#[derive(Debug, Clone, Copy)]
pub enum Vegetation {
    Desert,
    Grassland,
    Forest,
    Tundra,
}

#[cfg(test)]
mod test {
    use crate::generation::WorldGenerator;
    use std::time::Instant;

    #[test]
    fn test() {
        let mut n = Instant::now();
        let world = WorldGenerator::new(2000, 1000);
        println!("Generated generator time: {:?}", n.elapsed());
        n = Instant::now();
    }
}
