use std::mem;
use libc::{c_uint};
use super::ll;
use super::scancode;
use super::keycode;
use util::enum_set::{EnumSet, EnumSetUtil};

pub type Timestamp = u32;
pub type WindowID = u32;

/*
pub struct CommonEvent { timestamp: Timestamp }
pub struct TextEditingEvent {
    timestamp: Timestamp,
    windowID: Uint32,
    text: ~str,
    start: Sint32,
    length: Sint32,
}
pub struct TextInputEvent {
    timestamp: Timestamp,
    windowID: Uint32,
    text: ~str,
}
pub struct MouseMotionEvent {
    timestamp: Timestamp,
    windowID: Uint32,
    which: Uint32,
    state: Uint32,
    x: Sint32,
    y: Sint32,
    xrel: Sint32,
    yrel: Sint32,
}
pub struct MouseButtonEvent {
    timestamp: Timestamp,
    windowID: Uint32,
    which: Uint32,
    button: Uint8,
    state: Uint8,
    padding1: Uint8,
    padding2: Uint8,
    x: Sint32,
    y: Sint32,
}
pub struct MouseWheelEvent {
    timestamp: Timestamp,
    windowID: Uint32,
    which: Uint32,
    x: Sint32,
    y: Sint32,
}
pub struct JoyAxisEvent {
    timestamp: Timestamp,
    which: SDL_JoystickID,
    axis: Uint8,
    padding1: Uint8,
    padding2: Uint8,
    padding3: Uint8,
    value: Sint16,
    padding4: Uint16,
}
pub struct JoyBallEvent {
    timestamp: Timestamp,
    which: SDL_JoystickID,
    ball: Uint8,
    padding1: Uint8,
    padding2: Uint8,
    padding3: Uint8,
    xrel: Sint16,
    yrel: Sint16,
}
pub struct JoyHatEvent {
    timestamp: Timestamp,
    which: SDL_JoystickID,
    hat: Uint8,
    value: Uint8,
    padding1: Uint8,
    padding2: Uint8,
}
pub struct JoyButtonEvent {
    timestamp: Timestamp,
    which: SDL_JoystickID,
    button: Uint8,
    state: Uint8,
    padding1: Uint8,
    padding2: Uint8,
}
pub struct JoyDeviceEvent {
    timestamp: Timestamp,
    which: Sint32,
}
pub struct ControllerAxisEvent {
    timestamp: Timestamp,
    which: SDL_JoystickID,
    axis: Uint8,
    padding1: Uint8,
    padding2: Uint8,
    padding3: Uint8,
    value: Sint16,
    padding4: Uint16,
}
pub struct ControllerButtonEvent {
    timestamp: Timestamp,
    which: SDL_JoystickID,
    button: Uint8,
    state: Uint8,
    padding1: Uint8,
    padding2: Uint8,
}
pub struct ControllerDeviceEvent {
    timestamp: Timestamp,
    which: Sint32,
}
pub struct TouchFingerEvent {
    timestamp: Timestamp,
    touchId: SDL_TouchID,
    fingerId: SDL_FingerID,
    x: c_float,
    y: c_float,
    dx: c_float,
    dy: c_float,
    pressure: c_float,
}
pub struct MultiGestureEvent {
    timestamp: Timestamp,
    touchId: SDL_TouchID,
    dTheta: c_float,
    dDist: c_float,
    x: c_float,
    y: c_float,
    numFingers: Uint16,
    padding: Uint16,
}
pub struct DollarGestureEvent {
    timestamp: Timestamp,
    touchId: SDL_TouchID,
    gestureId: SDL_GestureID,
    numFingers: Uint32,
    error: c_float,
    x: c_float,
    y: c_float,
}
pub struct DropEvent {
    timestamp: Timestamp,
    file: *mut c_schar,
}
pub struct QuitEvent { timestamp: Timestamp }
pub struct OSEvent   { timestamp: Timestamp }
pub struct UserEvent {
    timestamp: Timestamp,
    windowID: Uint32,
    code: Sint32,
    data1: *mut c_void,
    data2: *mut c_void,
}
pub struct SysWMEvent {
    timestamp: Timestamp,
    msg: *mut SDL_SysWMmsg,
}
*/

