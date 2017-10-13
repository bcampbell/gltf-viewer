use std::f32;

use cgmath;
pub use cgmath::prelude::*;
pub use cgmath::{vec3, vec4};

use num_traits::clamp;

pub type Vector2 = cgmath::Vector2<f32>;
pub type Vector3 = cgmath::Vector3<f32>;
pub type Vector4 = cgmath::Vector4<f32>;

pub type Point3 = cgmath::Point3<f32>;

pub type Matrix4 = cgmath::Matrix4<f32>;
pub type Quaternion = cgmath::Quaternion<f32>;

// TODO!!: replace bounds with collision-rs AAbb3?
/// Axis-aligned Bounding Box
#[derive(Clone, Debug)]
pub struct Bounds {
    pub min: Vector3,
    pub max: Vector3,
}

impl Default for Bounds {
    fn default() -> Self {
        Self {
            min: Vector3::from_value(f32::INFINITY),
            max: Vector3::from_value(f32::NEG_INFINITY),
        }
    }
}

impl Bounds {
    // TODO: make self-mutating? (if not switching to collision-rs)
    pub fn union(&self, other: &Self) -> Bounds {
        Bounds {
            min: vec3(
                self.min.x.min(other.min.x),
                self.min.y.min(other.min.x),
                self.min.z.min(other.min.x),
            ),
            max: vec3(
                self.max.x.max(other.max.x),
                self.max.y.max(other.max.x),
                self.max.z.max(other.max.x),
            ),
        }
    }

    /// Generate 8 points representing the corners of the box
    fn points(&self) -> [Vector3; 8] {
        // 2^3 combinations
        [
			vec3(self.min.x, self.min.y, self.min.z), // 000
			vec3(self.min.x, self.min.y, self.max.z), // 001
			vec3(self.min.x, self.max.y, self.min.z), // 010
			vec3(self.min.x, self.max.y, self.max.z), // 011
			vec3(self.max.x, self.min.y, self.min.z), // 100
			vec3(self.max.x, self.min.y, self.max.z), // 101
			vec3(self.max.x, self.max.y, self.min.z), // 110
			vec3(self.max.x, self.max.y, self.max.z), // 111
        ]
    }

    // TODO!: unit test?
    pub fn transform(&self, matrix: &Matrix4) -> Bounds {
        let transformed: Vec<_> = self.points().iter()
            .map(|p| matrix.transform_vector(*p))
            .collect(); // TODO: need transform_point??

        let mut bounds = Bounds::default();
        for point in transformed {
            bounds = bounds.union(&Bounds {min: point, max: point})
        }

        bounds
    }

    /// diagonal vector of this AABB
    pub fn size(&self) -> Vector3 {
        self.max - self.min
    }

    pub fn center(&self) -> Vector3 {
        (self.min + self.max) / 2.0
    }

    // TODO!: intersectsPlane? (three)

    /// Check if max >= min. Note: default bounds are not valid.
    pub fn is_valid(&self) -> bool {
        self.max.x >= self.min.x &&
        self.max.y >= self.min.y &&
        self.max.z >= self.min.z
    }
}

use std::convert::From;
use gltf;
impl From<gltf::mesh::Bounds<[f32; 3]>> for Bounds {
    fn from(bounds: gltf::mesh::Bounds<[f32; 3]>) -> Self {
        Bounds {
            min: bounds.min.into(),
            max: bounds.max.into()
        }
    }
}

// A point's spherical coordinates, inspired by ThreeJS version
pub struct Spherical {
    pub radius: f32,
    pub phi: f32,
    pub theta: f32,
}

impl Default for Spherical {
    fn default() -> Self {
        Spherical { radius: 1.0, phi: 0.0, theta: 0.0 }
    }
}

impl Spherical {
    pub fn from_vec3(vec3: Vector3) -> Self {
        let radius = vec3.magnitude();
        let (theta, phi) = if radius == 0.0 {
            (0.0, 0.0)
        } else {
            (
                vec3.x.atan2(vec3.z), // equator angle around y-up axis
                clamp(vec3.y / radius, -1.0, 1.0).acos() // polar angle
            )
        };
        Self {
            radius,
            theta,
            phi
        }
    }

    pub fn to_vec3(&self) -> Vector3 {
        let sin_phi_radius = self.phi.sin() * self.radius;
        let x = sin_phi_radius * self.theta.sin();
        let y = self.phi.cos() * self.radius;
        let z = sin_phi_radius * self.theta.cos();
        vec3(x, y, z)
    }
}
