mod vec2d;

#[derive(Debug)]
pub struct Body {
    pub position: vec2d::Vec2,
    pub velocity: vec2d::Vec2,
    pub acceleration: vec2d::Vec2,
    pub mass: f32,
    pub radius: f32,
}

impl Body {
    pub fn new(x: f32, y: f32, vx: f32, vy: f32, m: f32, r: f32) -> Self {
        Body {
            position: vec2d::Vec2::new(x, y),
            velocity: vec2d::Vec2::new(vx, vy),
            acceleration: vec2d::Vec2::new(0.0, 0.0),
            mass: m,
            radius: r,
        }
    }

    pub fn apply_force(&mut self, force_x: f32, force_y: f32) {
        let acc_x = force_x / self.mass;
        let acc_y = force_y / self.mass;
        self.acceleration += vec2d::Vec2::new(acc_x, acc_y);
    }

    pub fn update(&mut self, delta_t: f32) {
        self.velocity += self.acceleration * delta_t;
        self.position += self.velocity * delta_t;
        self.acceleration = vec2d::Vec2::new(0.0, 0.0);
    }
}

pub mod utils {
    use super::Body;

    pub fn point_distance(x1:f32, y1:f32, x2: f32, y2: f32) -> f32 {
        ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt()
    }
    
    pub fn body_distance(body1: &Body, body2: &Body) -> f32 {
        point_distance(body1.position.x, body1.position.y, body2.position.x, body2.position.y)
    }

    pub fn check_collision(body1: &Body, body2: &Body) -> bool {
        let distance = body_distance(body1, body2);
        if distance < body1.radius + body2.radius {
            return true
        }
        false
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
        let distance = utils::point_distance(0.0, 0.0, 1.0, 1.0);
        assert_eq!(distance, (2.0_f32).sqrt())
    }

    #[test]
    fn get_distance_from_body() {
        let b1 = Body::new(0.0, 0.0, 1.0, 4.0, 2.0, 3.0);
        let b2 = Body::new(1.0, 1.0, 0.0, 0.0, 0.0, 1.0);
        let distance= utils::body_distance(&b1, &b2);
        assert_eq!(distance, (2.0_f32).sqrt())
    }

    #[test]
    fn collision() {
        let b1 = Body::new(0.0, 0.0, 1.0, 4.0, 2.0, 3.0);
        let b2 = Body::new(1.0, 1.0, 0.0, 0.0, 0.0, 1.0);
        let collided = utils::check_collision(&b1, &b2);
        assert_eq!(collided, true)
    }

    #[test]
    fn no_collision() {
        let b1 = Body::new(0.0, 0.0, 1.0, 4.0, 2.0, 0.4);
        let b2 = Body::new(1.0, 1.0, 0.0, 0.0, 0.0, 1.0);
        let collided = utils::check_collision(&b1, &b2);
        assert_eq!(collided, false)
    }
}