use crate::{
    cielab::Cielab,
    cielch::Cielch,
    ciexyz::Ciexyz,
    error::OutOfGamut,
};
use super::{
    gamma_compress,
    Srgb,
};

//Conversion matrix to convert from CIEXYZ to sRGB.
const FROM_CIEXYZ_CONVERSION_MATRIX: [f32; 9] = {
    let xr = 12831f32 / 3959f32;
    let xg = -329f32 / 214f32;
    let xb = -1974f32 / 3959f32;
    let yr = -851781f32 / 878810f32;
    let yg = 1648619f32 / 878810f32;
    let yb = 36519f32 / 878810f32;
    let zr = 705f32 / 12673f32;
    let zg = -2585f32 / 12673f32;
    let zb = 705f32 / 667f32;
    [xr, xg, xb, yr, yg, yb, zr, zg, zb]
};

/// Conversion from CIEXYZ to sRGB.
/// The conversion may fail and return an OutOfGamut error
/// if the CIEXYZ color can not be represented in the sRGB color gamut.
///
/// # Examples
///
/// ```
/// use cowor::{ Ciexyz, Srgb };
/// let valid = Ciexyz::new(0.5, 0.5, 0.5).unwrap(); // Will land in the sRGB color gamut.
/// let invalid = Ciexyz::new(4.0, 1.0, 6.0).unwrap(); // Won't land in the sRGB color gamut.
/// assert!(Srgb::try_from(valid).is_ok());
/// assert!(Srgb::try_from(invalid).is_err());
/// ```
impl TryFrom<Ciexyz> for Srgb {
    type Error = OutOfGamut;

    fn try_from(ciexyz: Ciexyz) -> Result<Self, Self::Error> {
        let [xr, xg, xb, yr, yg, yb, zr, zg, zb] = FROM_CIEXYZ_CONVERSION_MATRIX;
        let [x, y, z] = [ciexyz.x(), ciexyz.y(), ciexyz.z()];
        let lr = x * xr + y * xg + z * xb;
        let lg = x * yr + y * yg + z * yb;
        let lb = x * zr + y * zg + z * zb;
        let [r, g, b] = [lr, lg, lb].map(gamma_compress);
        match [r, g, b].iter().all(|x| (0f32..=1f32).contains(x)) {
            true    => Ok(Self(r, g, b)),
            false   => Err(OutOfGamut),
        }
    }
}

/// Conversion from CIELAB to sRGB.
/// The conversion may fail and return an OutOfGamut error
/// if the CIELAB color can not be represented in the sRGB color gamut.
///
/// # Examples
///
/// ```
/// use cowor::{ Cielab, Srgb };
/// let valid = Cielab::new(50.0, 0.0, 0.0).unwrap(); // Will land in the sRGB color gamut.
/// let invalid = Cielab::new(100.0, 100.0, 0.0).unwrap(); // Won't land in the sRGB color gamut.
/// assert!(Srgb::try_from(valid).is_ok());
/// assert!(Srgb::try_from(invalid).is_err());
/// ```
impl TryFrom<Cielab> for Srgb {
    type Error = OutOfGamut;

    fn try_from(cielab: Cielab) -> Result<Self, Self::Error> {
        let ciexyz: Ciexyz = cielab.into();
        Self::try_from(ciexyz)
    }
}

/// Conversion from CIELCh to sRGB.
/// The conversion may fail and return an OutOfGamut error
/// if the CIELCh color can not be represented in the sRGB color gamut.
///
/// # Examples
///
/// ```
/// use cowor::{ Cielch, Srgb };
/// let valid = Cielch::new(50.0, 0.0, 0.0).unwrap(); // Will land in the sRGB color gamut.
/// let invalid = Cielch::new(100.0, 100.0, 0.0).unwrap(); // Won't land in the sRGB color gamut.
/// assert!(Srgb::try_from(valid).is_ok());
/// assert!(Srgb::try_from(invalid).is_err());
/// ```
impl TryFrom<Cielch> for Srgb {
    type Error = OutOfGamut;

    fn try_from(cielch: Cielch) -> Result<Self, Self::Error> {
        let ciexyz: Ciexyz = cielch.into();
        Self::try_from(ciexyz)
    }
}

/// New sRGB color from an array of 3 unsigned 8 bit integers.
///
/// # Examples
///
/// ```
/// use cowor::Srgb;
/// let srgb: Srgb = [10, 20, 30].into();
/// ```
impl From<[u8; 3]> for Srgb {
    fn from(array: [u8; 3]) -> Self {
        let [r, g, b] = array;
        Self::new(r, g, b)
    }
}

/// New sRGB color from an array of 3 floats.
///
/// # Examples
///
/// ```
/// use cowor::Srgb;
/// let srgb: Srgb = [0.1, 0.2, 0.3].into();
/// ```
impl From<[f32; 3]> for Srgb {
    fn from(array: [f32; 3]) -> Self {
        let [r, g, b] = array;
        Self(r, g, b)
    }
}
