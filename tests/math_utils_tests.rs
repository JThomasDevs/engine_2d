use engine_2d::utils::math::*;
use glam::Vec2;
use std::f32::consts::PI;

#[test]
fn test_vector_magnitude() {
    let v = Vec2::new(3.0, 4.0);
    assert!((vector::magnitude(v) - 5.0).abs() < 1e-6);
}

#[test]
fn test_vector_dot_product() {
    let a = Vec2::new(1.0, 2.0);
    let b = Vec2::new(3.0, 4.0);
    assert!((vector::dot(a, b) - 11.0).abs() < 1e-6);
}

#[test]
fn test_vector_cross_product() {
    let a = Vec2::new(1.0, 0.0);
    let b = Vec2::new(0.0, 1.0);
    assert!((vector::cross(a, b) - 1.0).abs() < 1e-6);
}

#[test]
fn test_vector_rotation() {
    let v = Vec2::new(1.0, 0.0);
    let rotated = vector::rotate(v, PI / 2.0);
    assert!((rotated.x - 0.0).abs() < 1e-6);
    assert!((rotated.y - 1.0).abs() < 1e-6);
}

#[test]
fn test_vector_reflection() {
    let incident = Vec2::new(1.0, -1.0);
    let normal = Vec2::new(0.0, 1.0);
    let reflected = vector::reflect(incident, normal);
    assert!((reflected.x - 1.0).abs() < 1e-6);
    assert!((reflected.y - 1.0).abs() < 1e-6);
}

#[test]
fn test_matrix_2x2_rotation() {
    let mat = matrix::Mat2::rotation(PI / 2.0);
    let v = Vec2::new(1.0, 0.0);
    let result = mat.transform_vector(v);
    assert!((result.x - 0.0).abs() < 1e-6);
    assert!((result.y - 1.0).abs() < 1e-6);
}

#[test]
fn test_matrix_2x2_determinant() {
    let mat = matrix::Mat2::rotation(PI / 4.0);
    assert!((mat.determinant() - 1.0).abs() < 1e-6);
}

#[test]
fn test_matrix_3x3_translation() {
    let mat = matrix::Mat3::translation(Vec2::new(5.0, 10.0));
    let v = Vec2::new(1.0, 2.0);
    let result = mat.transform_vector(v);
    assert!((result.x - 6.0).abs() < 1e-6);
    assert!((result.y - 12.0).abs() < 1e-6);
}

#[test]
fn test_interpolation_lerp() {
    let result = interpolation::lerp(0.0, 100.0, 0.5);
    assert!((result - 50.0).abs() < 1e-6);
}

#[test]
fn test_interpolation_ease_in() {
    assert!((interpolation::ease_in(0.0) - 0.0).abs() < 1e-6);
    assert!((interpolation::ease_in(1.0) - 1.0).abs() < 1e-6);
    assert!(interpolation::ease_in(0.5) < 0.5);
}

#[test]
fn test_interpolation_ease_out() {
    assert!((interpolation::ease_out(0.0) - 0.0).abs() < 1e-6);
    assert!((interpolation::ease_out(1.0) - 1.0).abs() < 1e-6);
    assert!(interpolation::ease_out(0.5) > 0.5);
}

#[test]
fn test_utils_clamp() {
    assert!((utils::clamp(150.0, 0.0, 100.0) - 100.0).abs() < 1e-6);
    assert!((utils::clamp(-50.0, 0.0, 100.0) - 0.0).abs() < 1e-6);
    assert!((utils::clamp(50.0, 0.0, 100.0) - 50.0).abs() < 1e-6);
}

#[test]
fn test_utils_map() {
    let result = utils::map(50.0, 0.0, 100.0, 0.0, 1.0);
    assert!((result - 0.5).abs() < 1e-6);
}

#[test]
fn test_utils_angle_conversion() {
    assert!((utils::deg_to_rad(90.0) - PI / 2.0).abs() < 1e-6);
    assert!((utils::rad_to_deg(PI / 2.0) - 90.0).abs() < 1e-6);
}

#[test]
fn test_random_consistency() {
    random::init_global(12345);
    let val1 = random::random();
    random::init_global(12345);
    let val2 = random::random();
    assert!((val1 - val2).abs() < 1e-6);
}

#[test]
fn test_random_range() {
    random::init_global(12345);
    for _ in 0..100 {
        let val = random::range(10.0, 20.0);
        assert!(val >= 10.0 && val <= 20.0);
    }
}

#[test]
fn test_geometry_rectangle_contains() {
    let rect = geometry::Rectangle::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
    assert!(rect.contains_point(Vec2::new(5.0, 5.0)));
    assert!(!rect.contains_point(Vec2::new(15.0, 5.0)));
}

#[test]
fn test_geometry_rectangle_intersection() {
    let rect1 = geometry::Rectangle::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
    let rect2 = geometry::Rectangle::new(Vec2::new(5.0, 5.0), Vec2::new(10.0, 10.0));
    assert!(rect1.intersects(&rect2));
}

#[test]
fn test_geometry_circle_contains() {
    let circle = geometry::Circle::new(Vec2::new(0.0, 0.0), 5.0);
    assert!(circle.contains_point(Vec2::new(3.0, 4.0)));
    assert!(!circle.contains_point(Vec2::new(6.0, 0.0)));
}

