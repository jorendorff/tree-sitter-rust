pub struct Student {
    name: String
}

pub struct ReportCard {
    student: Student,
    grades: Vec<f64>,
}

pub fn get_name(s: Student) -> String {
    s.name
//    ^ defined: 2
}

pub fn get_name_by_ref(s: &Student) -> &str {
    &s.name
    // ^ defined: 2
}

pub fn card_name(r: ReportCard) -> String {
    r.student.name
    //        ^ defined: 2
}

pub fn card_name_by_ref(r: &ReportCard) -> &str {
    &r.student.name
    //         ^ defined: 2
}

fn main() {
    let kaitlin = Student { name: "kaitlin".to_string() };
    let _ = kaitlin.name;
    //              ^ defined: 2
}
