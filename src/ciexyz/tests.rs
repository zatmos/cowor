use crate::{
    Cielab,
    Srgb,
};
use super::*;

#[test]
fn new() {
    let a = Ciexyz(0.1, 0.2, 0.3);
    let b = Ciexyz::new(0.1, 0.2, 0.3).unwrap();
    let c = Ciexyz::new(-0.1f32, 0.2f32, 0.3f32);
    let d = Ciexyz::new(0.1f32, 2.0f32, 0.3f32);
    assert_eq!(a, b);
    assert!(c.is_none());
    assert!(d.is_none());
}

#[test]
fn x() {
    let a = Ciexyz(0.0, 0.2, 0.3);
    assert_eq!(a.x(), 0.0);
    let b = Ciexyz(0.1, 0.2, 0.3);
    assert_eq!(b.x(), 0.1);
}

#[test]
fn y() {
    let a = Ciexyz(0.1, 0.0, 0.3);
    assert_eq!(a.y(), 0.0);
    let b = Ciexyz(0.1, 0.2, 0.3);
    assert_eq!(b.y(), 0.2);
}

#[test]
fn z() {
    let a = Ciexyz(0.1, 0.2, 0.0);
    assert_eq!(a.z(), 0.0);
    let b = Ciexyz(0.1, 0.2, 0.3);
    assert_eq!(b.z(), 0.3);
}

#[test]
fn from_srgb_precision() {
    let ciexyz_black = Ciexyz::new(0f32, 0f32, 0f32).unwrap();
    let srgb_black = Srgb::new(0, 0, 0);
    assert_eq!(ciexyz_black, srgb_black.into());
}

#[test]
fn from_cielab_precision() {
    let ciexyz_black = Ciexyz::new(0f32, 0f32, 0f32).unwrap();
    let cielab_black = Cielab::new(0f32, 0f32, 0f32).unwrap();
    assert_eq!(ciexyz_black, cielab_black.into());
}

#[test]
fn d65() {
    assert_eq!(D65, Ciexyz::from(Srgb::new(255, 255, 255)));
}
