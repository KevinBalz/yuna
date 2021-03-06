extern crate libc;
extern crate lua52_sys as ffi;

mod lauxlib;

use std::cell::Cell;

/// Holds the raw `lua_State`.
pub struct LuaContext {
    /// The raw lua state
    pub l: *mut ffi::lua_State,
    refcount: Cell<usize>
}

impl LuaContext {
    pub fn new() -> Self {
        let l = unsafe { lauxlib::luaL_newstate() };
        let refcount = Cell::new(1);
        LuaContext { l: l, refcount: refcount }
    }
}

impl Clone for LuaContext {
    fn clone(&self) -> Self {
        let refcount = self.refcount.clone();
        refcount.set(refcount.get() + 1);
        LuaContext { l: self.l, refcount: refcount }
    }
}

impl Drop for LuaContext {
    fn drop(&mut self) {
        self.refcount.set(self.refcount.get() - 1);
        if self.refcount.get() == 0 {
            unsafe { ffi::lua_close(self.l); }
        }
    }
}

/// Holds the Lua State and provides functions for interacting with the Lua environment.
pub struct State {
    context: LuaContext,
}

impl State {

    /// Creates a new State.
    pub fn new() -> Self {
        let context = LuaContext::new();

        State { context: context }
    }

    /// Loads and runs the given string.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut state = yuna::State::new();
    /// state.openlibs();
    ///
    /// state.do_string("print(\"yuna rocks!\")"); // prints "yuna rocks!"
    /// ```
    pub fn do_string<S: AsRef<str>>(&mut self,code: S) {
        let cstr = std::ffi::CString::new(code.as_ref()).unwrap().as_ptr();
        unsafe {
            lauxlib::luaL_dostring(self.context.l,cstr);
        }
    }

    /// Opens all standard Lua libraries.
    pub fn openlibs(&mut self) {
        unsafe { ffi::luaL_openlibs(self.context.l); }
    }

    /// Returns the global Table.
    pub fn global(&self) -> Table {
        unsafe {
            ffi::lua_pushglobaltable(self.context.l);
            Table::ref_from_stack(&self.context)
        }

    }
}

impl LuaIndex for State {
    fn read<K: LuaWrite,V: LuaRead>(&self,key: K) -> Result<V,()> {
        let result;
        unsafe {
            ffi::lua_pushglobaltable(self.context.l);
            LuaWrite::lua_write(&self.context,key);
            ffi::lua_gettable(self.context.l,-2);
            result = LuaRead::lua_read_index(&self.context,-1);
            ffi::lua_pop(self.context.l,2);
        }
        result
    }

    fn set<K: LuaWrite,V: LuaWrite>(&mut self,key: K,value: V) {
        unsafe {
            ffi::lua_pushglobaltable(self.context.l);
            LuaWrite::lua_write(&self.context,key);
            LuaWrite::lua_write(&self.context,value);
            ffi::lua_settable(self.context.l,-3);
            ffi::lua_pop(self.context.l,1);
        }
    }
}

/// Trait for Objects which are reference values in lua. (e.g. table,function...).
pub trait LuaRef : Sized {
    fn get_context(&self) -> &LuaContext;
    fn get_refindex(&self) -> libc::c_int;

    unsafe fn from_refindex(context: &LuaContext,refindex: libc::c_int) -> Self;

    unsafe fn ref_from_stack(context: &LuaContext) -> Self {
        let refindex = lauxlib::luaL_ref(context.l,ffi::LUA_REGISTRYINDEX);
        Self::from_refindex(context,refindex)
    }

    unsafe fn push_reference(context: &LuaContext,refid: libc::c_int) {
        ffi::lua_rawgeti(context.l, ffi::LUA_REGISTRYINDEX, refid);
    }

    unsafe fn write_self(&self) {
        Self::push_reference(self.get_context(),self.get_refindex());
    }
}

impl<T: LuaRef> LuaRead for T {
    fn lua_read_index(context: &LuaContext,index: i32) -> Result<Self,()> {
        let value = unsafe {
            ffi::lua_pushvalue(context.l, index);
            LuaRef::ref_from_stack(context)
        };

        Ok(value)
    }
}


impl<'a,T: LuaRef> LuaWrite for &'a T {
    unsafe fn lua_write(context: &LuaContext,value: Self) {
        T::push_reference(context,value.get_refindex());
    }
}


