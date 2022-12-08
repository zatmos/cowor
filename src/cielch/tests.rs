use super::*;
use crate::Cielab;

#[test]
fn new() {
    let a = Cielch(10.0, 20.0, 30f32.to_radians());
    let b = Cielch::new(10.0, 20.0, 30f32.to_radians()).unwrap();
    let c = Cielch::new(-10.0, 20.0, 30f32.to_radians());
    let d = Cielch::new(110.0, 20.0, 30f32.to_radians());
    assert_eq!(a, b);
    assert!(c.is_none());
    assert!(d.is_none());
}

#[test]
fn lightness() {
    let a = Cielch(0.0, 20.0, 30f32.to_radians());
    assert_eq!(a.lightness(), 0.0);
    let b = Cielch(10.0, 20.0, 30f32.to_radians());
    assert_eq!(b.lightness(), 10.0);
}

#[test]
fn chroma() {
    let a = Cielch(10.0, 0.0, 30f32.to_radians());
    assert_eq!(a.chroma(), 0.0);
    let b = Cielch(10.0, 20.0, 30f32.to_radians());
    assert_eq!(b.chroma(), 20.0);
}

#[test]
fn hue() {
    let a = Cielch(10.0, 20.0, 0f32.to_radians());
    assert_eq!(a.hue(), 0f32.to_radians());
    let b = Cielch(10.0, 20.0, 30f32.to_radians());
    assert_eq!(b.hue(), 30f32.to_radians());
}

#[test]
fn from_cielab_precision() {
    let cielch_black = Cielch::new(0f32, 0f32, 0f32).unwrap();
    let cielab_black = Cielab::new(0f32, 0f32, 0f32).unwrap();
    assert_eq!(cielch_black, cielab_black.into());
}
