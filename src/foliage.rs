 
use crate::foliage_chunk::WarblerGrass;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::render::mesh::Indices;
use std::path::PathBuf;
use crate::foliage_chunk::RequestLoadFoliageChunkDensityTexture;
use crate::foliage_chunk::RequestLoadFoliageChunkYOffsetTexture;
use crate::foliage_chunk::FoliageChunkYOffsetTexture;
use crate::foliage_chunk::ChunkCoordinates;
use crate::foliage_chunk::ChunkCoords;
use crate::foliage_chunk::FoliageChunk;
use crate::foliage_chunk::FoliageChunkDensityTexture;
use bevy::asset::{AssetPath, LoadState};
use bevy::pbr::{ExtendedMaterial, OpaqueRendererMethod};
use bevy::prelude::*;
use bevy::render::render_resource::{
    TextureFormat,
};

use super::density_map::{DensityMapU8,DensityMap};

use bevy::utils::HashMap;

 
use crate::foliage_config::FoliageConfig;
 
 #[derive(Component)]
struct UpdatedGrassMesh ;


#[derive(Default)]
pub struct FoliageDataPlugin {
     
}
 
impl Plugin for FoliageDataPlugin {
    fn build(&self, app: &mut App) {
         
        app   

         //   . init_resource::<FoliageDataMapResource>() 

            .add_systems(Update , (
                initialize_foliage,
                replace_grass_mesh
                )  )
             

        ; 
         
    }
} 
 


//used by Edit system !? 
/*#[derive(Resource, Default)]
pub struct FoliageDataMapResource {
    pub density_map_data: Option<DensityMapU8>, // Keyed by chunk id
}*/



#[derive(Component)]
pub struct RegionPlaneMesh {

}

 

#[derive(Event)]
pub enum FoliageDataEvent {
    FoliageNeedsReloadFromResourceData
}


 pub const CUSTOM_GRASS_MESH_HANDLE: Handle<Mesh> = Handle::weak_from_u128(7_257_128_457_583_957_921);



//pub struct FoliageChunksLoaded {}



#[derive(Component, Default)]
pub struct  FoliageData {
     
   // pub foliage_data_status: FoliageDataStatus,

   // texture_image_handle: Option<Handle<Image>>,
   // color_map_texture_handle:  Option<Handle<Image>>,
 
    //foliage_image_data_load_status: bool ,
 
}

impl FoliageData {
    pub fn new() -> Self {
        let foliage_data = FoliageData::default();

         
        foliage_data
    }
}



 //index ? 

//pub type PlanarPbrBundle = MaterialMeshBundle<RegionsMaterialExtension>;


//spawn the chunks !? 
pub fn initialize_foliage(
    mut commands: Commands,

  //  mut asset_server: ResMut<AssetServer>, 

    foliage_root_query: Query<(Entity,  & FoliageData, &FoliageConfig),
       Added< FoliageData >>,

    mut meshes: ResMut <Assets<Mesh>>,
  //  mut region_materials: ResMut<Assets<RegionsMaterialExtension>>,

   // mut images: ResMut<Assets<Image>>
) {


  
    meshes.insert(CUSTOM_GRASS_MESH_HANDLE, custom_grass_mesh());



    for (foliage_root_entity, foliage_data, foliage_config) in foliage_root_query.iter (){
        
                

            let max_chunks = foliage_config.chunk_rows * foliage_config.chunk_rows;

            for chunk_id in 0..max_chunks {



                let chunk_coords = ChunkCoords::from_chunk_id(chunk_id  , foliage_config.chunk_rows); // [ chunk_id / terrain_config.chunk_rows ,  chunk_id  % terrain_config.chunk_rows];
                let chunk_dimensions = foliage_config.get_chunk_dimensions();

                let chunk_name = format!("Chunk {:?}", chunk_id);



                let height_scale =  foliage_config.height_scale   ;

                let chunk_entity = commands
                    .spawn(FoliageChunk::new(chunk_id))

                    .insert(Name::new(chunk_name))
                    .insert(SpatialBundle {
                        transform: Transform::from_xyz(
                            chunk_coords.x() as f32 * chunk_dimensions.x,
                            0.0,
                            chunk_coords.y() as f32 * chunk_dimensions.y,
                        ).with_scale( Vec3::new(1.0, height_scale, 1.0) ),


                      //  visibility: Visibility::Hidden,

                        ..Default::default()
                    })
                    .id();
 

                    let density_texture_path: Option<PathBuf> = foliage_config.density_folder_path.as_ref().map(|path| 
                        path.join(  format!("{}.png", chunk_id.to_string() )  )
                    ); 

                    if let Some(texture_path) = density_texture_path {
                        commands.entity(chunk_entity)
                         .insert(RequestLoadFoliageChunkDensityTexture {
                            texture_path
                         });
                     }

                     let y_offset_texture_path: Option<PathBuf> = foliage_config.grass_y_map_folder_path.as_ref().map(|path| 
                         path.join(  format!("{}.png", chunk_id.to_string() )  )
                     ); 

                     if let Some(texture_path) = y_offset_texture_path {

                         commands.entity(chunk_entity)
                         .insert(RequestLoadFoliageChunkYOffsetTexture {
                             texture_path
                         });
                     }

               // let mut terrain_entity_commands = commands.get_entity(foliage_root_entity).unwrap();

                //terrain_data.chunk_entity_lookup.insert(chunk_id,chunk_entity.clone());
                //terrain_entity_commands.add_child(chunk_entity);

                  commands.entity(foliage_root_entity).add_child(chunk_entity) ;


            }


            commands.entity(foliage_root_entity).insert(Name::new("Foliage Root")) ;

              //commands.entity(foliage_root_entity).insert(FoliageChunksLoaded)

         
                  
            //foliage_data.foliage_data_status = FoliageDataStatus::Loaded
         
    }
}




