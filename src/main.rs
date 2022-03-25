//! ca2d: 2D Web CAD engine

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod config;
// mod test;

#[macro_use]
extern crate lazy_static;

use gotham::state::State;

fn hello(state: State) -> (State, &'static str) {
    (state, "Hello")
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let argc: usize = argv.len();
    println!("#{argc} {argv:?}");
    //
    let addr = format!("{}:{}", config::IP, config::PORT);
    println!("serving @ http://{}", &addr);
    gotham::start(addr, || Ok(hello));
}
