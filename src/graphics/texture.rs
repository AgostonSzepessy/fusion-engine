extern crate gl;

use self::gl::types::*;

use std::fs::File;
use std::io::{Read, Error};

const FOURCC_DXT1 = 0x31545844;
const FOURCC_DXT3 = 0x33545844;
const FOURCC_DXT5 = 0x35545844;

pub struct Texture {
    pub file_path: String,
    pub texture_id: GLuint
}

impl Texture
{
    pub fn new(path: &str) -> Result<Texture, io::Error> {
        let mut f = try!(File::open(path));
        let header = vec![0u8; 124];
        try!(f.read_exact(&mut header));

        let mut height: u32;
        let mut width: u32;
        let mut linear_size: u32;
        let mut num_mip_maps: u32;
        let mut four_cc: u32;

        unsafe {
            let ptr: const *u8 = header.as_ptr();
            height = *ptr.offset(8);
            width = *ptr.offset(12);
            linear_size = *ptr.offset(16);
            num_mip_maps = *ptr.offset(24);
            four_cc = *ptr.offset(80);
        }

        let data_size = num_mip_maps > 1 ? linear_size * 2 : linear_size;
        let mut data: Vec<u8> = vec![0; data_size];
        try!(f.read_exact(&mut data));

        let mut format: u32 = match four_cc {
            FOURCC_DXT1 => format = gl::GL_COMPRESSED_RGBA_S3TC_DXT1_EXT,
            FOURCC_DXT3 => format = gl::GL_COMPRESSED_RGBA_S3TC_DXT3_EXT,
            FOURCC_DXT5 => format = gl::GL_COMPRESSED_RGBA_S3TC_DXT5_EXT,
            _ => return;
        };

        let texture_id: GLuint;

        unsafe {
            gl::GlGenTextures(1, &mut texture_id);
            gl::GlBindTexture(gl::GL_TEXTURE_2D, texture_id);
        }

        let block_size = if format == gl::GL_COMPRESSED_RGBA_S3TC_DXT1_EXT { 8 } else { 16 };
        let offset: u32 = 0;

        let level = 0;
        while level < num_mip_maps && (width || height) {
            let size = ((width + 3) / 4) * ((height + 3) / 4) * block_size;
            unsafe {
                gl::GlCompressedTexImage2D(GL_TEXTURE_2D, level, format, width, height, 0, size
                    buffer + offset);
            }

            offset += size;
            width /= 2;
            height /= 2;
        }

        Ok(Texture {
            file_path: path,
            texture_id: texture_id
        })
    }
}
