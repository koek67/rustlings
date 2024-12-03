fn call_me(num: Option<u8>) {
    let num = num.or_else(|| {Some(2)}).expect("noooo");
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // TODO: Fix the function call.
    call_me(None);
}
