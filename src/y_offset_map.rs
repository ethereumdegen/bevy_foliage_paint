use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum YOffsetMapError {
    #[error("failed to load the image")]
    LoadingError,
}

pub type YOffsetMapU16 = Vec<Vec<u16>>;

pub trait YOffsetMap {
    fn load_from_image(image: &Image) -> Result<Box<Self>, YOffsetMapError>;
    fn to_image(&self) -> Image;
}

impl YOffsetMap for YOffsetMapU16 {
    //this expects data to be stored [y][x]
    //rgba16uint
    fn to_image(&self) -> Image {
        let raw_data = self;
        let height = raw_data.len();
        let width = if height > 0 {
            raw_data[0].len()
        } else {
            0
        };

        let mut modified_data = Vec::with_capacity(height * width * 4 * 2);
        for row in raw_data {
            for &value in row {
                // Duplicate the grayscale value for each channel (R, G, B, A)
                modified_data.extend_from_slice(&value.to_le_bytes());
                modified_data.extend_from_slice(&value.to_le_bytes());
                modified_data.extend_from_slice(&value.to_le_bytes());
                modified_data.extend_from_slice(&u16::MAX.to_le_bytes());
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
            TextureFormat::Rgba16Unorm,
            RenderAssetUsages::default(),
        )
    }

    //rgba16uint
    fn load_from_image(image: &Image) -> Result<Box<Self>, YOffsetMapError> {
      
        let width = image.size().x as usize;
        let height = image.size().y as usize;

        let format = image.texture_descriptor.format;

        if format != TextureFormat::R16Uint {
            println!("heightmap: wrong format {:?}", format);
            return Err(YOffsetMapError::LoadingError);
        }

        //maybe somehow fail if the format is not R16uint

        // With the format being R16Uint, each pixel is represented by 2 bytes
        let mut height_map = Vec::with_capacity(height);

        for y in 0..height {
            let mut row = Vec::with_capacity(width);
            for x in 0..width {
                let index = 2 * (y * width + x); // 2 because of R16Uint
                let height_value = u16::from_le_bytes([image.data[index], image.data[index + 1]]);
                row.push(height_value);
            }
            height_map.push(row);
        }

        Ok(Box::new(height_map))
    
    }
}