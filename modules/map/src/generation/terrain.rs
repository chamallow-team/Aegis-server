use fastnoise2::{OutputMinMax, SafeNode};

/// Helper function to generate the terrain grayscale
///
/// This method will take the node tree, the boundaries, the frequency and the seed, to fill the buffer `noise`
///
/// # Example
/// ```rs
/// let node = SafeNode::from_encoded_node_tree("DgACAAAAAAAAABMAzczMPQ0ABwAAAAAAIEAJAABmZiY/AAAAAD8AexSuQAD2KNw/AK5HQUA=")
///     .unwrap();
///
/// let (min_max, noise) = generate_terrain(&node, (x_size, y_size), 0.02, 1337);
/// ```
pub fn generate_terrain(
    node: &SafeNode,
    (x_size, y_size): (i32, i32),
    frequency: f32,
    seed: i32,
) -> (OutputMinMax, Vec<f32>) {
    let mut noise = vec![0.0; (x_size * y_size) as usize];

    let min_max = node.gen_uniform_grid_2d(
        &mut noise,
        -x_size / 2,
        -y_size / 2,
        x_size,
        y_size,
        frequency,
        seed,
    );

    (min_max, noise)
}

/// Apply the terrain modifications to the given noise
///
/// The method takes a **modifier** which is a function that takes each noise value and returns a value.
///
/// # Example
/// ```rs
/// apply_boundaries(
///     vec![4.5, 6.9],
///     |v: f32| v / 2.0
/// )
/// ```
pub fn apply_boundaries<F>(noise: &mut [f32], modifier: F)
where
    F: Fn(f32) -> f32,
{
    noise.iter_mut().for_each(|v| *v = modifier(*v));
}

pub mod modifiers {
    pub mod v1 {
        pub fn boundaries(v: f32) -> f32 {
            match v {
                _ if v < -0.4 => 1.0, // Océan profond
                _ if v < -0.3 => 0.5, // Zone côtière
                _ => -1.0,            // Terre
            }
        }
    }
}
