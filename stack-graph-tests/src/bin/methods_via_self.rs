struct A;

impl self::A {
    fn b(self) {}
}

fn c(dd: A) {
    dd.b();
    // ^ defined: 4
}

fn main() { c(A); }
