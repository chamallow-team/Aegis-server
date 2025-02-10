use fastnoise2::{OutputMinMax, SafeNode};

pub mod biomes;

pub fn generate_layer(
    node: &SafeNode,
    size: (i32, i32),
    frequency: f32,
    seed: i32,
) -> (OutputMinMax, Vec<f32>) {
    let (x, y) = size;
    let mut data = vec![0.0; (x * y) as usize];

    let min_max = node.gen_uniform_grid_2d(&mut data, -x / 2, -y / 2, x, y, frequency, seed);

    (min_max, data)
}

pub mod encoded_tree_versions {

    pub mod v1 {
        pub const TERRAIN: &str =
            "DgACAAAAAAAAABMAzczMPQ0ABwAAAAAAIEAJAABmZiY/AAAAAD8AexSuQAD2KNw/AK5HQUA=";
        pub const BIOMES: &str =
            "IgCF64FAAAAAAA4AAgAAAAAAAAATAI/CdT0NAAcAAAAAACBACQAAZmYmPwAAAAA/AHsUrkAA9ijcPwCuR0FA";
    }
}
