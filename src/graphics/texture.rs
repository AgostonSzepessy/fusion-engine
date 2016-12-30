extern crate gl;

use self::gl::types::*;

use std::fs::File;
use std::io::Read;
use std::io;
use std::os;

const FOURCC_DXT1: u32 = 0x31545844;
const FOURCC_DXT3: u32 = 0x33545844;
const FOURCC_DXT5: u32 = 0x35545844;

const GL_COMPRESSED_RGB_S3TC_DXT1_EXT: u32 = 0x83F0;
const GL_COMPRESSED_RGBA_S3TC_DXT1_EXT: u32 = 0x83F1;
const GL_COMPRESSED_RGBA_S3TC_DXT3_EXT: u32 = 0x83F2;
const GL_COMPRESSED_RGBA_S3TC_DXT5_EXT: u32 = 0x83F3;

pub struct Texture {
    pub texture_id: GLuint,
    pub width: i32,
    pub height: i32
}

impl Texture
{
    pub fn new(path: &str) -> Result<Texture, io::Error> {
        let mut f = try!(File::open(path));
        let mut header = vec![0u8; 124];
        try!(f.read_exact(&mut header));

        let mut height: i32;
        let mut width: i32;
        let mut linear_size: u32;
        let mut num_mip_maps: u32;
        let mut four_cc: u32;

        unsafe {
            let ptr: *const u8 = header.as_ptr();
            height = *ptr.offset(8) as i32;
            width = *ptr.offset(12) as i32;
            linear_size = *ptr.offset(16) as u32;
            num_mip_maps = *ptr.offset(24) as u32;
            four_cc = *ptr.offset(80) as u32;
        }

        let data_size = if num_mip_maps > 1 { linear_size * 2 } else { linear_size };
        let mut data: Vec<u8> = vec![0; data_size as usize];
        try!(f.read_exact(&mut data));

        let mut format: u32 = match four_cc {
            FOURCC_DXT1 => GL_COMPRESSED_RGBA_S3TC_DXT1_EXT,
            FOURCC_DXT3 => GL_COMPRESSED_RGBA_S3TC_DXT3_EXT,
            FOURCC_DXT5 => GL_COMPRESSED_RGBA_S3TC_DXT5_EXT,
            _ => GL_COMPRESSED_RGBA_S3TC_DXT1_EXT
        };

        let mut texture_id: GLuint = 0;

        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
        }

        let block_size = if format == GL_COMPRESSED_RGBA_S3TC_DXT1_EXT { 8 } else { 16 };
        let mut offset: u32 = 0;

        let level = 0i32;

        let buffer;
        unsafe {
            buffer = data.as_ptr();
        }

        while level < num_mip_maps as i32 && (width > 0 || height > 0) {
            let size = ((width + 3) / 4) * ((height + 3) / 4) * block_size;
            unsafe {
                gl::CompressedTexImage2D(gl::TEXTURE_2D, level, format, width, height, 0, size,
                    buffer.offset(offset as isize) as *const os::raw::c_void);
            }

            offset += size as u32;
            width /= 2;
            height /= 2;
        }

        Ok(Texture {
            texture_id: texture_id,
            width: width,
            height: height
        })
    }
}
