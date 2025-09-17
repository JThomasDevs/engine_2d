use glam::Vec2;
use std::f32::consts::PI;

/// 2D Vector operations and utilities
pub mod vector {
    use super::*;

    /// Calculate the magnitude (length) of a vector
    pub fn magnitude(v: Vec2) -> f32 {
        v.length()
    }

    /// Calculate the squared magnitude of a vector (faster than magnitude)
    pub fn magnitude_squared(v: Vec2) -> f32 {
        v.length_squared()
    }

    /// Normalize a vector to unit length
    pub fn normalize(v: Vec2) -> Vec2 {
        v.normalize()
    }

    /// Calculate the dot product of two vectors
    pub fn dot(a: Vec2, b: Vec2) -> f32 {
        a.dot(b)
    }

    /// Calculate the cross product of two 2D vectors (returns scalar)
    pub fn cross(a: Vec2, b: Vec2) -> f32 {
        a.x * b.y - a.y * b.x
    }

    /// Calculate the distance between two points
    pub fn distance(a: Vec2, b: Vec2) -> f32 {
        (a - b).length()
    }

    /// Calculate the squared distance between two points (faster than distance)
    pub fn distance_squared(a: Vec2, b: Vec2) -> f32 {
        (a - b).length_squared()
    }

    /// Calculate the angle between two vectors in radians
    pub fn angle_between(a: Vec2, b: Vec2) -> f32 {
        a.angle_between(b)
    }

    /// Rotate a vector by an angle in radians
    pub fn rotate(v: Vec2, angle: f32) -> Vec2 {
        let cos = angle.cos();
        let sin = angle.sin();
        Vec2::new(
            v.x * cos - v.y * sin,
            v.x * sin + v.y * cos,
        )
    }

    /// Reflect a vector off a surface with the given normal
    pub fn reflect(incident: Vec2, normal: Vec2) -> Vec2 {
        incident - 2.0 * dot(incident, normal) * normal
    }

    /// Linear interpolation between two vectors
    pub fn lerp(a: Vec2, b: Vec2, t: f32) -> Vec2 {
        a.lerp(b, t)
    }

    /// Spherical linear interpolation between two vectors
    pub fn slerp(a: Vec2, b: Vec2, t: f32) -> Vec2 {
        let dot = dot(a, b).clamp(-1.0, 1.0);
        let theta = dot.acos() * t;
        let relative_vec = normalize(b - a * dot);
        a * theta.cos() + relative_vec * theta.sin()
    }

    /// Project vector a onto vector b
    pub fn project(a: Vec2, b: Vec2) -> Vec2 {
        let b_norm = normalize(b);
        dot(a, b_norm) * b_norm
    }

    /// Reject vector a from vector b (component perpendicular to b)
    pub fn reject(a: Vec2, b: Vec2) -> Vec2 {
        a - project(a, b)
    }
}

/// 2D Matrix operations and transformations
pub mod matrix {
    use super::*;

    /// 2x2 matrix for 2D transformations
    #[derive(Debug, Clone, Copy)]
    pub struct Mat2 {
        pub m: [[f32; 2]; 2],
    }

    impl Mat2 {
        /// Create an identity matrix
        pub fn identity() -> Self {
            Self {
                m: [[1.0, 0.0], [0.0, 1.0]],
            }
        }

        /// Create a rotation matrix
        pub fn rotation(angle: f32) -> Self {
            let cos = angle.cos();
            let sin = angle.sin();
            Self {
                m: [[cos, -sin], [sin, cos]],
            }
        }

        /// Create a scaling matrix
        pub fn scaling(scale: Vec2) -> Self {
            Self {
                m: [[scale.x, 0.0], [0.0, scale.y]],
            }
        }

        /// Multiply two matrices
        pub fn multiply(self, other: Self) -> Self {
            let mut result = Self::identity();
            for i in 0..2 {
                for j in 0..2 {
                    result.m[i][j] = 0.0;
                    for k in 0..2 {
                        result.m[i][j] += self.m[i][k] * other.m[k][j];
                    }
                }
            }
            result
        }

