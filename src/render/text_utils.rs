use crate::render::text::{Text, TextAlign};
use glam::Vec2;

/// Utility functions for creating common text configurations
pub struct TextUtils;

impl TextUtils {
    /// Create a simple text object with default settings
    pub fn simple_text(text: &str, position: Vec2, font_name: &str) -> Text {
        Text::new(text.to_string(), position, font_name.to_string())
    }

    /// Create a title text object (large, centered, yellow)
    pub fn title_text(text: &str, position: Vec2, font_name: &str) -> Text {
        let mut text_obj = Text::new(text.to_string(), position, font_name.to_string());
        text_obj.config.font_size = 24;
        text_obj.config.color = (1.0, 1.0, 0.0); // Yellow
        text_obj.config.align = TextAlign::Center;
        text_obj
    }

    /// Create a subtitle text object (medium, centered)
    pub fn subtitle_text(text: &str, position: Vec2, font_name: &str) -> Text {
        let mut text_obj = Text::new(text.to_string(), position, font_name.to_string());
        text_obj.config.font_size = 18;
        text_obj.config.align = TextAlign::Center;
        text_obj
    }

    /// Create info text (blue color)
    pub fn info_text(text: &str, position: Vec2, font_name: &str) -> Text {
        Self::colored_text(text, position, font_name, (0.0, 0.5, 1.0))
    }

    /// Create warning text (orange color)
    pub fn warning_text(text: &str, position: Vec2, font_name: &str) -> Text {
        Self::colored_text(text, position, font_name, (1.0, 0.5, 0.0))
    }

    /// Create error text (red color)
    pub fn error_text(text: &str, position: Vec2, font_name: &str) -> Text {
        Self::colored_text(text, position, font_name, (1.0, 0.0, 0.0))
    }

    /// Create colored text with custom color
    pub fn colored_text(text: &str, position: Vec2, font_name: &str, color: (f32, f32, f32)) -> Text {
        let mut text_obj = Text::new(text.to_string(), position, font_name.to_string());
        text_obj.config.color = color;
        text_obj
    }
}

