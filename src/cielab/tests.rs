use super::*;
use crate::{
    Cielch,
    Ciexyz,
    ciexyz::D65,
};

#[test]
fn new() {
    let a = Cielab(10.0, 20.0, 30.0);
    let b = Cielab::new(10.0, 20.0, 30.0).unwrap();
    let c = Cielab::new(-10.0, 20.0, 30.0);
    let d = Cielab::new(110.0, 20.0, 30.0);
    assert_eq!(a, b);
    assert!(c.is_none());
    assert!(d.is_none());
}

#[test]
fn lightness() {
    let a = Cielab(0.0, 20.0, 30.0);
    assert_eq!(a.lightness(), 0.0);
    let b = Cielab(10.0, 20.0, 30.0);
    assert_eq!(b.lightness(), 10.0);
}

#[test]
fn a() {
    let a = Cielab(10.0, 0.0, 30.0);
    assert_eq!(a.a(), 0.0);
    let b = Cielab(10.0, 20.0, 30.0);
    assert_eq!(b.a(), 20.0);
}

#[test]
fn b() {
    let a = Cielab(10.0, 20.0, 0.0);
    assert_eq!(a.b(), 0.0);
    let b = Cielab(10.0, 20.0, 30.0);
    assert_eq!(b.b(), 30.0);
}

#[test]
fn from_ciexyz_precision() {
    let cielab_black = Cielab::new(0f32, 0f32, 0f32).unwrap();
    let ciexyz_black = Ciexyz::new(0f32, 0f32, 0f32).unwrap();
    assert_eq!(cielab_black, ciexyz_black.into());
}

#[test]
fn from_cielch_precision() {
    let cielab_black = Cielab::new(0f32, 0f32, 0f32).unwrap();
    let cielch_black = Cielch::new(0f32, 0f32, 0f32).unwrap();
    assert_eq!(cielab_black, cielch_black.into());
}

#[test]
fn from_d65() {
    assert_eq!(Cielab::new(100f32, 0f32, 0f32).unwrap(), D65.into());
}
