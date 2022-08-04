/* --- path: src/bin/use_crate.rs --- */
// The full path matters because this test uses `crate`.

mod a {
    fn b() {}
}

use {{a::b}};
//    ^ defined: 4
//       ^ defined: 5
use {{a as aa}};
//    ^ unsupported
use a::{{b as bb}};
//  ^ defined: 4
//       ^ unsupported
use {{crate as c}};

fn main() {
    b();
//  ^ defined: 5
    aa::b();
//  ^ defined: 4
    bb();
//  ^ defined: 5
    c::a::b();
    // ^ defined: 4
}
