use crate::foliage_chunk::ChunkCoordinates;
use crate::foliage_chunk::FoliageChunk;
use crate::density_map::DensityMapU8;
use crate::foliage_chunk::FoliageChunkDensityData;
use std::fs::File;
use std::io::BufWriter;
use std::ops::{Add, Div, Neg};
use std::path::{Path, PathBuf};

use bevy::ecs::entity::Entity;
use bevy::math::Vec2;

use bevy::ecs::event::Event;
use bevy::prelude::EventReader;

use bevy::asset::{AssetServer, Assets};
use bevy::render::render_resource::{Extent3d, TextureFormat};
use bevy::render::texture::Image;

use bevy::prelude::*;
 
use core::fmt::{self, Display, Formatter};

  
use crate::foliage::{FoliageDataEvent,    FoliageData,  FoliageDataMapResource};
use crate::foliage_config::FoliageConfig;
 

 
 
use anyhow::{Context, Result};

use serde::{Deserialize, Serialize};
use serde_json;

use rand::Rng;

use core::cmp::{max, min};


pub struct BevyFoliageEditsPlugin {
    
}

impl Default for BevyFoliageEditsPlugin {
    fn default() -> Self {
        Self {
             
        }
    }
}
impl Plugin for BevyFoliageEditsPlugin {
    fn build(&self, app: &mut App) {


      app.add_event::<EditFoliageEvent>();
       app.add_event::<FoliageCommandEvent>();
       app.add_event::<FoliageBrushEvent>();
       app.add_systems(Update, apply_tool_edits); // add back in later 
        app.add_systems(Update, apply_command_events);


    }
}

#[derive(Debug, Clone)]
pub enum EditingTool {
    // SetFoliageIndex { foliage_index: u8 },        // height, radius, save to disk 
    SetFoliageDensity {density: u8 }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum BrushType {
    #[default]
    SetExact, // hardness ?
    Smooth,
    //Noise,
    EyeDropper,
}

impl Display for BrushType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let label = match self {
            BrushType::SetExact => "SetExact",
            BrushType::Smooth => "Smooth",
          //  BrushType::Noise => "Noise",
            BrushType::EyeDropper => "EyeDropper",
        };

        write!(f, "{}", label)
    }
}

// entity, editToolType, coords, magnitude
#[derive(Event, Debug, Clone)]
pub struct EditFoliageEvent {

   // pub entity: Entity, //should always be the plane 
    pub tool: EditingTool,
    pub radius: f32,
    pub brush_hardness: f32, //1.0 is full
    pub coordinates: Vec2,
    pub brush_type: BrushType,
}

#[derive(Event, Debug, Clone)]
pub enum FoliageBrushEvent {
    EyeDropFoliageDensity { density: u8 },
  //  EyeDropSplatMap { r: u8, g: u8, b: u8 },
}

#[derive(Event, Debug, Clone)]
pub enum FoliageCommandEvent {
    SaveAll ,  
}

pub fn apply_command_events(
    asset_server: Res<AssetServer>,

   // mut chunk_query: Query<(&Chunk, &mut ChunkData, &Parent, &Children)>, //chunks parent should have terrain data

   // mut images: ResMut<Assets<Image>>,
    //mut region_materials: ResMut<Assets<RegionsMaterialExtension>>,

    mut foliage_map_res: ResMut<FoliageDataMapResource>, //like height map resource 

   foliage_data_query: Query<(&FoliageData, &FoliageConfig)>,

    
    mut ev_reader: EventReader<FoliageCommandEvent>,
) {
    for ev in ev_reader.read() {
       
           

            let Some((foliage_data, foliage_config)) = foliage_data_query
                    .get_single().ok() else {continue};



            match ev {
                FoliageCommandEvent::SaveAll => {
                    //let file_name = format!("{}.png", chunk.chunk_id);
                     let asset_folder_path = PathBuf::from("assets");
                    let density_texture_path = &foliage_config.density_folder_path;
                     
                    
                //   do this in a chunked way ! Per foliage chunk.. like terrain height / splat
                      if let Some(region_data) =
                          &  foliage_map_res.density_map_data
                        {

                            if let Some( density_texture_path )= density_texture_path {

                                save_density_map_to_disk(
                                        &region_data,
                                        asset_folder_path.join( density_texture_path ),
                                );
                            }
                    }
                     
 

                     

                    println!("saved foliage density maps ");
                
            }
          }
        }
     

    //  Ok(())

}




