// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i32 
{
    let num_mult: i32 = num * num;
    num_mult        //lines without ; act as return statement
}

fn cube(num: i32) -> i64 
{
    let cube_of_num = (num * num * num) as i64;
    println!("Test from cube function");        //lines without ; needs to be the last line of the code, if not error is given, in future this can be used as early return statement if needed
    cube_of_num
}

fn main() {
    let answer = square(3);
    let cubic  = cube(3);
    println!("The square of 3 is {answer} and cube is {cubic}");
}
