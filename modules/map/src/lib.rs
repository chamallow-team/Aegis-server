pub mod generation;
pub mod mesh_builder;
pub mod regions;

#[cfg(test)]
mod test {
    use crate::generation::terrain::modifiers;
    use crate::generation::{self, terrain};
    use fastnoise2::SafeNode;
    use image::{GrayImage, Luma};
    use std::time::Instant;

    #[test]
    fn create_noise() {
        let node = SafeNode::from_encoded_node_tree(generation::encoded_tree_versions::V1)
            .expect("Failed to build Node tree from encoded tree");

        let (x_size, y_size) = (2000, 1000);

        let start = Instant::now();
        let (min_max, mut noise) = terrain::generate_terrain(&node, (x_size, y_size), 0.008, 14679);
        let elapsed = start.elapsed();

        terrain::apply_boundaries(&mut noise, modifiers::v1::boundaries);

        println!(
            "Took {elapsed:?} to generate {} values ({}/s): {min_max:?}",
            noise.len(),
            noise.len() as f32 / elapsed.as_secs_f32()
        );

        let mut img = GrayImage::new(x_size as u32, y_size as u32);
        for x in 0..x_size {
            for y in 0..y_size {
                let index = ((y_size - 1 - y) * x_size + x) as usize;
                let value = noise[index];

                let pixel_value = (255.0 * ((value + 1.0) / 2.0)) as u8;
                img.put_pixel(x as u32, y as u32, Luma([pixel_value]));
            }
        }

        save(img, "test_fastnoise.png")
    }

    fn save(img: GrayImage, filename: &str) {
        let output_dir = std::env::temp_dir().join("fastnoise2");
        std::fs::create_dir_all(&output_dir).expect("Failed to create directories");
        let output_path = output_dir.join(filename);
        img.save(&output_path).expect("Failed to save image");
        println!("Image successfully saved as {}", output_path.display());
    }
}
