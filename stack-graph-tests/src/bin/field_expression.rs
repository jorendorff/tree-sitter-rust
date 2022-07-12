struct Student {
    name: String
}

fn main() {
    let kaitlin = Student { name: "kaitlin".to_string() };
    let _ = kaitlin.name;
    //              ^ defined: 2
}
