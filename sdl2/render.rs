use std::ptr;
use std::mem;

use libc::{c_int, c_double};

use super::ll;
use super::{ToRect, RawRect, ToPoint, RawPoint};
use super::surface::Surface;
use super::get_error;

pub enum RendererFlip {
    /** Do not flip */
    FlipNone = 0x00000000,
    /** flip horizontally */
    FlipHorizontal = 0x00000001,
    /** flip vertically */
    FlipVertical = 0x00000002,
}

pub struct Renderer<'a> {
    pub p_renderer: &'a mut ll::SDL_Renderer,
}

pub struct Texture<'a> {
    pub parent: &'a Renderer<'a>,
    pub p_texture: *mut ll::SDL_Texture,
}

impl<'s> Renderer<'s> {
        pub fn create_texture_from_surface<'a>(&'a self, surface: &Surface) -> Result<Texture<'a>, StrBuf> {
        let p = unsafe { ll::SDL_CreateTextureFromSurface(self.p_renderer, surface.p_surface) };
        if p.is_null() {
            Err(get_error())
        } else {
            Ok(Texture{ parent: self, p_texture: p })
        }
    }

        pub fn clear(&self) {
        unsafe {
            ll::SDL_RenderClear(self.p_renderer);
        }
    }

        pub fn copy_<T:ToRect, U:ToRect>(&self, texture: &Texture, src: &T, dest: &U) {
        unsafe {
            let p_src = match src.to_rect() {
                Some(rect) => &rect as *_,
                None => ptr::null(),
            };
            let p_dest = match dest.to_rect() {
                Some(rect) => &rect as *_,
                None => ptr::null(),
            };
            ll::SDL_RenderCopy(self.p_renderer, texture.p_texture, p_src, p_dest);
        }
    }

        pub fn copy_ex<T:ToRect, U:ToRect, V:ToPoint>(&self, texture: &Texture, src: &T, dest: &U,
                                                  angle: f64, center: &V, flip: RendererFlip) {
        unsafe {
            rect_with_unsafe_ptr(src, |p_src| {
                rect_with_unsafe_ptr(dest, |p_dest| {
                    point_with_unsafe_ptr(center, |p_center| {
                        ll::SDL_RenderCopyEx(self.p_renderer, texture.p_texture,
                                             p_src, p_dest, angle as c_double,
                                             p_center, flip as ll::SDL_RendererFlip);
                    });
                });
            });
        }
    }

        pub fn present(&self) {
        unsafe {
            ll::SDL_RenderPresent(self.p_renderer);
        }
    }
}

impl super::video::Window {
        pub fn create_renderer<'a>(&'a self, index: int) -> Result<Renderer<'a>, StrBuf> {
        let flags = ll::SDL_RENDERER_ACCELERATED | ll::SDL_RENDERER_PRESENTVSYNC;
        unsafe {
            let p = ll::SDL_CreateRenderer(self.p_window, index as c_int, flags);
            if p.is_null() {
                Err(get_error())
            } else {
                Ok(Renderer{ p_renderer: mem::transmute(p) })
            }
        }
    }
}

#[unsafe_destructor]
impl<'a> Drop for Renderer<'a> {
        fn drop(&mut self) {
        unsafe {
            ll::SDL_DestroyRenderer(self.p_renderer);
        }
    }
}

impl<'a> Texture<'a> {
        pub fn size(&self) -> (uint, uint) {
        let mut w = 0;
        let mut h = 0;
        unsafe {
            ll::SDL_QueryTexture(self.p_texture,
                                 ptr::mut_null(), ptr::mut_null(),
                                 &mut w,
                                 &mut h);
        }
        (w as uint, h as uint)
    }
}

#[unsafe_destructor]
impl<'a> Drop for Texture<'a> {
        fn drop(&mut self) {
        unsafe {
            ll::SDL_DestroyTexture(self.p_texture);
        }
    }
}

unsafe fn rect_with_unsafe_ptr<T:ToRect>(rect: &T, cb: |*RawRect|) {
    let rect = rect.to_rect();
    let p = match rect {
        Some(r) => &r as *_,
        None => ptr::null(),
    };
    cb(p);
}


unsafe fn point_with_unsafe_ptr<T:ToPoint>(point: &T, cb: |*RawPoint|) {
    let point = point.to_point();
    let p = match point {
        Some(t) => &t as *_,
        None => ptr::null(),
    };
    cb(p);
}
