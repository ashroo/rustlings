fn call_me(num: u8)         //testing different datatypes and their range u8=unsigned int 8bit if this is changed to i8 the code will break
{
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // TODO: Fix the function call.
    call_me(129);
}
