use std::borrow::Cow::Borrowed;

use ahash::HashMapExt;
use rustpython::vm::stdlib::StdlibMap;

use kit;

fn main() {
    let mut modules = StdlibMap::new();
    modules.insert(Borrowed("umk"), Box::new(kit::module));
    kit::runtime::init(modules);
    kit::runtime::load(".unimake");
}
