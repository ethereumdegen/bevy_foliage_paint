use std::io::BufWriter;
use std::fs::File;
use std::path::Path;
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use thiserror::Error;

/*
https://github.com/norman784/gaiku/blob/master/crates/gaiku_baker_heightmap/src/lib.rs
*/

#[derive(Error, Debug)]
pub enum DensityMapError {
    #[error("failed to load the image")]
    LoadingError,
}

pub type DensityMapU8 = Vec<Vec<u8>>;

 /*
pub struct SubRegionMapU8(pub Vec<Vec<u8>>);

impl SubRegionMapU8 {
   


   

    pub fn append_x_row(&mut self, row: Vec<u8>) {
        self.0.push(row);
    }

    //this is busted ? \
    pub fn append_y_col(&mut self, col: Vec<u8>) {
        // Check if the number of elements in `col` matches the number of rows in the height data.
        // If not, you may need to handle this discrepancy based on your specific requirements.
        if col.len() != self.0.len() {
            // Handle error or discrepancy.
            // For example, you might return early or panic, depending on how strict you want to be.
            // e.g., panic!("Column length does not match the number of rows in height data.");
            println!("WARN: cannot append y col "); // Or handle this situation appropriately.
            panic!("Column length does not match the number of rows in height data.");
        }

        for (row, &value) in self.0.iter_mut().zip(col.iter()) {
            row.push(value);
        }
    }
}*/

pub trait DensityMap {
    fn load_from_image(image: &Image) -> Result<Box<Self>, DensityMapError>;

    fn to_image(&self) -> Image;

 fn save_density_map_to_disk<P>(
   &self, // Adjusted for direct Vec<Vec<u16>> input
    save_file_path: P,
) where
    P: AsRef<Path> ;

}

impl DensityMap for DensityMapU8 {

    //this expects data to be stored  [y][x]
    //rgba8uint
      fn to_image(&self) -> Image {
        let raw_data = self ;
        let height = raw_data.len();
        let width = if height > 0 {
            raw_data[0].len()
        } else {
            0
        };

        let mut modified_data = Vec::with_capacity(height * width * 4);

        for row in raw_data {
            for &value in row {
                // Duplicate the grayscale value for each channel (R, G, B, A)
                modified_data.extend_from_slice(&[value, value, value, 255]);
            }
        }

        let size = Extent3d {
            width: width as u32,
            height: height as u32,
            depth_or_array_layers: 1,
        };

        let dimension = TextureDimension::D2;

        Image::new(
            size,
            dimension,
            modified_data,
            TextureFormat::Rgba8UnormSrgb,  //??? 
            RenderAssetUsages::default(),
        )
    }



    //rgba8uint
   fn load_from_image(image: &Image) -> Result<Box<Self>, DensityMapError> {
       

         let width = image.size().x as usize;
        let height = image.size().y as usize;
        let format = image.texture_descriptor.format;

       if format!= TextureFormat::Rgba8Uint &&  format != TextureFormat::R8Uint && format != TextureFormat::Rgba8Unorm && format != TextureFormat::Rgba8UnormSrgb {
            println!("DensityMap: wrong format {:?}", format);
            return Err(DensityMapError::LoadingError);
        }

        let mut density_map = Vec::with_capacity(height);
          for y in 0..height {
           let mut row = Vec::with_capacity(width);
            
            for x in 0..width {
          
                let index = (y * width + x) * 4;
               
                row.push(image.data[index]  ); //only read the R channel 
            }
            density_map.push(row);
        }

        Ok(Box::new(density_map))


    }


 fn save_density_map_to_disk<P>(
     &self, // Adjusted for direct Vec<Vec<u16>> input
    save_file_path: P,
) where
    P: AsRef<Path>,
{
    let density_map_data = self ;

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



}