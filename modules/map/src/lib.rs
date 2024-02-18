use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::extensions::{Extension, ExtensionID};

pub mod map;
pub mod extensions;

/// Contain the full map, with the points, units, weapons, extensions and so on.
#[derive(Serialize, Deserialize)]
pub struct Map {
    map: map::GameMap,
    extensions: HashMap<ExtensionID, Extension>
}

impl Map {
    pub fn get_map(&self) -> &map::GameMap {
        &self.map
    }

    pub fn get_map_mut(&mut self) -> &mut map::GameMap {
        &mut self.map
    }

    pub fn get_extensions(&self) -> &HashMap<ExtensionID, Extension> {
        &self.extensions
    }

    pub fn get_extensions_mut(&mut self) -> &mut HashMap<ExtensionID, Extension> {
        &mut self.extensions
    }

    pub fn add_extension(&mut self, id: impl Into<ExtensionID>, extension: Extension) -> Option<Extension> {
        self.extensions.insert(id.into(), extension)
    }
}