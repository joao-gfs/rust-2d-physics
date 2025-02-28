mod vec2d;

pub struct Body {
    position: vec2d::Vec2,
    velocity: vec2d::Vec2,
    acceleration: vec2d::Vec2,
    mass: f32,
    radius: f32,
}

impl Body {
    fn new(x: f32, y: f32, vx: f32, vy: f32, m: f32, r: f32) -> Self {
        Body {
            position: vec2d::Vec2::new(x, y),
            velocity: vec2d::Vec2::new(vx, vy),
            acceleration: vec2d::Vec2::new(0.0, 0.0),
            mass: m,
            radius: r,
        }
    }

    fn apply_force(&mut self, force_x: f32, force_y: f32) {
        let acc_x = force_x / self.mass;
        let acc_y = force_y / self.mass;
        self.acceleration += vec2d::Vec2::new(acc_x, acc_y);
    }

    fn update(&mut self, delta_t: f32) {
        self.velocity += self.acceleration * delta_t;
        self.position += self.velocity * delta_t;
        self.acceleration = vec2d::Vec2::new(0.0, 0.0);
    }

    fn distance_from_point(&self, x: f32, y: f32) -> f32 {
        ((self.position.x - x).powi(2) + (self.position.y - y).powi(2)).sqrt()
    }

    fn distance_from_body(&self, other: Body) -> f32 {
        self.distance_from_point(other.position.x, other.position.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_body() {
        let mut b1 = Body::new(0.0, 0.0, 1.0, 4.0, 2.0, 3.0);
        b1.update(3.0);
        assert_eq!(b1.position, vec2d::Vec2::new(3.0, 12.0))
    }

    #[test]
    fn apply_force_body() {
        let mut b1 = Body::new(0.0, 0.0, 1.0, 4.0, 2.0, 3.0);
        b1.apply_force(5.0, 3.0);
        assert_eq!(b1.acceleration, vec2d::Vec2::new(2.5, 1.5))
    }

    #[test]
    fn get_distance_from_point() {
        let b1 = Body::new(0.0, 0.0, 1.0, 4.0, 2.0, 3.0);
        let distance= b1.distance_from_point(1.0, 1.0);
        assert_eq!(distance, (2.0_f32).sqrt())
    }

    #[test]
    fn get_distance_from_body() {
        let b1 = Body::new(0.0, 0.0, 1.0, 4.0, 2.0, 3.0);
        let b2 = Body::new(1.0, 1.0, 0.0, 0.0, 0.0, 1.0);
        let distance= b1.distance_from_body(b2);
        assert_eq!(distance, (2.0_f32).sqrt())
    }
}