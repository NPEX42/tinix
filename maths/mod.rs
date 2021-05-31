pub fn map01_f(x : f32, min : f32, max : f32) -> f32 {
    map_f(x, min, max, 0f32, 1f32)
}

pub fn map_f(x : f32, init_min : f32, init_max : f32, new_min : f32, new_max : f32) -> f32 {
    new_min + (new_max - new_min) * (x - init_min) / (init_max - init_min)
}

pub fn map01_d(x : f64, min : f64, max : f64) -> f64 {
    map_d(x, min, max, 0f64, 1f64)
}

pub fn map_d(x : f64, init_min : f64, init_max : f64, new_min : f64, new_max : f64) -> f64 {
    new_min + (new_max - new_min) * (x - init_min) / (init_max - init_min)
}

pub fn clamp_us(x : usize, min : usize, max : usize) -> usize {
    if x < min {min} 
    else if x > max {max} 
    else {x}
}