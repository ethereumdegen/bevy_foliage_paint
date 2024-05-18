use bevy::{diagnostic::LogDiagnosticsPlugin, prelude::*, render::primitives::Aabb};
use warbler_grass::{diagnostic::WarblerDiagnosticsPlugin, prelude::*};
 
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // This plugin is needed to initialize everything for the grass render pipeline
            WarblersPlugin,
            // Just a helper plugin for spawning a camera
            // As in all examples, you can use the wasd keys for movement and qe for rotation
            SimpleCamera,
            // Let's also log the amount of blades rendered
            WarblerDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
        ))
        .add_systems(Startup, setup_grass_chunks)
        .run();
}
fn setup_grass_chunks(mut commands: Commands, asset_server: Res<AssetServer>) {
    let density_map_handle = asset_server.load("grass_density_map.png");

    let density_map = DensityMap {
        density_map: density_map_handle.clone(),
        density: 2.,
    };
    let y_map_image = asset_server.load("grass_y_map.png");

    let y_map = YMap { y_map: y_map_image };
    // each chunk is 50x50
    let (chunk_width, chunk_height) = (50., 50.);
    // spawns a 20x20 grid of chunks
    for chunk in 0..400 {
        let offset = Vec3::new(
            (chunk / chunk_width as i32) as f32 * chunk_width * 1.05,
            0.,
            (chunk % chunk_width as i32) as f32 * chunk_height * 1.05,
        );
        // we can define the color of the grass on a chunk basis
        let color = Color::rgb(
            ((chunk / chunk_width as i32) as f32 / 400. * chunk_width) + 0.5,
            ((chunk % chunk_width as i32) as f32 / chunk_width) + 0.5,
            0.,
        );
        commands.spawn(WarblersBundle {
            // we could use seperate density maps for each one
            density_map: density_map.clone(),
            // or seperate height maps if we wanted to
            y_map: y_map.clone(),
            height: WarblerHeight::Texture(density_map_handle.clone()),
            // the aabb defined the dimensions of the box the chunk lives in
            aabb: Aabb::from_min_max(Vec3::ZERO, Vec3::new(chunk_width, 2., chunk_height)),
            grass_color: GrassColor {
                main_color: color,
                bottom_color: color * 0.4,
            },

            spatial: SpatialBundle {
                transform: Transform::from_translation(offset),
                ..default()
            },
            ..default()
        });
    }
}



//simple camera is below 

/// Used in the example to spawn a simple camera which is moves with qweasd keys
pub struct SimpleCamera;
impl Plugin for SimpleCamera {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, camera_movement);
    }
}
fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(-20.0, 15., -20.0)
            .looking_at(Vec3::new(0., 10., 0.), Vec3::Y),
        ..default()
    },));
}
pub fn camera_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    for mut transform in &mut query {
        let move_speed = 0.6;
        let rotate_speed = 0.03;
        let mut forward = *transform.forward();
        forward.y = 0.;
        let right = transform.right();
        let up = transform.up();

        if input.pressed(KeyCode::KeyW) {
            transform.translation += forward * move_speed;
        }
        if input.pressed(KeyCode::KeyS) {
            transform.translation -= forward * move_speed;
        }
        if input.pressed(KeyCode::Space) {
            transform.translation += up * move_speed;
        }
        if input.pressed(KeyCode::ShiftLeft) {
            transform.translation -= up * move_speed;
        }
        if input.pressed(KeyCode::KeyQ) {
            transform.rotate_y(rotate_speed);
        }
        if input.pressed(KeyCode::KeyE) {
            transform.rotate_y(-rotate_speed);
        }
        if input.pressed(KeyCode::KeyA) {
            transform.translation -= right * move_speed;
        }
        if input.pressed(KeyCode::KeyD) {
            transform.translation += right * move_speed;
        }
    }
}
