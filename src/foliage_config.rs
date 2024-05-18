/*

this is loaded from a RON file


also should incorporate the paths to the height and splat folders for their texture handles...

*/
use bevy::prelude::*;

use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Component, Deserialize, Serialize, Clone)]
pub struct FoliageConfig {
    pub boundary_dimensions: Vec2, 
    chunk_rows: usize,

 
    pub density_folder_path: PathBuf,
    pub grass_y_map_folder_path: PathBuf,
   // pub foliage_manifest_file: PathBuf,
    
}

impl Default for FoliageConfig {
    fn default() -> Self {
        Self {
            // chunk_width: 64.0 ,
            boundary_dimensions: Vec2::new(1024.0, 1024.0), //this should match the heightmap dimensions... consider removing this var or changing how it fundamentally works .
            chunk_rows: 4,


            density_folder_path: "foliage/regions.png".into(),
            grass_y_map_folder_path: "foliage/region_color_map.png".into(),
        
            
        }
    }
}

impl FoliageConfig {
    pub fn load_from_file(file_path: &str) -> Result<Self, ron::Error> {
        let mut file = File::open(file_path).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read file");
        Ok(ron::from_str(&contents)?)
    }

  
}
