use engine_2d::animation::Animation;
use engine_2d::engine::window::{WindowEvent, WindowManager};
use engine_2d::render::simple_text::SimpleTextRenderer;
use engine_2d::render::sprite::SpriteRenderer;
use engine_2d::render::text::{Text, TextBox, BoxAnchor, TextAlign, VerticalAlign, TextWrap};
use glam::Vec2;
use glfw::{Action, Key};

const DEFAULT_FONT_PATH: &str = "assets/fonts/default.ttf";

/// Simple text rendering demo showcasing the new TextBox anchoring system
pub struct SimpleTextDemo {
    current_demo: usize,
    demos: Vec<&'static str>,
    last_action_states: std::collections::HashMap<Key, bool>,
    fonts_registered: bool,
}

impl SimpleTextDemo {
    pub fn new() -> Self {
        Self {
            current_demo: 0,
            demos: vec![
                "Basic Text",
                "Text Boxes",
                "9-Point Anchors",
                "Vertical Alignment",
                "Text Wrapping",
                "Mixed Examples",
            ],
            last_action_states: std::collections::HashMap::new(),
            fonts_registered: false,
        }
    }

    /// Register fonts with the text renderer
    fn register_fonts(&mut self, text_renderer: &mut SimpleTextRenderer) {
        if !self.fonts_registered {
            text_renderer
                .load_font("default", DEFAULT_FONT_PATH, 16)
                .unwrap_or_else(|e| println!("Warning: Failed to load font: {}", e));
            self.fonts_registered = true;
        }
    }

