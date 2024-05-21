
 
use warbler_grass::prelude::DensityMap as WarblerDensityMap;
use bevy::asset::LoadedAsset;
use bevy::render::render_resource::TextureDescriptor;
use bevy::render::render_resource::TextureFormat;
use bevy::render::texture::ImageFormat;
use bevy::render::texture::ImageLoaderSettings;
 use std::path::PathBuf;

use crate::foliage_config::FoliageConfig;
use crate::y_offset_map::YOffsetMap;
use crate::y_offset_map::YOffsetMapU16;
use crate::density_map::DensityMap;
use crate::density_map::DensityMapU8;


use bevy::render::primitives::Aabb;
use warbler_grass::prelude::GrassColor;
use warbler_grass::prelude::WarblerHeight;
use warbler_grass::prelude::WarblersBundle;
use bevy::prelude::*;




#[derive(Default)]
pub struct FoliageChunkPlugin {
     
}
 
impl Plugin for FoliageChunkPlugin {
    fn build(&self, app: &mut App) {
         
        app   //use preUpdate for now to avoid race cond with warbler grass and remove entities ... 

          .add_systems(Update ,load_chunk_density_texture_handle.run_if( any_with_component::<RequestLoadFoliageChunkDensityTexture> )  )
           .add_systems(Update ,load_chunk_y_offset_texture_handle.run_if( any_with_component::<RequestLoadFoliageChunkYOffsetTexture> )  )

           .add_systems(Update ,load_chunk_density_texture.run_if( any_with_component::< FoliageChunkDensityTextureLoadHandle  > )  )
           .add_systems(Update ,load_chunk_y_offset_texture.run_if( any_with_component::< FoliageChunkYOffsetTextureLoadHandle> )  )

        .add_systems(Update ,add_chunk_density_data_from_texture.run_if( any_with_component::< FoliageChunk> )  )

 


        	.add_systems(Update ,rebuild_chunk_density_texture.run_if( any_with_component::<FoliageChunk> )  )
        	.add_systems(Update ,rebuild_chunk_y_offset_texture.run_if( any_with_component::<FoliageChunk> )  )
			.add_systems(Update ,rebuild_chunks.run_if( any_with_component::<FoliageChunk> )  )

        ; 
         
    }
} 
 
 


/*
Consider using this pattern for the terrain too !   Terrain Chunks height and splat map data for the chunk  should be a component .
*/
#[derive(Component)]
pub struct FoliageChunkDensityData {

	pub density_map_data: DensityMapU8

}

#[derive(Component)]
pub struct FoliageChunkYOffsetData {

	pub y_offset_map_data: YOffsetMapU16

}


#[derive(Component,Default)]
pub struct FoliageChunkDensityTexture {

	pub texture: Handle<Image>

}

#[derive(Component,Default)]
pub struct FoliageChunkYOffsetTexture {

	pub texture: Handle<Image>

}


#[derive(Component )]
pub struct RequestLoadFoliageChunkDensityTexture {
    pub texture_path:PathBuf
}


#[derive(Component )]
pub struct FoliageChunkDensityTextureLoadHandle {
    pub texture_handle:Handle<Image>
}

#[derive(Component )]
pub struct RequestLoadFoliageChunkYOffsetTexture {
    pub texture_path:PathBuf
}

#[derive(Component )]
pub struct FoliageChunkYOffsetTextureLoadHandle {
    pub texture_handle:Handle<Image>
}



#[derive(Component)]
pub struct FoliageChunk {
    pub chunk_id: u32 ,
 
    
} 


impl FoliageChunk {

	pub fn new(chunk_id :u32 ) -> Self {

		Self {

			chunk_id,  

		}

	}

}

#[derive(Component)]
pub struct WarblerGrass;

pub type ChunkCoords = [u32; 2];
 

pub trait ChunkCoordinates {
    fn new(x: u32, y: u32) -> Self;

    fn x(&self) -> u32;
    fn y(&self) -> u32;

    fn get_chunk_index(&self, chunk_rows: u32) -> u32;

    fn from_location(
        location: Vec3,
        terrain_origin: Vec3,
        terrain_dimensions: Vec2,
        chunk_rows: u32,
    ) -> Option<UVec2>;
    fn to_location(
        &self,
        terrain_origin: Vec3,
        terrain_dimensions: Vec2,
        chunk_rows: u32,
    ) -> Option<Vec3>;