        /// Transform a vector by this matrix
        pub fn transform_vector(self, v: Vec2) -> Vec2 {
            Vec2::new(
                self.m[0][0] * v.x + self.m[0][1] * v.y,
                self.m[1][0] * v.x + self.m[1][1] * v.y,
            )
        }

        /// Get the transpose of this matrix
        pub fn transpose(self) -> Self {
            Self {
                m: [[self.m[0][0], self.m[1][0]], [self.m[0][1], self.m[1][1]]],
            }
        }

        /// Get the determinant of this matrix
        pub fn determinant(self) -> f32 {
            self.m[0][0] * self.m[1][1] - self.m[0][1] * self.m[1][0]
        }

        /// Get the inverse of this matrix
        pub fn inverse(self) -> Option<Self> {
            let det = self.determinant();
            if det.abs() < 1e-6 {
                return None;
            }
            Some(Self {
                m: [
                    [self.m[1][1] / det, -self.m[0][1] / det],
                    [-self.m[1][0] / det, self.m[0][0] / det],
                ],
            })
        }
    }

    /// 3x3 matrix for 2D transformations with translation
    #[derive(Debug, Clone, Copy)]
    pub struct Mat3 {
        pub m: [[f32; 3]; 3],
    }

    impl Mat3 {
        /// Create an identity matrix
        pub fn identity() -> Self {
            Self {
                m: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            }
        }

        /// Create a translation matrix
        pub fn translation(translation: Vec2) -> Self {
            Self {
                m: [[1.0, 0.0, translation.x], [0.0, 1.0, translation.y], [0.0, 0.0, 1.0]],
            }
        }

        /// Create a rotation matrix
        pub fn rotation(angle: f32) -> Self {
            let cos = angle.cos();
            let sin = angle.sin();
            Self {
                m: [[cos, -sin, 0.0], [sin, cos, 0.0], [0.0, 0.0, 1.0]],
            }
        }

        /// Create a scaling matrix
        pub fn scaling(scale: Vec2) -> Self {
            Self {
                m: [[scale.x, 0.0, 0.0], [0.0, scale.y, 0.0], [0.0, 0.0, 1.0]],
            }
        }

        /// Create a transformation matrix (translation, rotation, scale)
        pub fn transform(translation: Vec2, rotation: f32, scale: Vec2) -> Self {
            let cos = rotation.cos();
            let sin = rotation.sin();
            Self {
                m: [
                    [scale.x * cos, -scale.y * sin, translation.x],
                    [scale.x * sin, scale.y * cos, translation.y],
                    [0.0, 0.0, 1.0],
                ],
            }
        }

        /// Multiply two matrices
        pub fn multiply(self, other: Self) -> Self {
            let mut result = Self::identity();
            for i in 0..3 {
                for j in 0..3 {
                    result.m[i][j] = 0.0;
                    for k in 0..3 {
                        result.m[i][j] += self.m[i][k] * other.m[k][j];
                    }
                }
            }
            result
        }

        /// Transform a 2D vector by this matrix (treats as homogeneous coordinates)
        pub fn transform_vector(self, v: Vec2) -> Vec2 {
            Vec2::new(
                self.m[0][0] * v.x + self.m[0][1] * v.y + self.m[0][2],
                self.m[1][0] * v.x + self.m[1][1] * v.y + self.m[1][2],
            )
        }

        /// Get the transpose of this matrix
        pub fn transpose(self) -> Self {
            Self {
                m: [
                    [self.m[0][0], self.m[1][0], self.m[2][0]],
                    [self.m[0][1], self.m[1][1], self.m[2][1]],
                    [self.m[0][2], self.m[1][2], self.m[2][2]],
                ],
            }
        }
    }
}

/// Interpolation and easing functions
pub mod interpolation {
    use super::*;

