extern crate yuna;
extern crate lua52_sys as ffi;

use yuna::{Table,LuaContext,LuaRead,LuaWrite};


#[test]
fn read_and_write_table() {
    let context = LuaContext::new();

    // Create empty table
    unsafe { ffi::lua_newtable(context.l) };
    // Read Table
    let t : Table = LuaRead::lua_read_index(&context,-1).unwrap();

    // Push Table via LuaWrite
    unsafe { LuaWrite::lua_write(&context,t) };

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
    unsafe { LuaWrite::lua_write(&context,t) };

    // Check if pushed table is really a table
    let is_table = unsafe { ffi::lua_istable(context.l,-1 ) };
    assert!( is_table );

}
