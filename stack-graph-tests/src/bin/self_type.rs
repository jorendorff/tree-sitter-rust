#![allow(dead_code)]

use std::sync::Arc;

struct List<T> {
    head: T,
    tail: Option<Box<Self>>,
    //               ^ defined: 5
}

impl<T> List<T> {
    fn reverse(self) -> Self {
        //              ^ defined: 5
        todo!();
    }
}

enum Value {
    Nil,
    FixNum(i64),
    Cons(Arc<Self>, Arc<Self>),
    //       ^ defined: 18
    //                  ^ defined: 18
}

trait Consable {
    type Head;

    type Tail;

    fn cons(head: Self::Head, tail: Self) -> Self;
    //            ^ defined: 26
    //                              ^ defined: 26
    //                                       ^ defined: 26
}

impl<T> Consable for List<T>
where
    Self: Sized // bizarre bound, but we want to test Self in this context
//  ^ defined: 5
{
    type Head = T;
    //          ^ defined: 37

    type Tail = Self; // type alias declarations do not bind Self, this is the Self of the impl
    //          ^ defined: 5

    fn cons(head: Self::Head, tail: Self) -> Self { // fn scopes do not bind Self
        //        ^ defined: 5
        //                          ^ defined: 5
        //                                   ^ defined: 5
        Self {
    //  ^ defined: 5
            head,
            tail: Some(Box::new(tail)),
        }
    }
}

union VeryReasonable {
    d: f64,
    u: usize,
    p: *mut Self,
    //      ^ defined: 60
}

fn main() {}
