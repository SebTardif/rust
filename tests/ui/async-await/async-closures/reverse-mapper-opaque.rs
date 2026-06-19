// Regression test for #469: ReverseMapper::fold_ty missing CoroutineClosure
// and fold_const not recursing through constants.
// Before the fix, async closures in contexts with opaque types could
// produce incorrect type mappings because CoroutineClosure was not
// handled in the ReverseMapper fold implementation.

//@ edition:2021
//@ check-pass

fn takes_async_fn(_f: impl AsyncFn()) {}

fn opaque_return() -> impl Sized {
    let c = async || {};
    takes_async_fn(c);
    42u32
}

fn main() {
    let _ = opaque_return();
}
