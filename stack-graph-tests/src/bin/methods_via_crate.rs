/* --- path: src/bin/methods_via_crate.rs --- */
// The full path matters because this test uses `crate::`.

pub struct A;

impl A {
    pub fn b(self) {}
}

mod c {
    pub struct D;

    impl D {
        pub fn e(self) {}
    }
}

impl crate::A {
    pub fn b2(self) {}
}

impl crate::c::D {
    pub fn e2(self) {}
}

fn f(gg: crate::A) {
    gg.b();
    // ^ defined: 7
    gg.b2();
    // ^ defined: 19
}

fn h(ii: crate::c::D) {
    ii.e();
    // ^ defined: 14
    ii.e2();
    // ^ defined: 23
}

fn main() { e(B); }