    fn render_demo(
        &self,
        text_renderer: &mut SimpleTextRenderer,
        demo_name: &str,
    ) -> Result<(), String> {
        let renderer = text_renderer.get_renderer_mut();

        match demo_name {
            "Basic Text" => {
                // Basic text without boxes (legacy mode)
                let mut title = Text::new(
                    "Text Box Demo".to_string(),
                    Vec2::new(0.5, 0.95),
                    "default".to_string(),
                );
                title.config.color = (1.0, 1.0, 0.0);
                title.config.align = TextAlign::Center;
                renderer.render_text(&title)?;

                let mut text1 = Text::new(
                    "Simple text without box".to_string(),
                    Vec2::new(0.1, 0.8),
                    "default".to_string(),
                );
                renderer.render_text(&text1)?;

                let mut text2 = Text::new(
                    "Another simple text".to_string(),
                    Vec2::new(0.1, 0.7),
                    "default".to_string(),
                );
                text2.config.color = (0.0, 1.0, 1.0);
                renderer.render_text(&text2)?;
            }
            "Text Boxes" => {
                // Demo title
                let mut title = Text::new(
                    "Text Box Examples".to_string(),
                    Vec2::new(0.5, 0.95),
                    "default".to_string(),
                );
                title.config.color = (1.0, 1.0, 0.0);
                title.config.align = TextAlign::Center;
                renderer.render_text(&title)?;

                // Box 1: Top-left anchor, small box
                let box1 = TextBox::with_anchor(
                    Vec2::new(0.1, 0.8),
                    0.25,
                    0.1,
                    BoxAnchor::TopLeft,
                );
                let mut text1 = Text::new(
                    "Top-Left Box".to_string(),
                    Vec2::new(0.0, 0.0), // Ignored when box is set
                    "default".to_string(),
                );
                text1.config.bounding_box = Some(box1);
                text1.config.color = (1.0, 0.0, 0.0);
                renderer.render_text(&text1)?;

                // Box 2: Center anchor, medium box
                let box2 = TextBox::with_anchor(
                    Vec2::new(0.5, 0.5),
                    0.4,
                    0.2,
                    BoxAnchor::MiddleCenter,
                );
                let mut text2 = Text::new(
                    "Centered Box".to_string(),
                    Vec2::new(0.0, 0.0),
                    "default".to_string(),
                );
                text2.config.bounding_box = Some(box2);
                text2.config.color = (0.0, 1.0, 0.0);
                text2.config.align = TextAlign::Center;
                text2.config.vertical_align = VerticalAlign::Middle;
                renderer.render_text(&text2)?;

                // Box 3: Top-right anchor
                let box3 = TextBox::with_anchor(
                    Vec2::new(0.9, 0.8),
                    0.25,
                    0.1,
                    BoxAnchor::TopRight,
                );
                let mut text3 = Text::new(
                    "Top-Right Box".to_string(),
                    Vec2::new(0.0, 0.0),
                    "default".to_string(),
                );
                text3.config.bounding_box = Some(box3);
                text3.config.color = (0.0, 0.0, 1.0);
                text3.config.align = TextAlign::Right;
                renderer.render_text(&text3)?;
            }
            "9-Point Anchors" => {
                // Demo title
                let mut title = Text::new(
                    "9-Point Anchor System".to_string(),
                    Vec2::new(0.5, 0.95),
                    "default".to_string(),
                );
                title.config.color = (1.0, 1.0, 0.0);
                title.config.align = TextAlign::Center;
                renderer.render_text(&title)?;

                // Draw reference crosshairs
                let positions = [
                    (0.2, 0.3, "TL"),
                    (0.5, 0.3, "TC"),
                    (0.8, 0.3, "TR"),
                    (0.2, 0.5, "ML"),
                    (0.5, 0.5, "MC"),
                    (0.8, 0.5, "MR"),
                    (0.2, 0.7, "BL"),
                    (0.5, 0.7, "BC"),
                    (0.8, 0.7, "BR"),
                ];

                let anchors = [
                    BoxAnchor::TopLeft,
                    BoxAnchor::TopCenter,
                    BoxAnchor::TopRight,
                    BoxAnchor::MiddleLeft,
                    BoxAnchor::MiddleCenter,
                    BoxAnchor::MiddleRight,
                    BoxAnchor::BottomLeft,
                    BoxAnchor::BottomCenter,
                    BoxAnchor::BottomRight,
                ];

                let colors = [
                    (1.0, 0.0, 0.0),
                    (1.0, 0.5, 0.0),
                    (1.0, 1.0, 0.0),
                    (0.0, 1.0, 0.0),
                    (0.0, 1.0, 1.0),
                    (0.0, 0.0, 1.0),
                    (0.5, 0.0, 1.0),
                    (1.0, 0.0, 1.0),
                    (1.0, 1.0, 1.0),
                ];

                for (i, ((x, y, label), (anchor, color))) in
                    positions.iter().zip(anchors.iter().zip(colors.iter())).enumerate()
                {
                    // Draw reference marker
                    let mut marker = Text::new(
                        "+".to_string(),
                        Vec2::new(*x, *y),
                        "default".to_string(),
                    );
                    marker.config.color = (1.0, 0.41, 0.71);
                    renderer.render_text(&marker)?;

                    // Draw text box with anchor
                    let text_box = TextBox::with_anchor(Vec2::new(*x, *y), 0.15, 0.08, *anchor);
                    let mut text = Text::new(label.to_string(), Vec2::new(0.0, 0.0), "default".to_string());
                    text.config.bounding_box = Some(text_box);
                    text.config.color = *color;
                    text.config.align = TextAlign::Center;
                    text.config.vertical_align = VerticalAlign::Middle;
                    renderer.render_text(&text)?;
                }
            }
            "Vertical Alignment" => {
                // Demo title
                let mut title = Text::new(
                    "Vertical Alignment".to_string(),
                    Vec2::new(0.5, 0.95),
                    "default".to_string(),
                );
                title.config.color = (1.0, 1.0, 0.0);
                title.config.align = TextAlign::Center;
                renderer.render_text(&title)?;

                let long_text = "This is a longer text that will demonstrate vertical alignment within boxes.";

                // Top aligned
                let box1 = TextBox::with_anchor(
                    Vec2::new(0.1, 0.7),
                    0.25,
                    0.15,
                    BoxAnchor::TopLeft,
                );
                let mut text1 = Text::new(long_text.to_string(), Vec2::new(0.0, 0.0), "default".to_string());
                text1.config.bounding_box = Some(box1);
                text1.config.color = (1.0, 0.0, 0.0);
                text1.config.vertical_align = VerticalAlign::Top;
                text1.config.wrap = TextWrap::Word;
                renderer.render_text(&text1)?;

                // Middle aligned
                let box2 = TextBox::with_anchor(
                    Vec2::new(0.4, 0.7),
                    0.25,
                    0.15,
                    BoxAnchor::TopLeft,
                );
                let mut text2 = Text::new(long_text.to_string(), Vec2::new(0.0, 0.0), "default".to_string());
                text2.config.bounding_box = Some(box2);
                text2.config.color = (0.0, 1.0, 0.0);
                text2.config.vertical_align = VerticalAlign::Middle;
                text2.config.wrap = TextWrap::Word;
                renderer.render_text(&text2)?;

                // Bottom aligned
                let box3 = TextBox::with_anchor(
                    Vec2::new(0.7, 0.7),
                    0.25,
                    0.15,
                    BoxAnchor::TopLeft,
                );
                let mut text3 = Text::new(long_text.to_string(), Vec2::new(0.0, 0.0), "default".to_string());
                text3.config.bounding_box = Some(box3);
                text3.config.color = (0.0, 0.0, 1.0);
                text3.config.vertical_align = VerticalAlign::Bottom;
                text3.config.wrap = TextWrap::Word;
                renderer.render_text(&text3)?;
            }
            "Text Wrapping" => {
                // Demo title
                let mut title = Text::new(
                    "Text Wrapping in Boxes".to_string(),
                    Vec2::new(0.5, 0.95),
                    "default".to_string(),
                );
                title.config.color = (1.0, 1.0, 0.0);
                title.config.align = TextAlign::Center;
                renderer.render_text(&title)?;

                let long_text = "This is a very long text that should wrap at word boundaries when it exceeds the maximum width of the bounding box. It demonstrates how text wrapping works in the engine.";

                // Word wrapping
                let box1 = TextBox::with_anchor(
                    Vec2::new(0.1, 0.7),
                    0.35,
                    0.2,
                    BoxAnchor::TopLeft,
                );
                let mut text1 = Text::new(long_text.to_string(), Vec2::new(0.0, 0.0), "default".to_string());
                text1.config.bounding_box = Some(box1);
                text1.config.color = (0.0, 1.0, 0.0);
                text1.config.wrap = TextWrap::Word;
                text1.config.vertical_align = VerticalAlign::Top;
                renderer.render_text(&text1)?;

                // Ellipsis truncation
                let box2 = TextBox::with_anchor(
                    Vec2::new(0.55, 0.7),
                    0.35,
                    0.2,
                    BoxAnchor::TopLeft,
                );
                let mut text2 = Text::new(long_text.to_string(), Vec2::new(0.0, 0.0), "default".to_string());
                text2.config.bounding_box = Some(box2);
                text2.config.color = (1.0, 0.0, 0.0);
                text2.config.wrap = TextWrap::Ellipsis;
                text2.config.vertical_align = VerticalAlign::Top;
                renderer.render_text(&text2)?;
            }
            "Mixed Examples" => {
                // Demo title
                let mut title = Text::new(
                    "Mixed Examples".to_string(),
                    Vec2::new(0.5, 0.95),
                    "default".to_string(),
                );
                title.config.color = (1.0, 1.0, 0.0);
                title.config.align = TextAlign::Center;
                renderer.render_text(&title)?;

                // UI panel example (top-left)
                let panel_box = TextBox::with_padding(
                    Vec2::new(0.05, 0.15),
                    0.4,
                    0.3,
                    (0.02, 0.02, 0.02, 0.02), // padding: left, right, top, bottom
                );
                let mut panel_title = Text::new(
                    "Game Status".to_string(),
                    Vec2::new(0.0, 0.0),
                    "default".to_string(),
                );
                panel_title.config.bounding_box = Some(panel_box);
                panel_title.config.color = (1.0, 1.0, 1.0);
                panel_title.config.align = TextAlign::Center;
                panel_title.config.vertical_align = VerticalAlign::Top;
                renderer.render_text(&panel_title)?;

                // Status items
                let status_text = "✓ Player Connected\nℹ Score: 1,250 points\n⚠ Low Health: 25%";
                let status_box = TextBox::with_padding(
                    Vec2::new(0.05, 0.15),
                    0.4,
                    0.3,
                    (0.04, 0.02, 0.06, 0.02),
                );
                let mut status = Text::new(status_text.to_string(), Vec2::new(0.0, 0.0), "default".to_string());
                status.config.bounding_box = Some(status_box);
                status.config.color = (0.0, 1.0, 0.0);
                status.config.vertical_align = VerticalAlign::Top;
                renderer.render_text(&status)?;

                // Right-aligned info box
                let info_box = TextBox::with_anchor(
                    Vec2::new(0.95, 0.3),
                    0.3,
                    0.15,
                    BoxAnchor::TopRight,
                );
                let mut info = Text::new(
                    "Press SPACE for next demo\nPress ESC to exit".to_string(),
                    Vec2::new(0.0, 0.0),
                    "default".to_string(),
                );
                info.config.bounding_box = Some(info_box);
                info.config.color = (1.0, 1.0, 0.0);
                info.config.align = TextAlign::Right;
                info.config.vertical_align = VerticalAlign::Top;
                renderer.render_text(&info)?;

                // Centered dialog box
                let dialog_box = TextBox::with_anchor(
                    Vec2::new(0.5, 0.6),
                    0.5,
                    0.2,
                    BoxAnchor::MiddleCenter,
                );
                let mut dialog = Text::new(
                    "Welcome to the Text Box Demo!\nThis demonstrates the new anchoring system.".to_string(),
                    Vec2::new(0.0, 0.0),
                    "default".to_string(),
                );
                dialog.config.bounding_box = Some(dialog_box);
                dialog.config.color = (1.0, 1.0, 1.0);
                dialog.config.align = TextAlign::Center;
                dialog.config.vertical_align = VerticalAlign::Middle;
                dialog.config.wrap = TextWrap::Word;
                renderer.render_text(&dialog)?;
            }
            _ => {}
        }

        Ok(())
    }
}

