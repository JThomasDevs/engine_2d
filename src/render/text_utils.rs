use super::text::{Text, TextConfig, TextAlign};
use glam::Vec2;

/// Utility functions for text rendering
pub struct TextUtils;

impl TextUtils {
    /// Create a simple text object with default settings
    pub fn simple_text(content: &str, position: Vec2, font_name: &str) -> Text {
        Text::new(content.to_string(), position, font_name.to_string())
    }

    /// Create text with a specific color
    pub fn colored_text(content: &str, position: Vec2, font_name: &str, color: (f32, f32, f32)) -> Text {
        let mut text = Text::new(content.to_string(), position, font_name.to_string());
        text.set_color(color);
        text
    }

    /// Create centered text
    pub fn centered_text(content: &str, position: Vec2, font_name: &str) -> Text {
        let mut text = Text::new(content.to_string(), position, font_name.to_string());
        text.set_align(TextAlign::Center);
        text
    }

    /// Create right-aligned text
    pub fn right_aligned_text(content: &str, position: Vec2, font_name: &str) -> Text {
        let mut text = Text::new(content.to_string(), position, font_name.to_string());
        text.set_align(TextAlign::Right);
        text
    }

    /// Create text with custom configuration
    pub fn custom_text(content: &str, position: Vec2, font_name: &str, config: TextConfig) -> Text {
        Text::with_config(content.to_string(), position, font_name.to_string(), config)
    }

    /// Create a title text (large, centered, bold color)
    pub fn title_text(content: &str, position: Vec2, font_name: &str) -> Text {
        let mut config = TextConfig::default();
        config.font_size = 24;
        config.color = (1.0, 1.0, 0.0); // Yellow
        config.align = TextAlign::Center;
        
        Text::with_config(content.to_string(), position, font_name.to_string(), config)
    }

    /// Create a subtitle text (medium, centered)
    pub fn subtitle_text(content: &str, position: Vec2, font_name: &str) -> Text {
        let mut config = TextConfig::default();
        config.font_size = 18;
        config.color = (0.8, 0.8, 0.8); // Light gray
        config.align = TextAlign::Center;
        
        Text::with_config(content.to_string(), position, font_name.to_string(), config)
    }

    /// Create a label text (small, left-aligned)
    pub fn label_text(content: &str, position: Vec2, font_name: &str) -> Text {
        let mut config = TextConfig::default();
        config.font_size = 12;
        config.color = (0.7, 0.7, 0.7); // Gray
        config.align = TextAlign::Left;
        
        Text::with_config(content.to_string(), position, font_name.to_string(), config)
    }

    /// Create a warning text (orange color)
    pub fn warning_text(content: &str, position: Vec2, font_name: &str) -> Text {
        let mut text = Text::new(content.to_string(), position, font_name.to_string());
        text.set_color((1.0, 0.6, 0.0)); // Orange
        text
    }

    /// Create an error text (red color)
    pub fn error_text(content: &str, position: Vec2, font_name: &str) -> Text {
        let mut text = Text::new(content.to_string(), position, font_name.to_string());
        text.set_color((1.0, 0.2, 0.2)); // Red
        text
    }

    /// Create a success text (green color)
    pub fn success_text(content: &str, position: Vec2, font_name: &str) -> Text {
        let mut text = Text::new(content.to_string(), position, font_name.to_string());
        text.set_color((0.2, 1.0, 0.2)); // Green
        text
    }

    /// Create an info text (blue color)
    pub fn info_text(content: &str, position: Vec2, font_name: &str) -> Text {
        let mut text = Text::new(content.to_string(), position, font_name.to_string());
        text.set_color((0.2, 0.6, 1.0)); // Blue
        text
    }

    /// Create a debug text (small, monospace-like)
    pub fn debug_text(content: &str, position: Vec2, font_name: &str) -> Text {
        let mut config = TextConfig::default();
        config.font_size = 10;
        config.color = (0.5, 0.5, 0.5); // Dark gray
        config.align = TextAlign::Left;
        
        Text::with_config(content.to_string(), position, font_name.to_string(), config)
    }

    /// Create a button text (medium, centered, white)
    pub fn button_text(content: &str, position: Vec2, font_name: &str) -> Text {
        let mut config = TextConfig::default();
        config.font_size = 16;
        config.color = (1.0, 1.0, 1.0); // White
        config.align = TextAlign::Center;
        
        Text::with_config(content.to_string(), position, font_name.to_string(), config)
    }

    /// Create a multiline text with proper line spacing
    pub fn multiline_text(content: &str, position: Vec2, font_name: &str, line_spacing: f32) -> Text {
        let mut config = TextConfig::default();
        config.line_spacing = line_spacing;
        
        Text::with_config(content.to_string(), position, font_name.to_string(), config)
    }

    /// Create text with transparency
    pub fn transparent_text(content: &str, position: Vec2, font_name: &str, alpha: f32) -> Text {
        let mut text = Text::new(content.to_string(), position, font_name.to_string());
        text.set_alpha(alpha);
        text
    }

    /// Create a text with a specific font size
    pub fn sized_text(content: &str, position: Vec2, font_name: &str, font_size: u32) -> Text {
        let mut config = TextConfig::default();
        config.font_size = font_size;
        
        Text::with_config(content.to_string(), position, font_name.to_string(), config)
    }
}

/// Common color constants for text
pub mod colors {
    pub const WHITE: (f32, f32, f32) = (1.0, 1.0, 1.0);
    pub const BLACK: (f32, f32, f32) = (0.0, 0.0, 0.0);
    pub const RED: (f32, f32, f32) = (1.0, 0.0, 0.0);
    pub const GREEN: (f32, f32, f32) = (0.0, 1.0, 0.0);
    pub const BLUE: (f32, f32, f32) = (0.0, 0.0, 1.0);
    pub const YELLOW: (f32, f32, f32) = (1.0, 1.0, 0.0);
    pub const CYAN: (f32, f32, f32) = (0.0, 1.0, 1.0);
    pub const MAGENTA: (f32, f32, f32) = (1.0, 0.0, 1.0);
    pub const ORANGE: (f32, f32, f32) = (1.0, 0.6, 0.0);
    pub const PURPLE: (f32, f32, f32) = (0.6, 0.0, 1.0);
    pub const PINK: (f32, f32, f32) = (1.0, 0.4, 0.8);
    pub const GRAY: (f32, f32, f32) = (0.5, 0.5, 0.5);
    pub const LIGHT_GRAY: (f32, f32, f32) = (0.8, 0.8, 0.8);
    pub const DARK_GRAY: (f32, f32, f32) = (0.2, 0.2, 0.2);
}

/// Common font sizes
pub mod sizes {
    pub const TINY: u32 = 10;
    pub const SMALL: u32 = 12;
    pub const NORMAL: u32 = 16;
    pub const MEDIUM: u32 = 20;
    pub const LARGE: u32 = 24;
    pub const HUGE: u32 = 32;
    pub const MASSIVE: u32 = 48;
}
