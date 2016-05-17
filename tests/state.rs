extern crate yuna;

use yuna::LuaIndex;

#[test]
fn create_and_drop_state() {
    let state = yuna::State::new();
    drop(state); //TODO: check if lua_State ist actually destroyed
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
