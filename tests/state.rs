extern crate yuna;

use yuna::LuaIndex;

#[test]
fn create_and_drop_state() {
    let state = yuna::State::new();
    drop(state); //TODO: check if lua_State ist actually destroyed
}

#[test]
fn initial_state_without_libs() {
    let state = yuna::State::new();

    let tablelib = state.get("table");
    assert_eq!(tablelib,yuna::LuaValue::Nil);
}

#[test]
fn state_openlibs() {
    let mut state = yuna::State::new();

    state.openlibs();

    let tablelib = state.get("table");
    assert!(tablelib != yuna::LuaValue::Nil);
}

#[test]
fn lua_index_state() {
    let mut state = yuna::State::new();

    state.set("answer",42);

    let readanswer : i32 = state.read("answer").unwrap();
    assert_eq!(readanswer,42);

    let getanswer = state.get("answer");
    assert_eq!(getanswer,yuna::LuaValue::LuaNumber(42.0));
}

#[test]
fn state_do_string() {
    let num : i32 = 86;
    let mut state = yuna::State::new();

    state.do_string(format!("num = {}",num));

    let r = state.read("num").unwrap();

    assert_eq!(num,r);
}

#[test]
fn state_global() {
    let mut state = yuna::State::new();


    let global : yuna::Table = state.global();
    state.set("answer",42 as i32);

    assert_eq!(global.get("answer"),state.get("answer"));
}
