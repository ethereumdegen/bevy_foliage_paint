use bevy_foliage_paint::y_offset_map::YOffsetMap;
use bevy_foliage_paint::density_map::DensityMap;
use bevy_foliage_paint::density_map::DensityMapU8;
use bevy_foliage_paint::foliage_chunk::FoliageChunk;
use bevy_foliage_paint::foliage_chunk::FoliageChunkDensityTexture;
use bevy_foliage_paint::foliage_chunk::FoliageChunkYOffsetTexture;
use bevy_foliage_paint::foliage_chunk::{FoliageChunkDensityData,FoliageChunkYOffsetData};
use bevy::input::mouse::MouseMotion;

use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::{pbr::ShadowFilteringMethod, prelude::*};
use bevy_foliage_paint::y_offset_map::YOffsetMapU16;
use bevy_foliage_paint::{
    foliage::{FoliageData },
    foliage_config::FoliageConfig,
    BevyFoliagePaintPlugin,
};
use image::{ImageBuffer, Rgba};

use bevy_inspector_egui::quick::WorldInspectorPlugin;
//#[derive(Resource)]
//pub struct TextureLoaderResource {}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
         .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(BevyFoliagePaintPlugin::default())
        .add_systems(Startup, setup)


        .init_resource::<SampleTexturesResource>()



        .add_systems(Update, add_sample_data_for_chunks)


     //   .add_systems(Startup,create_and_save_texture)
        .add_systems(Update, update_camera_look)
        .add_systems(Update, update_camera_move)
        .add_systems(Update, update_directional_light_position)
        .run();
}







#[derive(Resource,Default)]
struct SampleTexturesResource {
    sample_density_map: Handle<Image>,
    sample_y_offset_map: Handle<Image> , 
} 

fn add_sample_data_for_chunks(

    mut commands:Commands,
  //  asset_server: Res<AssetServer>, 


    sample_textures_res: Res<SampleTexturesResource>, 

    image_assets: Res<Assets<Image>>,


    chunks_query: Query< 
       Entity   , 
    (   With<FoliageChunk>, Without<FoliageChunkDensityData>   )
    > 

    ){


      for chunk_entity in chunks_query.iter() {



        let density_map = image_assets.get( sample_textures_res.sample_density_map.clone() );  
        
        let y_offset_map = image_assets.get( sample_textures_res.sample_y_offset_map.clone() );  



        if let Some(density_map) = density_map {
              if let Some(y_offset_map) = y_offset_map {



                

             //   let dimensions:Vec2  = Vec2::new(256.0,256.0);

                let density_map_data = DensityMapU8::load_from_image( density_map  ).unwrap();
                let y_offset_map_data = YOffsetMapU16::load_from_image( y_offset_map  ).unwrap();

                commands.entity(chunk_entity).insert( 

                    FoliageChunkDensityData {
                        density_map_data: *density_map_data


                    }

                );


                 commands.entity(chunk_entity).insert(     
               
                    FoliageChunkDensityTexture::default() 
 
                );


                commands.entity(chunk_entity).insert(     
               
                    FoliageChunkYOffsetData {
                        y_offset_map_data: *y_offset_map_data


                    } 

                );

                commands.entity(chunk_entity).insert(     
               
                    FoliageChunkYOffsetTexture::default() 
 
                );

            }


            }
        }

}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,

    mut sample_textures_resource: ResMut<SampleTexturesResource>, 

    ) {
    commands
        .spawn(SpatialBundle::default())
        .insert(FoliageConfig::load_from_file("assets/foliage/foliage_config.ron").unwrap())
        .insert(FoliageData::new())

        .insert(Visibility::Visible)  // only in editor 
        ;

    // light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            //shadow_depth_bias: 0.5,
            //shadow_normal_bias: 0.5,

            color: Color::WHITE,

            ..default()
        },
        transform: Transform::from_xyz(4.0, 6.0, 4.0),
        ..default()
    });
    // light

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 822.12,
    });

    // camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(20.0, 162.5, 20.0)
                .looking_at(Vec3::new(900.0, 0.0, 900.0), Vec3::Y),
            ..default()
        })
        //.insert(TerrainViewer::default())
       // .insert(ShadowFilteringMethod::Jimenez14)
       ;

    sample_textures_resource.sample_density_map = asset_server.load("grass_density_map.png");
    sample_textures_resource.sample_y_offset_map = asset_server.load("grass_y_map.png");


}   

fn update_camera_look(
    mut event_reader: EventReader<MouseMotion>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut Transform, &Camera3d)>,
) {
    const MOUSE_SENSITIVITY: f32 = 2.0;

    // Accumulate mouse delta
    let mut delta: Vec2 = Vec2::ZERO;
    for event in event_reader.read() {
        delta += event.delta;
    }

    if !mouse_input.pressed(MouseButton::Left) {
        return;
    }

    // Apply to each camera with the CameraTag
    for (mut transform, _) in query.iter_mut() {
        // let rotation = transform.rotation;

        let (mut yaw, mut pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);

        yaw -= delta.x / 180.0 * MOUSE_SENSITIVITY;
        pitch -= delta.y / 180.0 * MOUSE_SENSITIVITY;
        pitch = pitch.clamp(-std::f32::consts::PI / 2.0, std::f32::consts::PI / 2.0);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
    }
}

fn update_camera_move(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Camera3d)>,
) {
    const MOVE_SPEED: f32 = 10.0; // You can adjust this value as needed

    // Apply to each camera with the CameraTag
    for (mut transform, _) in query.iter_mut() {
        // Move the camera forward if W is pressed
        if keyboard_input.pressed(KeyCode::KeyW) {
            let forward = transform.forward();
            transform.translation += forward * MOVE_SPEED;
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            let forward = transform.forward();
            transform.translation -= forward * MOVE_SPEED;
        }
    }
}


fn update_directional_light_position(
    mut query: Query<&mut Transform, With<DirectionalLight>>,
   
    time: Res<Time>,
) {

    let current_time = time.elapsed();


 //   let delta_time = time.delta_seconds();
    
    let SECONDS_IN_A_CYCLE = 20.0;

    let angle = (current_time.as_millis() as f32 / (SECONDS_IN_A_CYCLE* 1000.0) ) * std::f32::consts::PI * 2.0; // Convert time to radians

    let radius = 20.0; // Adjust the radius of the sun's orbit
    let x = angle.cos() * radius;
    let y = angle.sin() * radius + 10.0; // Adjust the height of the sun
    let z = 0.0;

    for mut transform in query.iter_mut() {

        transform.translation = Vec3::new(x, y, z);
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}

 