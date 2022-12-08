use super::Cielch;
use crate::{
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
