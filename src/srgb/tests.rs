use super::*;
use crate::ciexyz::Ciexyz;

#[test]
fn new() {
    let a = Srgb(0.0, 1.0, 0.0);
    let b = Srgb::new(0x00, 0xff, 0x00);
    assert_eq!(a, b);
}

#[test]
fn red() {
    let a = Srgb(0.0, 1.0, 0.0);
    assert_eq!(a.red(), 0.0);
    let b = Srgb(1.0, 1.0, 0.0);
    assert_eq!(b.red(), 1.0);
}

#[test]
fn green() {
    let a = Srgb(0.0, 0.0, 0.0);
    assert_eq!(a.green(), 0.0);
    let b = Srgb(0.0, 1.0, 0.0);
    assert_eq!(b.green(), 1.0);
}

#[test]
fn blue() {
    let a = Srgb(0.0, 1.0, 0.0);
    assert_eq!(a.blue(), 0.0);
    let b = Srgb(0.0, 1.0, 1.0);
    assert_eq!(b.blue(), 1.0);
}

#[test]
fn red_as_u8() {
    let a = Srgb(0.0, 1.0, 0.0);
    assert_eq!(a.red_as_u8(), 0x00);
    let b = Srgb(1.0, 1.0, 0.0);
    assert_eq!(b.red_as_u8(), 0xff);
}

#[test]
fn green_as_u8() {
    let a = Srgb(0.0, 0.0, 0.0);
    assert_eq!(a.green_as_u8(), 0x00);
    let b = Srgb(0.0, 1.0, 0.0);
    assert_eq!(b.green_as_u8(), 0xff);
}

#[test]
fn blue_as_u8() {
    let a = Srgb(0.0, 1.0, 0.0);
    assert_eq!(a.blue_as_u8(), 0x00);
    let b = Srgb(0.0, 1.0, 1.0);
    assert_eq!(b.blue_as_u8(), 0xff);
}

#[test]
fn linear_red() {
    let a = Srgb(0.0, 1.0, 0.0);
    assert_eq!(a.linear_red(), 0.0);
    let b = Srgb(1.0, 1.0, 0.0);
    assert_eq!(b.linear_red(), 1.0);
}

#[test]
fn linear_green() {
    let a = Srgb(0.0, 0.0, 0.0);
    assert_eq!(a.linear_green(), 0.0);
    let b = Srgb(0.0, 1.0, 0.0);
    assert_eq!(b.linear_green(), 1.0);
}

#[test]
fn linear_blue() {
    let a = Srgb(0.0, 1.0, 0.0);
    assert_eq!(a.linear_blue(), 0.0);
    let b = Srgb(0.0, 1.0, 1.0);
    assert_eq!(b.linear_blue(), 1.0);
}

#[test]
fn from_ciexyz_precision() {
    let srgb_black = Srgb::new(0x00, 0x00, 0x00);
    let ciexyz_black = Ciexyz::new(0.0, 0.0, 0.0).unwrap();
    assert_eq!(srgb_black, ciexyz_black.try_into().unwrap());

    let srgb_white = Srgb::new(0xff, 0xff, 0xff);
    let ciexyz_white = crate::ciexyz::D65.try_into().unwrap();
    assert_eq!(srgb_white, ciexyz_white);
}
