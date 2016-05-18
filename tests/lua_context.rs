extern crate yuna;

use yuna::LuaContext;

#[test]
fn create_and_close_context() {
    let context = LuaContext::new();
    drop(context); //TODO: check if lua_State ist actually destroyed
}

#[test]
fn clone_context() {
    let context = LuaContext::new();
    let clone = context.clone();

    assert_eq!(context.l,clone.l);
}
