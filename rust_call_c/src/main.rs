extern crate libc;

extern {
	fn add_one(x:libc::c_int) -> libc::c_int;
}

fn main() {
	let x = 12;
	let y = unsafe{ add_one(x) };
	print!("x = {}, y = {}\n", x, y);
}
