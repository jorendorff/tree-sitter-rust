mod a {
    pub struct B;

    mod c {
        impl super::B {
            pub fn d(self) {}
        }
    }
}

fn e(ff: a::B) {
    ff.d();
    // ^ defined: 6
}

fn main() { e(a::B); }