    /// Linear interpolation between two values
    pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t
    }

    /// Smooth step interpolation (smoothstep)
    pub fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
        let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }

    /// Smoother step interpolation (smootherstep)
    pub fn smootherstep(edge0: f32, edge1: f32, x: f32) -> f32 {
        let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    }

    /// Ease-in interpolation
    pub fn ease_in(t: f32) -> f32 {
        t * t
    }

    /// Ease-out interpolation
    pub fn ease_out(t: f32) -> f32 {
        1.0 - (1.0 - t) * (1.0 - t)
    }

    /// Ease-in-out interpolation
    pub fn ease_in_out(t: f32) -> f32 {
        if t < 0.5 {
            2.0 * t * t
        } else {
            1.0 - 2.0 * (1.0 - t) * (1.0 - t)
        }
    }

    /// Bounce easing function
    pub fn bounce(t: f32) -> f32 {
        if t < 1.0 / 2.75 {
            7.5625 * t * t
        } else if t < 2.0 / 2.75 {
            let t = t - 1.5 / 2.75;
            7.5625 * t * t + 0.75
        } else if t < 2.5 / 2.75 {
            let t = t - 2.25 / 2.75;
            7.5625 * t * t + 0.9375
        } else {
            let t = t - 2.625 / 2.75;
            7.5625 * t * t + 0.984375
        }
    }

    /// Elastic easing function
    pub fn elastic(t: f32) -> f32 {
        if t == 0.0 || t == 1.0 {
            t
        } else {
            let c4 = (2.0 * PI) / 3.0;
            -(2.0_f32.powf(10.0 * t - 10.0)) * ((t * 10.0 - 10.75) * c4).sin()
        }
    }
}

/// Utility functions
pub mod utils {
    use super::*;

    /// Clamp a value between min and max
    pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
        value.max(min).min(max)
    }

    /// Clamp a value between 0 and 1
    pub fn clamp01(value: f32) -> f32 {
        clamp(value, 0.0, 1.0)
    }

    /// Map a value from one range to another
    pub fn map(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
        to_min + (value - from_min) * (to_max - to_min) / (from_max - from_min)
    }

    /// Convert degrees to radians
    pub fn deg_to_rad(degrees: f32) -> f32 {
        degrees * PI / 180.0
    }

    /// Convert radians to degrees
    pub fn rad_to_deg(radians: f32) -> f32 {
        radians * 180.0 / PI
    }

    /// Round to nearest integer
    pub fn round(value: f32) -> f32 {
        value.round()
    }

    /// Floor to nearest integer
    pub fn floor(value: f32) -> f32 {
        value.floor()
    }

    /// Ceil to nearest integer
    pub fn ceil(value: f32) -> f32 {
        value.ceil()
    }

    /// Check if two floats are approximately equal
    pub fn approximately(a: f32, b: f32, epsilon: f32) -> bool {
        (a - b).abs() < epsilon
    }

    /// Check if a value is approximately zero
    pub fn is_zero(value: f32, epsilon: f32) -> bool {
        value.abs() < epsilon
    }

    /// Sign of a number (-1, 0, or 1)
    pub fn sign(value: f32) -> f32 {
        if value > 0.0 {
            1.0
        } else if value < 0.0 {
            -1.0
        } else {
            0.0
        }
    }

    /// Absolute value
    pub fn abs(value: f32) -> f32 {
        value.abs()
    }

    /// Minimum of two values
    pub fn min(a: f32, b: f32) -> f32 {
        a.min(b)
    }

    /// Maximum of two values
    pub fn max(a: f32, b: f32) -> f32 {
        a.max(b)
    }
}

/// Random number generation utilities
pub mod random {
    use std::sync::Mutex;

    /// Simple pseudo-random number generator
    pub struct Random {
        seed: u64,
    }

    impl Random {
        /// Create a new random number generator with a seed
        pub fn new(seed: u64) -> Self {
            Self { seed }
        }

        /// Create a new random number generator with a random seed
        pub fn new_random() -> Self {
            Self {
                seed: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            }
        }

        /// Generate a random float between 0 and 1
        pub fn next_f32(&mut self) -> f32 {
            self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
            (self.seed & 0x7FFFFFFF) as f32 / i32::MAX as f32
        }

        /// Generate a random float between min and max
        pub fn range_f32(&mut self, min: f32, max: f32) -> f32 {
            min + (max - min) * self.next_f32()
        }

        /// Generate a random integer between min and max (inclusive)
        pub fn range_i32(&mut self, min: i32, max: i32) -> i32 {
            if min >= max {
                return min;
            }
            let range = (max - min + 1) as f32;
            min + (self.next_f32() * range) as i32
        }

