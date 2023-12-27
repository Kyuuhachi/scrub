macro_rules! b {
	($e:expr) => {
		let () = $e;
	};
}

fn main() {
	struct A;
	b!(A);
}