    fn from_chunk_id(chunk_id: u32, chunk_rows: u32) -> Self;
    fn get_location_offset(&self, chunk_dimensions: Vec2) -> Vec3;

    fn get_heightmap_subsection_bounds_pct(&self, chunk_rows: u32) -> [[f32; 2]; 2];
}


impl ChunkCoordinates for ChunkCoords {
    fn new(x: u32, y: u32) -> Self {
        [x, y]
    }

    fn x(&self) -> u32 {
        self[0]
    }
    fn y(&self) -> u32 {
        self[1]
    }

    //chunk index is   chunk_col * 64  + chunk_row   IF chunk_rows is 64
    fn get_chunk_index(&self, chunk_rows: u32) -> u32 {
        return self.y() * chunk_rows + self.x() as u32;
    }

    fn from_chunk_id(chunk_id: u32, chunk_rows: u32) -> Self {
        let coords_y = chunk_id / chunk_rows;
        let coords_x = chunk_id % chunk_rows;

        [coords_x, coords_y]
    }

    fn get_location_offset(&self, chunk_dimensions: Vec2) -> Vec3 {
        Vec3::new(
            chunk_dimensions.x * self.x() as f32,
            0.0,
            chunk_dimensions.y * self.y() as f32,
        )
    }

    fn from_location(
        from_location: Vec3,
        terrain_origin: Vec3,
        terrain_dimensions: Vec2,
        chunk_rows: u32,
    ) -> Option<UVec2> {
        let location_delta = from_location - terrain_origin;

        //let terrain_min = terrain_origin;
        //let terrain_max = terrain_origin + Vec3::new(terrain_dimensions.x, 0.0, terrain_dimensions.y);

        // Check if from_location is within the terrain bounds
        if location_delta.x >= 0.0
            && location_delta.x <= terrain_dimensions.x
            && location_delta.z >= 0.0
            && location_delta.z <= terrain_dimensions.y
        {
            // Calculate the chunk's x and z coordinates
            let chunk_x = (location_delta.x / terrain_dimensions.x * chunk_rows as f32) as u32;
            let chunk_z = (location_delta.z / terrain_dimensions.y * chunk_rows as f32) as u32;

            return Some(UVec2::new(chunk_x, chunk_z));
        }

        None
    }

    //returns the middle of the chunk
    fn to_location(
        &self,
        terrain_origin: Vec3,
        terrain_dimensions: Vec2,
        chunk_rows: u32,
    ) -> Option<Vec3> {
        // Ensure chunk coordinates are within bounds
        if self.x() < chunk_rows && self.y() < chunk_rows {
            // Calculate the dimensions of a single chunk
            let chunk_dim_x = terrain_dimensions.x / chunk_rows as f32;
            let chunk_dim_z = terrain_dimensions.y / chunk_rows as f32;

            // Calculate world location for the bottom-left corner of the chunk
            let world_x = terrain_origin.x + self.x() as f32 * chunk_dim_x + chunk_dim_x / 2.0;
            let world_z = terrain_origin.z + self.y() as f32 * chunk_dim_z + chunk_dim_z / 2.0;

            return Some(Vec3::new(world_x, terrain_origin.y, world_z));
        }

        None
    }

    fn get_heightmap_subsection_bounds_pct(&self, chunk_rows: u32) -> [[f32; 2]; 2] {
        let chunk_x = self.x();
        let chunk_y = self.y();

        let pct_per_row = 1.0 / chunk_rows as f32;

        return [
            [chunk_x as f32 * pct_per_row, chunk_y as f32 * pct_per_row], //start corner x and y
            [
                (chunk_x + 1) as f32 * pct_per_row,
                (chunk_y + 1) as f32 * pct_per_row,
            ], //end corner x and y
        ];
    }
}