        /// Generate a random boolean
        pub fn next_bool(&mut self) -> bool {
            self.next_f32() < 0.5
        }

        /// Choose a random element from a slice
        pub fn choice<'a, T>(&mut self, slice: &'a [T]) -> Option<&'a T> {
            if slice.is_empty() {
                None
            } else {
                Some(&slice[self.range_i32(0, slice.len() as i32 - 1) as usize])
            }
        }
    }

    /// Global random number generator
    static GLOBAL_RANDOM: Mutex<Option<Random>> = Mutex::new(None);

    /// Initialize the global random number generator
    pub fn init_global(seed: u64) {
        let mut global = GLOBAL_RANDOM.lock().unwrap();
        *global = Some(Random::new(seed));
    }

    /// Get a random float between 0 and 1 using the global generator
    pub fn random() -> f32 {
        let mut global = GLOBAL_RANDOM.lock().unwrap();
        if global.is_none() {
            *global = Some(Random::new_random());
        }
        global.as_mut().unwrap().next_f32()
    }

    /// Get a random float between min and max using the global generator
    pub fn range(min: f32, max: f32) -> f32 {
        let mut global = GLOBAL_RANDOM.lock().unwrap();
        if global.is_none() {
            *global = Some(Random::new_random());
        }
        global.as_mut().unwrap().range_f32(min, max)
    }

    /// Get a random integer between min and max using the global generator
    pub fn range_i32(min: i32, max: i32) -> i32 {
        let mut global = GLOBAL_RANDOM.lock().unwrap();
        if global.is_none() {
            *global = Some(Random::new_random());
        }
        global.as_mut().unwrap().range_i32(min, max)
    }
}

/// Geometric functions for collision detection and spatial queries
pub mod geometry {
    use super::*;

    /// Rectangle structure for geometric operations
    #[derive(Debug, Clone, Copy)]
    pub struct Rectangle {
        pub position: Vec2,
        pub size: Vec2,
    }

    impl Rectangle {
        /// Create a new rectangle
        pub fn new(position: Vec2, size: Vec2) -> Self {
            Self { position, size }
        }

        /// Create a rectangle from center point and size
        pub fn from_center(center: Vec2, size: Vec2) -> Self {
            Self {
                position: center - size * 0.5,
                size,
            }
        }

        /// Get the center point of the rectangle
        pub fn center(&self) -> Vec2 {
            self.position + self.size * 0.5
        }

        /// Get the top-left corner
        pub fn top_left(&self) -> Vec2 {
            self.position
        }

        /// Get the bottom-right corner
        pub fn bottom_right(&self) -> Vec2 {
            self.position + self.size
        }

        /// Check if a point is inside the rectangle
        pub fn contains_point(&self, point: Vec2) -> bool {
            point.x >= self.position.x
                && point.x <= self.position.x + self.size.x
                && point.y >= self.position.y
                && point.y <= self.position.y + self.size.y
        }

        /// Check if this rectangle intersects with another rectangle
        pub fn intersects(&self, other: &Rectangle) -> bool {
            self.position.x < other.position.x + other.size.x
                && self.position.x + self.size.x > other.position.x
                && self.position.y < other.position.y + other.size.y
                && self.position.y + self.size.y > other.position.y
        }

        /// Get the intersection rectangle with another rectangle
        pub fn intersection(&self, other: &Rectangle) -> Option<Rectangle> {
            if !self.intersects(other) {
                return None;
            }

            let left = self.position.x.max(other.position.x);
            let top = self.position.y.max(other.position.y);
            let right = (self.position.x + self.size.x).min(other.position.x + other.size.x);
            let bottom = (self.position.y + self.size.y).min(other.position.y + other.size.y);

            Some(Rectangle::new(
                Vec2::new(left, top),
                Vec2::new(right - left, bottom - top),
            ))
        }
    }

    /// Circle structure for geometric operations
    #[derive(Debug, Clone, Copy)]
    pub struct Circle {
        pub center: Vec2,
        pub radius: f32,
    }

    impl Circle {
        /// Create a new circle
        pub fn new(center: Vec2, radius: f32) -> Self {
            Self { center, radius }
        }

        /// Check if a point is inside the circle
        pub fn contains_point(&self, point: Vec2) -> bool {
            vector::distance_squared(self.center, point) <= self.radius * self.radius
        }

