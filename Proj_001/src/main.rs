fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd(){
    assert_eq!(gcd(14,15),1);
}

#[test]
fn test_gcd_1(){
    assert_eq!(gcd(12,15),3);
}
fn main() {
    let mut x: i32 = 10;
    x = x - 12;
    println!("Hello, world! {}", x);
    
    let res = gcd(10, 12);
    println!("Res is {}", res);
}