impl Animation for SimpleTextDemo {
    fn update(
        &mut self,
        _sprite_renderer: Option<&mut SpriteRenderer>,
        elapsed_time: f32,
        _delta_time: f32,
        _window_manager: Option<&mut WindowManager>,
        text_renderer: Option<&mut SimpleTextRenderer>,
    ) {
        // Render current demo
        if let Some(tr) = text_renderer {
            // Register fonts if not already registered
            self.register_fonts(tr);

            // Render the current demo
            if let Err(e) = self.render_demo(tr, self.demos[self.current_demo]) {
                println!("Error rendering demo: {}", e);
            }

            // Show demo info (top-left) using a text box
            let renderer = tr.get_renderer_mut();
            let demo_info = format!(
                "Demo {} of {}: {}",
                self.current_demo + 1,
                self.demos.len(),
                self.demos[self.current_demo]
            );
            let info_box = TextBox::with_anchor(
                Vec2::new(0.02, 0.98),
                0.4,
                0.05,
                BoxAnchor::TopLeft,
            );
            let mut info_text = Text::new(demo_info, Vec2::new(0.0, 0.0), "default".to_string());
            info_text.config.bounding_box = Some(info_box);
            info_text.config.color = (1.0, 1.0, 1.0);
            info_text.config.vertical_align = VerticalAlign::Top;
            let _ = renderer.render_text(&info_text);

            // Show controls (bottom-left) using a text box
            let controls_box = TextBox::with_anchor(
                Vec2::new(0.02, 0.05),
                0.5,
                0.05,
                BoxAnchor::BottomLeft,
            );
            let mut controls_text = Text::new(
                "SPACE=Next | BACKSPACE=Prev | ESC=Exit".to_string(),
                Vec2::new(0.0, 0.0),
                "default".to_string(),
            );
            controls_text.config.bounding_box = Some(controls_box);
            controls_text.config.color = (1.0, 1.0, 1.0);
            controls_text.config.vertical_align = VerticalAlign::Bottom;
            let _ = renderer.render_text(&controls_text);
        }

        // Print demo info every 5 seconds
        if (elapsed_time as u32) % 5 == 0 && (elapsed_time * 1000.0) as u32 % 5000 < 16 {
            println!(
                "Current Demo: {} ({})",
                self.demos[self.current_demo],
                self.current_demo + 1
            );
        }
    }

    fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::Glfw(glfw::WindowEvent::Key(key, _scancode, action, _mods)) => {
                let was_pressed = self.last_action_states.get(key).copied().unwrap_or(false);
                let is_pressed = *action == Action::Press || *action == Action::Repeat;

                if is_pressed && !was_pressed {
                    match key {
                        Key::Space => {
                            self.current_demo = (self.current_demo + 1) % self.demos.len();
                        }
                        Key::Backspace => {
                            self.current_demo = if self.current_demo == 0 {
                                self.demos.len() - 1
                            } else {
                                self.current_demo - 1
                            };
                        }
                        Key::Escape => {
                            // Exit will be handled by the engine
                        }
                        _ => {}
                    }
                }

                self.last_action_states.insert(*key, is_pressed);
            }
            _ => {}
        }
    }

    fn name(&self) -> &str {
        "Simple Text Demo"
    }
}

fn main() {
    let config = engine_2d::engine::config::EngineConfig {
        window_width: 1024,
        window_height: 768,
        window_title: "Text Box Demo".to_string(),
        target_fps: Some(60),
        show_fps: false,
        vsync: true,
        fullscreen: false,
        // Configure viewport for UI coordinates (0 to 1, 0 to 1)
        viewport: engine_2d::engine::config::ViewportConfig::ui_based(),
        fallback_font_path: DEFAULT_FONT_PATH.to_string(),
    };

    let animation = Box::new(SimpleTextDemo::new());

    match engine_2d::engine::core::Engine::new_with_config_and_animation(config, animation) {
        Ok(mut engine) => {
            println!("Text Box Demo");
            println!("=============");
            println!("Controls:");
            println!("  SPACE     - Next Demo");
            println!("  BACKSPACE - Previous Demo");
            println!("  ESC       - Exit");
            println!();

            if let Err(e) = engine.run() {
                eprintln!("Engine error: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to create engine: {}", e);
        }
    }
}