        /// Check if this circle intersects with another circle
        pub fn intersects(&self, other: &Circle) -> bool {
            let distance = vector::distance(self.center, other.center);
            distance <= self.radius + other.radius
        }

        /// Check if this circle intersects with a rectangle
        pub fn intersects_rect(&self, rect: &Rectangle) -> bool {
            let closest_x = utils::clamp(self.center.x, rect.position.x, rect.position.x + rect.size.x);
            let closest_y = utils::clamp(self.center.y, rect.position.y, rect.position.y + rect.size.y);
            let distance_squared = vector::distance_squared(self.center, Vec2::new(closest_x, closest_y));
            distance_squared <= self.radius * self.radius
        }
    }

    /// Line segment structure
    #[derive(Debug, Clone, Copy)]
    pub struct LineSegment {
        pub start: Vec2,
        pub end: Vec2,
    }

    impl LineSegment {
        /// Create a new line segment
        pub fn new(start: Vec2, end: Vec2) -> Self {
            Self { start, end }
        }

        /// Get the length of the line segment
        pub fn length(&self) -> f32 {
            vector::distance(self.start, self.end)
        }

        /// Get the direction vector of the line segment
        pub fn direction(&self) -> Vec2 {
            self.end - self.start
        }

        /// Get the closest point on the line segment to a given point
        pub fn closest_point(&self, point: Vec2) -> Vec2 {
            let line_vec = self.direction();
            let point_vec = point - self.start;
            
            let line_length_squared = line_vec.length_squared();
            if line_length_squared < 1e-6 {
                return self.start;
            }
            
            let t = vector::dot(point_vec, line_vec) / line_length_squared;
            let t_clamped = utils::clamp(t, 0.0, 1.0);
            
            self.start + line_vec * t_clamped
        }

        /// Check if this line segment intersects with another line segment
        pub fn intersects(&self, other: &LineSegment) -> bool {
            self.intersection_point(other).is_some()
        }

        /// Get the intersection point with another line segment (if any)
        pub fn intersection_point(&self, other: &LineSegment) -> Option<Vec2> {
            let p1 = self.start;
            let p2 = self.end;
            let p3 = other.start;
            let p4 = other.end;

            let denom = (p1.x - p2.x) * (p3.y - p4.y) - (p1.y - p2.y) * (p3.x - p4.x);
            if denom.abs() < 1e-6 {
                return None; // Lines are parallel
            }

            let t = ((p1.x - p3.x) * (p3.y - p4.y) - (p1.y - p3.y) * (p3.x - p4.x)) / denom;
            let u = -((p1.x - p2.x) * (p1.y - p3.y) - (p1.y - p2.y) * (p1.x - p3.x)) / denom;

            if t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0 {
                Some(Vec2::new(p1.x + t * (p2.x - p1.x), p1.y + t * (p2.y - p1.y)))
            } else {
                None
            }
        }
    }

    /// Check if a point is inside a rectangle
    pub fn point_in_rectangle(point: Vec2, rect: &Rectangle) -> bool {
        rect.contains_point(point)
    }

    /// Check if a point is inside a circle
    pub fn point_in_circle(point: Vec2, circle: &Circle) -> bool {
        circle.contains_point(point)
    }

    /// Check if two rectangles intersect
    pub fn rectangle_intersects(rect1: &Rectangle, rect2: &Rectangle) -> bool {
        rect1.intersects(rect2)
    }

    /// Check if two circles intersect
    pub fn circle_intersects(circle1: &Circle, circle2: &Circle) -> bool {
        circle1.intersects(circle2)
    }

    /// Check if a circle intersects with a rectangle
    pub fn circle_rectangle_intersects(circle: &Circle, rect: &Rectangle) -> bool {
        circle.intersects_rect(rect)
    }

    /// Check if two line segments intersect
    pub fn line_intersects(line1: &LineSegment, line2: &LineSegment) -> bool {
        line1.intersects(line2)
    }

    /// Get the intersection point of two line segments
    pub fn line_intersection_point(line1: &LineSegment, line2: &LineSegment) -> Option<Vec2> {
        line1.intersection_point(line2)
    }

