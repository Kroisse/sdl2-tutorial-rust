use std::ptr;
use std::libc::{c_int, c_double};

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

pub struct Renderer<'self> {
    priv parent: &'self super::video::Window,
    p_renderer: *mut ll::SDL_Renderer,
}

pub struct Texture<'self> {
    parent: &'self Renderer<'self>,
    p_texture: *mut ll::SDL_Texture,
}

impl<'self> Renderer<'self> {
    #[fixed_stack_segment]
    pub fn create_texture_from_surface<'a>(&'a self, surface: &Surface) -> Result<~Texture<'a>, ~str> {
        let p = unsafe { ll::SDL_CreateTextureFromSurface(self.p_renderer, surface.p_surface) };
        if ptr::is_null(p) {
            Err(get_error())
        } else {
            Ok(~Texture{ parent: self, p_texture: p })
        }
    }

    #[fixed_stack_segment]
    pub fn clear(&self) {
        unsafe {
            ll::SDL_RenderClear(self.p_renderer);
        }
    }

    #[fixed_stack_segment]
    pub fn copy_<T:ToRect, U:ToRect>(&self, texture: &Texture, src: &T, dest: &U) {
        unsafe {
            let p_src = match src.to_rect() {
                Some(rect) => ptr::to_unsafe_ptr(&rect),
                None => ptr::null(),
            };
            let p_dest = match dest.to_rect() {
                Some(rect) => ptr::to_unsafe_ptr(&rect),
                None => ptr::null(),
            };
            ll::SDL_RenderCopy(self.p_renderer, texture.p_texture, p_src, p_dest);
        }
    }

    #[fixed_stack_segment]
    pub fn copy_ex<T:ToRect, U:ToRect, V:ToPoint>(&self, texture: &Texture, src: &T, dest: &U,
                                                  angle: float, center: &V, flip: RendererFlip) {
        unsafe {
            do rect_with_unsafe_ptr(src) |p_src| {
                do rect_with_unsafe_ptr(dest) |p_dest| {
                    do point_with_unsafe_ptr(center) |p_center| {
                        ll::SDL_RenderCopyEx(self.p_renderer, texture.p_texture,
                                             p_src, p_dest, angle as c_double,
                                             p_center, flip as ll::SDL_RendererFlip);
                    }
                }
            }
        }
    }

    #[fixed_stack_segment]
    pub fn present(&self) {
        unsafe {
            ll::SDL_RenderPresent(self.p_renderer);
        }
    }
}

impl super::video::Window {
    #[fixed_stack_segment]
    pub fn create_renderer<'a>(&'a self, index: int) -> Result<~Renderer<'a>, ~str> {
        let flags = ll::SDL_RENDERER_ACCELERATED | ll::SDL_RENDERER_PRESENTVSYNC;
        let p = unsafe { ll::SDL_CreateRenderer(self.p_window, index as c_int, flags) };
        if ptr::is_null(p) {
            Err(get_error())
        } else {
            Ok(~Renderer{ parent: self, p_renderer: p })
        }
    }
}

#[unsafe_destructor]
impl<'self> Drop for Renderer<'self> {
    #[fixed_stack_segment]
    fn drop(&self) {
        unsafe {
            ll::SDL_DestroyRenderer(self.p_renderer);
        }
    }
}

impl<'self> Texture<'self> {
    #[fixed_stack_segment]
    pub fn size(&self) -> (uint, uint) {
        let mut w = 0;
        let mut h = 0;
        unsafe {
            ll::SDL_QueryTexture(self.p_texture,
                                 ptr::mut_null(), ptr::mut_null(),
                                 ptr::to_mut_unsafe_ptr(&mut w),
                                 ptr::to_mut_unsafe_ptr(&mut h));
        }
        (w as uint, h as uint)
    }
}

#[unsafe_destructor]
impl<'self> Drop for Texture<'self> {
    #[fixed_stack_segment]
    fn drop(&self) {
        unsafe {
            ll::SDL_DestroyTexture(self.p_texture);
        }
    }
}

unsafe fn rect_with_unsafe_ptr<T:ToRect>(rect: &T, cb: &fn(*RawRect)) {
    let rect = rect.to_rect();
    let p = match rect {
        Some(r) => ptr::to_unsafe_ptr(&r),
        None => ptr::null(),
    };
    cb(p);
}


unsafe fn point_with_unsafe_ptr<T:ToPoint>(point: &T, cb: &fn(*RawPoint)) {
    let point = point.to_point();
    let p = match point {
        Some(t) => ptr::to_unsafe_ptr(&t),
        None => ptr::null(),
    };
    cb(p);
}
