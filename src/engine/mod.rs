mod photon;

use image::ImageOutputFormat;

use crate::Spec;

pub use photon::Photon;

/// A trait for image engine.
pub trait Engine {
    /// Apply the specs to the image.
    fn apply(&mut self, specs: &[Spec]);
    /// Generate the image with the given format.
    fn generate(self, format: ImageOutputFormat) -> Vec<u8>;
}

/// A trait for transforming the image.
pub trait SpecTransform<T> {
    /// Transform the image with the given operation.
    fn transform(&mut self, op: T);
}