//need to do this w chunks ... each chunk has density data? and must be linked w height data.. 


 
pub fn apply_tool_edits(
  
    foliage_data_query: Query<(&mut FoliageData, &FoliageConfig)> , 

   

    //mut foliage_map_data_res: ResMut<FoliageDataMapResource>,
 
     // region_plane_mesh_query: Query<(Entity,   &GlobalTransform), With<RegionPlaneMesh>>,

     mut foliage_chunk_query: Query<(Entity, &FoliageChunk, &mut FoliageChunkDensityData,   &Parent, &GlobalTransform)>, //chunks parent should have terrain data
   

    mut ev_reader: EventReader<EditFoliageEvent>,

    mut evt_writer: EventWriter<FoliageBrushEvent>,

    mut foliage_data_event_writer: EventWriter<FoliageDataEvent>
) {
    for ev in ev_reader.read() {
        eprintln!("-- {:?} -- region edit event!", &ev.tool);

       let Some((foliage_data, foliage_config)) = foliage_data_query
                    .get_single().ok() else {
                          warn!("no regions entity found" );
                        continue
                    };



        //let intersected_entity = &ev.entity;

       
    /*   let Some((region_plane_entity,  _ )) = region_plane_mesh_query.get(intersected_entity.clone()).ok() else {
        warn!("region plane not intersected");
        continue
    } ;*/
            //let mut chunk_entities_within_range: Vec<Entity> = Vec::new();

            let boundary_dimensions = foliage_config.boundary_dimensions.clone(); //compute me from  config
          

      

             let tool_coords: &Vec2 = &ev.coordinates;
             info!("tool coords {:?}", tool_coords);

             // need to find chunk id here ! i think. 

            


            let mut chunk_entities_within_range: Vec<Entity> = Vec::new();

            let mut chunk_dimensions = [256, 256]; //compute me from terrain config
         


               //populate chunk_entities_within_range
            for (chunk_entity, _, _, _, chunk_transform) in foliage_chunk_query.iter() {
                let tool_coords: &Vec2 = &ev.coordinates;
                let chunk_transform = chunk_transform.translation();
                let chunk_transform_vec2: Vec2 = Vec2::new(chunk_transform.x, chunk_transform.z);

                let chunk_dimensions_vec: Vec2 =
                    Vec2::new(chunk_dimensions.x() as f32, chunk_dimensions.y() as f32);
                let chunk_center_transform =
                    chunk_transform_vec2.add(chunk_dimensions_vec.div(2.0));

                let chunk_local_distance = tool_coords.distance(chunk_center_transform);

                if chunk_local_distance < 800.0 {
                    chunk_entities_within_range.push(chunk_entity);
                }
            }





            
            let average_height = 0; //for now  // total_height as f32 / heights_len as f32;
            // ------
            let radius = &ev.radius;
            let brush_type = &ev.brush_type;

              info!("Region Set Exact 1 ");

              /* let Some(foliage_map_data) =
                                &mut foliage_map_data_res.density_map_data
                            else {
                                warn!("density data map is null ");
                                continue
                            }; */

             // let mut foliage_density_map_changed = false;

            let brush_hardness = &ev.brush_hardness;
            
             for chunk_entity_within_range in chunk_entities_within_range {
                if let Some((
                    chunk_entity,
                    _chunk,
                    mut chunk_density_data,
                    terrain_entity,
                    chunk_transform,
                )) = foliage_chunk_query.get_mut(chunk_entity_within_range.clone()).ok()
                {

                    
                            

                              
                    match &ev.tool {
                        EditingTool::SetFoliageDensity { density } => {


                                let tool_coords: &Vec2 = &ev.coordinates;


                                let chunk_transform = chunk_transform.translation();
                                let chunk_transform_vec2: Vec2 =
                                    Vec2::new(chunk_transform.x, chunk_transform.z);

                                let tool_coords_local = tool_coords.add(chunk_transform_vec2.neg());

 
                                // let mut density_changed = false;
                                let radius_clone = radius.clone();


                            let density_map_data = &mut chunk_density_data.density_map_data;
                             let img_data_length = density_map_data.len();


                                match brush_type {
                                    BrushType::SetExact => {
                                        for x in 0..img_data_length {
                                            for y in 0..img_data_length {
                                                let local_coords = Vec2::new(x as f32, y as f32);

                                                 // info!("local_coords {:?} ", local_coords);

                                                let hardness_multiplier = get_hardness_multiplier(
                                                    tool_coords_local.distance(local_coords),
                                                    radius_clone,
                                                    *brush_hardness,
                                                );
                                                let original_density = density_map_data[y][x];


                                                 //  info!("tool_coords_local {:?} ", tool_coords_local);


                                                
                                                if tool_coords_local.distance(local_coords)
                                                    < radius_clone
                                                {
                                                    let new_density = density.clone();
                                                    density_map_data[y][x] =
                                                        apply_hardness_multiplier(
                                                            original_density as f32,
                                                            new_density as f32,
                                                            hardness_multiplier,
                                                        )
                                                            as u8;
                                                  //  density_changed = true;
                                                }
                                            }
                                        }
                                    }

                                   /* BrushType::Smooth => {
                                        for x in 0..img_data_length {
                                            for y in 0..img_data_length {
                                                let local_coords = Vec2::new(x as f32, y as f32);
                                                if tool_coords_local.distance(local_coords)
                                                    < *radius
                                                {
                                                    let hardness_multiplier =
                                                        get_hardness_multiplier(
                                                            tool_coords_local
                                                                .distance(local_coords),
                                                            radius_clone,
                                                            *brush_hardness,
                                                        );

                                                    let original_density = density_map_data[y][x];
                                                    // Gather heights of the current point and its neighbors within the brush radius

                                                    let new_density = ((average_density
                                                        + original_density as f32)
                                                        / 2.0)
                                                        as u16;
                                                    density_map_data[y][x] =
                                                        apply_hardness_multiplier(
                                                            original_density as f32,
                                                            new_density as f32,
                                                            hardness_multiplier,
                                                        )
                                                            as u8;
                                                    //height_changed = true;
                                                }
                                            }
                                        }
                                    }*/

                                     

                                    BrushType::EyeDropper => {
                                        // Check if the clicked coordinates are within the current chunk
                                         
                                            
                                            let x = tool_coords_local.x as usize;
                                            let y = tool_coords_local.y as usize;

                                            if x < img_data_length && y < img_data_length {
                                              

                                                let local_data = density_map_data[y][x];
                                                evt_writer.send(
                                                    FoliageBrushEvent::EyeDropFoliageDensity  {
                                                        density: local_data,
                                                    },
                                                );
                                            }
                                        
                                    }

                                    _ => {
                                        warn!("tool not impl ! ");
                                    }
                                }

                              
                            }
                       
                     



                    } //match

                }// query iter 
                
             /* if foliage_density_map_changed {

                             

                                   foliage_data_event_writer.send(

                                         FoliageDataEvent::FoliageNeedsReloadFromResourceData
                                    );
                }*/
     }
 }
} 



