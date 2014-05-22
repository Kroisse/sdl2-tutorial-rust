use libc::{c_int};

pub type RawPoint = super::ll::SDL_Point;
pub type RawSize = (uint, uint);
pub type RawRect = super::ll::SDL_Rect;

pub fn RawPoint(x: int, y: int) -> RawPoint {
    super::ll::SDL_Point { x: x as c_int, y: y as c_int }
}

pub fn RawRect(x: int, y: int, w: uint, h: uint) -> RawRect {
    super::ll::Struct_SDL_Rect { x: x as c_int, y: y as c_int, w: w as c_int, h: h as c_int }
}

impl Eq for RawPoint {
    fn eq(&self, rhs: &RawPoint) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}

impl Eq for RawRect {
    fn eq(&self, rhs: &RawRect) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.w == rhs.w && self.h == rhs.h
    }
}

pub trait ToPoint { fn to_point(&self) -> Option<RawPoint>; }
pub trait ToSize  { fn to_size(&self) -> Option<RawSize>; }
pub trait ToRect : ToPoint + ToSize { fn to_rect(&self) -> Option<RawRect>; }

impl ToPoint for () { fn to_point(&self) -> Option<RawPoint> { None } }
impl ToSize  for () { fn to_size(&self)  -> Option<RawSize>  { None } }
impl ToRect  for () { fn to_rect(&self)  -> Option<RawRect>  { None } }

impl ToPoint for RawPoint { fn to_point(&self) -> Option<RawPoint> { Some(super::ll::SDL_Point{ x: self.x, y: self.y}) } }
impl ToSize  for RawSize  { fn to_size(&self)  -> Option<RawSize>  { Some(*self) } }
impl ToRect  for RawRect  { fn to_rect(&self)  -> Option<RawRect>  { Some(super::ll::Struct_SDL_Rect {x: self.x, y: self.y, w: self.w, h: self.h}) } }

impl ToPoint for (int, int) {
    fn to_point(&self) -> Option<RawPoint> {
        let &(x, y) = self;
        Some(RawPoint(x, y))
    }
}

impl ToPoint for RawRect {
    fn to_point(&self) -> Option<RawPoint> { Some(super::ll::SDL_Point { x: self.x, y: self.y }) }
}
impl ToSize for RawRect {
    fn to_size(&self) -> Option<RawSize> { Some((self.w as uint, self.h as uint)) }
}

impl ToRect for (int, int, uint, uint) {
    fn to_rect(&self) -> Option<RawRect> {
        let &(x, y, w, h) = self;
        Some(RawRect(x, y, w, h))
    }
}
impl ToPoint for (int, int, uint, uint) {
    fn to_point(&self) -> Option<RawPoint> {
        let &(x, y, _, _) = self;
        Some(RawPoint(x, y))
    }
}
impl ToSize for (int, int, uint, uint) {
    fn to_size(&self) -> Option<RawSize> {
        let &(_, _, w, h) = self;
        Some((w, h))
    }
}

#[test]
fn test_raw_rect() {
    let r = RawRect(1, 2, 3, 4);
    assert!(r == r.to_rect().unwrap());
}

#[test]
fn test_tuple() {
    assert!((1, 2, 3, 4).to_rect().unwrap() == RawRect(1, 2, 3, 4));
}
