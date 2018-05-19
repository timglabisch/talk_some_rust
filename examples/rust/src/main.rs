fn main() {
    let a: &[u8] = b"hello, rheinjug!";
    println!("{}", ::std::str::from_utf8(a).expect("i am sure it's valid utf8"));
}