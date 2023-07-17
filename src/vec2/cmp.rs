use super::Vec2;
use std::cmp::Ordering;

impl PartialOrd for Vec2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ox = self.x.partial_cmp(&other.x)?;
        let oy = self.y.partial_cmp(&other.y)?;

        (ox == oy).then_some(ox)
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        let eps = Vec2::splat(f32::EPSILON);
        let sub = *self - *other;

        -eps <= sub && sub <= eps
    }
}
