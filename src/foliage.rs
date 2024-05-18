 
use crate::foliage_chunk::ChunkCoords;
use crate::foliage_chunk::FoliageChunk;
use bevy::asset::{AssetPath, LoadState};
use bevy::pbr::{ExtendedMaterial, OpaqueRendererMethod};
use bevy::prelude::*;
use bevy::render::render_resource::{
    TextureFormat,
};

use super::density_map::{DensityMapU8,DensityMap};

use bevy::utils::HashMap;

 
use crate::foliage_config::FoliageConfig;
 

#[derive(Resource, Default)]
pub struct FoliageDataMapResource {
    pub density_map_data: Option<DensityMapU8>, // Keyed by chunk id
}



#[derive(Component)]
pub struct RegionPlaneMesh {

}

 

#[derive(Event)]
pub enum FoliageDataEvent {
    FoliageNeedsReloadFromResourceData
} 
#[derive(Default, PartialEq, Eq)]
pub enum FoliageDataStatus {
    //us this for texture image and splat image and alpha mask .. ?
    #[default]
    NotLoaded,
    Loaded,
}

#[derive(Component, Default)]
pub struct  FoliageData {
     
    pub foliage_data_status: FoliageDataStatus,

    texture_image_handle: Option<Handle<Image>>,
    color_map_texture_handle:  Option<Handle<Image>>,
 
    foliage_image_data_load_status: bool ,
 
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

    mut asset_server: ResMut<AssetServer>, 

    mut foliage_root_query: Query<(Entity, &mut FoliageData, &FoliageConfig)>,

    mut meshes: ResMut <Assets<Mesh>>,
  //  mut region_materials: ResMut<Assets<RegionsMaterialExtension>>,

    mut images: ResMut<Assets<Image>>
) {
    for (foliage_root_entity, mut foliage_data, foliage_config) in foliage_root_query.iter_mut() {
        if foliage_data.foliage_data_status ==  FoliageDataStatus::NotLoaded {
                

            let max_chunks = foliage_config.chunk_rows * foliage_config.chunk_rows;

            for chunk_id in 0..max_chunks {



                let chunk_coords = ChunkCoords::from_chunk_id(chunk_id, foliage_config.chunk_rows); // [ chunk_id / terrain_config.chunk_rows ,  chunk_id  % terrain_config.chunk_rows];
                let chunk_dimensions = foliage_config.get_chunk_dimensions();

                let chunk_name = format!("Chunk {:?}", chunk_id);

                let chunk_entity = commands
                    .spawn(FoliageChunk::new(chunk_id))
                    .insert(Name::new(chunk_name))
                    .insert(SpatialBundle {
                        transform: Transform::from_xyz(
                            chunk_coords.x() as f32 * chunk_dimensions.x,
                            0.0,
                            chunk_coords.y() as f32 * chunk_dimensions.y,
                        ),
                        visibility: Visibility::Hidden,

                        ..Default::default()
                    })
                    .id();

                let mut terrain_entity_commands = commands.get_entity(foliage_root_entity).unwrap();

                //terrain_data.chunk_entity_lookup.insert(chunk_id,chunk_entity.clone());
                terrain_entity_commands.add_child(chunk_entity);




            }


         
 
            foliage_data.foliage_data_status = FoliageDataStatus::Loaded
        }
    }
}

impl FoliageData {
    pub fn get_density_texture_image(&self) -> &Option<Handle<Image>> {
        &self.texture_image_handle 

      
    }
}

/*
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