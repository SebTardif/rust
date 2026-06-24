// Regression test for #479: maybe_drop_guard missing CoroutineClosure in type list.
// Before the fix, async closures holding Drop types could produce false
// "unused variable" warnings because liveness analysis did not recognize
// CoroutineClosure as a type that may have drop guards.

//@ edition:2021
//@ check-pass

#![deny(unused_variables)]

struct NeedsDrop(i32);

impl Drop for NeedsDrop {
    fn drop(&mut self) {}
}

fn main() {
    let _ = async {
        let d = NeedsDrop(1);
        let c = async move || {
            let _ = &d;
        };
        c().await;
    };
}
