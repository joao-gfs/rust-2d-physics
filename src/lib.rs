use vec2d::Vec2;

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

    fn apply_force(&mut self, force: f32) {
        let acceleration = force / self.mass;
        self.acceleration += acceleration;
    }

    fn update(&mut self, delta_t: f32) {
        self.velocity += self.acceleration * delta_t;
        self.position += self.velocity * delta_t;
        self.acceleration = Vec2::new(0.0, 0.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
