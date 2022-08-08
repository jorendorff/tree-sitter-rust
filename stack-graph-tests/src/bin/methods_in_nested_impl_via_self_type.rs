struct Broom;

impl Broom {
    #[allow(dead_code)]
    fn dance(&self) {
        impl Self {
            fn sweep(&self) {
                println!("ok");
            }
        }
    }
}

fn main() {
    Broom.sweep();
    //    ^ defined: 7
}
