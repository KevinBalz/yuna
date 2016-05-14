extern crate yuna;
extern crate lua52_sys as ffi;

use yuna::LuaRead;

#[test]
fn read_bool() {
    let context = yuna::LuaContext::new();

    unsafe { ffi::lua_pushboolean(context.l,1) };

    let t : Result<bool,()> = LuaRead::lua_read_index(&context, -1);
    assert_eq!(t,Ok(true));

    unsafe { ffi::lua_pushboolean(context.l,0) };

    let f : Result<bool,()> = LuaRead::lua_read_index(&context, -1);
    assert_eq!(f,Ok(false));
}

#[test]
fn read_integers() {
    let context = yuna::LuaContext::new();

    unsafe { ffi::lua_pushinteger(context.l,-84 as isize) };

    let a : Result<i8,()>  = LuaRead::lua_read_index(&context, -1);
    assert_eq!(a,Ok(-84));

    let b : Result<i16,()> = LuaRead::lua_read_index(&context, -1);
    assert_eq!(b,Ok(-84));

    let c : Result<i32,()> = LuaRead::lua_read_index(&context, -1);
    assert_eq!(c,Ok(-84));

}

#[test]
fn read_unsigned() {
    let context = yuna::LuaContext::new();

    unsafe { ffi::lua_pushinteger(context.l,116 as isize) };

    let a : Result<u8,()>  = LuaRead::lua_read_index(&context, -1);
    assert_eq!(a,Ok(116));

    let b : Result<u16,()> = LuaRead::lua_read_index(&context, -1);
    assert_eq!(b,Ok(116));

    let c : Result<u32,()> = LuaRead::lua_read_index(&context, -1);
    assert_eq!(c,Ok(116));

}

#[test]
fn read_float() {
    let context = yuna::LuaContext::new();

    unsafe { ffi::lua_pushnumber(context.l,38.342) };

    let a : Result<f32,()> = LuaRead::lua_read_index(&context, -1);
    assert_eq!(a,Ok(38.342));

    let b : Result<f64,()> = LuaRead::lua_read_index(&context, -1);
    assert_eq!(b,Ok(38.342));

}
