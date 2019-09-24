use tetra::graphics::Vec2;

pub fn vec2_from_reverse_usize_tuple(&(y, x): &(usize, usize)) -> Vec2 {
    vec2_from_usize_tuple(&(x, y))
}

pub fn vec2_from_usize_tuple(tuple: &(usize, usize)) -> Vec2 {
    Vec2::new(tuple.0 as f32, tuple.1 as f32)
}

pub fn vec2_from_tuple(tuple: &(f32, f32)) -> Vec2 {
    Vec2::new(tuple.0, tuple.1)
}
