use bevy::{
    core_pipeline::Skybox,
    prelude::*,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
};

pub struct SkyboxPlugin;

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_skybox)
            .add_systems(Update, apply_skybox);
    }
}

#[derive(Resource)]
struct SkyboxState {
    handle: Handle<Image>,
    applied: bool,
}

fn load_skybox(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("sky.png");

    commands.insert_resource(SkyboxState {
        handle,
        applied: false,
    });

    commands.insert_resource(GlobalAmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
        ..default()
    });
}

fn apply_skybox(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut state: ResMut<SkyboxState>,
    cameras: Query<Entity, With<Camera3d>>,
) {
    if state.applied {
        return;
    }

    if asset_server.load_state(&state.handle).is_loaded() {
        let image = images.get_mut(&state.handle).unwrap();

        let width = image.width();
        let height = image.height();

        if height != width * 6 {
            panic!("Skybox must be vertically stacked (height = width * 6)");
        }

        image.reinterpret_stacked_2d_as_array(6).unwrap();

        image.texture_view_descriptor = Some(TextureViewDescriptor {
            dimension: Some(TextureViewDimension::Cube),
            ..default()
        });

        for camera in &cameras {
            commands.entity(camera).insert(Skybox {
                image: state.handle.clone(),
                brightness: 1000.0,
                ..default()
            });
        }

        state.applied = true;
    }
}