/*
    original: 
        vec![
            [0., 0., 0.],
            [0.5, 0., 0.],
            [0.25, 0., 0.4],
            [0.25, 1., 0.15],
        ],

*/

const GRASS_WIDTH:f32 = 1.6;
const GRASS_HEIGHT:f32 = 1.0;

fn custom_grass_mesh() -> Mesh {
    let mut grass_mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    grass_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [0., 0., 0.],
            [0.1*GRASS_WIDTH, 0., 0.],
            [0.05*GRASS_WIDTH, 0., 0.08*GRASS_WIDTH],
            [0.05*GRASS_WIDTH, 1.0 * GRASS_HEIGHT, 0.03*GRASS_WIDTH],
        ],
    );
    grass_mesh.insert_indices(Indices::U32(vec![1, 0, 3, 2, 1, 3, 0, 2, 3]));
    grass_mesh
}


fn replace_grass_mesh(
    mut commands: Commands,
     grass_query: Query< Entity, (With< WarblerGrass>,Without<UpdatedGrassMesh>) >

  ) {

    for grass_entity in grass_query.iter(){

        commands.entity(grass_entity).try_insert((CUSTOM_GRASS_MESH_HANDLE,UpdatedGrassMesh));

    }
}


/*

fn change_colors(mut grass_colors: Query<&mut GrassColor>, time: Res<Time>) {
    // Most likely you'd want to choose other colors
    let r = ((time.elapsed_seconds() / 2.).sin() / 2.) + 0.5;
    let g = 1. - r;
    for mut color in &mut grass_colors {
        color.main_color.set_r(r);
        color.main_color.set_g(g);
        color.main_color.set_b((g * r).sin());
        // the bottom color should normally be a bit darker than the main color.
        color.bottom_color = color.main_color * 0.5;
    }
}


*/

/*
impl FoliageData {
    pub fn get_density_texture_image(&self) -> &Option<Handle<Image>> {
        &self.texture_image_handle 

      
    }
}


pub fn load_density_texture_from_image(
    mut regions_query: Query<(&mut RegionsData, &RegionsConfig)>,

    mut regions_data_res: ResMut<RegionsDataMapResource>,

    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    for (mut regions_data, regions_config) in regions_query.iter_mut() {
        if regions_data.texture_image_handle.is_none() {
            let texture_path = &regions_config.region_texture_path;
            let tex_image = asset_server.load(AssetPath::from_path(texture_path));
            regions_data.texture_image_handle = Some(tex_image);
        }

        if regions_data.regions_image_data_load_status ==false {
            let texture_image: &mut Image = match &regions_data.texture_image_handle {
                Some(texture_image_handle) => {
                    let texture_image_loaded = asset_server.get_load_state(texture_image_handle);

                    if texture_image_loaded != Some(LoadState::Loaded) {
                        println!("regions texture not yet loaded");
                        continue;
                    }

                    images.get_mut(texture_image_handle).unwrap()
                }
                None => continue,
            };

            let raw_data = RegionMapU8::load_from_image(texture_image).ok().unwrap();

            regions_data_res.regions_data_map = Some( *raw_data  ) ;

            // Specify the desired texture format
            let desired_format = TextureFormat::Rgba8Uint;


            texture_image.texture_descriptor.format = desired_format; 
            // Create a new texture descriptor with the desired format
           // let mut texture_descriptor = TextureDescriptor

             

            regions_data.regions_image_data_load_status =true;
        }
    }
}

*/




 /*


pub fn listen_for_foliage_events(
    mut commands : Commands, 
   mut  evt_reader: EventReader<RegionDataEvent>,

   regions_data_res: Res <RegionsDataMapResource>,
  mut region_data_query: Query<(&mut RegionsData, &RegionsConfig)> , 


  //   asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    //mut region_materials: ResMut<Assets<RegionsMaterialExtension>>,


    //plane_mesh_query: Query<Entity, With<RegionPlaneMesh>>,

     plane_mat_ext_handle_query: Query<&Handle<RegionsMaterialExtension>, With<RegionPlaneMesh>>,

    mut region_materials: ResMut<Assets<RegionsMaterialExtension>>,

    ){

    for evt in evt_reader.read(){


          let Some((mut region_data, _region_config)) = region_data_query
                    .get_single_mut().ok() else {continue};


       


        match evt{
            RegionDataEvent::RegionMapNeedsReloadFromResourceData =>  {

 

                let data_in_resource = &regions_data_res.regions_data_map;

                if let Some(data_map ) = data_in_resource {

                    let data_map_vec : RegionMapU8 = data_map.to_vec();
                    let new_regions_texture = data_map_vec.to_image();


                     region_data.texture_image_handle = Some(images.add(new_regions_texture));

                     info!("update texture image handle ");


                      let Some(mat_ext_handle) = plane_mat_ext_handle_query.get_single().ok() else {continue};

                      let Some(   mat_ext )  = region_materials.get_mut(mat_ext_handle) else {continue} ;

                      mat_ext.extension.regions_texture = region_data.texture_image_handle.clone();

 


                }
 


            },
        }




    }

}



*/