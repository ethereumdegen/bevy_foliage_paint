
 
use bevy::render::primitives::Aabb;
use warbler_grass::prelude::GrassColor;
use warbler_grass::prelude::WarblerHeight;
use warbler_grass::prelude::WarblersBundle;
use bevy::prelude::*;

use crate::density_map::DensityMapU8;


pub type HeightMapU8 = Vec<Vec<u8>>; 


/*
Consider using this pattern for the terrain too !   Terrain Chunks height and splat map data for the chunk  should be a component .
*/
#[derive(Component)]
pub struct FoliageChunkDensityMap {

	pub density_map_data: DensityMapU8

}

#[derive(Component)]
pub struct FoliageChunkHeightMap {

	pub height_map_data: HeightMapU8

}

#[derive(Component)]
pub struct FoliageChunk {
    chunk_id: usize ,
 
    
} 


impl FoliageChunk {

	pub fn new(chunk_id :usize ) -> Self {

		Self {

			chunk_id,  

		}

	}

}


pub type ChunkCoords = [u32; 2];




// the warbler grass will be a child of the foliage chunk -- ? 



fn rebuild_chunks(  
	mut commands: Commands, 
	chunks_query: Query< 
	(Entity,&FoliageChunk, &FoliageChunkDensityMap, &FoliageChunkHeightMap), 
	Or< (Changed<FoliageChunkDensityMap>, Changed<FoliageChunkHeightMap>) >     
	>
 ) {


	for (chunk_entity, foliage_chunk, density_map, height_map) in chunks_query.iter() {


		let density_map_data = &density_map.density_map_data ;
		let height_map_data = &height_map.height_map_data; 


		let chunk_dimensions= Vec2::new(256.0,256.0); //make me dynamic 

		let color = Color::rgb(0.2, 0.6, 0.4);
		 commands.entity(chunk_entity).despawn_descendants();


		 let chunk_id = foliage_chunk.chunk_id; 

		 let offset = Vec3::new(
            (chunk_id / chunk_dimensions.x as usize) as f32 * chunk_dimensions.x * 1.0,
            0.,
            (chunk_id % chunk_dimensions.x as usize) as f32 * chunk_dimensions.y * 1.0,
        );

		 let grass_bundle = commands.spawn(WarblersBundle {
            // we could use seperate density maps for each one
            density_map: density_map.clone(),
            // or seperate height maps if we wanted to
            y_map: y_map.clone(),
            height: WarblerHeight::Texture(density_map_handle.clone()),
            // the aabb defined the dimensions of the box the chunk lives in
            aabb: Aabb::from_min_max(Vec3::ZERO, Vec3::new(chunk_dimensions.x, 2., chunk_dimensions.y)),
            grass_color: GrassColor {
                main_color: color,
                bottom_color: color * 0.4,
            },

            spatial: SpatialBundle {
             transform: Transform::from_translation(offset),
                ..default()
            },
            ..default()
        }).id();


		 commands.entity(chunk_entity).add_child( grass_bundle ); 

	}

}