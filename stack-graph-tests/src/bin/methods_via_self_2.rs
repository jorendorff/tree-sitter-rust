mod a {
    pub struct B;

    impl self::B {
        pub fn d(self) {}
    }
}

fn e(ff: a::B) {
    ff.d();
    // ^ defined: 5
}

fn main() { e(a::B); }
