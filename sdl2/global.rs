use std::libc::{uint32_t};
use std::str::raw::from_c_str;
use std::cast;

use super::util::enum_set::*;
mod ll;

#[link_args="-framework SDL2"] extern {}

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


#[fixed_stack_segment]
pub fn init(flags: EnumSet<InitFlag>) -> Result<(), ~str> {
    let raw_flag = flags.to_uint() as u32;
    if unsafe { ll::SDL_Init(raw_flag) } < 0 {
        Err(get_error())
    } else {
        Ok(())
    }
}

#[fixed_stack_segment]
pub fn quit() {
    unsafe { ll::SDL_Quit(); }
}

#[fixed_stack_segment]
pub fn get_error() -> ~str {
    unsafe {
        let msg = ll::SDL_GetError();
        from_c_str(msg)
    }
}

#[fixed_stack_segment]
pub fn delay(ms: uint) {
    unsafe {
        ll::SDL_Delay(ms as uint32_t);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::util::enum_set;

    #[test]
    fn test_clike() {
        assert!(enum_set::bit(SDL_INIT_TIMER) == 0x00000001u);
        assert!(enum_set::bit(SDL_INIT_AUDIO) == 0x00000010u);
        assert!(enum_set::bit(SDL_INIT_VIDEO) == 0x00000020u);
        assert!(enum_set::bit(SDL_INIT_JOYSTICK) == 0x00000200u);
        assert!(enum_set::bit(SDL_INIT_HAPTIC) == 0x00001000u);
        assert!(enum_set::bit(SDL_INIT_GAMECONTROLLER) == 0x00002000u);
        assert!(enum_set::bit(SDL_INIT_EVENTS) == 0x00004000u);
        assert!(enum_set::bit(SDL_INIT_NOPARACHUTE) == 0x00100000u);
    }
}