fn load_chunk_density_texture_handle(

    mut commands: Commands, 
   
      chunks_query: Query< 
    ( Entity,   & RequestLoadFoliageChunkDensityTexture)  
     >,
      //  images: Res<Assets<Image>>, 
     asset_server: Res<AssetServer> 

 
){
 


    for (chunk_entity, load_texture_request ) in chunks_query.iter(){
 

          let texture_handle: Handle<Image> = asset_server.load(  load_texture_request.texture_path.clone() );
            
          commands.entity(chunk_entity).insert( FoliageChunkDensityTextureLoadHandle {
            texture_handle
          } );

 
 

          //need to change the texture format ?

          commands.entity(chunk_entity).remove::<RequestLoadFoliageChunkDensityTexture>();
        
    }




}





fn load_chunk_y_offset_texture_handle(

      mut commands: Commands, 

      chunks_query: Query< 
    ( Entity,   & RequestLoadFoliageChunkYOffsetTexture)  
     >,

 //    images: Res<Assets<Image>>, 

      asset_server: Res<AssetServer> 


){ 

    for (chunk_entity, load_texture_request ) in chunks_query.iter(){
 

          let texture_handle: Handle<Image> = asset_server.load(  load_texture_request.texture_path.clone() );

          commands.entity(chunk_entity).insert( FoliageChunkYOffsetTextureLoadHandle {
            texture_handle
          } );


       
          //need to change the texture format ?

          commands.entity(chunk_entity).remove::<RequestLoadFoliageChunkYOffsetTexture>();
        
    }



}



fn add_chunk_density_data_from_texture(
    mut commands:Commands, 
    chunks_query: Query< 
        ( Entity,     &  FoliageChunkDensityTexture)  , Without<FoliageChunkDensityData>
         >,

    image_assets: Res<Assets<Image>> ,

){

    for (chunk_entity, density_texture) in chunks_query.iter(){

        let Some(density_data_image) = image_assets.get(&density_texture.texture) else {continue} ;


        let raw_data_option :Option<Box<Vec<Vec<u8>>>> = DensityMap::load_from_image(density_data_image).ok();

        let Some(raw_data) =  raw_data_option else {continue};


        commands.entity(chunk_entity).insert( FoliageChunkDensityData{
            density_map_data: *raw_data
        } );



    }

}



fn load_chunk_density_texture(

     mut commands: Commands, 

      mut ev_asset: EventReader<AssetEvent<Image>>,
 
      chunks_query: Query< 
    ( Entity,   &  FoliageChunkDensityTextureLoadHandle)  
     >,
       mut images: ResMut<Assets<Image>>, 
    // asset_server: Res<AssetServer> 

 
){


    for evt in ev_asset.read(){



        match evt{
         
            AssetEvent::LoadedWithDependencies { id } =>  {

                for (chunk_entity, load_handle) in chunks_query.iter(){

                    if id ==  &load_handle.texture_handle.id() {

                        let Some(  tex_image) = images.get_mut( &load_handle.texture_handle ) else {continue};

                        //this is messed up  ??? 
                     //   tex_image.texture_descriptor.format = TextureFormat::R8Unorm;

                          commands.entity(chunk_entity).insert( FoliageChunkDensityTexture {
                            texture: load_handle.texture_handle.clone()
                          } );

                       commands.entity(chunk_entity).remove::<FoliageChunkDensityTextureLoadHandle>();
        

                    }


                }



            },

            _ => {}
        }

    }
   


}




fn load_chunk_y_offset_texture(

    mut commands: Commands, 

      mut ev_asset: EventReader<AssetEvent<Image>>,


   
      chunks_query: Query< 
    ( Entity,   &  FoliageChunkYOffsetTextureLoadHandle)  
     >,
       mut images: ResMut<Assets<Image>>, 
    // asset_server: Res<AssetServer> 

 
){


    for evt in ev_asset.read(){

        match evt{
         
            AssetEvent::LoadedWithDependencies { id } =>  {

                for (chunk_entity, load_handle) in chunks_query.iter(){

                    if id ==  &load_handle.texture_handle.id() {

                        let Some(  tex_image) = images.get_mut( &load_handle.texture_handle ) else {continue};

                        tex_image.texture_descriptor.format = TextureFormat::R8Unorm;


                          commands.entity(chunk_entity).insert( FoliageChunkYOffsetTexture {
                            texture: load_handle.texture_handle.clone()
                          } );

                       commands.entity(chunk_entity).remove::<FoliageChunkYOffsetTextureLoadHandle>();
        

                    }


                }



            },

            _ => {}
        }

    }
   


}


