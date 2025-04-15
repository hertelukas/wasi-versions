#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Comment me to get "module does not export `cabi_realloc`"
    let _ = getrandom::u64().unwrap();
}
