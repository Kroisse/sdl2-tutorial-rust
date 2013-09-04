use std::libc::{uint32_t};
use std::str::raw::from_c_str;
mod ll;

#[link_args="-framework SDL2"] extern {}

pub enum InitFlag {
    SDL_INIT_TIMER =          0x00000001u,
    SDL_INIT_AUDIO =          0x00000010u,
    /**< SDL_INIT_VIDEO implies SDL_INIT_EVENTS */
    SDL_INIT_VIDEO =          0x00000020u,
    /**< SDL_INIT_JOYSTICK implies SDL_INIT_EVENTS */
    SDL_INIT_JOYSTICK =       0x00000200u,
    SDL_INIT_HAPTIC =         0x00001000u,
    /**< SDL_INIT_GAMECONTROLLER implies SDL_INIT_JOYSTICK */
    SDL_INIT_GAMECONTROLLER = 0x00002000u,
    SDL_INIT_EVENTS =         0x00004000u,
    /**< Don't catch fatal signals */
    SDL_INIT_NOPARACHUTE =    0x00100000u,
}
pub struct InitFlagSet { priv bits: uint }
impl InitFlagSet {
    pub fn empty() -> InitFlagSet {
        InitFlagSet {bits: 0}
    }
    pub fn single(flag: InitFlag) -> InitFlagSet {
        InitFlagSet {bits: flag as uint}
    }
    pub fn add(&mut self, flag: InitFlag) {
        self.bits |= flag as uint;
    }
    pub fn bits(&self) -> uint {
        self.bits
    }
}
impl BitOr<InitFlagSet, InitFlagSet> for InitFlagSet {
    fn bitor(&self, rhs: &InitFlagSet) -> InitFlagSet {
        InitFlagSet {bits: self.bits | rhs.bits}
    }
}

pub fn SDL_INIT_EVERYTHING() -> InitFlagSet {
    let mut result = InitFlagSet::empty();
    result.add(SDL_INIT_TIMER);
    result.add(SDL_INIT_AUDIO);
    result.add(SDL_INIT_VIDEO);
    result.add(SDL_INIT_EVENTS);
    result.add(SDL_INIT_JOYSTICK);
    result.add(SDL_INIT_HAPTIC);
    result.add(SDL_INIT_GAMECONTROLLER);
    result
}


#[fixed_stack_segment]
pub fn init(flags: InitFlagSet) -> Result<(), ~str> {
    let raw_flag = flags.bits() as uint32_t;
    unsafe {
        if ll::SDL_Init(raw_flag) < 0 {
            let msg = ll::SDL_GetError();
            return Err(from_c_str(msg));
        }
    }
    Ok(())
}

#[fixed_stack_segment]
pub fn delay(ms: uint) {
    unsafe {
        ll::SDL_Delay(ms as uint32_t);
    }
}
