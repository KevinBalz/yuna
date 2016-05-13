extern crate libc;
extern crate lua52_sys as ffi;

mod lauxlib;

pub struct State {
    l: *mut ffi::lua_State
}

impl State {

    pub fn new() -> Self {
        let statep;
        unsafe {
            statep = lauxlib::luaL_newstate();
            ffi::luaL_openlibs(statep);
        }

        return State {l: statep};
    }
}

impl Drop for State {
    fn drop(&mut self) {
        unsafe { ffi::lua_close(self.l); }
    }
}
