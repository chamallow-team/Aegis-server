mod ui;
mod mousemove;
mod techno_tree;
mod map;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use ::map::Map;
use crate::ui::{button_hover_system, setup_ui};

#[derive(Default)]
pub struct MapEditorPlugin;

#[derive(Resource, Default)]
pub struct MapEditorData {
    map: Map
}

impl Plugin for MapEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MapEditorStage::Startup), (setup, setup_ui))
            .add_systems(
                Update,
                (
                    mousemove::mouse_moved,
                    button_hover_system
                ).run_if(run_if_in_map_editor)
            );
        app.add_plugins(EguiPlugin);

        app.init_resource::<MapEditorData>();
    }
}

pub fn run_if_in_map_editor(state: Res<State<MapEditorStage>>) -> bool {
    matches!(state.get(), MapEditorStage::MapEditor)
}

// define a new schedule
#[derive(States, Clone, Debug, Eq, PartialEq, Hash, Copy)]
pub enum MapEditorStage {
    Startup,
    MapEditor,
    Exit
}

fn setup(
    mut commands: Commands
) {
    commands.insert_resource(State::new(MapEditorStage::MapEditor));
}