pub struct WindowEvent {
    pub timestamp: Timestamp,
    pub window_id: WindowID,
}
pub struct WindowMovedEvent {
    pub timestamp: Timestamp,
    pub window_id: WindowID,
    pub position: (int, int),
}
pub struct WindowResizedEvent {
    pub timestamp: Timestamp,
    pub window_id: WindowID,
    pub size: (uint, uint),
}

pub enum KeyState { KeyPressed, KeyReleased }
pub struct KeyboardEvent {
    pub timestamp: Timestamp,
    pub window_id: WindowID,
    pub state: KeyState,
    pub repeat: bool,
    pub scancode: scancode::Scancode,
    pub sym: keycode::Keycode,
    pub modifiers: EnumSet<keycode::KeyModifier>,
}

pub enum Event {
    NoEvent,
    /* Application events */
    Quit(Timestamp),
    /* These application events have special meaning on iOS, see README-ios.txt for details */
    AppTerminating(Timestamp),
    AppLowMemory(Timestamp),
    AppWillEnterBackground(Timestamp),
    AppDidEnterBackground(Timestamp),
    AppWillEnterForeground(Timestamp),
    AppDidEnterForeground(Timestamp),
    /* Window events */
    WindowShown(WindowEvent),
    WindowHidden(WindowEvent),
    WindowExposed(WindowEvent),
    WindowMoved(WindowMovedEvent),
    WindowResized(WindowResizedEvent),
    WindowSizeChanged(WindowEvent),
    WindowMinimized(WindowEvent),
    WindowMaximized(WindowEvent),
    WindowRestored(WindowEvent),
    WindowEnter(WindowEvent),
    WindowLeave(WindowEvent),
    WindowFocusGained(WindowEvent),
    WindowFocusLost(WindowEvent),
    WindowClose(WindowEvent),
    SysWMEvent(Timestamp),
    /* Keyboard events */
    KeyDown(KeyboardEvent),
    KeyUp(KeyboardEvent),
    TextEditing(Timestamp),
    TextInput(Timestamp),
    /* Mouse events */
    MouseMotion(Timestamp),
    MouseButtonDown(Timestamp),
    MouseButtonUp(Timestamp),
    MouseWheel(Timestamp),
    /* Joystick events */
    JoyAxisMotion(Timestamp),
    JoyBallMotion(Timestamp),
    JoyHatMotion(Timestamp),
    JoyButtonDown(Timestamp),
    JoyButtonUp(Timestamp),
    JoyDeviceAdded(Timestamp),
    JoyDeviceRemoved(Timestamp),
    /* Game controller events */
    ControllerAxisMotion(Timestamp),
    ControllerButtonDown(Timestamp),
    ControllerButtonUp(Timestamp),
    ControllerDeviceAdded(Timestamp),
    ControllerDeviceRemoved(Timestamp),
    ControllerDeviceRemapped(Timestamp),
    /* Touch events */
    FingerDown(Timestamp),
    FingerUp(Timestamp),
    FingerMotion(Timestamp),
    /* Gesture events */
    DollarGesture(Timestamp),
    DollarRecord(Timestamp),
    MultiGesture(Timestamp),
    /* Clipboard events */
    ClipboardUpdate(Timestamp),
    /* Drag and drop events */
    DropFile(Timestamp),
    /** Events ::SDL_USEREVENT through ::SDL_LASTEVENT are for your use,
     *  and should be allocated with SDL_RegisterEvents()
     */
    UserEvent(Timestamp),
}

fn null_event() -> ll::SDL_Event {
    ll::Union_SDL_Event::new()
}

pub fn pump() {
    unsafe { ll::SDL_PumpEvents(); }
}

