mod vec2d;

pub struct Body {
    position: vec2d::Vec2,
    velocity: vec2d::Vec2,
    acceleration: vec2d::Vec2,
    radius: f32
}

impl Body {
    fn new(x: f32, y: f32, vx: f32, vy: f32, ax: f32, ay: f32, r: f32) -> Self {
        Body {
            position: vec2d::Vec2::new(x, y),
            velocity: vec2d::Vec2::new(vx, vy),
            acceleration: vec2d::Vec2::new(ax, ay),
            radius: r,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
