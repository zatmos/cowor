# cowor

A color conversion and manipulation library with color spaces' specifications enforced through the type system.
All colors are always valid and there are no "unclamped" values.

## Current state

Currently supporting the following color spaces:

- sRGB
- CIEXYZ
- CIELAB
- CIELCh(ab)

and the conversions between each of those.

## How to use

Currently, only the conversions are implemented.
To perform a conversion between two color spaces, simply use `from`/`into`
for conversions in which the starting color space's colors can always be represented in the destination color space.
Otherwise, use `try_from`/`try_into`. The error when the starting color falls outside the gamut of the destination color space
is a `OutOfGamut` error.

## License

Licensed under the GNU General Public License, Version 3.0
