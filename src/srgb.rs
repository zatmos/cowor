mod conversions;
#[cfg(test)]
mod tests;

use std::fmt;

/// A struct representing a color in the sRGB color space.
#[derive(Clone, Copy)]
pub struct Srgb(f32, f32, f32); // r, g, b

//Gamma, a value used to "compress" linear RGB values so to give more precision for darker values.
const GAMMA: f32 = 2.4;

/// Expands (or makes linear) a gamma compressed value.
///
/// # Examples
///
/// ```
/// use cowor::{ Srgb, srgb::gamma_expand };
/// let srgb = Srgb::new(10, 20, 30);
/// assert_eq!(gamma_expand(srgb.red()), srgb.linear_red());
/// ```
pub fn gamma_expand(x: f32) -> f32 {
    match x > 0.04045f32 {
        true    => f32::powf((x + 0.055f32) / 1.055f32, GAMMA),
        false   => x / 12.92f32,
    }
}

/// Compresses a gamma expanded (or linear) value.
///
/// # Examples
///
/// ```
/// use cowor::{ Srgb, srgb::gamma_compress };
/// let srgb = Srgb::new(10, 20, 30);
/// assert_eq!(gamma_compress(srgb.linear_red()), srgb.red());
/// ```
pub fn gamma_compress(x: f32) -> f32 {
    match x > 0.0031308f32 {
        true    => f32::powf(x, 1f32 / GAMMA) * 1.055f32 - 0.055f32,
        false   => x * 12.92f32,
    }
}

impl Srgb {
    /// Creates a Srgb instance from 3 unsigned 8 bit integers representing the red, green, and
    /// blue components.
    ///
    /// # Examples
    ///
    /// ```
    /// use cowor::Srgb;
    /// let srgb = Srgb::new(10, 20, 30);
    /// ```
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        let [norm_r, norm_g, norm_b] = [red, green, blue].map(|x| x as f32 / 255f32);
        Self(norm_r, norm_g, norm_b)
    }

    /// Red component of a sRGB color.
    /// Value is normalized (between 0 and 1 included).
    ///
    /// # Examples
    ///
    /// ```
    /// use cowor::Srgb;
    /// let srgb = Srgb::new(10, 20, 30);
    /// assert_eq!(srgb.red(), 10f32 / 255f32);
    /// ```
    pub fn red(&self) -> f32 {
        self.0
    }

    /// Green component of a sRGB color.
    /// Value is normalized (between 0 and 1 included).
    ///
    /// # Examples
    ///
    /// ```
    /// use cowor::Srgb;
    /// let srgb = Srgb::new(10, 20, 30);
    /// assert_eq!(srgb.green(), 20f32 / 255f32);
    /// ```
    pub fn green(&self) -> f32 {
        self.1
    }

    /// Blue component of a sRGB color.
    /// Value is normalized (between 0 and 1 included).
    ///
    /// # Examples
    ///
    /// ```
    /// use cowor::Srgb;
    /// let srgb = Srgb::new(10, 20, 30);
    /// assert_eq!(srgb.blue(), 30f32 / 255f32);
    /// ```
    pub fn blue(&self) -> f32 {
        self.2
    }

    /// Red component of a sRGB color as an 8 bit unsigned integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use cowor::Srgb;
    /// let srgb = Srgb::new(10, 20, 30);
    /// assert_eq!(srgb.red_as_u8(), 10);
    /// ```
    pub fn red_as_u8(&self) -> u8 {
        (self.0 * 255f32).round() as u8
    }

    /// Green component of a sRGB color as an 8 bit unsigned integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use cowor::Srgb;
    /// let srgb = Srgb::new(10, 20, 30);
    /// assert_eq!(srgb.green_as_u8(), 20);
    /// ```
    pub fn green_as_u8(&self) -> u8 {
        (self.1 * 255f32).round() as u8
    }

    /// Blue component of a sRGB color as an 8 bit unsigned integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use cowor::Srgb;
    /// let srgb = Srgb::new(10, 20, 30);
    /// assert_eq!(srgb.blue_as_u8(), 30);
    /// ```
    pub fn blue_as_u8(&self) -> u8 {
        (self.2 * 255f32).round() as u8
    }

    /// Linear (or gamma expanded) red component of a sRGB color.
    /// Value is normalized (between 0 and 1 included).
    ///
    /// # Examples
    ///
    /// ```
    /// use cowor::{ Srgb, srgb::gamma_expand };
    /// let srgb = Srgb::new(10, 20, 30);
    /// assert_eq!(srgb.linear_red(), gamma_expand(srgb.red()));
    /// ```
    pub fn linear_red(&self) -> f32 {
        gamma_expand(self.0)
    }

    /// Linear (or gamma expanded) green component of a sRGB color.
    /// Value is normalized (between 0 and 1 included).
    ///
    /// # Examples
    ///
    /// ```
    /// use cowor::{ Srgb, srgb::gamma_expand };
    /// let srgb = Srgb::new(10, 20, 30);
    /// assert_eq!(srgb.linear_green(), gamma_expand(srgb.green()));
    /// ```
    pub fn linear_green(&self) -> f32 {
        gamma_expand(self.1)
    }

    /// Linear (or gamma expanded) blue component of a sRGB color.
    /// Value is normalized (between 0 and 1 included).
    ///
    /// # Examples
    ///
    /// ```
    /// use cowor::{ Srgb, srgb::gamma_expand };
    /// let srgb = Srgb::new(10, 20, 30);
    /// assert_eq!(srgb.linear_blue(), gamma_expand(srgb.blue()));
    /// ```
    pub fn linear_blue(&self) -> f32 {
        gamma_expand(self.2)
    }
}


/// Two Srgb instances are considered equal if all their components as
/// unsigned 8 bit integers are equal.
impl PartialEq for Srgb {
    fn eq(&self, other: &Self) -> bool {
        let lhs: [u8; 3] = (*self).into();
        let rhs: [u8; 3] = (*other).into();
        (0..3).all(|x| lhs[x] == rhs[x])
    }
}

//Srgb needs a special implementation of Debug because it uses floats internally.
impl fmt::Debug for Srgb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Srgb({}, {}, {})", self.red_as_u8(), self.green_as_u8(), self.blue_as_u8())
    }
}
