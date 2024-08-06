// TODO: Add the missing type of the argument `num` after the colon `:`.
fn call_me(num:f32) 
{
    let num_i: i32 = num as i32 ;       //Experimentation with conversion of data types
    for i in 0..num_i {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3.5);
}