    /// Get the closest point on a line segment to a given point
    pub fn closest_point_on_line(point: Vec2, line: &LineSegment) -> Vec2 {
        line.closest_point(point)
    }

    /// Calculate the distance from a point to a line segment
    pub fn distance_point_to_line(point: Vec2, line: &LineSegment) -> f32 {
        vector::distance(point, line.closest_point(point))
    }

    /// Calculate the distance from a point to a rectangle
    pub fn distance_point_to_rectangle(point: Vec2, rect: &Rectangle) -> f32 {
        if rect.contains_point(point) {
            return 0.0;
        }

        let closest_x = utils::clamp(point.x, rect.position.x, rect.position.x + rect.size.x);
        let closest_y = utils::clamp(point.y, rect.position.y, rect.position.y + rect.size.y);
        
        vector::distance(point, Vec2::new(closest_x, closest_y))
    }

    /// Calculate the distance from a point to a circle
    pub fn distance_point_to_circle(point: Vec2, circle: &Circle) -> f32 {
        let distance_to_center = vector::distance(point, circle.center);
        (distance_to_center - circle.radius).max(0.0)
    }
}

/// Physics helper functions for game mechanics
pub mod physics {
    use super::*;

    /// Calculate velocity from position change over time
    pub fn calculate_velocity(initial_position: Vec2, final_position: Vec2, delta_time: f32) -> Vec2 {
        if delta_time <= 0.0 {
            return Vec2::ZERO;
        }
        (final_position - initial_position) / delta_time
    }

    /// Calculate acceleration from velocity change over time
    pub fn calculate_acceleration(initial_velocity: Vec2, final_velocity: Vec2, delta_time: f32) -> Vec2 {
        if delta_time <= 0.0 {
            return Vec2::ZERO;
        }
        (final_velocity - initial_velocity) / delta_time
    }

    /// Apply friction to a velocity vector
    pub fn apply_friction(velocity: Vec2, friction_coefficient: f32, delta_time: f32) -> Vec2 {
        let friction_force = velocity * friction_coefficient * delta_time;
        let new_velocity = velocity - friction_force;
        
        // Stop very small velocities to prevent jittering
        if new_velocity.length() < 0.01 {
            Vec2::ZERO
        } else {
            new_velocity
        }
    }

    /// Apply air resistance (drag) to a velocity vector
    pub fn apply_air_resistance(velocity: Vec2, drag_coefficient: f32, delta_time: f32) -> Vec2 {
        let drag_force = velocity * velocity.length() * drag_coefficient * delta_time;
        velocity - drag_force
    }

    /// Calculate bounce reflection with energy loss
    pub fn calculate_bounce(incident_velocity: Vec2, surface_normal: Vec2, restitution: f32) -> Vec2 {
        let normal = vector::normalize(surface_normal);
        let velocity_along_normal = vector::dot(incident_velocity, normal);
        
        if velocity_along_normal > 0.0 {
            return incident_velocity; // Already moving away from surface
        }
        
        let reflected_velocity = incident_velocity - (1.0 + restitution) * velocity_along_normal * normal;
        reflected_velocity
    }

    /// Calculate elastic collision between two objects
    pub fn elastic_collision(
        velocity1: Vec2,
        velocity2: Vec2,
        mass1: f32,
        mass2: f32,
        collision_normal: Vec2,
    ) -> (Vec2, Vec2) {
        let normal = vector::normalize(collision_normal);
        
        // Relative velocity along collision normal
        let relative_velocity = velocity1 - velocity2;
        let velocity_along_normal = vector::dot(relative_velocity, normal);
        
        // Do not resolve if velocities are separating
        if velocity_along_normal > 0.0 {
            return (velocity1, velocity2);
        }
        
        // Calculate restitution (elasticity)
        let restitution = 1.0; // Perfectly elastic collision
        
        // Calculate impulse scalar
        let mut impulse_scalar = -(1.0 + restitution) * velocity_along_normal;
        impulse_scalar /= (1.0 / mass1) + (1.0 / mass2);
        
        // Calculate impulse vector
        let impulse = impulse_scalar * normal;
        
        // Apply impulse to velocities
        let new_velocity1 = velocity1 + impulse / mass1;
        let new_velocity2 = velocity2 - impulse / mass2;
        
        (new_velocity1, new_velocity2)
    }

