use std::ptr;
use std::libc::{c_int};

use super::ll;
use super::ToRect;
use super::surface::Surface;
use super::get_error;


pub struct Renderer<'self> {
    priv parent: &'self super::window::Window,
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
    pub fn present(&self) {
        unsafe {
            ll::SDL_RenderPresent(self.p_renderer);
        }
    }
}

impl super::window::Window {
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