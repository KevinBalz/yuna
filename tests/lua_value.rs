extern crate yuna;

use yuna::LuaValue;

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