pub fn poll() -> Event {
    let mut event = null_event();
    let polled = unsafe { ll::SDL_PollEvent(&mut event) };
    if polled == 0 {
        return NoEvent;
    }

    wrap_event(event)
}

fn wrap_event(raw_event: ll::SDL_Event) -> Event {
    let mut raw_event = raw_event;
    unsafe {
        let type_ = raw_event._type();
        let type_ = if type_.is_null() {
            return NoEvent;
        } else {
            *type_
        };
        if type_ < ll::SDL_FIRSTEVENT || type_ > ll::SDL_LASTEVENT {
            return NoEvent;
        }
        let common: &ll::Struct_SDL_CommonEvent = mem::transmute(raw_event.common());
        let timestamp = common.timestamp as Timestamp;
        match type_ {
            ll::SDL_QUIT => Quit(timestamp),
            ll::SDL_APP_TERMINATING => AppTerminating(timestamp),
            ll::SDL_APP_LOWMEMORY => AppLowMemory(timestamp),
            ll::SDL_APP_WILLENTERBACKGROUND => AppWillEnterBackground(timestamp),
            ll::SDL_APP_DIDENTERBACKGROUND => AppDidEnterBackground(timestamp),
            ll::SDL_APP_WILLENTERFOREGROUND => AppWillEnterForeground(timestamp),
            ll::SDL_APP_DIDENTERFOREGROUND => AppDidEnterForeground(timestamp),
            ll::SDL_WINDOWEVENT => wrap_windowevent(*raw_event.window()),
            ll::SDL_SYSWMEVENT => SysWMEvent(timestamp),
            ll::SDL_KEYDOWN => wrap_keyevent(*raw_event.key()),
            ll::SDL_KEYUP => wrap_keyevent(*raw_event.key()),
            ll::SDL_TEXTEDITING => TextEditing(timestamp),
            ll::SDL_TEXTINPUT => TextInput(timestamp),
            ll::SDL_MOUSEMOTION => MouseMotion(timestamp),
            ll::SDL_MOUSEBUTTONDOWN => MouseButtonDown(timestamp),
            ll::SDL_MOUSEBUTTONUP => MouseButtonUp(timestamp),
            ll::SDL_MOUSEWHEEL => MouseWheel(timestamp),
            ll::SDL_JOYAXISMOTION => JoyAxisMotion(timestamp),
            ll::SDL_JOYBALLMOTION => JoyBallMotion(timestamp),
            ll::SDL_JOYHATMOTION => JoyHatMotion(timestamp),
            ll::SDL_JOYBUTTONDOWN => JoyButtonDown(timestamp),
            ll::SDL_JOYBUTTONUP => JoyButtonUp(timestamp),
            ll::SDL_JOYDEVICEADDED => JoyDeviceAdded(timestamp),
            ll::SDL_JOYDEVICEREMOVED => JoyDeviceRemoved(timestamp),
            ll::SDL_CONTROLLERAXISMOTION => ControllerAxisMotion(timestamp),
            ll::SDL_CONTROLLERBUTTONDOWN => ControllerButtonDown(timestamp),
            ll::SDL_CONTROLLERBUTTONUP => ControllerButtonUp(timestamp),
            ll::SDL_CONTROLLERDEVICEADDED => ControllerDeviceAdded(timestamp),
            ll::SDL_CONTROLLERDEVICEREMOVED => ControllerDeviceRemoved(timestamp),
            ll::SDL_CONTROLLERDEVICEREMAPPED => ControllerDeviceRemapped(timestamp),
            ll::SDL_FINGERDOWN => FingerDown(timestamp),
            ll::SDL_FINGERUP => FingerUp(timestamp),
            ll::SDL_FINGERMOTION => FingerMotion(timestamp),
            ll::SDL_DOLLARGESTURE => DollarGesture(timestamp),
            ll::SDL_DOLLARRECORD => DollarRecord(timestamp),
            ll::SDL_MULTIGESTURE => MultiGesture(timestamp),
            ll::SDL_CLIPBOARDUPDATE => ClipboardUpdate(timestamp),
            ll::SDL_DROPFILE => DropFile(timestamp),
            ll::SDL_USEREVENT => UserEvent(timestamp),
            _ => fail!("std::events::poll() couldn't handle event type: {:?}", type_),
        }
    }
}

