pub mod v1 {
    use crate::generation::generate_layer;
    use fastnoise2::SafeNode;

    #[derive(Debug)]
    pub enum Biome {
        Ocean = 1,
        Coastal = 2,
        Desert = 3,
        Savanna = 4,
        TropicalForest = 5,
        Tundra = 6,
        Taiga = 7,
        TemperateForest = 8,
        Plains = 9,
    }

    impl Biome {
        pub(crate) fn normalized_value(&self) -> f32 {
            match self {
                Biome::Ocean => 0.0,
                Biome::Coastal => 0.1,
                Biome::Desert => 0.2,
                Biome::Savanna => 0.3,
                Biome::TropicalForest => 0.4,
                Biome::Tundra => 0.5,
                Biome::Taiga => 0.6,
                Biome::TemperateForest => 0.7,
                Biome::Plains => 0.8,
            }
        }
    }

    pub fn generate_biomes(
        elevation: &[f32],
        temperature: &[f32],
        humidity: &[f32],
        _sizes: (i32, i32),
    ) -> Vec<Biome> {
        let mut biomes = Vec::with_capacity(elevation.len());

        for ((&elev, &temp), &hum) in elevation.iter().zip(temperature).zip(humidity) {
            biomes.push(determine_biome(elev, temp, hum));
        }

        biomes
    }

    fn determine_biome(elevation: f32, temperature: f32, humidity: f32) -> Biome {
        match elevation {
            e if e < -0.4 => Biome::Ocean,
            e if e < -0.3 => Biome::Coastal,
            _ => match (temperature, humidity) {
                // Cold areas
                (t, _) if t < -0.5 => Biome::Tundra,
                (t, _) if t < 0.0 => Biome::Taiga,

                // Temperate zones
                (t, h) if t < 0.5 => {
                    if h < 0.3 {
                        Biome::Plains
                    } else {
                        Biome::TemperateForest
                    }
                }

                // Hot areas
                _ => {
                    if humidity < 0.2 {
                        Biome::Desert
                    } else if humidity < 0.6 {
                        Biome::Savanna
                    } else {
                        Biome::TropicalForest
                    }
                }
            },
        }
    }

    /// General configuration
    pub struct BiomeConfig {
        pub elevation_seed: i32,
        pub temperature_seed: i32,
        pub humidity_seed: i32,
        pub frequency: f32,
        pub size: (i32, i32),
    }

    /// The general interface for generating the biomes. It takes a `SafeNode` in entry (For example a encoded tree) and the config.
    ///
    /// It returns, in order:
    /// - **Biomes**: The list of biomes
    /// - **Height**: The heightmap
    /// - **Temperature**: The temperature noise
    /// - **Humidity**: The humidity noise
    pub fn full_biome_generation(
        node: &SafeNode,
        config: &BiomeConfig,
    ) -> (Vec<Biome>, Vec<f32>, Vec<f32>, Vec<f32>) {
        let (_, height) =
            generate_layer(node, config.size, config.frequency, config.elevation_seed);

        let (_, temp) =
            generate_layer(node, config.size, config.frequency, config.temperature_seed);

        let (_, humidity) =
            generate_layer(node, config.size, config.frequency, config.humidity_seed);

        let biomes = generate_biomes(&height, &temp, &humidity, config.size);
        (biomes, height, temp, humidity)
    }
}
