#![cfg_attr(not(test), allow(dead_code))]
#![feature(inclusive_range_syntax)]
#![feature(fn_traits)]
#![feature(unboxed_closures)]

extern crate lalrpop_intern;
extern crate itertools;

#[macro_use]
mod formula;
mod fol;
mod prop;
mod util;

#[test]
fn it_works() {
}
