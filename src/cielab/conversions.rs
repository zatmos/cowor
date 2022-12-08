use super::Cielab;
use crate::{
    Cielch,
    Ciexyz,
    ciexyz::D65,
    Srgb,
};

/// Conversion from CIEXYZ to CIELAB.
///
/// # Examples
///
/// ```
/// use cowor::{ Cielab, Ciexyz };
/// let cielab: Cielab = Ciexyz::new(0.1, 0.2, 0.3).unwrap().into();
/// ```
impl From<Ciexyz> for Cielab {
    fn from(ciexyz: Ciexyz) -> Self {
        let delta = 6f32 / 29f32;
        let f = |t| match t > f32::powf(delta, 3f32) {
            true    => f32::powf(t, 1f32 / 3f32),
            false   => t / (3f32 * f32::powf(delta, 2f32)) + 4f32 / 29f32,
        };
        let x_ratio = ciexyz.x() / D65.x();
        let y_ratio = ciexyz.y() / D65.y();
        let z_ratio = ciexyz.z() / D65.z();
        let l = 116f32 * f(y_ratio) - 16f32;
        let a = 500f32 * (f(x_ratio) - f(y_ratio));
        let b = 200f32 * (f(y_ratio) - f(z_ratio));
        Self(l, a, b)
    }
}

/// Conversion from sRGB to CIEXYZ.
///
/// # Examples
///
/// ```
/// use cowor::{ Cielab, Srgb };
/// let cielab: Cielab = Srgb::new(10, 20, 30).into();
/// ```
impl From<Srgb> for Cielab {
    fn from(srgb: Srgb) -> Self {
        let ciexyz: Ciexyz = srgb.into();
        Self::from(ciexyz)
    }
}

/// Conversion from CIELCh to CIELAB.
///
/// # Examples
///
/// ```
/// use cowor::{ Cielab, Cielch };
/// let cielab: Cielab = Cielch::new(10.0, 20.0, 30.0).unwrap().into();
/// ```
impl From<Cielch> for Cielab {
    fn from(cielch: Cielch) -> Self {
        let l = cielch.lightness();
        let a = cielch.chroma() * cielch.hue().cos();
        let b = cielch.chroma() * cielch.hue().sin();
        Self(l, a, b)
    }
}
