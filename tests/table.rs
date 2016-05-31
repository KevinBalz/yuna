extern crate yuna;
extern crate lua52_sys as ffi;

use yuna::{Table,LuaContext,LuaRead,LuaWrite, LuaIndex};


#[test]
fn read_and_write_table() {
    let context = LuaContext::new();

    // Create empty table
    unsafe { ffi::lua_newtable(context.l) };
    // Read Table
    let t : Table = LuaRead::lua_read_index(&context,-1).unwrap();

    // Push Table via LuaWrite
    unsafe { LuaWrite::lua_write(&context,&t) };

    // Compare created table with pushed table
    let comp = unsafe { ffi::lua_compare(context.l,-2,-1,ffi::LUA_OPEQ) };
    assert!( comp == 1 );

}

#[test]
fn new_table() {
    let context = LuaContext::new();

    // Create Table
    let t : Table = Table::new(&context);

    // Push Table
    unsafe { LuaWrite::lua_write(&context,&t) };

    // Check if pushed table is really a table
    let is_table = unsafe { ffi::lua_istable(context.l,-1 ) };
    assert!( is_table );

}

#[test]
fn clone_table() {
    let context = LuaContext::new();

    // Create and clone table
    let table : Table = Table::new(&context);
    let clone = table.clone();

    // Push both tables
    unsafe { LuaWrite::lua_write(&context,&table) };
    unsafe { LuaWrite::lua_write(&context,&clone) };

    // Compare created table with cloned table
    let comp = unsafe { ffi::lua_compare(context.l,-2,-1,ffi::LUA_OPEQ) };
    assert!( comp == 1 );

}

#[test]
fn compare_table() {
    let context = LuaContext::new();

    let table : Table = Table::new(&context);
    let same = table.clone();
    let notsame = Table::new(&context);

    assert!(table.eq(&same));
    assert!(table.ne(&notsame));
}

#[test]
fn lua_index_table() {
    let context = LuaContext::new();
    let mut table = Table::new(&context);

    table.set("answer",42);

    let readanswer : i32 = table.read("answer").unwrap();
    assert_eq!(readanswer,42);

    let getanswer = table.get("answer");
    assert_eq!(getanswer,yuna::LuaValue::LuaNumber(42.0));
}
