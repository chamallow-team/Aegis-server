use image::{ImageBuffer, Rgb};
use petgraph::prelude::EdgeRef;
use std::time::Instant;
use voronator::delaunator::Coord;

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

    pub fn generate_image(&self, name: &str) -> Result<(), image::ImageError> {
        let n = Instant::now();
        let scale = 10.0;
        let image_width = (self.width as f64 * scale) as u32;
        let image_height = (self.height as f64 * scale) as u32;

        let mut img = ImageBuffer::new(image_width, image_height);

        // Draw nodes
        for node_idx in self.terrain.node_indices() {
            if let Some(node) = self.terrain.node_weight(node_idx) {
                let x = (node.center.x() * scale) as u32;
                let y = (node.center.y() * scale) as u32;

                img.put_pixel(
                    x.min(image_width - 1),
                    y.min(image_height - 1),
                    Rgb([255, 0, 0]),
                );
            }
        }

        // Draw edges
        for edge_ref in self.terrain.edge_references() {
            let (source_idx, target_idx) = (edge_ref.source(), edge_ref.target());
            if let (Some(source_node), Some(target_node)) = (
                self.terrain.node_weight(source_idx),
                self.terrain.node_weight(target_idx),
            ) {
                let x1 = (source_node.center.x() * scale) as i32;
                let y1 = (source_node.center.y() * scale) as i32;
                let x2 = (target_node.center.x() * scale) as i32;
                let y2 = (target_node.center.y() * scale) as i32;

                self.draw_line(&mut img, x1, y1, x2, y2, Rgb([0, 0, 255])); // Blue for edges
            }
        }

        // Save the image to the temporary directory
        let mut temp_dir = std::env::temp_dir();
        temp_dir.push(name);
        let path = temp_dir.as_path();

        img.save(path)?;
        println!(
            "Image saved to: {:?}",
            path.to_string_lossy().to_string().replace("\\", "/")
        );
        println!("Image generation time: {:?}", n.elapsed());
        Ok(())
    }

    fn draw_line(
        &self,
        img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        color: Rgb<u8>,
    ) {
        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx - dy;

        let mut x = x1;
        let mut y = y1;

        while x != x2 || y != y2 {
            if x >= 0 && y >= 0 {
                img.put_pixel(
                    (x as u32).min(img.width() - 1),
                    (y as u32).min(img.height() - 1),
                    color,
                );
            }
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
        if x >= 0 && y >= 0 {
            img.put_pixel(
                (x as u32).min(img.width() - 1),
                (y as u32).min(img.height() - 1),
                color,
            );
        }
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
        std::fs::create_dir_all(std::env::temp_dir().join("aegis")).unwrap();

        let mut n = Instant::now();
        let world = WorldGenerator::new(64, 64).expect("Cannot generate base world");
        println!("Generated generator time: {:?}", n.elapsed());
        world
            .generate_image("aegis/aegis_map_gen.png")
            .expect("Cannot generate image");
        n = Instant::now();
    }
}
