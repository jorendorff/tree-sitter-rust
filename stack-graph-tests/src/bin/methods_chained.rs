struct A;
struct B;
struct C;

impl A {
    fn b(self) -> B { B }
}

impl B {
    fn c(self) -> C { C }
}

impl C {
    fn xyz(self) {}
}

fn a() -> A {
    A
}

fn main() {
    a().b().c().xyz();
//  ^ defined: 17
    //  ^ defined: 6
    //      ^ defined: 10
    //          ^ defined: 14
}
