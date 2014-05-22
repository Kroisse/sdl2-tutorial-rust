#![crate_id = "sdl2#0.1"]
#![license = "MIT"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![feature(macro_rules, phase)]

#[phase(syntax, link)] extern crate log;
extern crate collections;
extern crate libc;

use std::str::raw::from_c_str;

use libc::{uint32_t};

use util::enum_set::{EnumSet, EnumSetUtil};

pub use pixels::{Color, RGBA};
pub use rect::{ToRect, RawRect, ToPoint, RawPoint};

pub mod util;

pub mod rect;
pub mod pixels;
pub mod video;
pub mod render;
pub mod surface;
pub mod events;
pub mod scancode;
pub mod keycode;

mod ll;

pub mod ext;

#[allow(non_camel_case_types)]
#[deriving(FromPrimitive)]
pub enum InitFlag {
    SDL_INIT_TIMER =          0,
    SDL_INIT_AUDIO =          4,
    /**< SDL_INIT_VIDEO implies SDL_INIT_EVENTS */
    SDL_INIT_VIDEO =          5,
    /**< SDL_INIT_JOYSTICK implies SDL_INIT_EVENTS */
    SDL_INIT_JOYSTICK =       9,
    SDL_INIT_HAPTIC =         12,
    /**< SDL_INIT_GAMECONTROLLER implies SDL_INIT_JOYSTICK */
    SDL_INIT_GAMECONTROLLER = 13,
    SDL_INIT_EVENTS =         14,
    /**< Don't catch fatal signals */
    SDL_INIT_NOPARACHUTE =    20,
}
impl_clike!(InitFlag)

pub fn SDL_INIT_EVERYTHING() -> EnumSet<InitFlag> {
    enum_set!(SDL_INIT_TIMER | SDL_INIT_AUDIO | SDL_INIT_VIDEO | SDL_INIT_EVENTS |
              SDL_INIT_JOYSTICK | SDL_INIT_HAPTIC | SDL_INIT_GAMECONTROLLER)
}


pub fn init(flags: EnumSet<InitFlag>) -> Result<(), StrBuf> {
    let raw_flag = flags.to_uint() as u32;
    if unsafe { ll::SDL_Init(raw_flag) } < 0 {
        Err(get_error())
    } else {
        Ok(())
    }
}

pub fn quit() {
    unsafe { ll::SDL_Quit(); }
}

pub fn get_error() -> StrBuf {
    unsafe {
        let msg = ll::SDL_GetError();
        from_c_str(msg).to_strbuf()
    }
}

pub fn delay(ms: uint) {
    unsafe {
        ll::SDL_Delay(ms as uint32_t);
    }
}

#[test]
fn test_clike() {
    use util::enum_set::bit;

    assert!(bit(SDL_INIT_TIMER)          == 0x00000001u);
    assert!(bit(SDL_INIT_AUDIO)          == 0x00000010u);
    assert!(bit(SDL_INIT_VIDEO)          == 0x00000020u);
    assert!(bit(SDL_INIT_JOYSTICK)       == 0x00000200u);
    assert!(bit(SDL_INIT_HAPTIC)         == 0x00001000u);
    assert!(bit(SDL_INIT_GAMECONTROLLER) == 0x00002000u);
    assert!(bit(SDL_INIT_EVENTS)         == 0x00004000u);
    assert!(bit(SDL_INIT_NOPARACHUTE)    == 0x00100000u);
}
