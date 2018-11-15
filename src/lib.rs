pub mod camera;
pub mod material;
pub mod object;
pub mod ray;
pub mod sphere;

use rulinalg::vector::Vector;
use rulinalg::norm::Euclidean;

pub fn unitize(vect: &Vector<f64>) -> Vector<f64> {
    let norm = vect.norm(Euclidean);
    vect / norm
}
