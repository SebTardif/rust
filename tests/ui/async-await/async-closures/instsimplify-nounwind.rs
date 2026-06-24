// Regression test for #477: simplify_nounwind_call missing CoroutineClosure arm.
// Before the fix, the MIR instsimplify pass would ICE (bug!("unexpected body ty"))
// when encountering an async closure body type during nounwind simplification.

//@ edition:2021
//@ build-pass

async fn call_once(f: impl std::future::Future<Output = ()>) {
    f.await;
}

fn main() {
    let _ = async {
        let x = 1i32;
        let c = async move || {
            let _ = x;
        };
        call_once(c()).await;
    };
}
