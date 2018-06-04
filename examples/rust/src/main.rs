pub fn greet(name : &String) {
	println!("Hello {}", name);
}

pub fn main() {
	let name = "Rheinjug";
	greet(&name);
}