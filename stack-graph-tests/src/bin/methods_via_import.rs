/* --- path: src/bin/methods_via_import.rs --- */
// The full path matters because this test uses `crate::`.

mod a {
    pub struct B;
}

mod c {
    use crate::a::B;

    impl B {
        pub fn d(self) {}
    }
}

fn e(ff: a::B) {
    ff.d();
    // ^ defined: 12
}

fn main() { e(a::B); }
