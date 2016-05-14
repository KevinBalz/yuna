extern crate libc;
extern crate lua52_sys as ffi;

mod lauxlib;

use std::rc::Rc;

pub struct LuaContext {
    l: *mut ffi::lua_State,
}

impl LuaContext {
    pub fn new() -> Self {
        let l = unsafe { lauxlib::luaL_newstate() };
        LuaContext { l: l }
    }
}

impl Drop for LuaContext {
    fn drop(&mut self) {
        unsafe {
            ffi::lua_close(self.l);
        }
    }
}


pub struct State {
    context: Rc<LuaContext>,
}

impl State {
    pub fn new() -> Self {
        let context = LuaContext::new();
        unsafe {
            ffi::luaL_openlibs(context.l);
        }

        return State { context: Rc::new(context) };
    }
}

#[derive(Debug,Clone,PartialEq)]
pub enum LuaValue {
    LuaBoolean(bool),
    LuaNumber(f64),
    LuaString(String),
}

impl LuaValue {
    pub fn from_bool(b: bool) -> Self {
        LuaValue::LuaBoolean(b)
    }

    pub fn from_number<N: Into<f64>>(number: N) -> Self {
        LuaValue::LuaNumber(number.into())
    }

    pub fn from_string<S: Into<String>>(s: S) -> Self {
        LuaValue::LuaString(s.into())
    }
}