/// Holds a reference to a lua table.
pub struct Table {
    context: LuaContext,
    refindex: libc::c_int,
}

//TODO: proper implementation
impl std::fmt::Debug for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Table {{ {} }}", self.refindex)
    }
}

impl Table {

    /// Creates a new Table and returns a reference to it.
    pub fn new(context: &LuaContext) -> Self {
        unsafe { ffi::lua_newtable(context.l) };
        let t = LuaRead::lua_read_index(context,-1).unwrap();
        unsafe { ffi::lua_pop(context.l,1) };
        t
    }
}

impl LuaRef for Table {
    fn get_context(&self) -> &LuaContext {
        &self.context
    }

    fn get_refindex(&self) -> libc::c_int {
        self.refindex
    }

    unsafe fn from_refindex(context: &LuaContext,refindex: libc::c_int) -> Self {
        Table {context: context.clone(), refindex: refindex }
    }
}


impl LuaIndex for Table {
    fn read<K: LuaWrite,V: LuaRead>(&self,key: K) -> Result<V,()> {
        let result;
        unsafe {
            LuaWrite::lua_write(&self.context,self);
            LuaWrite::lua_write(&self.context,key);
            ffi::lua_gettable(self.context.l,-2);
            result = LuaRead::lua_read_index(&self.context,-1);
            ffi::lua_pop(self.context.l,2);
        }
        result
    }

    fn set<K: LuaWrite,V: LuaWrite>(&mut self,key: K,value: V) {
        unsafe {
            self.write_self();
            LuaWrite::lua_write(&self.context,key);
            LuaWrite::lua_write(&self.context,value);
            ffi::lua_settable(self.context.l,-3);
            ffi::lua_pop(self.context.l,1);
        }
    }
}

//TODO: find a way to implement these generally for LuaRef
impl Clone for Table {
    fn clone(&self) -> Self {
        unsafe {
            self.write_self();
            Self::ref_from_stack(&self.context)
        }
    }
}

impl PartialEq for Table {
    fn eq(&self, other: &Table) -> bool {
        // Push both references
        unsafe { LuaWrite::lua_write(&self.context,self) };
        unsafe { LuaWrite::lua_write(&self.context,other) };

        // Compare both references on the stack lua_compare
        let comp = unsafe { ffi::lua_compare(self.context.l,-2,-1,ffi::LUA_OPEQ) };
        comp == 1
    }
}

/// A Enum which can represent every possible type in Lua.
#[derive(Debug,Clone,PartialEq)]
pub enum LuaValue {
    LuaBoolean(bool),
    LuaNumber(f64),
    LuaString(String),
    LuaTable(Table),
    Nil
}

impl LuaValue {

    /// Creates a `LuaValue` from a `boolean`
    pub fn from_bool(b: bool) -> Self {
        LuaValue::LuaBoolean(b)
    }

    /// Creates a `LuaValue` from any number
    pub fn from_number<N: Into<f64>>(number: N) -> Self {
        LuaValue::LuaNumber(number.into())
    }

    /// Creates a `LuaValue` from a string
    pub fn from_string<S: Into<String>>(s: S) -> Self {
        LuaValue::LuaString(s.into())
    }
}

impl LuaRead for LuaValue {
    fn lua_read_index(context: &LuaContext,index: i32) -> Result<Self,()> {
        let tp = unsafe { ffi::lua_type(context.l,index) };
        Ok(match tp {
            ffi::LUA_TBOOLEAN  => LuaValue::LuaBoolean(LuaRead::lua_read_index(context,index).unwrap()),
            ffi::LUA_TNUMBER   => LuaValue::LuaNumber(LuaRead::lua_read_index(context,index).unwrap()),
            ffi::LUA_TSTRING   => LuaValue::LuaString(LuaRead::lua_read_index(context,index).unwrap()),
            ffi::LUA_TTABLE    => LuaValue::LuaTable(LuaRead::lua_read_index(context,index).unwrap()),
            ffi::LUA_TFUNCTION => unimplemented!(),
            ffi::LUA_TUSERDATA => unimplemented!(),
            ffi::LUA_TNIL      => LuaValue::Nil,
            i => panic!("Unknown lua type \"{}\"",i)
        })
    }
}

