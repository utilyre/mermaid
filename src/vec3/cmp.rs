use super::Vec3;
use std::cmp::Ordering;

impl PartialOrd for Vec3 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ox = self.x.partial_cmp(&other.x)?;
        let oy = self.y.partial_cmp(&other.y)?;
        let oz = self.z.partial_cmp(&other.z)?;

        (ox == oy && oy == oz).then_some(ox)
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        let eps = Vec3::splat(f32::EPSILON);
        let sub = *self - *other;

        -eps <= sub && sub <= eps
    }
}
