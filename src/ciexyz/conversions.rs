use crate::{
    Cielab,
    Cielch,
    Srgb
};
use super::{
    Ciexyz,
    D65,
};

//Conversion matrix to convert from sRGB to CIEXYZ.
pub(super) const FROM_SRGB_CONVERSION_MATRIX: [f32; 9] = {
    let xr = 506752f32 / 1228815f32;
    let xg = 87881f32 / 245763f32;
    let xb = 12673f32 / 70218f32;
    let yr = 87098f32 / 409605f32;
    let yg = 175762f32 / 245763f32;
    let yb = 12673f32 / 175545f32;
    let zr = 7918f32 / 409605f32;
    let zg = 87881f32 / 737289f32;
    let zb = 1001167f32 / 1053270f32;
    [xr, xg, xb, yr, yg, yb, zr, zg, zb]
};

/// Conversion from sRGB to CIEXYZ.
///
/// # Examples
///
/// ```
/// use cowor::{ Ciexyz, Srgb };
/// let ciexyz: Ciexyz = Srgb::new(10, 20, 30).into();
/// ```
impl From<Srgb> for Ciexyz {
    fn from(srgb: Srgb) -> Self {
        let lr = srgb.linear_red();
        let lg = srgb.linear_green();
        let lb = srgb.linear_blue();
        let [xr, xg, xb, yr, yg, yb, zr, zg, zb] = FROM_SRGB_CONVERSION_MATRIX;
        let x = lr * xr + lg * xg + lb * xb;
        let y = lr * yr + lg * yg + lb * yb;
        let z = lr * zr + lg * zg + lb * zb;
        Self(x, y, z)
    }
}

/// Conversion from CIELAB to CIEXYZ.
///
/// # Examples
///
/// ```
/// use cowor::{ Cielab, Ciexyz };
/// let ciexyz: Ciexyz = Cielab::new(10.0, 20.0, 30.0).unwrap().into();
/// ```
impl From<Cielab> for Ciexyz {
    fn from(cielab: Cielab) -> Self {
        let delta = 6f32 / 29f32;
        let f = |t| match t > delta {
            true    => f32::powf(t, 3f32),
            false   => 3f32 * f32::powf(delta, 2f32) * (t - 4f32 / 29f32),
        };
        let p = (cielab.lightness() + 16f32) / 116f32;
        let x = D65.x() * f(p + cielab.a() / 500f32);
        let y = D65.y() * f(p);
        let z = D65.z() * f(p - cielab.b() / 200f32);
        Self(x, y, z)
    }
}

/// Conversion from CIELCh to CIEXYZ.
///
/// # Examples
///
/// ```
/// use cowor::{ Cielch, Ciexyz };
/// let ciexyz: Ciexyz = Cielch::new(10.0, 20.0, 30f32.to_radians()).unwrap().into();
/// ```
impl From<Cielch> for Ciexyz {
    fn from(cielch: Cielch) -> Self {
        let cielab: Cielab = cielch.into();
        Self::from(cielab)
    }
}

/// New CIEXYZ color from an array of 3 floats.
///
/// # Examples
///
/// ```
/// use cowor::Ciexyz;
/// let ciexyz: Ciexyz = [0.1, 0.2, 0.3].into();
/// ```
impl From<[f32; 3]> for Ciexyz {
    fn from(array: [f32; 3]) -> Self {
        let [x, y, z] = array;
        Self(x, y, z)
    }
}

/// Convert an CIEXYZ color into an array of 3 floats.
///
/// # Examples
///
/// ```
/// use cowor::Ciexyz;
/// let [x, y, z]: [f32; 3] = Ciexyz::new(0.1, 0.2, 0.3).unwrap().into();
/// ```
impl From<Ciexyz> for [f32; 3] {
    fn from(ciexyz: Ciexyz) -> Self {
        [ciexyz.x(), ciexyz.y(), ciexyz.z()]
    }
}
