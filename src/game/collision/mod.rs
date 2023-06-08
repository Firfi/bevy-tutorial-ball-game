use bevy::prelude::*;

pub fn detect_collision_circles(
    (translation_a, size_a): (&Vec3, f32),
    (translation_b, size_b): (&Vec3, f32),
) -> bool {
    // TODO glam why we clone???
    let distance = translation_a.distance(translation_b.clone());
    distance < size_a / 2.0 + size_b / 2.0
}