fn rebuild_chunk_density_texture( 	

    mut commands: Commands, 
	  asset_server: ResMut<AssetServer> , 

	  chunks_query: Query< 
	  (  Entity,&FoliageChunkDensityData) , 
	(   With<FoliageChunk>, Changed<FoliageChunkDensityData>   ),


	>

) {

	for  (chunk_entity, density_data_comp)  in chunks_query.iter () {
 
		let density_tex_image = density_data_comp.density_map_data.to_image(); 

	    

         commands.entity(chunk_entity).insert( FoliageChunkDensityTexture {
            texture: asset_server.add( density_tex_image ) 
         } );


	} 


 }  




fn rebuild_chunk_y_offset_texture( 

      mut commands: Commands, 
	  asset_server: ResMut<AssetServer> , 

	  chunks_query: Query< 
	  (Entity, &FoliageChunkYOffsetData ) , 
	(   With<FoliageChunk>, Changed<FoliageChunkYOffsetData>   )
	>

 ) {

	 for   (chunk_entity,y_offset_data_comp)  in chunks_query.iter() {
  
		let y_offset_tex_image = y_offset_data_comp.y_offset_map_data.to_image(); 

		//chunk_y_offset_texture_comp.texture = asset_server.add( density_tex_image );

 
        commands.entity(chunk_entity).insert( FoliageChunkYOffsetTexture {
            texture: asset_server.add( y_offset_tex_image ) 
         } );
 
	} 


 }  





// the warbler grass will be a child of the foliage chunk -- ? 


// is this necessary ?  bc of the way shader works ? 
fn rebuild_chunks(  
	mut commands: Commands, 
	chunks_query: Query< 
	(Entity,&FoliageChunk, &FoliageChunkDensityTexture, &FoliageChunkYOffsetTexture), 
	Or< (Changed<FoliageChunkDensityTexture>, Changed<FoliageChunkYOffsetTexture>) >     
	>,


      mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>,
 ) {


	for (chunk_entity, foliage_chunk, density_map, y_offset_map) in chunks_query.iter() {


		let density_map  = &density_map.texture ;
		let y_offset_map  = &y_offset_map.texture; 


		let chunk_dimensions= Vec2::new(256.0,256.0); //make me dynamic 

        //let blade_height = 16.0; //fix me

 
 
		let color = Color::rgb(0.3, 0.4, 0.1); 
		 commands.entity(chunk_entity).despawn_descendants();


		  let chunk_id = foliage_chunk.chunk_id; 

		  info!("rebuild chunk {:?}", chunk_id );

          //65536
          let scale_height = 256.0;

          let density_scale_factor = 2.5;

		 	//could make this more efficient by only modifying the handles if the entity alrdy exists ?
		 let grass_bundle = commands.spawn(WarblersBundle {
            // we could use seperate density maps for each one
            density_map: WarblerDensityMap {
                    density_map: density_map.clone(),
                    density: density_scale_factor,
                },

 //   density_map.clone().into(),
            // or seperate height maps if we wanted to
            y_map: y_offset_map.clone().into(),

            height: WarblerHeight::Uniform( 2.0 ),

            // the aabb defined the dimensions of the box the chunk lives in
            aabb: Aabb::from_min_max(Vec3::ZERO, Vec3::new(chunk_dimensions.x, scale_height, chunk_dimensions.y)),
            grass_color: GrassColor {
                main_color: color,
                bottom_color: color * 0.9,
            },

            spatial: SpatialBundle {
                transform: Transform::default()
                .with_scale(Vec3::new(1.0,256.0,1.0)) , 

                ..default()
            },
            ..default()
        }).id();

         commands.entity(grass_bundle).insert( WarblerGrass  );
        

/*


        let plane =  commands.spawn(PbrBundle {
                mesh: meshes.add(Plane3d::default().mesh().size(100.0, 100.0)),
               material: materials.add( Color::SILVER ),

              /* material:  materials.add( StandardMaterial {
                    base_color_texture: Some( density_map.clone() ) , 
                ..default()
               } ) ,
                */

                ..default()
            }).id();
*/


		 commands.entity(chunk_entity).add_child( grass_bundle ); 
   //  commands.entity(chunk_entity).add_child( plane ); 

	}

}