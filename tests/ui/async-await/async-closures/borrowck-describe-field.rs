// Regression test for #478: describe_field_from_ty missing CoroutineClosure.
// Before the fix, borrow errors involving async closure captures would show
// "field 0" instead of the captured variable name, because the CoroutineClosure
// type was not recognized and fell through to the default arm.

//@ edition:2021

fn main() {
    let mut x = String::from("hello");

    let c = async || {
        x.push_str(" world"); //~ ERROR
    };

    // Immutable borrow of x while async closure holds mutable borrow
    println!("{}", x); //~ ERROR

    let _ = c;
}
