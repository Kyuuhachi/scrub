macro_rules! b {
	($e:expr) => { scrub::scrub! {
		let () = $e;
	} };
}

fn main() {
	struct A;
	b!(A);
}