#[test]
fn test_geometry_circle_intersection() {
    let circle1 = geometry::Circle::new(Vec2::new(0.0, 0.0), 5.0);
    let circle2 = geometry::Circle::new(Vec2::new(8.0, 0.0), 5.0);
    assert!(circle1.intersects(&circle2));
}

#[test]
fn test_geometry_line_intersection() {
    let line1 = geometry::LineSegment::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
    let line2 = geometry::LineSegment::new(Vec2::new(0.0, 10.0), Vec2::new(10.0, 0.0));
    assert!(line1.intersects(&line2));

    if let Some(intersection) = line1.intersection_point(&line2) {
        assert!((intersection.x - 5.0).abs() < 1e-6);
        assert!((intersection.y - 5.0).abs() < 1e-6);
    }
}

#[test]
fn test_geometry_line_closest_point() {
    let line = geometry::LineSegment::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0));
    let point = Vec2::new(5.0, 5.0);
    let closest = line.closest_point(point);
    assert!((closest.x - 5.0).abs() < 1e-6);
    assert!((closest.y - 0.0).abs() < 1e-6);
}

#[test]
fn test_physics_velocity_calculation() {
    let initial = Vec2::new(0.0, 0.0);
    let final_pos = Vec2::new(10.0, 5.0);
    let velocity = physics::calculate_velocity(initial, final_pos, 1.0);
    assert!((velocity.x - 10.0).abs() < 1e-6);
    assert!((velocity.y - 5.0).abs() < 1e-6);
}

#[test]
fn test_physics_friction() {
    let velocity = Vec2::new(10.0, 0.0);
    let friction_velocity = physics::apply_friction(velocity, 0.5, 0.1);
    assert!(friction_velocity.length() < velocity.length());
}

#[test]
fn test_physics_bounce() {
    let incident = Vec2::new(5.0, -3.0);
    let normal = Vec2::new(0.0, 1.0);
    let bounced = physics::calculate_bounce(incident, normal, 0.8);
    assert!(bounced.y > 0.0); // Should bounce upward
}

#[test]
fn test_physics_projectile_motion() {
    let initial_pos = Vec2::new(0.0, 0.0);
    let initial_velocity = Vec2::new(10.0, 20.0);
    let gravity = Vec2::new(0.0, -9.81);

    let pos_at_2s = physics::projectile_position(initial_pos, initial_velocity, gravity, 2.0);
    assert!(pos_at_2s.x > 0.0); // Should move forward
    assert!(pos_at_2s.y > 0.0); // Should still be above ground
}

#[test]
fn test_physics_elastic_collision() {
    let velocity1 = Vec2::new(5.0, 0.0);
    let velocity2 = Vec2::new(3.0, 0.0);
    let normal = Vec2::new(1.0, 0.0);

    let (new_v1, new_v2) = physics::elastic_collision(velocity1, velocity2, 1.0, 1.0, normal);

    // Test that the function returns valid velocities
    assert!(new_v1.x.is_finite());
    assert!(new_v1.y.is_finite());
    assert!(new_v2.x.is_finite());
    assert!(new_v2.y.is_finite());

    // Test that Y components remain unchanged (collision is along X axis)
    assert!((new_v1.y - 0.0).abs() < 1e-6);
    assert!((new_v2.y - 0.0).abs() < 1e-6);

    // Test that the function doesn't crash and returns reasonable values
    assert!(new_v1.length() < 1000.0); // Reasonable velocity limit
    assert!(new_v2.length() < 1000.0); // Reasonable velocity limit
}

#[test]
fn test_physics_spring_force() {
    let pos1 = Vec2::new(0.0, 0.0);
    let pos2 = Vec2::new(5.0, 0.0);
    let rest_length = 3.0;
    let spring_constant = 1.0;

    let force = physics::spring_force(pos1, pos2, rest_length, spring_constant);
    assert!(force.x < 0.0); // Should pull back toward rest length
}

#[test]
fn test_physics_gravitational_force() {
    let pos1 = Vec2::new(0.0, 0.0);
    let pos2 = Vec2::new(10.0, 0.0);
    let mass1 = 1.0;
    let mass2 = 1.0;
    let g_constant = 1.0;

    let force = physics::gravitational_force(pos1, pos2, mass1, mass2, g_constant);
    assert!(force.x > 0.0); // Should attract toward pos2
}

#[test]
fn test_physics_integration() {
    let position = Vec2::new(0.0, 0.0);
    let velocity = Vec2::new(10.0, 5.0);
    let delta_time = 0.1;

    let new_position = physics::integrate_position(position, velocity, delta_time);
    assert!((new_position.x - 1.0).abs() < 1e-6);
    assert!((new_position.y - 0.5).abs() < 1e-6);
}

#[test]
fn test_physics_terminal_velocity() {
    let mass = 1.0;
    let gravity = 9.81;
    let drag_coefficient = 0.1;

    let terminal = physics::terminal_velocity(mass, gravity, drag_coefficient);
    assert!(terminal > 0.0);
    assert!(terminal.is_finite());
}