    /// Calculate gravitational force between two objects
    pub fn gravitational_force(
        position1: Vec2,
        position2: Vec2,
        mass1: f32,
        mass2: f32,
        gravitational_constant: f32,
    ) -> Vec2 {
        let distance_vec = position2 - position1;
        let distance = distance_vec.length();
        
        if distance < 1e-6 {
            return Vec2::ZERO; // Avoid division by zero
        }
        
        let force_magnitude = gravitational_constant * mass1 * mass2 / (distance * distance);
        let force_direction = vector::normalize(distance_vec);
        
        force_direction * force_magnitude
    }

    /// Calculate spring force (Hooke's law)
    pub fn spring_force(
        position1: Vec2,
        position2: Vec2,
        rest_length: f32,
        spring_constant: f32,
    ) -> Vec2 {
        let distance_vec = position2 - position1;
        let distance = distance_vec.length();
        
        if distance < 1e-6 {
            return Vec2::ZERO; // Avoid division by zero
        }
        
        let displacement = distance - rest_length;
        let force_magnitude = -spring_constant * displacement;
        let force_direction = vector::normalize(distance_vec);
        
        force_direction * force_magnitude
    }

    /// Calculate centripetal force for circular motion
    pub fn centripetal_force(velocity: Vec2, position: Vec2, center: Vec2, mass: f32) -> Vec2 {
        let speed = velocity.length();
        let radius = vector::distance(center, position);
        
        if speed < 1e-6 || radius < 1e-6 {
            return Vec2::ZERO;
        }
        
        let force_magnitude = mass * speed * speed / radius;
        let force_direction = vector::normalize(center - position); // Points toward center
        
        force_direction * force_magnitude
    }

    /// Integrate position using Euler method
    pub fn integrate_position(position: Vec2, velocity: Vec2, delta_time: f32) -> Vec2 {
        position + velocity * delta_time
    }

    /// Integrate velocity using Euler method
    pub fn integrate_velocity(velocity: Vec2, acceleration: Vec2, delta_time: f32) -> Vec2 {
        velocity + acceleration * delta_time
    }

    /// Integrate position and velocity using Verlet integration (more stable)
    pub fn verlet_integration(
        position: Vec2,
        previous_position: Vec2,
        acceleration: Vec2,
        delta_time: f32,
    ) -> Vec2 {
        let dt_squared = delta_time * delta_time;
        2.0 * position - previous_position + acceleration * dt_squared
    }

    /// Calculate terminal velocity (maximum velocity under drag)
    pub fn terminal_velocity(mass: f32, gravity: f32, drag_coefficient: f32) -> f32 {
        if drag_coefficient <= 0.0 {
            return f32::INFINITY;
        }
        (mass * gravity / drag_coefficient).sqrt()
    }

    /// Calculate projectile motion position
    pub fn projectile_position(
        initial_position: Vec2,
        initial_velocity: Vec2,
        gravity: Vec2,
        time: f32,
    ) -> Vec2 {
        initial_position + initial_velocity * time + 0.5 * gravity * time * time
    }

    /// Calculate projectile motion velocity
    pub fn projectile_velocity(initial_velocity: Vec2, gravity: Vec2, time: f32) -> Vec2 {
        initial_velocity + gravity * time
    }

    /// Calculate time to reach maximum height in projectile motion
    pub fn time_to_max_height(initial_velocity_y: f32, gravity_y: f32) -> f32 {
        if gravity_y >= 0.0 {
            return 0.0;
        }
        -initial_velocity_y / gravity_y
    }

    /// Calculate maximum height in projectile motion
    pub fn max_height(initial_position_y: f32, initial_velocity_y: f32, gravity_y: f32) -> f32 {
        if gravity_y >= 0.0 {
            return initial_position_y;
        }
        let time_to_max = time_to_max_height(initial_velocity_y, gravity_y);
        projectile_position(
            Vec2::new(0.0, initial_position_y),
            Vec2::new(0.0, initial_velocity_y),
            Vec2::new(0.0, gravity_y),
            time_to_max,
        ).y
    }
}
