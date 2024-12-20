extern crate libc;

extern {
    fn addition(input1: libc::c_int,input2: libc::c_int) -> libc::c_int;
}

fn main() {
    let input1 = 4;
    let input2 = 5;
    let output = unsafe { addition(input1,input2) };
    println!("{} + {} = {}", input1, input2 , output);
}