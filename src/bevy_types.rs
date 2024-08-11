use bevy::prelude::Transform;

use crate::interpolation::Interpolate;



impl Interpolate for Transform {
    fn interpolate(&self, other: Self, t: f32) -> Self {
        let translation = self.translation.lerp(other.translation, t);
        let rotation = self.rotation.lerp(other.rotation, t);
        let scale = self.scale.lerp(other.scale, t);
        return Self { translation, rotation, scale };
    }
}