extern crate yuna;

#[test]
fn create_and_drop_state() {
    let state = yuna::State::new();
    drop(state); //TODO: check if lua_State ist actually destroyed
}
