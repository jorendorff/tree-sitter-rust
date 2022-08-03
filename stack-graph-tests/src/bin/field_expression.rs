pub struct Student {
    name: String
}

pub struct ReportCard {
    student: Student,
    #[allow(dead_code)]
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

pub fn if_expr_type(select_roman: bool) -> String {
    let student = if select_roman {
        Student { name: "roman".to_string() }
    } else {
        Student { name: "ram".to_string() }
    };
    student.name
    //      ^ defined: 2
}

pub fn block_type(select_roman: bool) -> String {
    let student1 = {
        Student { name: "ram".to_string() }
    };
    let _ = &student1.name;
    //                ^ defined: 2

    // Blocks where the final expression ends with a block are tricky.
    // Doable, but for a bug in the tree-sitter query engine:
    // <https://github.com/tree-sitter/tree-sitter/issues/1811>
    let roman = {
        {
            Student { name: "roman".to_string() }
        }
    };
    let _ = &roman.name;
    //             ^ UNSUPPORTED

    let student = {
        if select_roman {
            roman
        } else {
            student1
        }
    };
    student.name
    //      ^ UNSUPPORTED
}

fn main() {
    let kaitlin = Student { name: "kaitlin".to_string() };
    let _ = kaitlin.name;
    //               ^ defined: 2
    let roman = Student { name: "roman".to_string() };
    let _ = (&roman).name;
    //               ^ defined: 2
}
