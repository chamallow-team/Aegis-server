pub mod generation;
pub mod mesh_builder;
pub mod regions;

#[cfg(test)]
mod test {
    use crate::generation;
    use crate::generation::biomes;
    use fastnoise2::SafeNode;
    use image::{GrayImage, Luma};
    use std::time::Instant;

    #[test]
    fn create_noise() {
        const FREQUENCY: f32 = 0.008;
        const ELEVATION_SEED: i32 = 1337;

        let biomes_node =
            SafeNode::from_encoded_node_tree(generation::encoded_tree_versions::v1::BIOMES)
                .expect("Failed to build biomes Node tree from encoded tree");

        let (x_size, y_size) = (2000, 1000);

        println!("Generating biomes and terrain...");
        let start = Instant::now();

        let config = biomes::v1::BiomeConfig {
            elevation_seed: ELEVATION_SEED,
            temperature_seed: 456,
            humidity_seed: 789,
            frequency: FREQUENCY,
            size: (x_size, y_size),
        };
        let (biomes, elev, temp, hum) = biomes::v1::full_biome_generation(&biomes_node, &config);

        let elapsed = start.elapsed();

        // Normalize biomes buffer
        let biomes_normalized = biomes
            .iter()
            .map(|b| b.normalized_value())
            .collect::<Vec<f32>>();

        println!(
            "Biomes: Took {elapsed:?} to generate {} values ({}/s)",
            biomes.len(),
            biomes.len() as f32 / elapsed.as_secs_f32()
        );

        let img = generate_image((x_size, y_size), &biomes_normalized);
        save(img, "biomes.png");

        let img = generate_image((x_size, y_size), &elev);
        save(img, "elevation.png");

        let img = generate_image((x_size, y_size), &temp);
        save(img, "temperature.png");

        let img = generate_image((x_size, y_size), &hum);
        save(img, "humidity.png");
    }

    fn generate_image((x_size, y_size): (i32, i32), noise: &[f32]) -> GrayImage {
        let mut img = GrayImage::new(x_size as u32, y_size as u32);
        for x in 0..x_size {
            for y in 0..y_size {
                let index = ((y_size - 1 - y) * x_size + x) as usize;
                let value = noise[index];

                let pixel_value = (255.0 * ((value + 1.0) / 2.0)) as u8;
                img.put_pixel(x as u32, y as u32, Luma([pixel_value]));
            }
        }

        img
    }

    fn save(img: GrayImage, filename: &str) {
        let output_dir = std::env::temp_dir().join("fastnoise2");
        std::fs::create_dir_all(&output_dir).expect("Failed to create directories");
        let output_path = output_dir.join(filename);
        img.save(&output_path).expect("Failed to save image");
        println!("Image successfully saved as {}", output_path.display());
    }
}
