#[macro_use]
extern crate lazy_static;

mod backend;
mod terminator;

use backend::Backend;

use gdnative::prelude::*;

pub type Coord = PoolArray<i32>;

fn init(handle: InitHandle) {
    handle.add_class::<Backend>();
}

godot_init!(init);
