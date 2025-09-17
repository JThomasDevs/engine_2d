pub mod math;
pub mod resource;

#[cfg(test)]
mod tests {

    #[test]
    fn test_math_module_availability() {
        // Test that math functions are accessible
        use crate::utils::math::vector;
        use crate::utils::math::utils;
        use glam::Vec2;
        
        let v = Vec2::new(3.0, 4.0);
        assert!((vector::magnitude(v) - 5.0).abs() < 1e-6);
        
        let clamped = utils::clamp(150.0, 0.0, 100.0);
        assert!((clamped - 100.0).abs() < 1e-6);
    }

    #[test]
    fn test_math_module_interpolation() {
        use crate::utils::math::interpolation;
        
        // Test linear interpolation
        let result = interpolation::lerp(0.0, 100.0, 0.5);
        assert!((result - 50.0).abs() < 1e-6);
        
        // Test easing functions
        assert!((interpolation::ease_in(0.0) - 0.0).abs() < 1e-6);
        assert!((interpolation::ease_in(1.0) - 1.0).abs() < 1e-6);
        assert!(interpolation::ease_in(0.5) < 0.5);
    }

    #[test]
    fn test_math_module_geometry() {
        use crate::utils::math::geometry;
        use glam::Vec2;
        
        // Test rectangle creation and operations
        let rect = geometry::Rectangle::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
        assert!(rect.contains_point(Vec2::new(5.0, 5.0)));
        assert!(!rect.contains_point(Vec2::new(15.0, 5.0)));
        
        // Test circle creation and operations
        let circle = geometry::Circle::new(Vec2::new(0.0, 0.0), 5.0);
        assert!(circle.contains_point(Vec2::new(3.0, 4.0)));
        assert!(!circle.contains_point(Vec2::new(6.0, 0.0)));
    }

    #[test]
    fn test_math_module_physics() {
        use crate::utils::math::physics;
        use glam::Vec2;
        
        // Test velocity calculation
        let initial = Vec2::new(0.0, 0.0);
        let final_pos = Vec2::new(10.0, 5.0);
        let velocity = physics::calculate_velocity(initial, final_pos, 1.0);
        assert!((velocity.x - 10.0).abs() < 1e-6);
        assert!((velocity.y - 5.0).abs() < 1e-6);
        
        // Test friction
        let velocity = Vec2::new(10.0, 0.0);
        let friction_velocity = physics::apply_friction(velocity, 0.5, 0.1);
        assert!(friction_velocity.length() < velocity.length());
    }

    #[test]
    fn test_math_module_random() {
        use crate::utils::math::random;
        
        // Test random number generation
        random::init_global(12345);
        let val1 = random::random();
        random::init_global(12345);
        let val2 = random::random();
        assert!((val1 - val2).abs() < 1e-6);
        
        // Test range generation
        for _ in 0..10 {
            let val = random::range(10.0, 20.0);
            assert!(val >= 10.0 && val <= 20.0);
        }
    }

}
