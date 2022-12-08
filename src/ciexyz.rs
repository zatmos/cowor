mod conversions;
#[cfg(test)]
mod tests;

use conversions::FROM_SRGB_CONVERSION_MATRIX;

/// A struct representing a color in the CIEXYZ color space.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Ciexyz(f32, f32, f32); // x, y, z

/// D65 white point. Equivalent to Ciexyz(0.9504559f32, 1f32, 1.0890577f32);
pub const D65: Ciexyz = {
    let [xr, xg, xb, yr, yg, yb, zr, zg, zb] = FROM_SRGB_CONVERSION_MATRIX;
    let x = xr + xg + xb;
    let y = yr + yg + yb;
    let z = zr + zg + zb;
    Ciexyz(x, y, z)
};

impl Ciexyz {
    /// Creates a Ciexyz instance from 3 float
    /// representing the X, Y, and Z components.
    ///
    /// Each value must be positive and Y must also be less than or equal to 1
    /// otherwise it doesn't represent a valid CIEXYZ color
    /// and the return value is None.
    ///
    /// # Examples
    ///
    /// ```
    /// use cowor::Ciexyz;
    /// let valid = Ciexyz::new(0.1, 0.2, 0.3); // Ok.
    /// let invalid = Ciexyz::new(-0.1, 0.2, 0.3); // Not ok. One of the components is negative.
    /// let also_invalid = Ciexyz::new(0.1, 2.0, 0.3); // Not ok. Y is bigger than 1.
    /// assert!(valid.is_some());
    /// assert!(invalid.is_none());
    /// assert!(also_invalid.is_none());
    /// ```
    pub fn new(x: f32, y: f32, z: f32) -> Option<Self> {
        match x >= 0f32 && (0f32..=1f32).contains(&y) && z >= 0f32 {
            true    => Some(Self(x, y, z)),
            false   => None,
        }
    }

    /// X component of a CIEXYZ color.
    ///
    /// # Examples
    ///
    /// ```
    /// use cowor::Ciexyz;
    /// let ciexyz = Ciexyz::new(0.1, 0.2, 0.3).unwrap();
    /// assert_eq!(ciexyz.x(), 0.1);
    /// ```
    pub fn x(&self) -> f32 {
        self.0
    }

    /// Y component of a CIEXYZ color.
    /// Value is normalized (between 0 and 1 included).
    ///
    /// # Examples
    ///
    /// ```
    /// use cowor::Ciexyz;
    /// let ciexyz = Ciexyz::new(0.1, 0.2, 0.3).unwrap();
    /// assert_eq!(ciexyz.y(), 0.2);
    /// ```
    pub fn y(&self) -> f32 {
        self.1
    }

    /// Z component of a CIEXYZ color.
    ///
    /// # Examples
    ///
    /// ```
    /// use cowor::Ciexyz;
    /// let ciexyz = Ciexyz::new(0.1, 0.2, 0.3).unwrap();
    /// assert_eq!(ciexyz.z(), 0.3);
    /// ```
    pub fn z(&self) -> f32 {
        self.2
    }
}
