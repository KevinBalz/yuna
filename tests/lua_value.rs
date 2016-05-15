extern crate yuna;
extern crate lua52_sys as ffi;

use yuna::{LuaValue, LuaContext,LuaRead,LuaWrite};

#[test]
fn create_luavalue_from_number_float() {
    let num = 43.5;

    let value = LuaValue::from_number(num);

    assert_eq!(LuaValue::LuaNumber(num),value);
}

#[test]
fn create_luavalue_from_number_integer() {
    let num : i32 = -123;

    let value = LuaValue::from_number(num);

    assert_eq!(LuaValue::LuaNumber(num.into()),value);
}

#[test]
fn create_luavalue_from_number_unsigned() {
    let num : u32 = 256;

    let value = LuaValue::from_number(num);

    assert_eq!(LuaValue::LuaNumber(num.into()),value);
}

#[test]
fn create_luavalue_from_boolean() {
    let truevalue = LuaValue::from_bool(true);

    assert_eq!(LuaValue::LuaBoolean(true),truevalue);

    let falsevalue = LuaValue::from_bool(false);

    assert_eq!(LuaValue::LuaBoolean(false),falsevalue);
}

#[test]
fn create_luavalue_from_string() {
    let teststr = "LuaRocks";

    let valuefromstring = LuaValue::from_string(String::from(teststr));

    assert_eq!(LuaValue::LuaString(String::from(teststr)),valuefromstring);

    let valuefromstr = LuaValue::from_string(teststr);

    assert_eq!(LuaValue::LuaString(String::from(teststr)),valuefromstr);
}

#[test]
fn read_luavalue() {
    let context = LuaContext::new();
    let teststr = "LuaRocks";

    unsafe { ffi::lua_pushnil(context.l) };
    let valnil : LuaValue = LuaRead::lua_read_index(&context,-1).unwrap();
    assert_eq!(valnil,LuaValue::Nil);

    unsafe { LuaWrite::lua_write(&context, true) };
    let valb : LuaValue = LuaRead::lua_read_index(&context,-1).unwrap();
    assert_eq!(valb,LuaValue::LuaBoolean(true));

    unsafe { LuaWrite::lua_write(&context, teststr) };
    let vals : LuaValue = LuaRead::lua_read_index(&context,-1).unwrap();
    assert_eq!(vals,LuaValue::LuaString(String::from(teststr)));

    unsafe { LuaWrite::lua_write(&context, 68.3) };
    let valn : LuaValue = LuaRead::lua_read_index(&context,-1).unwrap();
    assert_eq!(valn,LuaValue::LuaNumber(68.3));
}

#[test]
fn write_luavalue() {
    let context = LuaContext::new();

    unsafe { LuaWrite::lua_write(&context,LuaValue::Nil) };
    let nilread : LuaValue = LuaRead::lua_read_index(&context,-1).unwrap();
    assert_eq!(LuaValue::Nil,nilread);

    unsafe { LuaWrite::lua_write(&context,LuaValue::LuaBoolean(false)) };
    let boolread : LuaValue = LuaRead::lua_read_index(&context,-1).unwrap();
    assert_eq!(LuaValue::LuaBoolean(false),boolread);

    unsafe { LuaWrite::lua_write(&context,LuaValue::LuaNumber(22.43)) };
    let numread : LuaValue = LuaRead::lua_read_index(&context,-1).unwrap();
    assert_eq!(LuaValue::LuaNumber(22.43),numread);

    let strval = LuaValue::LuaString(String::from("LuaRocks"));
    unsafe { LuaWrite::lua_write(&context,strval.clone()) };
    let strread : LuaValue = LuaRead::lua_read_index(&context,-1).unwrap();
    assert_eq!(strval,strread);
}
