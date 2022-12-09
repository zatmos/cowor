mod conversions;
#[cfg(test)]
mod tests;

/// A struct representing a color in the CIELCh color space.
#[derive(Debug, Clone, Copy)]
pub struct Cielch(f32, f32, f32); // lightness, chroma, hue

impl Cielch {
    /// Creates a Cielch instance from 3 float
    /// representing the lightness, chroma, and hue components.
    ///
    /// The lightness component must between 0 and 100 included
    /// otherwise it doesn't represent a valid CIELCh color
    /// and the return value is None.
    ///
    /// The hue component is in radians.
    ///
    /// # Examples
    ///
    /// ```
    /// use cowor::Cielch;
    /// let valid = Cielch::new(10.0, 20.0, 30f32.to_radians()); // Ok.
    /// let invalid = Cielch::new(110.0, 20.0, 30f32.to_radians()); // Not ok. The lightness component is greater than 100.
    /// assert!(valid.is_some());
    /// assert!(invalid.is_none());
    /// ```
    pub fn new(lightness: f32, chroma: f32, hue: f32) -> Option<Self> {
        match (0f32..=100f32).contains(&lightness) && chroma >= 0.0 {
            true    => Some(Self(lightness, chroma, hue)),
            false   => None,
        }
    }

    /// Lightness component of a CIELCh color.
    /// Value is between 0 and 100 included.
    ///
    /// # Examples
    ///
    /// ```
    /// use cowor::Cielch;
    /// let cielab = Cielch::new(10.0, 20.0, 30f32.to_radians()).unwrap();
    /// assert_eq!(cielab.lightness(), 10.0);
    /// ```
    pub fn lightness(&self) -> f32 {
        self.0
    }

    /// Chroma component of a CIELCh color.
    ///
    /// # Examples
    ///
    /// ```
    /// use cowor::Cielch;
    /// let cielab = Cielch::new(10.0, 20.0, 30f32.to_radians()).unwrap();
    /// assert_eq!(cielab.chroma(), 20.0);
    /// ```
    pub fn chroma(&self) -> f32 {
        self.1
    }

    /// Hue component of a CIELCh color.
    /// Value is in radians.
    ///
    /// # Examples
    ///
    /// ```
    /// use cowor::Cielch;
    /// let cielab = Cielch::new(10.0, 20.0, 30f32.to_radians()).unwrap();
    /// assert_eq!(cielab.hue(), 30f32.to_radians());
    /// ```
    pub fn hue(&self) -> f32 {
        self.2
    }
}

/// Two Cielch instances are equal if all their components are equal
/// unless the chroma components are both 0 in which case
/// only the lightness components are compared.
///
/// # Examples
///
/// ```
/// use cowor::Cielch;
/// let a = Cielch::new(10.0, 20.0, 30f32.to_radians()).unwrap();
/// let b = Cielch::new(10.0, 0.0, 30f32.to_radians()).unwrap();
/// let c = Cielch::new(10.0, 0.0, 60f32.to_radians()).unwrap();
/// assert_ne!(a, b);
/// assert_eq!(b, c);
/// assert_eq!(a, a);
/// ```
impl PartialEq for Cielch {
    fn eq(&self, other: &Self) -> bool {
        let lhs = [self.lightness(), self.chroma(), self.hue()];
        let rhs = [other.lightness(), other.chroma(), other.hue()];
        match self.chroma() != 0f32 || other.chroma() != 0f32 {
            true    => (0..3).all(|x| lhs[x] == rhs[x]),
            false   => self.lightness() == other.lightness(),
        }
    }
}