fn get_hardness_multiplier(pixel_distance: f32, brush_radius: f32, brush_hardness: f32) -> f32 {
    // Calculate the distance as a percentage of the radius
    let distance_percent = pixel_distance / brush_radius;
    let adjusted_distance_percent = f32::min(1.0, distance_percent); // Ensure it does not exceed 1

    // Calculate the fade effect based on brush hardness
    // When hardness is 0, this will linearly interpolate from 1 at the center to 0 at the edge
    // When hardness is between 0 and 1, it adjusts the fade effect accordingly
    let fade_effect = 1.0 - adjusted_distance_percent;

    // Apply the brush hardness to scale the fade effect, ensuring a minimum of 0
    f32::max(
        0.0,
        fade_effect * (1.0 + brush_hardness) - (adjusted_distance_percent * brush_hardness),
    )
}

fn apply_hardness_multiplier(
    original_height: f32,
    new_height: f32,
    hardness_multiplier: f32,
) -> f32 {
    original_height + (new_height - original_height) * hardness_multiplier
}


 
// outputs as R16 grayscale
pub fn save_density_map_to_disk<P>(
    density_map_data: &DensityMapU8, // Adjusted for direct Vec<Vec<u16>> input
    save_file_path: P,
) where
    P: AsRef<Path>,
{
    let density_map_data = density_map_data.clone();

    let height = density_map_data.len();
    let width = density_map_data.first().map_or(0, |row| row.len());

    let file = File::create(save_file_path).expect("Failed to create file");
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight); // Change to 8-bit depth
    let mut writer = encoder.write_header().expect("Failed to write PNG header");

    // Flatten the Vec<Vec<u8>> to a Vec<u8> for the PNG encoder
    let buffer: Vec<u8> = density_map_data.iter().flatten().cloned().collect();

    // Write the image data
    writer
        .write_image_data(&buffer)
        .expect("Failed to write PNG data");
}