fn wrap_windowevent(raw_event: ll::SDL_WindowEvent) -> Event {
    let e = raw_event;
    let t = e.timestamp;
    let w = e.windowID;
    match e.event as c_uint {
        ll::SDL_WINDOWEVENT_SHOWN =>        WindowShown(WindowEvent {timestamp: t, window_id: w}),
        ll::SDL_WINDOWEVENT_HIDDEN =>       WindowHidden(WindowEvent {timestamp: t, window_id: w}),
        ll::SDL_WINDOWEVENT_EXPOSED =>      WindowExposed(WindowEvent {timestamp: t, window_id: w}),
        ll::SDL_WINDOWEVENT_MOVED =>
            WindowMoved(WindowMovedEvent {timestamp: t, window_id: w, position: (e.data1 as int, e.data2 as int)}),
        ll::SDL_WINDOWEVENT_RESIZED =>
            WindowResized(WindowResizedEvent {timestamp: t, window_id: w, size: (e.data1 as uint, e.data2 as uint)}),
        ll::SDL_WINDOWEVENT_SIZE_CHANGED => WindowSizeChanged(WindowEvent {timestamp: t, window_id: w}),
        ll::SDL_WINDOWEVENT_MINIMIZED =>    WindowMinimized(WindowEvent {timestamp: t, window_id: w}),
        ll::SDL_WINDOWEVENT_MAXIMIZED =>    WindowMaximized(WindowEvent {timestamp: t, window_id: w}),
        ll::SDL_WINDOWEVENT_RESTORED =>     WindowRestored(WindowEvent {timestamp: t, window_id: w}),
        ll::SDL_WINDOWEVENT_ENTER =>        WindowEnter(WindowEvent {timestamp: t, window_id: w}),
        ll::SDL_WINDOWEVENT_LEAVE =>        WindowLeave(WindowEvent {timestamp: t, window_id: w}),
        ll::SDL_WINDOWEVENT_FOCUS_GAINED => WindowFocusGained(WindowEvent {timestamp: t, window_id: w}),
        ll::SDL_WINDOWEVENT_FOCUS_LOST =>   WindowFocusLost(WindowEvent {timestamp: t, window_id: w}),
        ll::SDL_WINDOWEVENT_CLOSE =>        WindowClose(WindowEvent {timestamp: t, window_id: w}),
        _ => {
            debug!("std::events::wrap_windowevent() got unknown event {:?}", e);
            NoEvent
        }
    }
}

fn wrap_keyevent(raw_event: ll::SDL_KeyboardEvent) -> Event {
    let e = KeyboardEvent {
        timestamp: raw_event.timestamp,
        window_id: raw_event.windowID,
        state: match raw_event.state {
            ll::SDL_PRESSED => KeyPressed,
            ll::SDL_RELEASED => KeyReleased,
            _ => fail!("std::events::wrap_keyevent() got unknown key state: {:?}", raw_event),
        },
        repeat: raw_event.repeat != 0,
        scancode: unsafe { mem::transmute_copy(&(raw_event.keysym.scancode as uint)) },
        sym: unsafe { mem::transmute_copy(&(raw_event.keysym.sym as uint)) },
        modifiers: EnumSetUtil::from_uint(raw_event.keysym._mod as uint),
    };
    match raw_event._type {
        ll::SDL_KEYDOWN => KeyDown(e),
        ll::SDL_KEYUP => KeyUp(e),
        _ => {
            debug!("std::events::wrap_keyevent() got unknown event {:?}", e);
            NoEvent
        }
    }
}
