mod skybox;

use bevy::core_pipeline::Skybox;
use bevy::dev_tools::fps_overlay::FpsOverlayPlugin;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode};

use skybox::SkyboxPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SkyboxPlugin))
        .add_plugins(FpsOverlayPlugin { ..default() })
        .add_systems(Startup, (setup, unlock_fps))
        .add_systems(Update, move_camera)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(-2.5, 4.5, 9.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn move_camera(mut cameras: Query<&mut Transform, With<Camera3d>>, timer: Res<Time>) {
    for mut camera in &mut cameras {
        let angle: f64 = timer.elapsed_secs_f64() * 0.5;
        camera.translation = Vec3 {
            x: angle.sin() as f32 * 9.0,
            y: 6.0,
            z: angle.cos() as f32 * 9.0,
        };
        camera.look_at(Vec3::ZERO, Vec3::Y);
    }
}

fn unlock_fps(mut windows: Query<&mut Window>) {
    for mut window in &mut windows {
        window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Primary);
        window.present_mode = PresentMode::AutoNoVsync;
    }
}
