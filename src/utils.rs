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

pub fn i32_tuple_from_vec2(vec2: &Vec2) -> (i32, i32) {
    (vec2[0] as i32, vec2[1] as i32)
}
