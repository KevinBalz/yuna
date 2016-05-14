extern crate libc;
extern crate lua52_sys as ffi;

mod lauxlib;

use std::rc::Rc;

pub struct LuaContext {
    pub l: *mut ffi::lua_State,
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

pub trait LuaRead: Sized {
    fn lua_read_index(context: &LuaContext,index: i32) -> Result<Self,()>;
}

pub trait LuaWrite {
    unsafe fn lua_write(context: &LuaContext,value: Self);
}

impl LuaRead for bool {
    fn lua_read_index(context: &LuaContext,index: i32) -> Result<Self,()> {
        let b = unsafe { ffi::lua_toboolean(context.l,index) };
        Ok(b != 0)
    }
}

impl LuaWrite for bool {
    unsafe fn lua_write(context: &LuaContext,value: Self) {
        let cbool = if value {1} else {0};
        ffi::lua_pushboolean(context.l,cbool);
    }
}

macro_rules! impl_integer(
    ($t:ident) => (
        impl LuaRead for $t {
            fn lua_read_index(context: &LuaContext,index: i32) -> Result<Self,()> {
                let mut isnum = 0;
                let i = unsafe { ffi::lua_tointegerx(context.l,index,&mut isnum) };
                match isnum {
                    0 => Err(()),
                    _ => Ok(i as $t),
                }
            }
        }

        impl LuaWrite for $t {
            unsafe fn lua_write(context: &LuaContext,value: Self) {
                ffi::lua_pushinteger(context.l,value as ffi::lua_Integer);
            }
        }
    );
);

impl_integer!(i8);
impl_integer!(i16);
impl_integer!(i32);

macro_rules! impl_unsigned(
    ($t:ident) => (
        impl LuaRead for $t {
            fn lua_read_index(context: &LuaContext,index: i32) -> Result<Self,()> {
                let mut isnum = 0;
                let u = unsafe { ffi::lua_tounsignedx(context.l,index,&mut isnum) };
                match isnum {
                    0 => Err(()),
                    _ => Ok(u as $t),
                }
            }
        }

        impl LuaWrite for $t {
            unsafe fn lua_write(context: &LuaContext,value: Self) {
                ffi::lua_pushunsigned(context.l,value as ffi::lua_Unsigned);
            }
        }
    );
);

impl_unsigned!(u8);
impl_unsigned!(u16);
impl_unsigned!(u32);

macro_rules! impl_float(
    ($t:ident) => (
        impl LuaRead for $t {
            fn lua_read_index(context: &LuaContext,index: i32) -> Result<Self,()> {
                let mut isnum = 0;
                let f = unsafe { ffi::lua_tonumberx(context.l,index,&mut isnum) };
                match isnum {
                    0 => Err(()),
                    _ => Ok(f as $t),
                }
            }
        }

        impl LuaWrite for $t {
            unsafe fn lua_write(context: &LuaContext,value: Self) {
                ffi::lua_pushnumber(context.l,value as ffi::lua_Number);
            }
        }
    );
);

impl_float!(f32);
impl_float!(f64);
