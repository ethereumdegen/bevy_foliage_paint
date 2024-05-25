//use crate::tool_preview::update_tool_uniforms;
use warbler_grass::prelude::WarblersPlugin;
use crate::foliage::FoliageDataPlugin;
use crate::foliage_chunk::FoliageChunkPlugin;
use crate::foliage::{  FoliageDataEvent };
use crate::edit::BevyFoliageEditsPlugin;
//use crate::regions::load_regions_texture_from_image;
 
use bevy::time::common_conditions::on_timer;
use bevy::{asset::load_internal_asset, prelude::*};
 
use foliage::{ initialize_foliage,  };

use std::time::Duration;
   
  
 
pub mod edit;

pub mod foliage_chunk;

pub mod density_map;
pub mod y_offset_map;
 
pub mod foliage;
pub mod foliage_config;
  
pub mod tool_preview;

pub struct BevyFoliagePaintPlugin {
    task_update_rate: Duration,
}

impl Default for BevyFoliagePaintPlugin {
    fn default() -> Self {
        Self {
            task_update_rate: Duration::from_millis(250),
        }
    }
}
impl Plugin for BevyFoliagePaintPlugin {
    fn build(&self, app: &mut App) {
        
           app.add_plugins( WarblersPlugin ) ;
        app.add_plugins( BevyFoliageEditsPlugin::default() ) ;

        app.add_plugins( FoliageChunkPlugin::default() ) ;

         app.add_plugins( FoliageDataPlugin::default() ) ;

         
        app.add_event::<FoliageDataEvent>() ;
        app.init_resource::<tool_preview::ToolPreviewResource>();
       
 
       
        
        
 
    

        
    }
}
