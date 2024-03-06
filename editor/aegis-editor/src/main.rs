mod constants;

use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::ExitCondition;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use editor::MapEditorStage;
use crate::constants::TITLE;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: TITLE.to_string(),
                    resizable: true,
                    decorations: true,
                    transparent: false,
                    focused: true,
                    canvas: None,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                exit_condition: ExitCondition::OnPrimaryClosed,
                close_when_requested: true,
            }
        )
    );

    app.add_plugins(
        (
            editor::MapEditorPlugin,
            EmbeddedAssetPlugin::default()
        )
    );

    app.add_systems(Startup, setup);

    app.insert_state(MapEditorStage::Startup);

    app.run();
}

fn setup(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Rectangle { half_size: Vec2::new(50., 100.) })),
        material: materials.add(Color::rgb(1., 1., 1.)),
        transform: Transform::from_translation(Vec3::ZERO),
        ..default()
    });

}