impl LuaWrite for LuaValue {
    unsafe fn lua_write(context: &LuaContext,value: Self) {
        match value {
            LuaValue::LuaBoolean(b) => LuaWrite::lua_write(context,b),
            LuaValue::LuaNumber(n)  => LuaWrite::lua_write(context,n),
            LuaValue::LuaString(st) => LuaWrite::lua_write(context,st.as_str()),
            LuaValue::LuaTable(t)   => LuaWrite::lua_write(context,&t),
            LuaValue::Nil           => ffi::lua_pushnil(context.l),
        }
    }
}

/// A Trait which represent types which can be read from the lua context
pub trait LuaRead: Sized {
    fn lua_read_index(context: &LuaContext,index: i32) -> Result<Self,()>;
}

/// A Trait which represent types which can be pushed to the lua context
pub trait LuaWrite {
    unsafe fn lua_write(context: &LuaContext,value: Self);
}

impl LuaRead for bool {
    fn lua_read_index(context: &LuaContext,index: i32) -> Result<Self,()> {
        let b = unsafe { ffi::lua_toboolean(context.l,index) };
        Ok(b != 0)
    }
}

impl LuaWrite for bool {
    unsafe fn lua_write(context: &LuaContext,value: Self) {
        let cbool = if value {1} else {0};
        ffi::lua_pushboolean(context.l,cbool);
    }
}

impl LuaRead for String {
    fn lua_read_index(context: &LuaContext,index: i32) -> Result<Self,()> {
        let cstr = unsafe { ffi::lua_tostring(context.l,index) };
        let s = unsafe { std::ffi::CStr::from_ptr(cstr).to_string_lossy().into_owned() };
        Ok(s)
    }
}

impl<'s> LuaWrite for &'s str {
    unsafe fn lua_write(context: &LuaContext,value: Self) {
        let cstr = std::ffi::CString::new(value).unwrap();;
        ffi::lua_pushstring(context.l,cstr.as_ptr());
    }
}

macro_rules! impl_integer(
    ($t:ident) => (
        impl LuaRead for $t {
            fn lua_read_index(context: &LuaContext,index: i32) -> Result<Self,()> {
                let mut isnum = 0;
                let i = unsafe { ffi::lua_tointegerx(context.l,index,&mut isnum) };
                match isnum {
                    0 => Err(()),
                    _ => Ok(i as $t),
                }
            }
        }

        impl LuaWrite for $t {
            unsafe fn lua_write(context: &LuaContext,value: Self) {
                ffi::lua_pushinteger(context.l,value as ffi::lua_Integer);
            }
        }
    );
);

impl_integer!(i8);
impl_integer!(i16);
impl_integer!(i32);

macro_rules! impl_unsigned(
    ($t:ident) => (
        impl LuaRead for $t {
            fn lua_read_index(context: &LuaContext,index: i32) -> Result<Self,()> {
                let mut isnum = 0;
                let u = unsafe { ffi::lua_tounsignedx(context.l,index,&mut isnum) };
                match isnum {
                    0 => Err(()),
                    _ => Ok(u as $t),
                }
            }
        }

        impl LuaWrite for $t {
            unsafe fn lua_write(context: &LuaContext,value: Self) {
                ffi::lua_pushunsigned(context.l,value as ffi::lua_Unsigned);
            }
        }
    );
);

impl_unsigned!(u8);
impl_unsigned!(u16);
impl_unsigned!(u32);

macro_rules! impl_float(
    ($t:ident) => (
        impl LuaRead for $t {
            fn lua_read_index(context: &LuaContext,index: i32) -> Result<Self,()> {
                let mut isnum = 0;
                let f = unsafe { ffi::lua_tonumberx(context.l,index,&mut isnum) };
                match isnum {
                    0 => Err(()),
                    _ => Ok(f as $t),
                }
            }
        }

        impl LuaWrite for $t {
            unsafe fn lua_write(context: &LuaContext,value: Self) {
                ffi::lua_pushnumber(context.l,value as ffi::lua_Number);
            }
        }
    );
);

impl_float!(f32);
impl_float!(f64);

/// A trait which represents objects which can be indexed like e.g. a lua table.
pub trait LuaIndex {
    fn read<K: LuaWrite,V: LuaRead>(&self,key: K) -> Result<V,()>;
    fn set<K: LuaWrite,V: LuaWrite>(&mut self,key: K,value: V);

    fn get<K: LuaWrite>(&self,key: K) -> LuaValue {
        self.read(key).unwrap()
    }
}
