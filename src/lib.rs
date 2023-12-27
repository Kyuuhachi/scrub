#![feature(proc_macro_span)]
#![feature(proc_macro_quote)]

/*!
A macro for letting macros absolve themselves of guilt, blaming any and all errors on their callers.

For example, consider a macro that has as its contract that the input must be an expression of a specific type.
If a caller breaks this contract, a part of the macro body is highlighted as the error cause, even though it *obviously* was the caller's fault.

<style type="text/css">
.diag-r { color: var(--code-highlight-prelude-val-color); }
.diag-b { color: var(--code-highlight-prelude-color); }
</style>

<div class="example-wrap"><pre class="language-text"><code><b><span class="diag-r">error[E0308]</span>: mismatched types</b>
<b class="diag-b"> --&gt;</b> examples/blame.rs:3:7
<b class="diag-b">  |</b>
<b class="diag-b">3 |</b>         let () = $e;
<b class="diag-b">  |</b>             <b class="diag-r">^^</b> <b class="diag-r">expected `A`, found `()`</b>
<b class="diag-b">...</b>
<b class="diag-b">9 |</b>     b!(A);
<b class="diag-b">  |</b>     <b class="diag-b">---<b class="diag-b">-</b>-</b>
<b class="diag-b">  |</b>     <b class="diag-b">|</b>  <b class="diag-b">|</b>
<b class="diag-b">  |</b>     <b class="diag-b">|</b>  <b class="diag-b">this expression has type `A`</b>
<b class="diag-b">  |</b>     <b class="diag-b">in this macro invocation</b>
</code></pre></div>

Now you can finally tell those callers whose fault it *really* is.

<div class="example-wrap"><pre class="language-text"><code><b><span class="diag-r">error[E0308]</span>: mismatched types</b>
<b class="diag-b">  --&gt;</b> examples/blameless.rs:11:2
<b class="diag-b">   |</b>
<b class="diag-b">11 |</b>     b!(A);
<b class="diag-b">   |</b>     <b class="diag-r">^^^<b class="diag-b">-</b>^</b>
<b class="diag-b">   |</b>     <b class="diag-r">|</b>  <b class="diag-b">|</b>
<b class="diag-b">   |</b>     <b class="diag-r">|</b>  <b class="diag-b">this expression has type `A`</b>
<b class="diag-b">   |</b>     <b class="diag-r">expected `A`, found `()`</b>
</code></pre></div>

Due to using the `proc_macro_span` feature, this crate requires nightly.
*/

use proc_macro::{Delimiter, Group, Spacing, Span, TokenStream, TokenTree};

/// Hides the contained token tree from diagnostics.
#[proc_macro]
pub fn scrub(body: TokenStream) -> TokenStream {
	let parent = Span::call_site()
		.parent()
		.unwrap()
		.parent()
		.expect("called outside macro");
	map_stream(body, |mut t| {
		if let Some(p) = t.span().parent() {
			t.set_span(p.located_at(parent).resolved_at(t.span()));
		}
		if let TokenTree::Group(g) = t {
			t = map_group(g, scrub)
		}
		t
	})
}

fn map_stream(stream: TokenStream, f: impl FnMut(TokenTree) -> TokenTree) -> TokenStream {
	stream.into_iter().map(f).collect()
}

fn map_group(g: Group, f: impl FnOnce(TokenStream) -> TokenStream) -> TokenTree {
	let mut g2 = Group::new(g.delimiter(), f(g.stream()));
	g2.set_span(g.span());
	TokenTree::Group(g2)
}

/// Scrubs all arms of a macro.
#[proc_macro_attribute]
pub fn scrubbed(attr: TokenStream, body: TokenStream) -> TokenStream {
	assert!(attr.is_empty());
	let mut func: fn(TokenStream) -> TokenStream = scrub_macro_body;
	map_stream(body, |mut t| {
		match t {
			TokenTree::Group(ref g) if g.delimiter() == Delimiter::Parenthesis => {
				func = add_scrub;
			}
			TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => {
				t = map_group(g, func);
			}
			_ => {}
		}
		t
	})
}

fn scrub_macro_body(body: TokenStream) -> TokenStream {
	let mut state = 0;
	map_stream(body, |mut t| {
		match t {
			TokenTree::Punct(ref p) if p.spacing() == Spacing::Joint => state = 1,
			TokenTree::Punct(ref p) if p.as_char() == '>' && state == 1 => state = 2,
			TokenTree::Group(g) if g.delimiter() == Delimiter::Brace && state == 2 => {
				t = map_group(g, add_scrub);
				state = 0;
			}
			_ => state = 0,
		};
		t
	})
}

fn add_scrub(body: TokenStream) -> TokenStream {
	proc_macro::quote! { ::scrub::scrub! { $body } }
}
