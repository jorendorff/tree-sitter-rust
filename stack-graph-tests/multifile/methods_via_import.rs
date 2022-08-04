/* --- path: src/a.rs --- */

pub struct B;

/* --- path: src/c.rs --- */

use crate::a::B;

impl B {
    pub fn d(self) {}
}

/* --- path: src/main.rs --- */

mod a;
mod c;

fn e(ff: a::B) {
    ff.d();
    // ^ defined: 10
}

fn main() { e(a::B); }
