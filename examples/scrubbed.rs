#![feature(decl_macro)]

#[scrub::scrubbed]
macro_rules! rules {
	($e:expr) => {
		let () = $e;
	};
}

#[scrub::scrubbed]
macro decl($e:expr) {
	let () = $e;
}

#[scrub::scrubbed]
macro decl2($e:expr) {
	let () = $e;
}

fn main() {
	struct A;
	rules!(A);
	decl!(A);
	decl2!(A);
}
