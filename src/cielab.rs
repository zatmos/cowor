mod conversions;
#[cfg(test)]
mod tests;

/// A struct representing a color in the CIELAB color space.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Cielab(f32, f32, f32); // lightness, a, b

impl Cielab {
    /// Creates a Cielab instance from 3 float
    /// representing the lightness, a, and b components.
    ///
    /// The lightness component must between 0 and 100 included
    /// otherwise it doesn't represent a valid CIELAB color
    /// and the return value is None.
    ///
    /// # Examples
    ///
    /// ```
    /// use cowor::Cielab;
    /// let valid = Cielab::new(10.0, 20.0, 30.0); // Ok.
    /// let invalid = Cielab::new(110.0, 20.0, 30.0); // Not ok. The lightness component is greater than 100.
    /// assert!(valid.is_some());
    /// assert!(invalid.is_none());
    /// ```
    pub fn new(lightness: f32, a: f32, b: f32) -> Option<Self> {
        match (0f32..=100f32).contains(&lightness) {
            true    => Some(Self(lightness, a, b)),
            false   => None,
        }
    }

    /// Lightness component of a CIELAB color.
    /// Value is between 0 and 100 included.
    ///
    /// # Examples
    ///
    /// ```
    /// use cowor::Cielab;
    /// let cielab = Cielab::new(10.0, 20.0, 30.0).unwrap();
    /// assert_eq!(cielab.lightness(), 10.0);
    /// ```
    pub fn lightness(&self) -> f32 {
        self.0
    }

    /// A component of a CIELAB color.
    ///
    /// # Examples
    ///
    /// ```
    /// use cowor::Cielab;
    /// let cielab = Cielab::new(10.0, 20.0, 30.0).unwrap();
    /// assert_eq!(cielab.a(), 20.0);
    /// ```
    pub fn a(&self) -> f32 {
        self.1
    }

    /// B component of a CIELAB color.
    ///
    /// # Examples
    ///
    /// ```
    /// use cowor::Cielab;
    /// let cielab = Cielab::new(10.0, 20.0, 30.0).unwrap();
    /// assert_eq!(cielab.b(), 30.0);
    /// ```
    pub fn b(&self) -> f32 {
        self.2
    }
}
