use std::libc::{c_int};


pub type Rect = super::ll::SDL_Rect;

pub fn Rect(x: int, y: int, w: uint, h: uint) -> Rect {
    super::ll::Struct_SDL_Rect { x: x as c_int, y: y as c_int, w: w as c_int, h: h as c_int }
}
