use std::rc::Rc;
use super::gl_wrapper::GlWrapper;
use image::{ImageBuffer, RgbaImage};
use std::collections::HashMap;
use std::path::Path;

/// A texture handle that can be used for rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureId(pub u32);

/// Texture information
#[derive(Debug, Clone)]
pub struct TextureInfo {
    pub id: TextureId,
    pub width: u32,
    pub height: u32,
}

/// Texture manager that handles loading and managing textures
pub struct TextureManager {
    gl: Rc<GlWrapper>,
    textures: HashMap<String, TextureInfo>,
}

impl TextureManager {
    /// Create a new texture manager
    pub fn new(gl: Rc<GlWrapper>) -> Self {
        Self {
            gl,
            textures: HashMap::new(),
        }
    }

    /// Load a texture from a file path
    pub fn load_texture(&mut self, path: &str) -> Result<TextureId, String> {
        // Check if texture is already loaded
        if let Some(texture_info) = self.textures.get(path) {
            return Ok(texture_info.id);
        }

        // Load image from file
        let img = image::open(Path::new(path))
            .map_err(|e| format!("Failed to load image '{}': {}", path, e))?;

        // Convert to RGBA format
        let rgba_img = img.to_rgba8();
        let (width, height) = rgba_img.dimensions();

        // Generate OpenGL texture
        let texture_id = self.create_texture_from_image(&rgba_img)?;

        let texture_info = TextureInfo {
            id: TextureId(texture_id),
            width,
            height,
        };

        // Store texture info
        self.textures.insert(path.to_string(), texture_info.clone());

        Ok(texture_info.id)
    }

    /// Create a texture from image data
    pub fn create_texture_from_image(&mut self, img: &RgbaImage) -> Result<u32, String> {
        let (width, height) = img.dimensions();
        
        // Generate OpenGL texture ID
        let texture_id = self.gl.gen_texture()?;
        self.gl.bind_texture(0x0DE1, texture_id)?; // GL_TEXTURE_2D

        // Set texture parameters
        self.gl.tex_parameter_i(0x0DE1, 0x2800, 0x2601)?; // GL_TEXTURE_MIN_FILTER, GL_LINEAR
        self.gl.tex_parameter_i(0x0DE1, 0x2801, 0x2601)?; // GL_TEXTURE_MAG_FILTER, GL_LINEAR
        self.gl.tex_parameter_i(0x0DE1, 0x2802, 0x2901)?; // GL_TEXTURE_WRAP_S, GL_REPEAT
        self.gl.tex_parameter_i(0x0DE1, 0x2803, 0x2901)?; // GL_TEXTURE_WRAP_T, GL_REPEAT

        // Upload texture data
        self.gl.tex_image_2d(
            0x0DE1, // GL_TEXTURE_2D
            0,      // level
            0x1908, // GL_RGBA
            width as i32,
            height as i32,
            0,      // border
            0x1908, // GL_RGBA
            0x1401, // GL_UNSIGNED_BYTE
            Some(img.as_raw())
        )?;

        // Unbind texture
        self.gl.bind_texture(0x0DE1, 0)?;

        Ok(texture_id)
    }

    /// Create a texture from raw pixel data
    pub fn create_texture_from_data(&mut self, width: u32, height: u32, data: &[u8]) -> Result<TextureId, String> {
        // Generate OpenGL texture ID
        let texture_id = self.gl.gen_texture()?;
        self.gl.bind_texture(0x0DE1, texture_id)?; // GL_TEXTURE_2D
        
        // Set pixel alignment to 1 byte for any texture width
        self.gl.pixel_store_i(0x0CF5, 1)?; // GL_UNPACK_ALIGNMENT, 1
        
        // Set texture parameters for high-quality font rendering
        self.gl.tex_parameter_i(0x0DE1, 0x2800, 0x2703)?; // GL_TEXTURE_MIN_FILTER, GL_LINEAR_MIPMAP_LINEAR
        self.gl.tex_parameter_i(0x0DE1, 0x2801, 0x2601)?; // GL_TEXTURE_MAG_FILTER, GL_LINEAR
        self.gl.tex_parameter_i(0x0DE1, 0x2802, 0x812F)?; // GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE
        self.gl.tex_parameter_i(0x0DE1, 0x2803, 0x812F)?; // GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE
        
        // Upload texture data
        self.gl.tex_image_2d(
            0x0DE1, 0, 0x1908, // GL_TEXTURE_2D, level, GL_RGBA
            width as i32, height as i32, 0,
            0x1908, 0x1401, // GL_RGBA, GL_UNSIGNED_BYTE
            Some(data)
        )?;
        
        // Generate mipmaps for better quality at different scales
        self.gl.generate_mipmap(0x0DE1)?; // GL_TEXTURE_2D
        
        let texture_info = TextureInfo {
            id: TextureId(texture_id),
            width,
            height,
        };
        
        // Store with a unique name
        let name = format!("data_texture_{}", texture_id);
        self.textures.insert(name, texture_info.clone());
        
        Ok(texture_info.id)
    }

    /// Create a solid color texture
    pub fn create_color_texture(&mut self, width: u32, height: u32, color: (u8, u8, u8, u8)) -> Result<TextureId, String> {
        // Create image data
        let mut img_data = Vec::with_capacity((width * height * 4) as usize);
        for _ in 0..(width * height) {
            img_data.push(color.0);
            img_data.push(color.1);
            img_data.push(color.2);
            img_data.push(color.3);
        }

        // Create image buffer
        let img = ImageBuffer::<image::Rgba<u8>, Vec<u8>>::from_raw(width, height, img_data)
            .ok_or("Failed to create image buffer")?;

        // Generate texture
        let texture_id = self.create_texture_from_image(&img)?;

        let texture_info = TextureInfo {
            id: TextureId(texture_id),
            width,
            height,
        };

        // Store with a unique name
        let name = format!("color_texture_{}", texture_id);
        self.textures.insert(name, texture_info.clone());

        Ok(texture_info.id)
    }

    /// Get texture information by ID
    pub fn get_texture_info(&self, texture_id: TextureId) -> Option<&TextureInfo> {
        self.textures.values().find(|info| info.id == texture_id)
    }

    /// Get texture information by path
    pub fn get_texture_info_by_path(&self, path: &str) -> Option<&TextureInfo> {
        self.textures.get(path)
    }

    /// Bind a texture for rendering
    pub fn bind_texture(&self, texture_id: TextureId) -> Result<(), String> {
        self.gl.bind_texture(0x0DE1, texture_id.0)?; // GL_TEXTURE_2D
        Ok(())
    }

    /// Unbind current texture
    pub fn unbind_texture(&self) -> Result<(), String> {
        self.gl.bind_texture(0x0DE1, 0)?; // GL_TEXTURE_2D
        Ok(())
    }

    /// Delete a texture
    pub fn delete_texture(&mut self, texture_id: TextureId) -> Result<(), String> {
        self.gl.delete_texture(texture_id.0)?;
        
        // Remove from hashmap
        self.textures.retain(|_, info| info.id != texture_id);
        
        Ok(())
    }

    /// Get all loaded texture paths
    pub fn get_loaded_textures(&self) -> Vec<String> {
        self.textures.keys().cloned().collect()
    }

    /// Clear all textures
    pub fn clear_all(&mut self) -> Result<(), String> {
        for (_, texture_info) in &self.textures {
            let _ = self.gl.delete_texture(texture_info.id.0);
        }
        self.textures.clear();
        Ok(())
    }
}

impl Drop for TextureManager {
    fn drop(&mut self) {
        let _ = self.clear_all();
    }
}
