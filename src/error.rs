use std::error;
use std::fmt;

/// An error for when a color, through a conversion, can not be represented in the color gamut of a
/// destination color space.
#[derive(Debug)]
pub struct OutOfGamut;

impl fmt::Display for OutOfGamut {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "color is out of gamut")
    }
}

impl error::Error for OutOfGamut {}
