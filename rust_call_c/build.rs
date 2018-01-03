extern crate cc;

fn main() {
	cc::Build::new()
		.file("src/native.c")
		.compile("libnative.a");
}
