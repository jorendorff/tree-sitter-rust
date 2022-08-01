pub struct B;

impl B {
    pub fn d(self) {}
}

fn e(ff: B) {
    ff.d();
    // ^ defined: 4
}

fn main() { e(B); }
