use std::ptr;
use std::cast;
use std::libc::{c_uint};
use super::ll;

type Timestamp = u32;
type WindowID = u32;
/*
pub struct CommonEvent { timestamp: Timestamp }
pub struct KeyboardEvent {
    timestamp: Timestamp,
    windowID: Uint32,
    state: Uint8,
    repeat: Uint8,
    padding2: Uint8,
    padding3: Uint8,
    keysym: SDL_Keysym,
}
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
    WindowShown       {timestamp: Timestamp, window_id: WindowID},
    WindowHidden      {timestamp: Timestamp, window_id: WindowID},
    WindowExposed     {timestamp: Timestamp, window_id: WindowID},
    WindowMoved       {timestamp: Timestamp, window_id: WindowID, position: (int, int)},
    WindowResized     {timestamp: Timestamp, window_id: WindowID, size: (uint, uint)},
    WindowSizeChanged {timestamp: Timestamp, window_id: WindowID},
    WindowMinimized   {timestamp: Timestamp, window_id: WindowID},
    WindowMaximized   {timestamp: Timestamp, window_id: WindowID},
    WindowRestored    {timestamp: Timestamp, window_id: WindowID},
    WindowEnter       {timestamp: Timestamp, window_id: WindowID},
    WindowLeave       {timestamp: Timestamp, window_id: WindowID},
    WindowFocusGained {timestamp: Timestamp, window_id: WindowID},
    WindowFocusLost   {timestamp: Timestamp, window_id: WindowID},
    WindowClose       {timestamp: Timestamp, window_id: WindowID},
    SysWMEvent(Timestamp),
    /* Keyboard events */
    KeyDown(Timestamp),
    KeyUp(Timestamp),
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

#[fixed_stack_segment]
pub fn poll() -> Event {
    let mut event = ll::Union_SDL_Event { data: [0u8, ..56] };
    unsafe {
        if ll::SDL_PollEvent(ptr::to_mut_unsafe_ptr(&mut event)) == 0 {
            return NoEvent;
        }
        let common: &ll::Struct_SDL_CommonEvent = cast::transmute(event.common());
        let timestamp = common.timestamp as Timestamp;
        let type_ = *event._type();
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
            ll::SDL_KEYDOWN => KeyDown(timestamp),
            ll::SDL_KEYUP => KeyUp(timestamp),
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
            _ => fail!("std::events::poll() couldn't handle event type: %?", type_),
        }
    }
}

#[fixed_stack_segment]
fn wrap_windowevent(raw_event: ll::SDL_WindowEvent) -> Event {
    let e = raw_event;
    let t = e.timestamp;
    let w = e.windowID;
    match e.event as c_uint {
        ll::SDL_WINDOWEVENT_SHOWN =>        WindowShown       { timestamp: t, window_id: w},
        ll::SDL_WINDOWEVENT_HIDDEN =>       WindowHidden      { timestamp: t, window_id: w},
        ll::SDL_WINDOWEVENT_EXPOSED =>      WindowExposed     { timestamp: t, window_id: w},
        ll::SDL_WINDOWEVENT_MOVED =>        WindowMoved       { timestamp: t, window_id: w,
                                                                position: (e.data1 as int, e.data2 as int)},
        ll::SDL_WINDOWEVENT_RESIZED =>      WindowResized     { timestamp: t, window_id: w,
                                                                size: (e.data1 as uint, e.data2 as uint)},
        ll::SDL_WINDOWEVENT_SIZE_CHANGED => WindowSizeChanged { timestamp: t, window_id: w},
        ll::SDL_WINDOWEVENT_MINIMIZED =>    WindowMinimized   { timestamp: t, window_id: w},
        ll::SDL_WINDOWEVENT_MAXIMIZED =>    WindowMaximized   { timestamp: t, window_id: w},
        ll::SDL_WINDOWEVENT_RESTORED =>     WindowRestored    { timestamp: t, window_id: w},
        ll::SDL_WINDOWEVENT_ENTER =>        WindowEnter       { timestamp: t, window_id: w},
        ll::SDL_WINDOWEVENT_LEAVE =>        WindowLeave       { timestamp: t, window_id: w},
        ll::SDL_WINDOWEVENT_FOCUS_GAINED => WindowFocusGained { timestamp: t, window_id: w},
        ll::SDL_WINDOWEVENT_FOCUS_LOST =>   WindowFocusLost   { timestamp: t, window_id: w},
        ll::SDL_WINDOWEVENT_CLOSE =>        WindowClose       { timestamp: t, window_id: w},
        _ => {
            debug!("std::events::wrap_windowevent() got unknown event %?", e);
            NoEvent
        }
    }
}
