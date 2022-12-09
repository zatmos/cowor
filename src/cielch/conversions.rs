use super::Cielch;
use crate::{
    error::OutOfSpecification,
    Cielab,
    Ciexyz,
    Srgb,
};

/// Conversion from CIELAB to CIELCh.
///
/// # Examples
///
/// ```
/// use cowor::{ Cielab, Cielch };
/// let cielab: Cielch = Cielab::new(10.0, 20.0, 30.0).unwrap().into();
/// ```
impl From<Cielab> for Cielch {
    fn from(cielab: Cielab) -> Self {
        let l = cielab.lightness();
        let a = cielab.a();
        let b = cielab.b();
        let c = f32::sqrt(a * a + b * b);
        let h = f32::atan2(b, a);
        Self(l, c, h)
    }
}

/// Conversion from CIEXYZ to CIELCh.
///
/// # Examples
///
/// ```
/// use cowor::{ Ciexyz, Cielch };
/// let cielab: Cielch = Ciexyz::new(0.1, 0.2, 0.3).unwrap().into();
/// ```
impl From<Ciexyz> for Cielch {
    fn from(ciexyz: Ciexyz) -> Self {
        let cielab: Cielab = ciexyz.into();
        Self::from(cielab)
    }
}

/// Conversion from sRGB to CIELCh.
///
/// # Examples
///
/// ```
/// use cowor::{ Srgb, Cielch };
/// let cielab: Cielch = Srgb::new(10, 20, 30).into();
/// ```
impl From<Srgb> for Cielch {
    fn from(srgb: Srgb) -> Self {
        let cielab: Cielab = srgb.into();
        Self::from(cielab)
    }
}

/// New CIELCh color from an array of 3 floats.
/// Convertion may fail if the resulting color would fall outside
/// the CIELCh specification. The error type in that case is an
/// OutOfSpecification error.
///
/// # Examples
///
/// ```
/// use cowor::Cielch;
/// let cielch: Cielch = [10.0, 20.0, 30f32.to_radians()].try_into().unwrap();
/// ```
impl TryFrom<[f32; 3]> for Cielch {
    type Error = OutOfSpecification;

    fn try_from(array: [f32; 3]) -> Result<Self, Self::Error> {
        let [l, c, h] = array;
        match (0f32..100f32).contains(&l) {
            true    => Ok(Self(l, c, h)),
            false   => Err(OutOfSpecification),
        }
    }
}

/// Convert an CIELCh color into an array of 3 floats.
///
/// # Examples
///
/// ```
/// use cowor::Cielch;
/// let [x, y, z]: [f32; 3] = Cielch::new(10.0, 20.0, 30f32.to_radians()).unwrap().into();
/// ```
impl From<Cielch> for [f32; 3] {
    fn from(cielch: Cielch) -> Self {
        let Cielch(l, c, h) = cielch;
        [l, c, h]
    }
}
