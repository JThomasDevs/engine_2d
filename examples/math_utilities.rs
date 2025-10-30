use engine_2d::utils::math::*;
use glam::Vec2;

fn main() {
    println!("🧮 Testing Math Utilities");
    println!("========================");

    // Test vector operations
    println!("\n📐 Vector Operations:");
    let v1 = Vec2::new(3.0, 4.0);
    let v2 = Vec2::new(1.0, 2.0);

    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
    println!("Magnitude of v1: {:.2}", vector::magnitude(v1));
    println!(
        "Distance between v1 and v2: {:.2}",
        vector::distance(v1, v2)
    );
    println!("Dot product: {:.2}", vector::dot(v1, v2));
    println!("Cross product: {:.2}", vector::cross(v1, v2));

    let rotated = vector::rotate(v1, std::f32::consts::PI / 4.0);
    println!("v1 rotated 45°: {:?}", rotated);

    // Test matrix operations
    println!("\n🔢 Matrix Operations:");
    let mat2 = matrix::Mat2::rotation(std::f32::consts::PI / 2.0);
    let transformed = mat2.transform_vector(v1);
    println!("v1 rotated 90° with matrix: {:?}", transformed);

    let mat3 = matrix::Mat3::translation(Vec2::new(10.0, 20.0));
    let translated = mat3.transform_vector(v1);
    println!("v1 translated (10, 20): {:?}", translated);

    // Test interpolation
    println!("\n📈 Interpolation:");
    let start = 0.0;
    let end = 100.0;
    let t = 0.5;

    println!(
        "Linear interpolation: {:.2}",
        interpolation::lerp(start, end, t)
    );
    println!("Ease-in: {:.2}", interpolation::ease_in(t));
    println!("Ease-out: {:.2}", interpolation::ease_out(t));
    println!("Ease-in-out: {:.2}", interpolation::ease_in_out(t));
    println!("Bounce: {:.2}", interpolation::bounce(t));

    // Test utility functions
    println!("\n🛠️ Utility Functions:");
    let value = 150.0;
    println!(
        "Clamp 150 to [0, 100]: {:.2}",
        utils::clamp(value, 0.0, 100.0)
    );
    println!(
        "Map 50 from [0, 100] to [0, 1]: {:.2}",
        utils::map(50.0, 0.0, 100.0, 0.0, 1.0)
    );
    println!("45° to radians: {:.2}", utils::deg_to_rad(45.0));
    println!(
        "π/2 radians to degrees: {:.2}",
        utils::rad_to_deg(std::f32::consts::PI / 2.0)
    );

    // Test random number generation
    println!("\n🎲 Random Number Generation:");
    random::init_global(12345); // Seed for reproducible results
    println!("Random float [0, 1]: {:.3}", random::random());
    println!("Random float [10, 20]: {:.3}", random::range(10.0, 20.0));
    println!("Random integer [1, 6]: {}", random::range_i32(1, 6));

    let mut rng = random::Random::new(54321);
    println!("Custom RNG - Random float: {:.3}", rng.next_f32());
    println!("Custom RNG - Random bool: {}", rng.next_bool());

    let choices = vec!["apple", "banana", "cherry", "date"];
    if let Some(choice) = rng.choice(&choices) {
        println!("Random choice: {}", choice);
    }

    // Test geometric functions
    println!("\n📐 Geometric Functions:");
    let rect = geometry::Rectangle::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
    let circle = geometry::Circle::new(Vec2::new(5.0, 5.0), 3.0);
    let point = Vec2::new(2.0, 2.0);

    println!(
        "Point {:?} in rectangle: {}",
        point,
        rect.contains_point(point)
    );
    println!(
        "Point {:?} in circle: {}",
        point,
        circle.contains_point(point)
    );
    println!(
        "Rectangle intersects circle: {}",
        circle.intersects_rect(&rect)
    );

    let line1 = geometry::LineSegment::new(Vec2::new(0.0, 0.0), Vec2::new(10.0, 10.0));
    let line2 = geometry::LineSegment::new(Vec2::new(0.0, 10.0), Vec2::new(10.0, 0.0));
    println!("Lines intersect: {}", line1.intersects(&line2));

    if let Some(intersection) = line1.intersection_point(&line2) {
        println!("Intersection point: {:?}", intersection);
    }

    // Test physics functions
    println!("\n⚡ Physics Functions:");
    let initial_pos = Vec2::new(0.0, 0.0);
    let final_pos = Vec2::new(10.0, 5.0);
    let velocity = physics::calculate_velocity(initial_pos, final_pos, 1.0);
    println!("Velocity from position change: {:?}", velocity);

    let friction_velocity = physics::apply_friction(velocity, 0.5, 0.1);
    println!("Velocity after friction: {:?}", friction_velocity);

    let bounce_velocity = physics::calculate_bounce(Vec2::new(5.0, -3.0), Vec2::new(0.0, 1.0), 0.8);
    println!("Bounce velocity: {:?}", bounce_velocity);

    let projectile_pos = physics::projectile_position(
        Vec2::new(0.0, 0.0),
        Vec2::new(10.0, 20.0),
        Vec2::new(0.0, -9.81),
        2.0,
    );
    println!("Projectile position after 2s: {:?}", projectile_pos);

    println!("\n✅ Math utilities test completed successfully!");
}
