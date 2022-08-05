trait TextSprite {
    fn render(self) -> String;
}

struct Broom;

impl TextSprite for Broom {
    fn render(self) -> String {
        "a plain straw broom".to_string()
    }
}

fn main() {
    let mcgee = Broom;
    mcgee.render();
    //    ^ defined: 8
}
