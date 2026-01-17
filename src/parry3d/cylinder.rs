use parry3d::shape::Cylinder;

pub trait CylinderExt {
    fn height(&self) -> f32;
}

impl CylinderExt for Cylinder {
    fn height(&self) -> f32 {
        return self.half_height * 2.0;
    }
}
