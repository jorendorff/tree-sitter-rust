struct X;

trait Y {
    type X;

    fn z(&self) -> usize {
        impl X { // not to be confused with Self::X
        //   ^ defined: 1
            fn abc(&self) -> &'static str { "ok" }
        }
        2
    }
}

fn main() {
    let n = X.abc();
    //        ^ defined: 9
    println!("{n}");
}
