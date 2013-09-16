use std::libc::{c_int};

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

pub trait MaybePoint { fn maybe_point(&self) -> Option<RawPoint>; }
pub trait MaybeSize  { fn maybe_size(&self) -> Option<RawSize>; }
pub trait MaybeRect : MaybePoint + MaybeSize { fn maybe_rect(&self) -> Option<RawRect>; }

impl MaybePoint for () { fn maybe_point(&self) -> Option<RawPoint> { None } }
impl MaybeSize  for () { fn maybe_size(&self)  -> Option<RawSize>  { None } }
impl MaybeRect  for () { fn maybe_rect(&self)  -> Option<RawRect>  { None } }

pub trait ToPoint : MaybePoint { fn to_point(&self) -> RawPoint; }
pub trait ToSize  : MaybeSize  { fn to_size(&self)  -> RawSize; }
pub trait ToRect  : ToPoint + ToSize + MaybeRect { fn to_rect(&self) -> RawRect; }

impl ToPoint for RawPoint { fn to_point(&self) -> RawPoint { super::ll::SDL_Point{ x: self.x, y: self.y} } }
impl MaybePoint for RawPoint { fn maybe_point(&self) -> Option<RawPoint> { Some(self.to_point()) } }
impl ToSize for RawSize { fn to_size(&self) -> RawSize { *self } }
impl MaybeSize for RawSize { fn maybe_size(&self) -> Option<RawSize> { Some(self.to_size()) } }
impl ToRect for RawRect { fn to_rect(&self) -> RawRect { super::ll::Struct_SDL_Rect {x: self.x, y: self.y, w: self.w, h: self.h} } }
impl MaybeRect for RawRect { fn maybe_rect(&self) -> Option<RawRect> { Some(self.to_rect()) } }

impl ToPoint for (int, int) {
    fn to_point(&self) -> RawPoint {
        let &(x, y) = self;
        RawPoint(x, y)
    }
}

impl MaybePoint for (int, int) { fn maybe_point(&self) -> Option<RawPoint> { Some(self.to_point()) } }

impl ToPoint for RawRect {
    fn to_point(&self) -> RawPoint { super::ll::SDL_Point { x: self.x, y: self.y } }
}
impl MaybePoint for RawRect {
    fn maybe_point(&self) -> Option<RawPoint> { Some(self.to_point()) }
}
impl ToSize for RawRect {
    fn to_size(&self) -> RawSize { (self.w as uint, self.h as uint) }
}
impl MaybeSize for RawRect {
    fn maybe_size(&self) -> Option<RawSize> { Some(self.to_size()) }
}

impl ToRect for (int, int, uint, uint) {
    fn to_rect(&self) -> RawRect {
        let &(x, y, w, h) = self;
        RawRect(x, y, w, h)
    }
}
impl MaybeRect for (int, int, uint, uint) {
    fn maybe_rect(&self) -> Option<RawRect> { Some(self.to_rect()) }
}
impl ToPoint for (int, int, uint, uint) {
    fn to_point(&self) -> RawPoint {
        let &(x, y, _, _) = self;
        RawPoint(x, y)
    }
}
impl MaybePoint for (int, int, uint, uint) {
    fn maybe_point(&self) -> Option<RawPoint> { Some(self.to_point()) }
}
impl ToSize for (int, int, uint, uint) {
    fn to_size(&self) -> RawSize {
        let &(_, _, w, h) = self;
        (w, h)
    }
}
impl MaybeSize for (int, int, uint, uint) {
    fn maybe_size(&self) -> Option<RawSize> { Some(self.to_size()) }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_raw_rect() {
        let r = RawRect(1, 2, 3, 4);
        assert_eq!(r, r.to_rect());
    }

    #[test]
    fn test_tuple() {
        assert_eq!((1, 2, 3, 4).to_rect(), RawRect(1, 2, 3, 4));
    }
}
